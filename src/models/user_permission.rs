use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Debug, Deserialize)]
pub enum Permission {
    USER,
    ADMIN
}

impl Permission {
    pub fn from_str(str: &str) -> Option<Self> {
        match str.to_lowercase().as_str() {
            "user" => Some(Self::USER),
            "admin" => Some(Self::ADMIN),
            _ => None,
        }
    }
}

