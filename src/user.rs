use crate::RivaToken;

pub struct RivaUser {
    name: String,
    token: RivaToken,
}

impl RivaUser {
    pub fn new(name: String, token: RivaToken) -> Self {
        Self {
            name,
            token,
        }
    }
}
