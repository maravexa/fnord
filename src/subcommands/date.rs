use chrono::Local;
use serde_json::json;

use crate::cli::DateArgs;
use crate::config::Config;
use crate::date::convert::{parse_date_arg, to_discordian};
use crate::date::types::{ordinal_suffix, DiscordianDate};
use crate::error::FnordError;
use crate::holydays::defaults::builtin_holydays;
use crate::holydays::registry::HolydayRegistry;

pub fn run(args: &DateArgs, config: &Config, json: bool, no_color: bool) -> Result<(), FnordError> {
    let naive_date = match &args.date {
        Some(s) => parse_date_arg(s)?,
        None => Local::now().date_naive(),
    };

    let disc = to_discordian(naive_date);
    let registry = HolydayRegistry::build(builtin_holydays(), vec![], vec![]);
    let holydays = registry.lookup(&disc);

    if json {
        return print_json(&disc, &holydays);
    }

    if let Some(fmt) = &args.format {
        let output = apply_format(fmt, &disc, &holydays);
        println!("{output}");
        return Ok(());
    }

    // Default or --short output
    let date_line = format_date_line(&disc, no_color);
    println!("Today is {date_line}");

    if !args.short {
        if (args.holydays || config.calendar.show_holyday) && !holydays.is_empty() {
            for h in &holydays {
                println!("Holyday: {}", h.name);
                if let Some(desc) = &h.description {
                    println!("  {desc}");
                }
                if let Some(greeting) = &h.greeting {
                    println!("  {greeting}");
                }
            }
        }

        if args.apostle || config.calendar.show_apostle {
            if let Some(apostle) = get_apostle(&disc) {
                println!("Apostle: {apostle}");
            }
        }
    }

    Ok(())
}

fn format_date_line(disc: &DiscordianDate, _no_color: bool) -> String {
    disc.to_string()
}

fn apply_format(fmt: &str, disc: &DiscordianDate, holydays: &[&crate::holydays::types::Holyday]) -> String {
    let holyday_name = holydays.first().map(|h| h.name.as_str()).unwrap_or("");
    let apostle = get_apostle(disc).unwrap_or_default();

    let mut out = String::with_capacity(fmt.len() * 2);
    let mut chars = fmt.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('A') => match disc {
                    DiscordianDate::SeasonDay { weekday, .. } => out.push_str(&weekday.to_string()),
                    DiscordianDate::StTibsDay { .. } => out.push_str("St. Tib's Day"),
                },
                Some('B') => match disc {
                    DiscordianDate::SeasonDay { season, .. } => out.push_str(&season.to_string()),
                    DiscordianDate::StTibsDay { .. } => out.push_str("(none)"),
                },
                Some('d') => match disc {
                    DiscordianDate::SeasonDay { day, .. } => out.push_str(&day.to_string()),
                    DiscordianDate::StTibsDay { .. } => out.push('0'),
                },
                Some('e') => match disc {
                    DiscordianDate::SeasonDay { day, .. } => {
                        let suf = ordinal_suffix(*day);
                        out.push_str(&format!("{day}{suf}"));
                    }
                    DiscordianDate::StTibsDay { .. } => out.push_str("0th"),
                },
                Some('Y') => out.push_str(&disc.year().to_string()),
                Some('H') => out.push_str(holyday_name),
                Some('a') => out.push_str(&apostle),
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('%') => out.push('%'),
                Some(other) => {
                    out.push('%');
                    out.push(other);
                }
                None => out.push('%'),
            }
        } else {
            out.push(c);
        }
    }

    out
}

fn get_apostle(disc: &DiscordianDate) -> Option<String> {
    match disc {
        DiscordianDate::SeasonDay { season, .. } => Some(season.apostle().to_string()),
        DiscordianDate::StTibsDay { .. } => None,
    }
}

fn print_json(disc: &DiscordianDate, holydays: &[&crate::holydays::types::Holyday]) -> Result<(), FnordError> {
    let obj = match disc {
        DiscordianDate::StTibsDay { year } => json!({
            "type": "st_tibs_day",
            "year": year,
            "holyday": holydays.first().map(|h| &h.name),
        }),
        DiscordianDate::SeasonDay {
            year,
            season,
            day,
            weekday,
        } => {
            let suf = ordinal_suffix(*day);
            json!({
                "type": "season_day",
                "year": year,
                "season": season.to_string(),
                "day": day,
                "day_ordinal": format!("{day}{suf}"),
                "weekday": weekday.to_string(),
                "apostle": season.apostle(),
                "holyday": holydays.first().map(|h| &h.name),
                "holydays": holydays.iter().map(|h| &h.name).collect::<Vec<_>>(),
            })
        }
    };

    println!("{}", serde_json::to_string_pretty(&obj).unwrap());
    Ok(())
}
