pub mod ascii;
pub mod calc;
pub mod info;
pub mod phase;
pub mod upcoming;

// Flat re-exports for convenient use by callers.
#[allow(unused_imports)]
pub use ascii::{ascii_moon, ascii_moon_default, ascii_moon_unicode, moon_status_line};
#[allow(unused_imports)]
pub use calc::{
    days_to_full, days_to_new, illumination_fraction, phase_angle, phase_name_for_angle, Body,
    PhaseName,
};
#[allow(unused_imports)]
pub use info::{lunar_info, lunar_info_for_body, LunarInfo};
#[allow(unused_imports)]
pub use phase::{
    illumination_percent, phase_age, phase_age_for_body, phase_age_with_period, phase_name,
    PhaseAge, PhaseGranularity,
};
#[allow(unused_imports)]
pub use upcoming::{upcoming_phases, upcoming_phases_for_body, UpcomingPhases};
