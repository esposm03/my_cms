//! Request handlers
//!
//! This module contains functions that are used as handlers by the web framework,
//! `actix-web`.

mod health_check;
mod posts;

pub use health_check::*;
pub use posts::*;
