//! Phase naming, phase age, and illumination helpers.

use chrono::{Duration, NaiveDate};

use super::calc::{illumination_fraction, Body, PhaseName};

/// Granularity for phase name classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseGranularity {
    /// 4 phases: New, First Quarter, Full, Last Quarter.
    Simple,
    /// 8 phases: New, Waxing Crescent, First Quarter, Waxing Gibbous,
    ///           Full, Waning Gibbous, Last Quarter, Waning Crescent.
    Standard,
}

/// Map a phase fraction in `0.0..1.0` to a named phase using equal-width buckets.
///
/// Fractions outside `[0.0, 1.0)` are wrapped via `rem_euclid`.
pub fn phase_name(fraction: f64, granularity: PhaseGranularity) -> PhaseName {
    let f = fraction.rem_euclid(1.0);
    match granularity {
        PhaseGranularity::Simple => {
            if f < 0.125 || f >= 0.875 {
                PhaseName::NewMoon
            } else if f < 0.375 {
                PhaseName::FirstQuarter
            } else if f < 0.625 {
                PhaseName::FullMoon
            } else {
                PhaseName::LastQuarter
            }
        }
        PhaseGranularity::Standard => {
            if f < 0.0625 || f >= 0.9375 {
                PhaseName::NewMoon
            } else if f < 0.1875 {
                PhaseName::WaxingCrescent
            } else if f < 0.3125 {
                PhaseName::FirstQuarter
            } else if f < 0.4375 {
                PhaseName::WaxingGibbous
            } else if f < 0.5625 {
                PhaseName::FullMoon
            } else if f < 0.6875 {
                PhaseName::WaningGibbous
            } else if f < 0.8125 {
                PhaseName::LastQuarter
            } else {
                PhaseName::WaningCrescent
            }
        }
    }
}

/// Illumination percentage (0.0–100.0) from a phase fraction.
pub fn illumination_percent(fraction: f64) -> f64 {
    illumination_fraction(fraction) * 100.0
}

/// Phase age: position within the current synodic cycle.
pub struct PhaseAge {
    /// Days since the last new moon (0.0 to < synodic period).
    pub days: f64,
    /// Fraction of the synodic month completed (0.0 to 1.0).
    pub fraction: f64,
    /// Approximate date of the most recent new moon.
    pub last_new_moon: NaiveDate,
    /// Approximate date of the next new moon.
    pub next_new_moon: NaiveDate,
}

/// Compute phase age relative to an arbitrary epoch and synodic period.
///
/// Works for any body; does not account for retrograde motion (the fraction
/// returned matches the forward orbital cycle, not the inverted display angle).
pub fn phase_age_with_period(
    date: NaiveDate,
    synodic_period: f64,
    epoch: NaiveDate,
) -> PhaseAge {
    let days_since = (date - epoch).num_days() as f64;
    let cycles = days_since / synodic_period;
    let fraction = cycles.rem_euclid(1.0);
    let days = fraction * synodic_period;

    let last_cycle = cycles.floor();
    let last_new_moon =
        epoch + Duration::days((last_cycle * synodic_period).round() as i64);
    let next_new_moon =
        epoch + Duration::days(((last_cycle + 1.0) * synodic_period).round() as i64);

    PhaseAge {
        days,
        fraction,
        last_new_moon,
        next_new_moon,
    }
}

/// Compute phase age for a specific body.
pub fn phase_age_for_body(body: Body, date: NaiveDate) -> PhaseAge {
    phase_age_with_period(date, body.orbital_period(), body.reference_new_moon())
}

/// Compute phase age for Earth's moon (Luna).
pub fn phase_age(date: NaiveDate) -> PhaseAge {
    phase_age_for_body(Body::Luna, date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_age_new_moon_epoch() {
        let date = NaiveDate::from_ymd_opt(2000, 1, 6).unwrap();
        let age = phase_age(date);
        assert!(age.days < 0.5, "epoch should be near new moon, got {}", age.days);
    }

    #[test]
    fn phase_age_full_moon() {
        let date = NaiveDate::from_ymd_opt(2000, 1, 21).unwrap();
        let age = phase_age(date);
        assert!(
            age.days > 14.0 && age.days < 16.0,
            "should be near full moon, got {}",
            age.days
        );
    }

    #[test]
    fn phase_age_fraction_in_range() {
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        for i in 0..100 {
            let date = start + Duration::days(i * 7);
            let age = phase_age(date);
            assert!(
                age.fraction >= 0.0 && age.fraction < 1.0,
                "fraction out of range: {}",
                age.fraction
            );
            assert!(
                age.days >= 0.0 && age.days < 30.0,
                "days out of range: {}",
                age.days
            );
        }
    }

    #[test]
    fn phase_name_standard_boundaries() {
        assert_eq!(phase_name(0.0, PhaseGranularity::Standard), PhaseName::NewMoon);
        assert_eq!(phase_name(0.125, PhaseGranularity::Standard), PhaseName::WaxingCrescent);
        assert_eq!(phase_name(0.25, PhaseGranularity::Standard), PhaseName::FirstQuarter);
        assert_eq!(phase_name(0.375, PhaseGranularity::Standard), PhaseName::WaxingGibbous);
        assert_eq!(phase_name(0.5, PhaseGranularity::Standard), PhaseName::FullMoon);
        assert_eq!(phase_name(0.625, PhaseGranularity::Standard), PhaseName::WaningGibbous);
        assert_eq!(phase_name(0.75, PhaseGranularity::Standard), PhaseName::LastQuarter);
        assert_eq!(phase_name(0.875, PhaseGranularity::Standard), PhaseName::WaningCrescent);
    }

    #[test]
    fn phase_name_wraps() {
        assert_eq!(phase_name(1.0, PhaseGranularity::Standard), PhaseName::NewMoon);
        assert_eq!(phase_name(1.5, PhaseGranularity::Standard), PhaseName::FullMoon);
        assert_eq!(phase_name(-0.25, PhaseGranularity::Standard), PhaseName::LastQuarter);
    }

    #[test]
    fn phase_name_simple_boundaries() {
        assert_eq!(phase_name(0.0, PhaseGranularity::Simple), PhaseName::NewMoon);
        assert_eq!(phase_name(0.25, PhaseGranularity::Simple), PhaseName::FirstQuarter);
        assert_eq!(phase_name(0.5, PhaseGranularity::Simple), PhaseName::FullMoon);
        assert_eq!(phase_name(0.75, PhaseGranularity::Simple), PhaseName::LastQuarter);
    }

    #[test]
    fn illumination_percent_range() {
        assert!(illumination_percent(0.0) < 1.0);
        assert!((illumination_percent(0.5) - 100.0).abs() < 0.01);
    }
}
