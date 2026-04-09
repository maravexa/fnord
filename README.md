# fnord

A Discordian calendar and chaos utility — spiritual successor to `ddate`.

**All Hail Discordia.**

---

## Installation

```bash
cargo install fnord
```

> Note: not yet published to crates.io. Install from source:
> ```bash
> git clone https://github.com/maravexa/fnord
> cd fnord
> cargo install --path .
> ```

## Quick Start

```bash
# Display today's Discordian date
fnord date

# Display the current season's calendar
fnord cal

# Discover your papal credentials
fnord pope
```

Example output:

```
Today is Prickle-Prickle, the 16th of Chaos, in the YOLD 3191
Apostle: Hung Mung
```

---

## Subcommands

| Command      | Description                                               |
|--------------|-----------------------------------------------------------|
| `date`       | Display today's (or a given) Discordian date              |
| `cal`        | Display a Discordian season calendar                      |
| `holyday`    | Look up or list holydays                                  |
| `moon`       | Show the current moon phase                               |
| `omens`      | Read the omens (weather + chaos report)                   |
| `fortune`    | Dispense a Discordian fortune                             |
| `log`        | Write in the Discordian grimoire                          |
| `wake`       | Set an erisian alarm or reminder                          |
| `pope`       | Display your Discordian papal credentials                 |
| `pineal`     | Consult the pineal gland oracle                           |
| `oracle`     | Ask the oracle a question                                 |
| `fnord`      | Apply fnord redaction to text                             |
| `hotdog`     | Determine if a hotdog is a sandwich                       |
| `cabbage`    | Dispense a head of cabbage                                |
| `chaos`      | Invoke chaos                                              |
| `law`        | Consult the law (Principia Discordia)                     |
| `pentabarf`  | Display the Five Commandments of Discordia                |
| `erisian`    | Erisian utilities and miscellany                          |
| `koan`       | Dispense a Zen koan (Discordian edition)                  |
| `zodiac`     | Display your zodiac sign                                  |

---

## `fnord date` usage

```bash
fnord date                          # today's date
fnord date --date 2025-01-05        # specific date (ISO 8601)
fnord date --date tomorrow          # relative: today, yesterday, tomorrow
fnord date --date +7                # 7 days from now
fnord date --short                  # date line only
fnord date --apostle                # include apostle name
fnord date --holydays               # include holyday info
fnord date --format "%A, %e of %B, YOLD %Y"   # custom format
fnord date --json                   # JSON output
```

### Format tokens

| Token | Meaning                          |
|-------|----------------------------------|
| `%A`  | Weekday name                     |
| `%B`  | Season name                      |
| `%d`  | Day of season (numeric)          |
| `%e`  | Day of season (ordinal: 1st, …)  |
| `%Y`  | YOLD year                        |
| `%H`  | Holyday name (empty if none)     |
| `%a`  | Apostle name                     |
| `%n`  | Newline                          |
| `%t`  | Tab                              |

---

## `fnord cal` usage

```bash
fnord cal                           # current season
fnord cal --season discord          # specific season
fnord cal --year 3191               # specific year
fnord cal --all                     # all 5 seasons
fnord cal --no-color                # disable ANSI colors
```

---

## Config file

Config is read from (in order, later layers override earlier):

1. `/etc/eris/fnord.toml` — system-wide
2. `~/.config/eris/fnord.toml` — user config
3. `$FNORD_CONFIG` — env var path override
4. `--config <file>` — CLI override

Example `~/.config/eris/fnord.toml`:

```toml
[identity]
pope_title = "Pope Awesome McSomeone I"
sect_name = "The Wholly Confused"
cabal = "POEE"

[calendar]
show_apostle = true
show_holyday = true

[output]
color = "auto"
```

---

## Holyday file format

Personal holydays go in a TOML file referenced by `calendar.holyday_files`:

```toml
[[holyday]]
name = "The Incident"
date = "chaos-15"
description = "We do not speak of it"
recurring = true

[[holyday]]
name = "The One-Time Thing"
date = "discord-5"
year = 3191
recurring = false
```

Valid `date` values: `st-tibs`, `chaos-5`, `discord-50`, `confusion-23`, etc.

---

## License

WTFPL — see [LICENSE](LICENSE).

*A Discordian calendar and chaos utility. All Hail Eris. Contains 5 tons of flax.*
