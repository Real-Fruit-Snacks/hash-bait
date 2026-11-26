//! hash-bait library surface. The binary in src/main.rs is a thin dispatcher
//! over these modules; integration tests exercise the library directly.

pub mod cli;
pub mod out;
pub mod payloads;
pub mod responder;
pub mod smb_ops;
