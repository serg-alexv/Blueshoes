use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionState {
    Start,
    Validate,
    Commit,
    Rollback,
    Abort,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEvent {
    pub timestamp_utc: u64,
    pub event_type: String, // "transaction"
    pub transaction_id: String,
    pub state: TransactionState,
    pub intent: Option<String>,
    pub dry_run_plan: Option<Vec<String>>,
    pub mutation_performed: bool,
}

impl TransactionEvent {
    pub fn new(
        transaction_id: String,
        state: TransactionState,
        intent: Option<String>,
        dry_run_plan: Option<Vec<String>>,
    ) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Self {
            timestamp_utc: since_the_epoch.as_secs(),
            event_type: "transaction".to_string(),
            transaction_id,
            state,
            intent,
            dry_run_plan,
            // STRICT M2 CONSTRAINT: Always false until mutation milestones
            mutation_performed: false,
        }
    }
}
