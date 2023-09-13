use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum LlamaCppLogitBiasType {
    InputIds,
    Tokens,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
    /// Number of tokens in the prompt.
    prompt_tokens: u32,
    /// Number of tokens in the generated completion.
    completion_tokens: u32,
    /// Total number of tokens used in the request (prompt + completion).
    total_tokens: u32,
}
