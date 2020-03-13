# DispatchTable
A rust library used for storing functions inside a key-based collection.


```rust
use dispatchtable::{Dispatch, DispatchTable};

fn add(p: &(u8, u8)) -> u8 { p.0 + p.1 }
fn sub(p: &(u8, u8)) -> u8 { p.0 - p.1 }

fn main() {
  let mut dispatchtable = DispatchTable::new();

  dispatchtable.insert("add".to_string(), Box::new(add));
  dispatchtable.insert("sub".to_string(), Box::new(sub));

  assert_eq!(dispatchtable.call("sub", &(10, 5)), Some(5));
}

```
