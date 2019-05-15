//! Type-safe authentication information extractors

pub mod basic;
pub mod bearer;
mod config;
mod errors;

pub use self::errors::AuthenticationError;
