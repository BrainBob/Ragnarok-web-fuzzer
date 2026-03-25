use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn detect(client: &Client, base_url: &str) -> Option<(u16, u64)> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let fake = format!(
        "{}/ragnarok_wc_{:x}",
        base_url.trim_end_matches('/'),
        ts
    );
    if let Ok(resp) = client.get(&fake).send().await {
        let code = resp.status().as_u16();
        let size = resp.content_length().unwrap_or(0);
        if code != 404 {
            return Some((code, size));
        }
    }
    None
}
