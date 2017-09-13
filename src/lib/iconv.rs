
use std::io;
//use std::io::{Result, Error, Write, Read};
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;
//use std::iter;
use libc::{c_char, size_t, c_int}; //, c_void
use iconv_rs::iconv_t;
use iconv_rs::iconv;
use iconv_rs::iconv_open;
use iconv_rs::iconv_close;

pub struct Converter {
    cd: iconv_t
}

impl Converter {
    /// Creates a new Converter from ``from`` encoding and ``to`` encoding.
    pub fn new(from: &str, to: &str) -> Converter {
        let from_encoding = CString::new(from).unwrap();
        let to_encoding = CString::new(to).unwrap();

        let handle = unsafe {
            iconv_open(to_encoding.as_ptr(), from_encoding.as_ptr())
        };
        if handle as isize == -1 {
            panic!("Error creating conversion descriptor from {:} to {:}", from, to);
        }
        Converter { cd: handle }
    }

    /// Convert from input into output.
    /// Returns (bytes_read, bytes_written, errno).
    pub fn convert(&self, input: &[u8], output: &mut [u8]) -> (usize, usize, c_int) {
        let input_left = input.len() as size_t;
        let output_left = output.len() as size_t;

        if input_left > 0 && output_left > 0 {
            let input_ptr = input.as_ptr();
            let output_ptr = output.as_ptr();

            let ret = unsafe { iconv(self.cd,
                                     mem::transmute(&input_ptr), mem::transmute(&input_left),
                                     mem::transmute(&output_ptr), mem::transmute(&output_left))
            };
            let bytes_read = input.len() - input_left as usize;
            let bytes_written = output.len() - output_left as usize;

            return (bytes_read, bytes_written, if ret as isize == -1 { io::Error::last_os_error().raw_os_error().unwrap() as c_int } else { 0 })
        } else if input_left == 0 && output_left > 0 {
            let output_ptr = output.as_ptr();

            let ret = unsafe { iconv(self.cd,
                                     ptr::null_mut::<*mut c_char>(), mem::transmute(&input_left),
                                     mem::transmute(&output_ptr), mem::transmute(&output_left))
            };

            let bytes_written = output.len() - output_left as usize;

            return (0, bytes_written, if ret as isize == -1 { io::Error::last_os_error().raw_os_error().unwrap() as c_int } else { 0 })
        } else {
            let ret = unsafe { iconv(self.cd,
                                     ptr::null_mut::<*mut c_char>(), mem::transmute(&input_left),
                                     ptr::null_mut::<*mut c_char>(), mem::transmute(&output_left))
            };

            return (0, 0, if ret as isize == -1 { io::Error::last_os_error().raw_os_error().unwrap() as c_int } else { 0 })
        }
    }
}


impl Drop for Converter {
    fn drop(&mut self) {
        unsafe { iconv_close(self.cd) };
    }
}
