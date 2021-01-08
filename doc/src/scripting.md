# Scripting

## Using Rust

```rust,editable
use dlog_lib::{DResult, Dlog};

fn main() -> DResult<()> {
    Dlog::default().run?;
    Ok(())
}
```
