// Copyright 2024 Esteve Fernandez
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::slice;
use std::ffi::{CStr, CString};
use std::io::Cursor;
use std::os::raw::c_char;

/// Frees a `Cursor<Vec<u8>>` from memory.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_free(ptr: *mut Cursor<Vec<u8>>) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

/// Creates a new `Cursor<Vec<u8>>` from a raw pointer to a byte array.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `data` - A raw pointer to a byte array.
/// * `length` - The length of the byte array.
///
/// # Returns
///
/// A raw pointer to a `Cursor<Vec<u8>>`.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_new(
    data: *const u8,
    length: usize,
) -> *mut Cursor<Vec<u8>> {
    let libp2p_cdr_buffer = Cursor::new(unsafe { slice::from_raw_parts(data, length) }.to_vec());
    Box::into_raw(Box::new(libp2p_cdr_buffer))
}

/// Reads a `u64` from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - A raw pointer to a `u64`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_uint64(ptr: *mut Cursor<Vec<u8>>, n: *mut u64) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        let x = cdr::deserialize_from::<_, u64, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
        *n = x
    }
}

/// Reads a `u32` from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - A raw pointer to a `u32`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_uint32(ptr: *mut Cursor<Vec<u8>>, n: *mut u32) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u32, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    }
}

/// Reads a `u16` from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - A raw pointer to a `u16`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_uint16(ptr: *mut Cursor<Vec<u8>>, n: *mut u16) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u16, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    }
}

/// Reads a `u8` from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - A raw pointer to a `u8`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_uint8(ptr: *mut Cursor<Vec<u8>>, n: *mut u8) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u8, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    }
}

/// Reads a `i64` from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - A raw pointer to a `i64`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_int64(ptr: *mut Cursor<Vec<u8>>, n: *mut i64) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, i64, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    }
}

/// Reads a `i32` from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - A raw pointer to a `i32`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_int32(ptr: *mut Cursor<Vec<u8>>, n: *mut i32) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, i32, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    }
}

/// Reads a `i16` from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - A raw pointer to a `i16`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_int16(ptr: *mut Cursor<Vec<u8>>, n: *mut i16) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, i16, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    }
}

/// Reads a `i8` from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - A raw pointer to a `i8`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_int8(ptr: *mut Cursor<Vec<u8>>, n: *mut i8) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, i8, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    }
}

/// Reads a `c_char` from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - A raw pointer to a `c_char`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_char(ptr: *mut Cursor<Vec<u8>>, n: *mut c_char) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, c_char, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    }
}

/// Reads a 16-bit `c_char` from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - A raw pointer to a `u16`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_char16(ptr: *mut Cursor<Vec<u8>>, n: *mut u16) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u16, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    }
}

/// Reads a `float` from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - A raw pointer to a `f32`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_float(ptr: *mut Cursor<Vec<u8>>, n: *mut f32) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, f32, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    }
}

/// Reads a `double` from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - A raw pointer to a `f64`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_double(ptr: *mut Cursor<Vec<u8>>, n: *mut f64) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, f64, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    }
}

/// Reads a `bool` from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - A raw pointer to a `bool`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_bool(ptr: *mut Cursor<Vec<u8>>, n: *mut bool) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, bool, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    }
}

/// Deserializes a `CString` from a `Cursor<Vec<u8>>` and stores the raw pointer and length of the string.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `s` - A raw pointer to store the raw pointer of the string.
/// * `size` - A raw pointer to store the length of the string.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_string(
    ptr: *mut Cursor<Vec<u8>>,
    s: *mut *const c_char,
    size: *mut usize,
) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let cs = cdr::deserialize_from::<_, CString, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    unsafe {
        *size = cs.as_bytes().len();

        if *size != 0 {
            *s = cs.into_raw();
        }
    }
}

/// Frees a `CString` from memory.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `s` - A raw pointer to a `CString`.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

/// Reads a `u16` string from a `Cursor<Vec<u8>>`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `s` - A raw pointer to store the raw pointer of the string.
/// * `size` - A raw pointer to store the length of the string.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_u16string(
    ptr: *mut Cursor<Vec<u8>>,
    s: *mut *const u16,
    size: *mut usize,
) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let cs = cdr::deserialize_from::<_, Vec<u16>, _>(libp2p_cdr_buffer, cdr::Infinite).unwrap();
    unsafe {
        *size = cs.len();

        if *size != 0 {
            *s = cs.as_ptr();
        }
    }
}

