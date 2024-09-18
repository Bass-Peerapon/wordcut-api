use serde::Deserialize;

#[derive(Deserialize)]
pub struct WordcutRequest {
    pub text: String,
}
