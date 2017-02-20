
use std::{io, mem, slice};



// http://stackoverflow.com/questions/25410028/how-to-read-a-struct-from-a-file-in-rust
pub fn read<T, R: io::Read>(reader: &mut R) -> io::Result<T> {
    let num_bytes = mem::size_of::<T>();
    unsafe {
        let mut result: T = mem::uninitialized();
        let mut buffer: &mut [u8] = slice::from_raw_parts_mut(&mut result as *mut T as *mut u8, num_bytes);
        match reader.read_exact(buffer) {
            Ok(()) => Ok(result),
            Err(e) => {
                mem::forget(result);
                Err(e)
            }
        }
    }
}


pub fn write<T, W: io::Write>(writer: &mut W, mut value: T) -> io::Result<usize> {
    let num_bytes = mem::size_of::<T>();
    unsafe {
        let buffer: &mut [u8] = slice::from_raw_parts_mut(&mut value as *mut T as *mut u8, num_bytes);
        writer.write(buffer)
    }
}
