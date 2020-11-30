# assert-not-modified

Rust macro which, given a variable and a block of code, executes the block of code and checks that 
the variable has not changed.

For instance, this can check that a function does not have side effects.

The given variable must implement Clone and Debug.

### Panics

Panics if data is modified with the message "Data was modified where it should not have been".

### Example

```rust
#[macro_use] extern crate assert_not_modified;

// This function returns Err but modifies x anyway. This is misleading.
fn modify_x_or_err(x: &mut i32) -> Result<(), String> {
    *x = *x + 1;
    Err("Something wrong happened !".to_owned())
}

// This test will expose the lying function :
assert!(std::panic::catch_unwind(|| {
    let mut x = 3;
    assert_not_modified!(x, modify_x_or_err(&mut x)); // Panics
})
.is_err());
```
