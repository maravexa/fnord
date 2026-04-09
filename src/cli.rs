use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "fnord",
    about = "A Discordian calendar and chaos utility",
    long_about = "fnord — All Hail Discordia!\n\nA spiritual successor to ddate with a full suite of Discordian-themed subcommands.",
    version
)]
pub struct Cli {
    /// Path to config file (overrides default locations)
    #[arg(long, global = true, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Output JSON instead of human-readable text
    #[arg(long, global = true)]
    pub json: bool,

    /// Disable color output
    #[arg(long = "no-color", global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Display today's (or a given) Discordian date
    Date(DateArgs),

    /// Display a Discordian season calendar
    Cal(CalArgs),

    /// Look up or list holydays
    Holyday(HolydayArgs),

    /// Show the current moon phase
    Moon(StubArgs),

    /// Read the omens (weather + chaos report)
    Omens(StubArgs),

    /// Dispense a Discordian fortune
    Fortune(StubArgs),

    /// Write in the Discordian grimoire (log)
    Log(StubArgs),

    /// Set an erisian alarm or reminder
    Wake(StubArgs),

    /// Display your Discordian papal credentials
    Pope(StubArgs),

    /// Consult the pineal gland oracle
    Pineal(StubArgs),

    /// Ask the oracle a question
    Oracle(StubArgs),

    /// Apply fnord redaction to text
    Fnord(StubArgs),

    /// Determine if a hotdog is a sandwich
    Hotdog(StubArgs),

    /// Dispense a head of cabbage
    Cabbage(StubArgs),

    /// Invoke chaos
    Chaos(StubArgs),

    /// Consult the law (Principia Discordia)
    Law(StubArgs),

    /// Display the Five Commandments of Discordia
    Pentabarf(StubArgs),

    /// Erisian utilities and miscellany
    Erisian(StubArgs),

    /// Dispense a Zen koan (Discordian edition)
    Koan(StubArgs),

    /// Display your zodiac sign
    Zodiac(StubArgs),
}

#[derive(Args, Debug, Default)]
pub struct DateArgs {
    /// Date to display (today, yesterday, tomorrow, YYYY-MM-DD, +N, -N)
    #[arg(long, short = 'd', value_name = "DATE")]
    pub date: Option<String>,

    /// Show only the date line (no holyday info)
    #[arg(long, short = 's')]
    pub short: bool,

    /// Include apostle information
    #[arg(long, short = 'a')]
    pub apostle: bool,

    /// Include holyday information if applicable
    #[arg(long, short = 'H')]
    pub holydays: bool,

    /// Custom format string (%A weekday, %B season, %d day, %e ordinal day, %Y year, %H holyday, %a apostle, %n newline, %t tab)
    #[arg(long, short = 'f', value_name = "FORMAT")]
    pub format: Option<String>,
}

#[derive(Args, Debug, Default)]
pub struct CalArgs {
    /// Season to display (chaos, discord, confusion, bureaucracy, aftermath)
    #[arg(long, short = 's', value_name = "SEASON")]
    pub season: Option<String>,

    /// Year (YOLD) to display
    #[arg(long, short = 'y', value_name = "YEAR")]
    pub year: Option<i32>,

    /// Display all 5 seasons
    #[arg(long, short = 'a')]
    pub all: bool,

    /// Terminal width for responsive layout
    #[arg(long, short = 'w', value_name = "WIDTH")]
    pub width: Option<usize>,
}

#[derive(Args, Debug, Default)]
pub struct HolydayArgs {
    /// Look up a specific holyday key (e.g. chaos-5, discord-50, st-tibs)
    #[arg(value_name = "KEY")]
    pub key: Option<String>,

    /// List all known holydays
    #[arg(long, short = 'l')]
    pub list: bool,
}

/// Stub args for unimplemented subcommands
#[derive(Args, Debug, Default)]
pub struct StubArgs {}
