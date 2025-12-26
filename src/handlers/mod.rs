pub mod upload;

use crate::AppState;
use std::sync::Arc;

pub type SharedState = Arc<AppState>;