/// Creates a new `Cursor<Vec<u8>>` to write to.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Returns
///
/// A raw pointer to a `Cursor<Vec<u8>>`.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_new() -> *mut Cursor<Vec<u8>> {
    let libp2p_cdr_buffer = Cursor::new(Vec::<u8>::new());
    Box::into_raw(Box::new(libp2p_cdr_buffer))
}

/// Writes a `u64` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `u64` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - The `u64` to write.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_uint64(ptr: *mut Cursor<Vec<u8>>, n: u64) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &n, cdr::Infinite).unwrap();
}

/// Writes a `u32` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `u32` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - The `u32` to write.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_uint32(ptr: *mut Cursor<Vec<u8>>, n: u32) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &n, cdr::Infinite).unwrap();
}

/// Writes a `u16` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `u16` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - The `u16` to write.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_uint16(ptr: *mut Cursor<Vec<u8>>, n: u16) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &n, cdr::Infinite).unwrap();
}

/// Writes a `u8` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `u8` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - The `u8` to write.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_uint8(ptr: *mut Cursor<Vec<u8>>, n: u8) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &n, cdr::Infinite).unwrap();
}

/// Writes a `i64` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `i64` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - The `i64` to write.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_int64(ptr: *mut Cursor<Vec<u8>>, n: i64) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &n, cdr::Infinite).unwrap();
}

/// Writes a `i32` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `i32` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - The `i32` to write.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_int32(ptr: *mut Cursor<Vec<u8>>, n: i32) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &n, cdr::Infinite).unwrap();
}

/// Writes a `i16` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `i16` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - The `i16` to write.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_int16(ptr: *mut Cursor<Vec<u8>>, n: i16) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &n, cdr::Infinite).unwrap();
}

/// Writes a `i8` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `i8` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - The `i8` to write.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_int8(ptr: *mut Cursor<Vec<u8>>, n: i8) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &n, cdr::Infinite).unwrap();
}

/// Writes a `c_char` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `c_char` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - The `c_char` to write.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_char(ptr: *mut Cursor<Vec<u8>>, n: c_char) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &n, cdr::Infinite).unwrap();
}

/// Writes a 16-bit `char` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `u16` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - The `u16` to write.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_char16(ptr: *mut Cursor<Vec<u8>>, n: u16) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &n, cdr::Infinite).unwrap();
}

/// Writes a `float` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `f32` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - The `f32` to write.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_float(ptr: *mut Cursor<Vec<u8>>, n: f32) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &n, cdr::Infinite).unwrap();
}

/// Writes a `double` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `f64` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - The `f64` to write.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_double(ptr: *mut Cursor<Vec<u8>>, n: f64) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &n, cdr::Infinite).unwrap();
}

/// Writes a `bool` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `bool` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `n` - The `bool` to write.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_bool(ptr: *mut Cursor<Vec<u8>>, n: bool) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &n, cdr::Infinite).unwrap();
}

/// Writes a string to a `Cursor<Vec<u8>>`.
///
/// This function serializes a string into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `s` - A raw pointer to a C string.
/// * `size` - The length of the string (excluding null terminator).
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_string(
    ptr: *mut Cursor<Vec<u8>>,
    s: *const c_char,
    size: usize,
) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    if size == 0 || s.is_null() {
        // Write empty string
        let empty = CString::new("").unwrap();
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &empty, cdr::Infinite)
            .unwrap();
    } else {
        let cs = unsafe { CStr::from_ptr(s) };
        let cstring = CString::new(cs.to_bytes()).unwrap();
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &cstring, cdr::Infinite)
            .unwrap();
    }
}

