use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path: String = std::env::args()
        .nth(1)
        .ok_or("First (and only) argument must be the path to a zevtc file")?;
    let compression = if file_path.ends_with(".zip") || file_path.ends_with(".zevtc") {
        evtclib::Compression::Zip
    } else {
        evtclib::Compression::None
    };
    let log = evtclib::process_file(file_path, compression)?;
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    match log.encounter() {
        Some(encounter) => write!(out, "{encounter} ")?,
        None => write!(out, "encounter {} ", log.encounter_id())?,
    }

    if let Some(analyzer) = log.analyzer() {
        if analyzer.is_cm() {
            write!(out, "CM ")?;
        }
        let outcome = match analyzer.outcome() {
            Some(evtclib::Outcome::Success) => "kill after ",
            Some(evtclib::Outcome::Failure) => "wipe after ",
            None => "fighting for ",
        };
        write!(out, "{outcome}")?;
    }

    let duration = humantime::Duration::from(std::time::Duration::from_millis(log.span()));
    write!(out, "{duration} ")?;

    if let Some(timestamp) = log.local_end_timestamp().and_then(|unix_timestamp| {
        chrono::NaiveDateTime::from_timestamp_opt(unix_timestamp.into(), 0)
    }) {
        writeln!(out, "on {}", timestamp.format("%a %e %b %Y, %H:%M"))?;
    }

    for player in log.players() {
        write!(
            out,
            "  {subgroup:2} {account:22} {character:19}",
            subgroup = player.subgroup(),
            account = player.account_name(),
            character = player.character_name(),
        )?;
        match player.elite() {
            Some(elite) => writeln!(out, " {elite:12}"),
            None => writeln!(out, " {:12}", player.profession()),
        }?;
    }

    if log.errors().len() > 0 {
        writeln!(out)?;
        writeln!(out, "Errors in log")?;
        for error in log.errors() {
            writeln!(out, "  {error}")?;
        }
    }
    out.flush()?;
    Ok(())
}
