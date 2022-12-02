# BDD Sugar

Behaviour-driven development syntactic sugar to make tests more readable in rust lang

## Installation
inside `Cargo.toml` add
```toml
[dev-dependencies]
bdd-sugar = "0.1.0"
```

## Example usage
```rust
use bdd_sugar::{given, when, then};

#[test]
#[given(valid email address)]
#[when(filled and pressed forgot password)]
#[then(success toast appears)]
fn forget_password_happy_path_test() {
    // test code
}
```