# mut_immut

Change value of an immutable variable!!!

Example:
```rust
extern crate mut_immut;
use mut_immut::change;

fn main() {
    let a: u8 = 6;
    change(&a, 255);
    println!("{a}"); // >> 255
}
```