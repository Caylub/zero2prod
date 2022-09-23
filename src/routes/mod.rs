//! src/routes/mod.rs

// This declares health_check and subscriptions as modules
// and exports their contents to that they an be used in other
// files in the project

// In this case, the handlers for the endpoints /subscriptions
// and /health_check and associated structs are all that
// is getting exported.

mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;
