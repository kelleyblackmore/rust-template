//! # Rust Template
//!
//! A template repository for Rust projects.
//!
//! ## Usage
//!
//! ```
//! use rust_template::add;
//!
//! let result = add(2, 3);
//! assert_eq!(result, 5);
//! ```

/// Adds two numbers together.
///
/// # Arguments
///
/// * `left` - The first number
/// * `right` - The second number
///
/// # Returns
///
/// The sum of `left` and `right`
///
/// # Examples
///
/// ```
/// use rust_template::add;
///
/// assert_eq!(add(2, 2), 4);
/// assert_eq!(add(0, 5), 5);
/// ```
#[must_use]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 5), 5);
        assert_eq!(add(5, 0), 5);
    }

    #[test]
    fn test_add_large_numbers() {
        assert_eq!(add(1_000_000, 2_000_000), 3_000_000);
    }
}
