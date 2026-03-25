mod args;
mod banner;
mod client;
mod models;
mod output;
mod scanner;
mod wildcard;

use args::Args;
use clap::Parser;
use colored::Colorize;
use dashmap::DashSet;
use indicatif::MultiProgress;
use models::ScanCtx;
use std::{fs, sync::Arc, time::Instant};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    banner::print_banner();

    let allowed_codes: Vec<u16> = args.valhalla
        .split(',').filter_map(|s| s.trim().parse().ok()).collect();

    let filter_words: Vec<usize> = args.filter_words
        .split(',').filter_map(|s| s.trim().parse().ok()).collect();

    let extensions: Vec<String> = if args.runes.is_empty() {
        vec!["".to_string()]
    } else {
        let mut e: Vec<String> = args.runes
            .split(',').map(|e| format!(".{}", e.trim())).collect();
        e.insert(0, "".to_string());
        e
    };

    let content = fs::read_to_string(&args.wordlist)?;
    let words = Arc::new(
        content.lines()
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .flat_map(|w| extensions.iter().map(|e| format!("{}{}", w, e)).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let headers     = client::build_headers(&args.headers)?;
    let main_client = Arc::new(client::build_client(
        args.timeout, &args.shield,
        args.proxy.as_deref(), args.cookies.as_deref(),
        headers.clone(),
    )?);
    let replay_client = match &args.replay_proxy {
        Some(rp) => Some(Arc::new(client::build_client(
            args.timeout, &args.shield, Some(rp), None, headers,
        )?)),
        None => None,
    };

    let wildcard_sig = if args.wildcard {
        let sig = wildcard::detect(&main_client, &args.url).await;
        if let Some((c, s)) = sig {
            println!("  ⚠️  Wildcard: код={} размер={}b — фильтруется\n", c, s);
        }
        sig
    } else { None };

    // ↓ все \n исправлены
    println!("  {} {}", "🏹 Цель:".bold().cyan(),    args.url.yellow());
    println!("  {} {}", "📜 Свиток:".bold().cyan(),  args.wordlist.yellow());
    println!("  {} {}", "⚔  Воины:".bold().cyan(),   args.warriors.to_string().green());
    println!("  {} {}", "🌿 Глубина:".bold().cyan(), args.depth.to_string().green());
    println!("  {} {}", "📖 Рун:".bold().cyan(),      words.len().to_string().green());
    println!("  {} {}", "🌐 Метод:".bold().cyan(),    args.method.yellow());
    println!();
    println!("{}", "━".repeat(72).bright_black());

    let ctx = Arc::new(ScanCtx {
        client:        main_client,
        replay_client,
        words,
        allowed_codes: Arc::new(allowed_codes),
        filter_words:  Arc::new(filter_words),
        visited:       Arc::new(DashSet::new()),
        loot:          Arc::new(Mutex::new(vec![])),
        mp:            Arc::new(MultiProgress::new()),
        warriors:      args.warriors,
        min_size:      args.min_size,
        max_size:      args.max_size,
        no_recurse:    args.no_recurse,
        max_depth:     args.depth,
        max_dirs:      args.max_dirs,
        method:        args.method.clone(),
        body:          args.body.clone(),
        rate_limit:    args.rate_limit,
        wildcard_sig,
    });

    let start = Instant::now();
    scanner::scan(args.url.clone(), 0, Arc::clone(&ctx)).await;

    println!("{}", "━".repeat(72).bright_black());

    let loot = ctx.loot.lock().await;
    println!(
        "\n  {} {} путей | ⏱ {:.1}s\n",
        "🪓 Добыча:".bold().yellow(),
        loot.len().to_string().green().bold(),
        start.elapsed().as_secs_f64()
    );

    if let Some(p) = &args.loot { output::save_text(&loot, p)?; println!("  💾 → {}", p.yellow()); }
    if let Some(p) = &args.json { output::save_json(&loot, p)?; println!("  📋 → {}", p.yellow()); }

    println!("\n  {}\n", "⚡ Один доволен — Рейд завершён!".bold().bright_yellow());
    Ok(())
}
