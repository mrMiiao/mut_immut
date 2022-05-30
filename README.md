[![crates.io](https://img.shields.io/crates/v/mut_immut.svg)](https://crates.io/crates/mut_immut)
[![License](https://img.shields.io/crates/l/mut_immut.svg)](https://choosealicense.com/licenses/mpl-2.0/)
[![Documentation](https://img.shields.io/docsrs/mut_immut/latest)](https://docs.rs/mut_immut)

# mut_immut

Change value of an immutable variable!!!

Examples:
```rust
extern crate mut_immut;
use mut_immut::change;

fn main() {
    let a: u8 = 6;
    change(&a, 255);
    println!("{a}"); // >> 255
}
```

```rust
extern crate mut_immut;
use mut_immut::*;

fn main() {
    let a: u8 = 15;
    let mut mut_a = get_mut(&a);
    *mut_a = 8;
    println!("{a}"); // >> 8
}
```