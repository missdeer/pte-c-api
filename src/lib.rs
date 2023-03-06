// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! C bindings.

#![allow(non_camel_case_types)]
#![warn(missing_docs)]

use std::ffi::CString;
use std::os::raw::c_char;

use plantuml_encoding::{decode_plantuml_deflate, encode_plantuml_deflate};

/// @brief Convert a C char* string to Rust String
unsafe fn char_ptr_to_string(ptr: *const c_char, len: usize) -> String {
    // Create a slice from the pointer and length
    let bytes = std::slice::from_raw_parts(ptr as *const u8, len);

    // Convert the slice to a Rust string
    let rust_string = String::from_utf8_lossy(bytes).to_string();

    rust_string
}

/// @brief Return a char * & length
#[repr(C)]
pub struct Result {
    ptr: *const c_char,
    len: usize,
}

/// @brief Encoding PlantUML source to url
#[no_mangle]
pub extern "C" fn plantuml_encode(input: *const c_char, length: usize) -> Result {
    let puml = unsafe { char_ptr_to_string(input, length) };
    let encoded_deflate =
        encode_plantuml_deflate(puml).unwrap_or_else(|_| "It's not encoded deflate".to_string());
    let cstr = CString::new(encoded_deflate).unwrap();
    let len = cstr.as_bytes_with_nul().len();
    Result {
        ptr: (cstr.into_raw()),
        len: (len),
    }
}

/// @brief Decoding url to PlantUML source
#[no_mangle]
pub extern "C" fn plantuml_decode(input: *const c_char, length: usize) -> Result {
    let url = unsafe { char_ptr_to_string(input, length) };
    let decoded_deflate =
        decode_plantuml_deflate(&url).unwrap_or_else(|_| "It's not decoded deflate".to_string());
    let cstr = CString::new(decoded_deflate).unwrap();
    let len = cstr.as_bytes_with_nul().len();
    Result {
        ptr: (cstr.into_raw()),
        len: (len),
    }
}
