# wraited-struct

Read and write `struct`


# Usage

```
extern crate wraited_struct;
use std::fs::File;

#[derive(Debug)]
struct Something {
    a: u8,
    b: u16,
    c: u32,
}

fn main() {
    let mut file = File::create("customer.bin").unwrap();
    wraited_struct::write::<Something, File>(&mut file, Something { a: 97, b: 98, c: 99 }).unwrap();

    let mut file = File::open("customer.bin").unwrap();
    let something = wraited_struct::read::<Something, File>(&mut file).unwrap();
    println!("Something is {:?}", something);
}
```
