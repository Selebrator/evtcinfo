use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path: String = std::env::args()
        .nth(1)
        .ok_or("First (and only) argument must be the path to a zevtc file")?;
    let log = evtclib::process_file(file_path, evtclib::Compression::Zip)?;
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    match log.encounter() {
        Some(encounter) => write!(out, "{encounter} ")?,
        None => write!(out, "encounter {} ", log.encounter_id())?,
    }

    let outcome = match log.analyzer().and_then(|a| a.outcome()) {
        Some(evtclib::Outcome::Success) => "kill",
        Some(evtclib::Outcome::Failure) => "wipe",
        None => "crash",
    };
    write!(out, "{outcome}")?;

    let duration = humantime::Duration::from(std::time::Duration::from_millis(log.span()));
    write!(out, " after {duration} ")?;

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
    out.flush()?;
    Ok(())
}
