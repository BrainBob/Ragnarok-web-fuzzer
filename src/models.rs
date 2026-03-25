use dashmap::DashSet;
use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use indicatif::MultiProgress;

#[derive(Serialize, Clone)]
pub struct LootEntry {
    pub url:    String,
    pub status: u16,
    pub size:   u64,
    pub words:  usize,
    pub depth:  usize,
}

pub struct ScanCtx {
    pub client:        Arc<Client>,
    pub replay_client: Option<Arc<Client>>,
    pub words:         Arc<Vec<String>>,
    pub allowed_codes: Arc<Vec<u16>>,
    pub filter_words:  Arc<Vec<usize>>,
    pub visited:       Arc<DashSet<String>>,
    pub loot:          Arc<Mutex<Vec<LootEntry>>>,
    pub mp:            Arc<MultiProgress>,
    pub warriors:      usize,
    pub min_size:      u64,
    pub max_size:      u64,
    pub no_recurse:    bool,
    pub max_depth:     usize,
    pub max_dirs:      usize,    
    pub method:        String,
    pub body:          Option<String>,
    pub rate_limit:    u64,
    pub wildcard_sig:  Option<(u16, u64)>,
}
