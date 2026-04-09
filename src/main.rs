mod cli;
mod config;
mod date;
mod error;
mod holydays;
mod moon;
mod subcommands;
mod wake;
mod zodiac;

use clap::Parser;
use cli::{Cli, Command};
use config::load_config;

fn main() {
    let cli = Cli::parse();

    let config = match load_config(cli.config.as_ref()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("fnord: config error: {e}");
            std::process::exit(1);
        }
    };

    let no_color = cli.no_color;
    let no_unicode = cli.no_unicode;
    let json = cli.json;

    let result = match cli.command.unwrap_or(Command::Date(cli::DateArgs::default())) {
        Command::Date(args) => subcommands::date::run(&args, &config, json, no_color),
        Command::Cal(args) => subcommands::cal::run(&args, &config, no_color),
        Command::Holyday(args) => run_holyday(&args, &config, json),
        Command::Pope(args) => subcommands::pope::run(&args, &config, json, no_color, no_unicode),
        Command::Oracle(args) => subcommands::oracle::run(&args, &config, json, no_color, no_unicode),
        Command::Fortune(args) => subcommands::fortune::run(&args, &config, json, no_color, no_unicode),
        Command::Koan(args) => subcommands::koan::run(&args, &config, json, no_color, no_unicode),
        Command::Moon(args) => subcommands::moon::run(&args, &config, json, no_color, no_unicode),
        Command::Zodiac(args) => subcommands::zodiac::run(&args, &config, json, no_color, no_unicode),
        Command::Omens(args) => subcommands::omens::run(&args, &config, json, no_color, no_unicode),
        Command::Log(args) => subcommands::log::run(&args, &config, json, no_color),
        Command::Wake(args) => subcommands::wake::run(&args, &config, json, no_color, no_unicode),
        Command::Pineal(args) => subcommands::pineal::run(&args, &config, json, no_color, no_unicode),
        cmd => run_stub(cmd),
    };

    if let Err(e) = result {
        eprintln!("fnord: {e}");
        std::process::exit(1);
    }
}

fn run_holyday(args: &cli::HolydayArgs, _config: &config::Config, _json: bool) -> Result<(), error::FnordError> {
    use holydays::defaults::builtin_holydays;
    use holydays::registry::HolydayRegistry;
    use holydays::types::HolydayKey;

    let registry = HolydayRegistry::build(builtin_holydays(), vec![], vec![]);

    if args.list {
        let all = builtin_holydays();
        for h in &all {
            let key_str = match &h.key {
                HolydayKey::StTibs => "st-tibs".to_string(),
                HolydayKey::SeasonDay { season, day } => {
                    format!("{}-{}", season.to_string().to_lowercase(), day)
                }
            };
            println!("{}: {}", key_str, h.name);
        }
        return Ok(());
    }

    if let Some(key_str) = &args.key {
        let key = HolydayKey::from_str(key_str)?;
        let fake_date = match &key {
            HolydayKey::StTibs => date::types::DiscordianDate::StTibsDay { year: 0 },
            HolydayKey::SeasonDay { season, day } => date::types::DiscordianDate::SeasonDay {
                year: 0,
                season: *season,
                day: *day,
                weekday: date::types::Weekday::from_day_of_season(*day),
            },
        };
        let results = registry.lookup(&fake_date);
        if results.is_empty() {
            println!("No holyday found for key: {key_str}");
        } else {
            for h in results {
                println!("Name: {}", h.name);
                if let Some(d) = &h.description {
                    println!("Description: {d}");
                }
                if let Some(g) = &h.greeting {
                    println!("Greeting: {g}");
                }
            }
        }
        return Ok(());
    }

    println!("fnord holyday: use --list to see all holydays, or provide a key argument");
    Ok(())
}

fn run_stub(cmd: Command) -> Result<(), error::FnordError> {
    let name = match &cmd {
        Command::Fnord(_) => "fnord",
        Command::Hotdog(_) => "hotdog",
        Command::Cabbage(_) => "cabbage",
        Command::Chaos(_) => "chaos",
        Command::Law(_) => "law",
        Command::Pentabarf(_) => "pentabarf",
        Command::Erisian(_) => "erisian",
        _ => "unknown",
    };
    println!("fnord {name}: coming soon — all hail discordia");
    Ok(())
}
