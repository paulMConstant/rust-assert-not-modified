//! Rust wrapper function which checks that the given closure does not modify input data.
//! This is helpful when checking that a function which returns Err(...) does not have
//! side-effects.

/// Helper function which panics if the given closure mutates the input data.
///
/// # Panics
///
/// Panics if data is modified with message "Data was modified where it should not have been".
///
/// # Example
///
/// ```
/// use assert_not_modified::assert_not_modified;
///
/// // This bugged function wil return Err but still modify the data.
/// fn misleading_err(x: &mut i32) -> Result<(), String> {
///     *x = *x + 1;
///     // Throws an error but x is modified anyway. This is misleading.
///     Err("Something wrong happened !".to_owned())
/// }
///
/// // This test will expose the lying function :
/// assert!(std::panic::catch_unwind(|| {
///     let mut x = 3;
///     assert_not_modified(&mut x, |x| misleading_err(x)); // Panics
/// })
/// .is_err());
/// ```
pub fn assert_not_modified<F, Data, Any>(data: &mut Data, test: F)
where
    F: FnOnce(&mut Data) -> Any,
    Data: Clone + Eq + std::fmt::Debug,
{
    let old_data = data.clone();
    test(data);
    assert_eq!(
        &old_data, data,
        "Data was modified where it should not have been"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Data was modified where it should not have been")]
    fn test_should_not_modify_data_panics() {
        let mut x = 4;

        assert_not_modified(&mut x, |x| *x = 1);
    }

    #[test]
    fn test_should_not_modify_data_does_not_panic() {
        let mut x = 4;

        assert_not_modified(&mut x, |x| *x + 1);
    }
}
