//! Convenience aggregate: everything about a lunar phase in one call.

use chrono::NaiveDate;

use super::calc::{illumination_fraction, phase_angle, Body, PhaseName};
use super::phase::{phase_age_for_body, phase_name, PhaseAge, PhaseGranularity};
use super::upcoming::{upcoming_phases_for_body, UpcomingPhases};

/// Complete lunar phase information for a given date and body.
pub struct LunarInfo {
    /// Phase fraction (0.0 = new, 0.5 = full, 1.0 = new).
    pub fraction: f64,
    /// Illumination percentage (0.0–100.0).
    pub illumination: f64,
    /// Phase name (8-phase Standard granularity).
    pub name: PhaseName,
    /// Phase age: days since last new moon plus date estimates.
    pub age: PhaseAge,
    /// Next occurrences of all four major phases.
    pub upcoming: UpcomingPhases,
}

/// Complete lunar information for a specific body on a given date.
///
/// This is the primary entry point for consumers that need everything at once.
pub fn lunar_info_for_body(body: Body, date: NaiveDate) -> LunarInfo {
    let fraction = phase_angle(body, date);
    let illumination = illumination_fraction(fraction) * 100.0;
    let name = phase_name(fraction, PhaseGranularity::Standard);
    let age = phase_age_for_body(body, date);
    let upcoming = upcoming_phases_for_body(body, date);

    LunarInfo {
        fraction,
        illumination,
        name,
        age,
        upcoming,
    }
}

/// Complete lunar information for Earth's moon on a given date.
pub fn lunar_info(date: NaiveDate) -> LunarInfo {
    lunar_info_for_body(Body::Luna, date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lunar_info_fraction_matches_phase_angle() {
        let date = NaiveDate::from_ymd_opt(2026, 4, 22).unwrap();
        let info = lunar_info(date);
        let direct = phase_angle(Body::Luna, date);
        assert!((info.fraction - direct).abs() < 1e-12);
    }

    #[test]
    fn lunar_info_illumination_in_range() {
        let date = NaiveDate::from_ymd_opt(2026, 4, 22).unwrap();
        let info = lunar_info(date);
        assert!(info.illumination >= 0.0 && info.illumination <= 100.0);
    }

    #[test]
    fn lunar_info_upcoming_all_future() {
        let date = NaiveDate::from_ymd_opt(2026, 4, 22).unwrap();
        let info = lunar_info(date);
        assert!(info.upcoming.next_new_moon > date);
        assert!(info.upcoming.next_full_moon > date);
    }
}
