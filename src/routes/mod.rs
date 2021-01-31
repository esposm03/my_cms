//! Request handlers
//!
//! This module contains functions that are used as handlers by the web framework,
//! `actix-web`.

mod health_check;

pub use health_check::*;
