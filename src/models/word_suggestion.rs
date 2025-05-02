use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct SearchResults {
    pub spelling_fix: String,
}
