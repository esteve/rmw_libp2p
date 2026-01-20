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
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &empty, cdr::Infinite).unwrap();
    } else {
        let cs = unsafe { CStr::from_ptr(s) };
        let cstring = CString::new(cs.to_bytes()).unwrap();
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &cstring, cdr::Infinite).unwrap();
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
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &empty, cdr::Infinite).unwrap();
    } else {
        let slice = unsafe { slice::from_raw_parts(s, size) };
        let vec: Vec<u16> = slice.to_vec();
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p_cdr_buffer, &vec, cdr::Infinite).unwrap();
    }
}
