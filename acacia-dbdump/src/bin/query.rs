// query specific attrs: gid 22 for prologue chain + gid 4 full dump
use acacia_database::db::{db, init_db};
use acacia_database::repositories::player::PlayerRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "D:/test/cbjq-re/work/dbcopy".into());
    let pid: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10001);
    let addr = format!("surrealkv://{path}");
    init_db(addr.as_str()).await?;
    let _ = db();

    let repo = PlayerRepository::new(pid);
    let attrs = repo.attrs.find_all().await?;

    let get = |gid: u32, sid: u32| -> u32 {
        let key = (gid << 16) | sid;
        attrs.get(&key).copied().unwrap_or(0)
    };

    let prologue = [10121u32, 10122, 10123, 10124, 50171, 50172, 50173, 50174];
    println!("--- gid 22 (pass count) prologue chain ---");
    for id in prologue {
        println!("  22.{id} = {}", get(22, id));
    }
    println!("--- gid 21 (stars) prologue chain ---");
    for id in prologue {
        println!("  21.{id} = {}", get(21, id));
    }
    println!("--- gid 187 (beginner) ---");
    println!("  187.1 = {}", get(187, 1));

    println!("--- gid 4 (all sids) ---");
    let mut v: Vec<(u32, u32)> = Vec::new();
    for (&k, &val) in attrs.iter() {
        if (k >> 16) == 4 {
            v.push(((k & 0xffff) as u32, val));
        }
    }
    v.sort();
    for (sid, val) in v {
        println!("  4.{sid} = {val}");
    }
    Ok(())
}
