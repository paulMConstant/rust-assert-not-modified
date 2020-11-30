//! Macro which, given a variable and a block of code, executes the block of code and checks that the
//! variable has not changed.
//!
//! For instance, this can check that a function does not have side-effects.

/// Given a variable and a block of code, executes the block of code and checks that the variable has not
/// changed.
///
/// The given variable must implement Clone and Debug.
///
/// # Panics
///
/// Panics if data is modified with message "Data was modified where it should not have been".
///
/// # Example
///
///```
/// #[macro_use] extern crate assert_not_modified;
///
/// // This function returns Err but modifies x anyway. This is misleading.
/// fn modify_x_or_err(x: &mut i32) -> Result<(), String> {
///     *x = *x + 1;
///     Err("Something wrong happened !".to_owned())
/// }
///
/// // This test will expose the lying function :
/// assert!(std::panic::catch_unwind(|| {
///     let mut x = 3;
///     assert_not_modified!(x, modify_x_or_err(&mut x)); // Panics
/// })
/// .is_err());
/// ```
#[macro_export]
macro_rules! assert_not_modified {
    ($data: ident, $block_which_should_not_modify_data: expr) => {
        let old_data = $data.clone();
        let _ = $block_which_should_not_modify_data;
        assert_eq!(
            old_data, $data,
            "Data was modified where it should not have been"
        );
    };
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "Data was modified where it should not have been")]
    fn test_should_not_modify_data_panics() {
        let mut x = 4;
        assert_not_modified!(x, x += 1);
    }

    #[test]
    fn test_should_not_modify_data_does_not_panic() {
        let mut x = 4;
        assert_not_modified!(x, {
            x += 1;
            x -= 1;
        });
    }
}
