use bounce::prelude::*;

use shared_types::types::UserInfo;

/// Authentication Credential information
#[derive(Debug, PartialEq, Atom)]
pub enum AuthCredentials {
    Verified(UserInfo),
    Guest
}

impl Default for AuthCredentials {
    fn default() -> Self { AuthCredentials::Guest }
}
