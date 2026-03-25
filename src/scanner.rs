use crate::{
    banner::{depth_prefix, status_color, status_label},
    models::{LootEntry, ScanCtx},
};
use colored::Colorize;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use tokio::{sync::Mutex, time::{sleep, Duration}};

fn looks_like_dir(url: &str) -> bool {
    let last = url.split('/').filter(|s| !s.is_empty()).last().unwrap_or("");
    url.ends_with('/') || !last.contains('.')
}

fn is_dir_code(code: u16) -> bool {
    matches!(code, 200 | 301 | 302 | 307)
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

pub async fn scan(base_url: String, depth: usize, ctx: Arc<ScanCtx>) {
    if depth > ctx.max_depth {
        return;
    }

    let base_url = base_url.trim_end_matches('/').to_string();

    // Защищаем только директорию — не отдельные слова
    if !ctx.visited.insert(base_url.clone()) {
        return;
    }

    let pb = ctx.mp.add(ProgressBar::new(ctx.words.len() as u64));
    pb.set_style(
        ProgressStyle::with_template(&format!(
            " {{spinner:.yellow}} {} [{{bar:40.yellow/bright_black}}] {{pos}}/{{len}} | {{per_sec}} req/s",
            depth_prefix(depth)
        ))
        .unwrap()
        .progress_chars("█▓░"),
    );

    let found_dirs: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    stream::iter(ctx.words.iter().cloned())
        .map(|word| {
            let ctx        = Arc::clone(&ctx);
            let pb         = pb.clone();
            let base       = base_url.clone();
            let found_dirs = Arc::clone(&found_dirs);

            async move {
                if ctx.rate_limit > 0 {
                    sleep(Duration::from_millis(1000 / ctx.rate_limit)).await;
                }

                let url = if base.contains("ODIN") {
                    base.replace("ODIN", &word)
                } else {
                    format!("{}/{}", base, word)
                };

                let req = match ctx.method.to_uppercase().as_str() {
                    "POST"   => ctx.client.post(&url),
                    "PUT"    => ctx.client.put(&url),
                    "DELETE" => ctx.client.delete(&url),
                    "HEAD"   => ctx.client.head(&url),
                    _        => ctx.client.get(&url),
                };

                let req = match &ctx.body {
                    Some(b) => req.body(b.replace("ODIN", &word)),
                    None    => req,
                };

                match req.send().await {
                    Ok(resp) => {
                        let code = resp.status().as_u16();
                        let size = resp.content_length().unwrap_or(0);

                        if let Some((wc, ws)) = ctx.wildcard_sig {
                            if code == wc && size == ws {
                                pb.inc(1);
                                return;
                            }
                        }

                        if size < ctx.min_size || size > ctx.max_size {
                            pb.inc(1);
                            return;
                        }

                        let body_text  = resp.text().await.unwrap_or_default();
                        let word_count = count_words(&body_text);

                        if !ctx.filter_words.is_empty()
                            && ctx.filter_words.contains(&word_count)
                        {
                            pb.inc(1);
                            return;
                        }

                        pb.inc(1);

                        if ctx.allowed_codes.contains(&code) {
                            pb.suspend(|| {
                                println!(
                                    "  {}  [{}]  {:<12}  {:>8}b  {:>6}w  {}",
                                    depth_prefix(depth).bright_black(),
                                    status_color(code),
                                    status_label(code).bright_white(),
                                    size.to_string().dimmed(),
                                    word_count.to_string().dimmed(),
                                    url.cyan()
                                );
                            });

                            ctx.loot.lock().await.push(LootEntry {
                                url: url.clone(),
                                status: code,
                                size,
                                words: word_count,
                                depth,
                            });

                            if let Some(rc) = &ctx.replay_client {
                                let _ = rc.get(&url).send().await;
                            }

                            if !ctx.no_recurse
                                && depth < ctx.max_depth
                                && is_dir_code(code)
                                && looks_like_dir(&url)
                            {
                                found_dirs.lock().await.push(url);
                            }
                        }
                    }
                    Err(_) => pb.inc(1),
                }
            }
        })
        .buffer_unordered(ctx.warriors)
        .collect::<Vec<_>>()
        .await;

    pb.finish_and_clear();

    if !ctx.no_recurse && depth < ctx.max_depth {
        let dirs = found_dirs.lock().await.clone();
        for dir in dirs.into_iter().take(ctx.max_dirs) {
            println!(
                "\n  {} {}\n",  // ← исправлено \\n → \n
                "🪓 Новый рейд →".bold().yellow(),
                dir.cyan().bold()
            );
            Box::pin(scan(dir, depth + 1, Arc::clone(&ctx))).await;
        }
    }
}
