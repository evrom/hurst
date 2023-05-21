/// This crates error type
#[derive(Debug, Clone, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    // NOTE: `error(transparent)` cannot be used here because `linreg::Error` does not implement `std::error::Error`.
    #[error("An error occurred while fitting the linear regression")]
    LinearRegression,
}

/// Convenient wrapper for this crates error type
pub type Result<T> = std::result::Result<T, Error>;
