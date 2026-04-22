//! Next major lunar phase date estimates.

use chrono::{Duration, NaiveDate};

use super::calc::{phase_angle, Body};

/// Upcoming major lunar phases.
pub struct UpcomingPhases {
    pub next_new_moon: NaiveDate,
    pub next_first_quarter: NaiveDate,
    pub next_full_moon: NaiveDate,
    pub next_last_quarter: NaiveDate,
}

/// Upcoming major phases for an arbitrary body.
pub fn upcoming_phases_for_body(body: Body, from_date: NaiveDate) -> UpcomingPhases {
    let angle = phase_angle(body, from_date);
    let period = body.orbital_period();

    // Days until the cycle reaches `target` fraction (strictly after today).
    let next_date = |target: f64| -> NaiveDate {
        let mut delta = target - angle;
        if delta <= 0.0 {
            delta += 1.0;
        }
        let days = (delta * period).round() as i64;
        from_date + Duration::days(days.max(1))
    };

    UpcomingPhases {
        next_new_moon: next_date(0.0),
        next_first_quarter: next_date(0.25),
        next_full_moon: next_date(0.5),
        next_last_quarter: next_date(0.75),
    }
}

/// Upcoming major phases for Earth's moon (Luna).
pub fn upcoming_phases(from_date: NaiveDate) -> UpcomingPhases {
    upcoming_phases_for_body(Body::Luna, from_date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upcoming_phases_all_in_future() {
        let date = NaiveDate::from_ymd_opt(2026, 4, 15).unwrap();
        let u = upcoming_phases(date);
        assert!(u.next_new_moon > date);
        assert!(u.next_first_quarter > date);
        assert!(u.next_full_moon > date);
        assert!(u.next_last_quarter > date);
    }

    #[test]
    fn upcoming_phases_within_one_synodic_month() {
        let date = NaiveDate::from_ymd_opt(2026, 4, 15).unwrap();
        let u = upcoming_phases(date);
        let max_wait = Duration::days(30);
        assert!(u.next_new_moon - date < max_wait);
        assert!(u.next_full_moon - date < max_wait);
    }
}