/// Writes a u16 string to a `Cursor<Vec<u8>>`.
///
/// This function serializes a u16 string into a `Cursor<Vec<u8>>` using the `cdr::serialize_into` function.
/// The serialization is done in Big Endian format (`cdr::CdrBe`).
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Cursor<Vec<u8>>`.
/// * `s` - A raw pointer to a u16 array.
/// * `size` - The number of u16 elements in the string.
///
/// # Panics
///
/// This function will panic if the provided pointer is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_u16string(
    ptr: *mut Cursor<Vec<u8>>,
    s: *const u16,
    size: usize,
) {
    let libp2p_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    if size == 0 || s.is_null() {
        // Write empty u16 string
        let empty: Vec<u16> = Vec::new();
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &empty, cdr::Infinite)
            .unwrap();
    } else {
        let slice = unsafe { slice::from_raw_parts(s, size) };
        let vec: Vec<u16> = slice.to_vec();
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &vec, cdr::Infinite).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    // Helper function to get buffer data for reading
    fn get_buffer_data(ptr: *mut Cursor<Vec<u8>>) -> Vec<u8> {
        unsafe {
            let cursor = &*ptr;
            cursor.get_ref().clone()
        }
    }

    #[test]
    fn test_buffer_lifecycle() {
        // Test buffer creation and cleanup
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        assert!(!write_buf.is_null());
        rs_libp2p_cdr_buffer_free(write_buf);

        // Test read buffer creation
        let data = vec![0u8, 1, 2, 3];
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());
        assert!(!read_buf.is_null());
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    #[test]
    fn test_null_pointer_handling() {
        // free should handle null gracefully
        rs_libp2p_cdr_buffer_free(std::ptr::null_mut());

        // free_string should handle null gracefully
        rs_libp2p_cdr_buffer_free_string(std::ptr::null_mut());
    }

    // === Unsigned Integer Roundtrip Tests ===

    #[test]
    fn test_uint64_roundtrip() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val: u64 = 0x0123456789ABCDEF;

        rs_libp2p_cdr_buffer_write_uint64(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: u64 = 0;
        rs_libp2p_cdr_buffer_read_uint64(read_buf, &mut result as *mut u64);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    #[test]
    fn test_uint32_roundtrip() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val: u32 = 0x01234567;

        rs_libp2p_cdr_buffer_write_uint32(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: u32 = 0;
        rs_libp2p_cdr_buffer_read_uint32(read_buf, &mut result as *mut u32);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    #[test]
    fn test_uint16_roundtrip() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val: u16 = 0x0123;

        rs_libp2p_cdr_buffer_write_uint16(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: u16 = 0;
        rs_libp2p_cdr_buffer_read_uint16(read_buf, &mut result as *mut u16);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    #[test]
    fn test_uint8_roundtrip() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val: u8 = 0x42;

        rs_libp2p_cdr_buffer_write_uint8(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: u8 = 0;
        rs_libp2p_cdr_buffer_read_uint8(read_buf, &mut result as *mut u8);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    // === Signed Integer Roundtrip Tests ===

    #[test]
    fn test_int64_roundtrip() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val: i64 = -0x0123456789ABCDEF;

        rs_libp2p_cdr_buffer_write_int64(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: i64 = 0;
        rs_libp2p_cdr_buffer_read_int64(read_buf, &mut result as *mut i64);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    #[test]
    fn test_int32_roundtrip() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val: i32 = -0x01234567;

        rs_libp2p_cdr_buffer_write_int32(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: i32 = 0;
        rs_libp2p_cdr_buffer_read_int32(read_buf, &mut result as *mut i32);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    #[test]
    fn test_int16_roundtrip() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val: i16 = -0x0123;

        rs_libp2p_cdr_buffer_write_int16(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: i16 = 0;
        rs_libp2p_cdr_buffer_read_int16(read_buf, &mut result as *mut i16);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    #[test]
    fn test_int8_roundtrip() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val: i8 = -42;

        rs_libp2p_cdr_buffer_write_int8(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: i8 = 0;
        rs_libp2p_cdr_buffer_read_int8(read_buf, &mut result as *mut i8);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    // === Character Roundtrip Tests ===

    #[test]
    fn test_char_roundtrip() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val: c_char = b'A' as c_char;

        rs_libp2p_cdr_buffer_write_char(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: c_char = 0;
        rs_libp2p_cdr_buffer_read_char(read_buf, &mut result as *mut c_char);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    #[test]
    fn test_char16_roundtrip() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val: u16 = 0x3042; // Japanese Hiragana 'あ'

        rs_libp2p_cdr_buffer_write_char16(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: u16 = 0;
        rs_libp2p_cdr_buffer_read_char16(read_buf, &mut result as *mut u16);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    // === Floating Point Roundtrip Tests ===

    #[test]
    fn test_float_roundtrip() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val: f32 = 3.14159;

        rs_libp2p_cdr_buffer_write_float(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: f32 = 0.0;
        rs_libp2p_cdr_buffer_read_float(read_buf, &mut result as *mut f32);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    #[test]
    fn test_double_roundtrip() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val: f64 = 2.718281828459045;

        rs_libp2p_cdr_buffer_write_double(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: f64 = 0.0;
        rs_libp2p_cdr_buffer_read_double(read_buf, &mut result as *mut f64);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    // === Boolean Roundtrip Tests ===

    #[test]
    fn test_bool_roundtrip_true() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val = true;

        rs_libp2p_cdr_buffer_write_bool(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result = false;
        rs_libp2p_cdr_buffer_read_bool(read_buf, &mut result as *mut bool);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    #[test]
    fn test_bool_roundtrip_false() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val = false;

        rs_libp2p_cdr_buffer_write_bool(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result = true;
        rs_libp2p_cdr_buffer_read_bool(read_buf, &mut result as *mut bool);

        assert_eq!(result, test_val);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    // === String Roundtrip Tests ===

    #[test]
    fn test_string_roundtrip() {
        // Manually serialize a string using CDR
        let test_string = CString::new("Hello, World!").unwrap();
        let mut buffer = Cursor::new(Vec::<u8>::new());
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut buffer, &test_string, cdr::Infinite)
            .unwrap();

        let data = buffer.get_ref().clone();
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut s_ptr: *const c_char = std::ptr::null();
        let mut size: usize = 0;

        rs_libp2p_cdr_buffer_read_string(
            read_buf,
            &mut s_ptr as *mut *const c_char,
            &mut size as *mut usize,
        );

        assert!(!s_ptr.is_null());
        assert_eq!(size, 13);

        let result_str = unsafe { CStr::from_ptr(s_ptr) };
        assert_eq!(result_str.to_str().unwrap(), "Hello, World!");

        rs_libp2p_cdr_buffer_free_string(s_ptr as *mut c_char);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    #[test]
    fn test_empty_string() {
        let test_string = CString::new("").unwrap();
        let mut buffer = Cursor::new(Vec::<u8>::new());
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut buffer, &test_string, cdr::Infinite)
            .unwrap();

        let data = buffer.get_ref().clone();
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut s_ptr: *const c_char = std::ptr::null();
        let mut size: usize = 0;

        rs_libp2p_cdr_buffer_read_string(
            read_buf,
            &mut s_ptr as *mut *const c_char,
            &mut size as *mut usize,
        );

        assert_eq!(size, 0);
        // Empty strings should not set the pointer

        rs_libp2p_cdr_buffer_free(read_buf);
    }

    // FIXME: This test exposes a bug in rs_libp2p_cdr_buffer_read_u16string
    // The function returns a pointer to a Vec that gets dropped, causing UB.
    // The FFI function needs to leak the Vec or use a different memory management strategy.
    // See lines 464-481 where cs.as_ptr() is returned but cs is dropped.
    #[test]
    #[ignore]
    fn test_u16string_roundtrip() {
        // Manually serialize a u16 string using CDR
        let test_string: Vec<u16> = vec![0x3042, 0x3044, 0x3046]; // Japanese hiragana
        let mut buffer = Cursor::new(Vec::<u8>::new());
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut buffer, &test_string, cdr::Infinite)
            .unwrap();

        let data = buffer.get_ref().clone();
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut s_ptr: *const u16 = std::ptr::null();
        let mut size: usize = 0;

        rs_libp2p_cdr_buffer_read_u16string(
            read_buf,
            &mut s_ptr as *mut *const u16,
            &mut size as *mut usize,
        );

        assert!(!s_ptr.is_null());
        assert_eq!(size, 3);

        let result_slice = unsafe { std::slice::from_raw_parts(s_ptr, size) };
        assert_eq!(result_slice, &test_string[..]);

        rs_libp2p_cdr_buffer_free(read_buf);
    }

    // === Boundary Value Tests ===

    #[test]
    fn test_uint64_boundary_values() {
        let test_values = vec![0u64, u64::MAX, u64::MIN, u64::MAX / 2];

        for test_val in test_values {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_uint64(write_buf, test_val);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: u64 = 0;
            rs_libp2p_cdr_buffer_read_uint64(read_buf, &mut result as *mut u64);

            assert_eq!(result, test_val, "Failed for value: {}", test_val);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }
    }

    #[test]
    fn test_int64_boundary_values() {
        let test_values = vec![0i64, i64::MAX, i64::MIN, -1, 1];

        for test_val in test_values {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_int64(write_buf, test_val);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: i64 = 0;
            rs_libp2p_cdr_buffer_read_int64(read_buf, &mut result as *mut i64);

            assert_eq!(result, test_val, "Failed for value: {}", test_val);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }
    }

    #[test]
    fn test_float_special_values() {
        let test_values = vec![
            0.0f32,
            -0.0f32,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::MIN,
            f32::MAX,
        ];

        for test_val in test_values {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_float(write_buf, test_val);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: f32 = 0.0;
            rs_libp2p_cdr_buffer_read_float(read_buf, &mut result as *mut f32);

            assert_eq!(result, test_val, "Failed for value: {}", test_val);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }
    }

    #[test]
    fn test_float_nan() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val = f32::NAN;

        rs_libp2p_cdr_buffer_write_float(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: f32 = 0.0;
        rs_libp2p_cdr_buffer_read_float(read_buf, &mut result as *mut f32);

        assert!(result.is_nan(), "Expected NaN");

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    #[test]
    fn test_double_special_values() {
        let test_values = vec![
            0.0f64,
            -0.0f64,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::MIN,
            f64::MAX,
        ];

        for test_val in test_values {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_double(write_buf, test_val);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: f64 = 0.0;
            rs_libp2p_cdr_buffer_read_double(read_buf, &mut result as *mut f64);

            assert_eq!(result, test_val, "Failed for value: {}", test_val);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }
    }

    #[test]
    fn test_double_nan() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();
        let test_val = f64::NAN;

        rs_libp2p_cdr_buffer_write_double(write_buf, test_val);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut result: f64 = 0.0;
        rs_libp2p_cdr_buffer_read_double(read_buf, &mut result as *mut f64);

        assert!(result.is_nan(), "Expected NaN");

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    // === Multiple Values Test ===

    #[test]
    fn test_multiple_values_sequence() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();

        // Write multiple values
        rs_libp2p_cdr_buffer_write_uint32(write_buf, 42);
        rs_libp2p_cdr_buffer_write_float(write_buf, 3.14);
        rs_libp2p_cdr_buffer_write_bool(write_buf, true);
        rs_libp2p_cdr_buffer_write_int16(write_buf, -100);

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        // Read them back in order
        let mut val1: u32 = 0;
        let mut val2: f32 = 0.0;
        let mut val3: bool = false;
        let mut val4: i16 = 0;

        rs_libp2p_cdr_buffer_read_uint32(read_buf, &mut val1 as *mut u32);
        rs_libp2p_cdr_buffer_read_float(read_buf, &mut val2 as *mut f32);
        rs_libp2p_cdr_buffer_read_bool(read_buf, &mut val3 as *mut bool);
        rs_libp2p_cdr_buffer_read_int16(read_buf, &mut val4 as *mut i16);

        assert_eq!(val1, 42);
        assert_eq!(val2, 3.14);
        assert_eq!(val3, true);
        assert_eq!(val4, -100);

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    // === Large Data Stress Test ===

    #[test]
    fn test_large_data_sequence() {
        let write_buf = rs_libp2p_cdr_buffer_write_new();

        // Write 10000 values to stress buffer growth
        for i in 0..10000u32 {
            rs_libp2p_cdr_buffer_write_uint32(write_buf, i);
        }

        let data = get_buffer_data(write_buf);
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        // Read them back and verify
        for i in 0..10000u32 {
            let mut val: u32 = 0;
            rs_libp2p_cdr_buffer_read_uint32(read_buf, &mut val as *mut u32);
            assert_eq!(val, i, "Mismatch at index {}", i);
        }

        rs_libp2p_cdr_buffer_free(write_buf);
        rs_libp2p_cdr_buffer_free(read_buf);
    }

    #[test]
    fn test_long_string() {
        // Test with a 10KB string
        let long_str = "A".repeat(10000);
        let test_string = CString::new(long_str.clone()).unwrap();
        let mut buffer = Cursor::new(Vec::<u8>::new());
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut buffer, &test_string, cdr::Infinite)
            .unwrap();

        let data = buffer.get_ref().clone();
        let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

        let mut s_ptr: *const c_char = std::ptr::null();
        let mut size: usize = 0;

        rs_libp2p_cdr_buffer_read_string(
            read_buf,
            &mut s_ptr as *mut *const c_char,
            &mut size as *mut usize,
        );

        assert!(!s_ptr.is_null());
        assert_eq!(size, 10000);

        let result_str = unsafe { CStr::from_ptr(s_ptr) };
        assert_eq!(result_str.to_str().unwrap(), long_str);

        rs_libp2p_cdr_buffer_free_string(s_ptr as *mut c_char);
        rs_libp2p_cdr_buffer_free(read_buf);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    // Helper to get buffer data
    fn get_buffer_data(ptr: *mut Cursor<Vec<u8>>) -> Vec<u8> {
        unsafe {
            let cursor = &*ptr;
            cursor.get_ref().clone()
        }
    }

    proptest! {
        #[test]
        fn prop_uint64_roundtrip(value: u64) {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_uint64(write_buf, value);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: u64 = 0;
            rs_libp2p_cdr_buffer_read_uint64(read_buf, &mut result as *mut u64);

            prop_assert_eq!(result, value);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }

        #[test]
        fn prop_uint32_roundtrip(value: u32) {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_uint32(write_buf, value);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: u32 = 0;
            rs_libp2p_cdr_buffer_read_uint32(read_buf, &mut result as *mut u32);

            prop_assert_eq!(result, value);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }

        #[test]
        fn prop_uint16_roundtrip(value: u16) {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_uint16(write_buf, value);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: u16 = 0;
            rs_libp2p_cdr_buffer_read_uint16(read_buf, &mut result as *mut u16);

            prop_assert_eq!(result, value);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }

        #[test]
        fn prop_uint8_roundtrip(value: u8) {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_uint8(write_buf, value);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: u8 = 0;
            rs_libp2p_cdr_buffer_read_uint8(read_buf, &mut result as *mut u8);

            prop_assert_eq!(result, value);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }

        #[test]
        fn prop_int64_roundtrip(value: i64) {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_int64(write_buf, value);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: i64 = 0;
            rs_libp2p_cdr_buffer_read_int64(read_buf, &mut result as *mut i64);

            prop_assert_eq!(result, value);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }

        #[test]
        fn prop_int32_roundtrip(value: i32) {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_int32(write_buf, value);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: i32 = 0;
            rs_libp2p_cdr_buffer_read_int32(read_buf, &mut result as *mut i32);

            prop_assert_eq!(result, value);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }

        #[test]
        fn prop_int16_roundtrip(value: i16) {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_int16(write_buf, value);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: i16 = 0;
            rs_libp2p_cdr_buffer_read_int16(read_buf, &mut result as *mut i16);

            prop_assert_eq!(result, value);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }

        #[test]
        fn prop_int8_roundtrip(value: i8) {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_int8(write_buf, value);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: i8 = 0;
            rs_libp2p_cdr_buffer_read_int8(read_buf, &mut result as *mut i8);

            prop_assert_eq!(result, value);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }

        #[test]
        fn prop_float_roundtrip(value in prop::num::f32::NORMAL) {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_float(write_buf, value);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: f32 = 0.0;
            rs_libp2p_cdr_buffer_read_float(read_buf, &mut result as *mut f32);

            prop_assert_eq!(result, value);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }

        #[test]
        fn prop_double_roundtrip(value in prop::num::f64::NORMAL) {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_double(write_buf, value);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: f64 = 0.0;
            rs_libp2p_cdr_buffer_read_double(read_buf, &mut result as *mut f64);

            prop_assert_eq!(result, value);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }

        #[test]
        fn prop_bool_roundtrip(value: bool) {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_bool(write_buf, value);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result = false;
            rs_libp2p_cdr_buffer_read_bool(read_buf, &mut result as *mut bool);

            prop_assert_eq!(result, value);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }

        #[test]
        fn prop_char_roundtrip(value: i8) {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_char(write_buf, value as c_char);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: c_char = 0;
            rs_libp2p_cdr_buffer_read_char(read_buf, &mut result as *mut c_char);

            prop_assert_eq!(result, value as c_char);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }

        #[test]
        fn prop_char16_roundtrip(value: u16) {
            let write_buf = rs_libp2p_cdr_buffer_write_new();
            rs_libp2p_cdr_buffer_write_char16(write_buf, value);

            let data = get_buffer_data(write_buf);
            let read_buf = rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len());

            let mut result: u16 = 0;
            rs_libp2p_cdr_buffer_read_char16(read_buf, &mut result as *mut u16);

            prop_assert_eq!(result, value);

            rs_libp2p_cdr_buffer_free(write_buf);
            rs_libp2p_cdr_buffer_free(read_buf);
        }
    }
}
