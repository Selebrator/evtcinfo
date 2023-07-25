use chrono::{Duration, NaiveDateTime};
use evtclib::{Compression, Log, Outcome};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let log: Log = evtclib::process_file(file, Compression::Zip)?;
    let date = log
        .local_end_timestamp()
        .and_then(|unix_timestamp| NaiveDateTime::from_timestamp_opt(unix_timestamp.into(), 0));
    let encounter = log
        .encounter()
        .map(|e| e.to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let outcome = log
        .analyzer()
        .and_then(|a| a.outcome())
        .map(|o| match o {
            Outcome::Success => "kill",
            Outcome::Failure => "wipe",
        })
        .unwrap_or("crash");
    let duration =
        humantime::Duration::from(Duration::milliseconds(log.span() as i64).to_std().unwrap());
    println!(
        "{} {} after {} on {}",
        encounter,
        outcome,
        duration,
        date.map_or_else(
            || "invalid date".to_string(),
            |t| format!("{}", t.format("%a %e %b %Y, %H:%M"))
        )
    );

    for player in log.players() {
        println!(
            "  {:2} {:22} {:19} {:12}",
            player.subgroup(),
            player.account_name(),
            player.character_name(),
            player
                .elite()
                .map(|e| e.to_string())
                .unwrap_or_else(|| player.profession().to_string()),
        )
    }
    Ok(())
}
