use crate::config::Config;
use crate::metrics::Metrics;
use crate::websocket::ConnectionMap;
use std::sync::atomic::AtomicUsize;
use tokio::sync::broadcast::Sender;

pub struct AppState {
    pub counters: Counters,
    pub concurrent_users: AtomicUsize,
    pub total_users: AtomicUsize,
    pub broadcast_tx: Sender<String>,
    pub metrics: Metrics,
    pub config: Config,
    pub connection_map: ConnectionMap,
}

pub struct Counters {
    pub red: AtomicUsize,
    pub green: AtomicUsize,
    pub blue: AtomicUsize,
    pub purple: AtomicUsize,
    pub total: AtomicUsize,
}

impl Default for Counters {
    fn default() -> Self {
        Self {
            red: AtomicUsize::new(0),
            green: AtomicUsize::new(0),
            blue: AtomicUsize::new(0),
            purple: AtomicUsize::new(0),
            total: AtomicUsize::new(0),
        }
    }
}
