//! Custom errors used throughout the goat dashboard app.
//! Designed for extensibility and detailed error reporting.

use std::fmt;
use thiserror::Error; // Use thiserror crate for convenient error derive

/// Enumerates possible application errors for goat management.
#[derive(Debug, Error)]
pub enum AppError {
    /// Represents errors during API data fetching or network.
    #[error("Network or API error: {0}")]
    NetworkError(String),

    /// Errors related to invalid user input or form data.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Error when a goat record is not found in the database.
    #[error("Goat not found: {0}")]
    NotFound(String),

    /// Other uncategorized or unexpected errors.
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

impl AppError {
    /// Creates a network error with details.
    pub fn network<S: Into<String>>(msg: S) -> Self {
        AppError::NetworkError(msg.into())
    }

    /// Creates an invalid input error with details.
    pub fn invalid_input<S: Into<String>>(msg: S) -> Self {
        AppError::InvalidInput(msg.into())
    }

    /// Creates a not found error for a given goat identifier.
    pub fn not_found<S: Into<String>>(identifier: S) -> Self {
        AppError::NotFound(identifier.into())
    }

    /// Creates a general unexpected error.
    pub fn unexpected<S: Into<String>>(msg: S) -> Self {
        AppError::Unexpected(msg.into())
    }
}
