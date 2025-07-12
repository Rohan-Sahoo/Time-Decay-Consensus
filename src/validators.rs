#[derive(Debug, Clone)]
pub struct Validator {
    pub id: String,
    pub is_trusted: bool,
}

impl Validator {
    pub fn new(id: String, is_trusted: bool) -> Self {
        Validator { id, is_trusted }
    }

    pub fn verify_signature(&self, signature: &str) -> bool {
        signature == "signed_data_placeholder"
    }
}
