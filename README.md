# assert-not-modified

Rust wrapper function which checks that the given closure does not modify input data.

This is helpful when checking that a function which returns Err(...) does not have
side-effects.

## Example

```rust
use assert_not_modified::assert_not_modified;

// This bugged function wil return Err but still modify the data.
fn misleading_err(x: &mut i32) -> Result<(), String> {
    *x = *x + 1;
    // Throws an error but x is modified anyway. This is misleading.
    Err("Something wrong happened !".to_owned())
}

// This test will expose the lying function :
assert!(std::panic::catch_unwind(|| {
    let mut x = 3;
    assert_not_modified(&mut x, |x| misleading_err(x)); // Panics
})
.is_err());
```

