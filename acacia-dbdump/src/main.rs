// Offline inspector: open a COPY of the acacia surrealkv database and dump
// player attrs / info to stdout. Run with the game server stopped, or against
// a copied database directory.
//
// usage: acacia-dbdump <surrealkv-path> [pid]
use acacia_commons::bits::get_attribute;
use acacia_database::db::{db, init_db};
use acacia_database::repositories::player::PlayerRepository;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "surrealkv://data/database".into());
    let pid: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10001);
    // path without scheme -> prefix surrealkv://
    let addr = if path.contains("://") { path } else { format!("surrealkv://{path}") };
    init_db(addr.as_str()).await?;
    let _ = db();

    let repo = PlayerRepository::new(pid);
    let info = repo.info.find_one().await?;
    match info {
        Some(p) => {
            println!("player {pid}: level={} name={} create_time={}", p.level, p.name, p.create_time);
        }
        None => println!("player {pid}: NOT FOUND"),
    }

    let attrs = repo.attrs.find_all().await?;
    let mut grouped: BTreeMap<u32, Vec<(u32, u32)>> = BTreeMap::new();
    for (key, value) in attrs {
        let (gid, sid) = get_attribute(key as u64);
        grouped.entry(gid as u32).or_default().push((sid as u32, value));
    }
    for (gid, mut rows) in grouped {
        rows.sort();
        println!("== gid {gid} ({} entries)", rows.len());
        for (sid, val) in rows.iter().take(40) {
            println!("   sid {sid} = {val}");
        }
        if rows.len() > 40 {
            println!("   ... {} more", rows.len() - 40);
        }
    }
    Ok(())
}
