use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum LlamaCppLogitBiasType {
    InputIds,
    Tokens,
}
