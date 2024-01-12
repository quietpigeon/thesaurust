use std::fmt::Debug;

use serde_derive::{ Deserialize, Serialize };

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SearchResults {
    pub spelling_fix: String,
}
