use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum LlamaCppLogitBiasType {
    input_ids,
    tokens,
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
