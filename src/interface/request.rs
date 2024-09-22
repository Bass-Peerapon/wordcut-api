use serde::Deserialize;

#[derive(Deserialize)]
pub struct WordcutRequest {
    pub text: String,
}

#[derive(Deserialize)]
pub struct WordRequest {
    pub text: String,
}
