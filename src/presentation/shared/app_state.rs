use crate::infrastructure::keyclock_client::KeyclockClient;

pub struct AppState {
    pub keyclock_client: KeyclockClient,
}

impl AppState {
    pub fn new(keyclock_client: KeyclockClient) -> Self {
        Self { keyclock_client }
    }
}
