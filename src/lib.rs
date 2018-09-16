
use std::{io, mem, slice};



/// Read `T` with byte array
///
/// # Example
///
/// ```
/// extern crate wraited_struct;
/// use std::fs::File;
///
/// struct Customer {
///     a: u8,
///     b: u16,
///     c: u32,
/// }
///
/// fn main() {
///     let mut file = File::open("customer.bin").unwrap();
///     wraited_struct::read::<Customer, File>(&mut file).unwrap();
/// }
/// ```
pub unsafe fn read<T, R: io::Read>(reader: &mut R) -> io::Result<T> {
    // http://stackoverflow.com/questions/25410028/how-to-read-a-struct-from-a-file-in-rust
    let num_bytes = mem::size_of::<T>();
    let mut result: T = mem::uninitialized();
    let buffer: &mut [u8] = slice::from_raw_parts_mut(&mut result as *mut T as *mut u8, num_bytes);
    match reader.read_exact(buffer) {
        Ok(()) => Ok(result),
        Err(e) => {
            mem::forget(result);
            Err(e)
        }
    }
}


/// Write `T` with byte array
///
/// # Example
///
/// ```
/// extern crate wraited_struct;
/// use std::fs::File;
///
/// struct Customer {
///     a: u8,
///     b: u16,
///     c: u32,
/// }
///
/// fn main() {
///     let mut file = File::create("customer.bin").unwrap();
///     wraited_struct::write::<Customer, File>(&mut file, Customer { a: 97, b: 98, c: 99 }).unwrap();
/// }
/// ```
pub unsafe fn write<T, W: io::Write>(writer: &mut W, mut value: T) -> io::Result<usize> {
    let num_bytes = mem::size_of::<T>();
    let buffer: &mut [u8] = slice::from_raw_parts_mut(&mut value as *mut T as *mut u8, num_bytes);
    writer.write(buffer)
}
