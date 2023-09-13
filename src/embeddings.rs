use serde::{Deserialize, Serialize};

pub struct CreateEmbeddingsRequestBuilder {
    req: CreateEmbeddingsRequest,
}
impl CreateEmbeddingsRequestBuilder {
    pub fn new(model: impl Into<String>, input: Vec<String>) -> Self {
        Self {
            req: CreateEmbeddingsRequest {
                model: model.into(),
                input,
                user: None,
            },
        }
    }

    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.req.user = Some(user.into());
        self
    }

    pub fn build(self) -> CreateEmbeddingsRequest {
        self.req
    }
}

/// Creates an embedding vector representing the input text.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateEmbeddingsRequest {
    /// ID of the model to use.
    model: String,
    /// Input text to embed. Each input must not exceed the max input tokens for the model (8191 tokens for text-embedding-ada-002) and cannot be an empty string.
    input: Vec<String>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    user: Option<String>,
}

/// Represents an embedding vector returned by embedding endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct EmbeddingObject {
    /// The index of the embedding in the list of embeddings.
    index: u32,
    /// The object type, which is always "embedding".
    object: String,
    /// The embedding vector, which is a list of floats.
    embedding: Vec<f64>,
}
