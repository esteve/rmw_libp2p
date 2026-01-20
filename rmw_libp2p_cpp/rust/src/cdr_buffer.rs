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
use std::ffi::CString;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // Helper function to create a write buffer and return raw pointer
    fn create_write_buffer() -> *mut Cursor<Vec<u8>> {
        rs_libp2p_cdr_buffer_write_new()
    }

    // Helper function to get the bytes from a write buffer
    fn get_buffer_bytes(ptr: *mut Cursor<Vec<u8>>) -> Vec<u8> {
        unsafe {
            assert!(!ptr.is_null());
            (*ptr).get_ref().clone()
        }
    }

    // Helper function to create a read buffer from bytes
    fn create_read_buffer(data: &[u8]) -> *mut Cursor<Vec<u8>> {
        rs_libp2p_cdr_buffer_read_new(data.as_ptr(), data.len())
    }

    // ==================== Write Tests ====================

    #[test]
    fn test_write_uint8() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_uint8(buffer, 42);
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    #[test]
    fn test_write_uint16() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_uint16(buffer, 1234);
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    #[test]
    fn test_write_uint32() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_uint32(buffer, 123456);
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    #[test]
    fn test_write_uint64() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_uint64(buffer, 1234567890123);
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    #[test]
    fn test_write_int8() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_int8(buffer, -42);
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    #[test]
    fn test_write_int16() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_int16(buffer, -1234);
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    #[test]
    fn test_write_int32() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_int32(buffer, -123456);
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    #[test]
    fn test_write_int64() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_int64(buffer, -1234567890123);
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    #[test]
    fn test_write_float() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_float(buffer, 3.14159);
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    #[test]
    fn test_write_double() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_double(buffer, 3.141592653589793);
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    #[test]
    fn test_write_bool_true() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_bool(buffer, true);
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    #[test]
    fn test_write_bool_false() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_bool(buffer, false);
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    #[test]
    fn test_write_char() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_char(buffer, b'A' as c_char);
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    #[test]
    fn test_write_char16() {
        let buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_char16(buffer, 0x0041); // 'A' in UTF-16
        let bytes = get_buffer_bytes(buffer);
        assert!(!bytes.is_empty());
        rs_libp2p_cdr_buffer_free(buffer);
    }

    // ==================== Read Tests ====================

    #[test]
    fn test_read_uint8() {
        // First write a value
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_uint8(write_buffer, 42);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        // Then read it back
        let read_buffer = create_read_buffer(&bytes);
        let mut result: u8 = 0;
        rs_libp2p_cdr_buffer_read_uint8(read_buffer, &mut result);
        assert_eq!(result, 42);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_read_uint16() {
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_uint16(write_buffer, 1234);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: u16 = 0;
        rs_libp2p_cdr_buffer_read_uint16(read_buffer, &mut result);
        assert_eq!(result, 1234);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_read_uint32() {
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_uint32(write_buffer, 123456);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: u32 = 0;
        rs_libp2p_cdr_buffer_read_uint32(read_buffer, &mut result);
        assert_eq!(result, 123456);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_read_uint64() {
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_uint64(write_buffer, 1234567890123);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: u64 = 0;
        rs_libp2p_cdr_buffer_read_uint64(read_buffer, &mut result);
        assert_eq!(result, 1234567890123);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_read_int8() {
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_int8(write_buffer, -42);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: i8 = 0;
        rs_libp2p_cdr_buffer_read_int8(read_buffer, &mut result);
        assert_eq!(result, -42);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_read_int16() {
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_int16(write_buffer, -1234);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: i16 = 0;
        rs_libp2p_cdr_buffer_read_int16(read_buffer, &mut result);
        assert_eq!(result, -1234);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_read_int32() {
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_int32(write_buffer, -123456);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: i32 = 0;
        rs_libp2p_cdr_buffer_read_int32(read_buffer, &mut result);
        assert_eq!(result, -123456);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_read_int64() {
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_int64(write_buffer, -1234567890123);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: i64 = 0;
        rs_libp2p_cdr_buffer_read_int64(read_buffer, &mut result);
        assert_eq!(result, -1234567890123);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_read_float() {
        let write_buffer = create_write_buffer();
        let expected: f32 = 3.14159;
        rs_libp2p_cdr_buffer_write_float(write_buffer, expected);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: f32 = 0.0;
        rs_libp2p_cdr_buffer_read_float(read_buffer, &mut result);
        assert!((result - expected).abs() < 0.0001);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_read_double() {
        let write_buffer = create_write_buffer();
        let expected: f64 = 3.141592653589793;
        rs_libp2p_cdr_buffer_write_double(write_buffer, expected);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: f64 = 0.0;
        rs_libp2p_cdr_buffer_read_double(read_buffer, &mut result);
        assert!((result - expected).abs() < 0.0000001);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_read_bool_true() {
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_bool(write_buffer, true);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: bool = false;
        rs_libp2p_cdr_buffer_read_bool(read_buffer, &mut result);
        assert!(result);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_read_bool_false() {
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_bool(write_buffer, false);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: bool = true;
        rs_libp2p_cdr_buffer_read_bool(read_buffer, &mut result);
        assert!(!result);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_read_char() {
        let write_buffer = create_write_buffer();
        let expected: c_char = b'X' as c_char;
        rs_libp2p_cdr_buffer_write_char(write_buffer, expected);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: c_char = 0;
        rs_libp2p_cdr_buffer_read_char(read_buffer, &mut result);
        assert_eq!(result, expected);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_read_char16() {
        let write_buffer = create_write_buffer();
        let expected: u16 = 0x0041; // 'A' in UTF-16
        rs_libp2p_cdr_buffer_write_char16(write_buffer, expected);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: u16 = 0;
        rs_libp2p_cdr_buffer_read_char16(read_buffer, &mut result);
        assert_eq!(result, expected);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_write_read_uint8_boundary_values() {
        // Test min value (0)
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_uint8(write_buffer, u8::MIN);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: u8 = 255;
        rs_libp2p_cdr_buffer_read_uint8(read_buffer, &mut result);
        assert_eq!(result, u8::MIN);
        rs_libp2p_cdr_buffer_free(read_buffer);

        // Test max value (255)
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_uint8(write_buffer, u8::MAX);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: u8 = 0;
        rs_libp2p_cdr_buffer_read_uint8(read_buffer, &mut result);
        assert_eq!(result, u8::MAX);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_write_read_int32_boundary_values() {
        // Test min value
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_int32(write_buffer, i32::MIN);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: i32 = 0;
        rs_libp2p_cdr_buffer_read_int32(read_buffer, &mut result);
        assert_eq!(result, i32::MIN);
        rs_libp2p_cdr_buffer_free(read_buffer);

        // Test max value
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_int32(write_buffer, i32::MAX);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: i32 = 0;
        rs_libp2p_cdr_buffer_read_int32(read_buffer, &mut result);
        assert_eq!(result, i32::MAX);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_write_read_multiple_values_sequential() {
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_uint32(write_buffer, 100);
        rs_libp2p_cdr_buffer_write_uint32(write_buffer, 200);
        rs_libp2p_cdr_buffer_write_uint32(write_buffer, 300);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result1: u32 = 0;
        let mut result2: u32 = 0;
        let mut result3: u32 = 0;
        rs_libp2p_cdr_buffer_read_uint32(read_buffer, &mut result1);
        rs_libp2p_cdr_buffer_read_uint32(read_buffer, &mut result2);
        rs_libp2p_cdr_buffer_read_uint32(read_buffer, &mut result3);
        assert_eq!(result1, 100);
        assert_eq!(result2, 200);
        assert_eq!(result3, 300);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_write_read_mixed_types() {
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_uint8(write_buffer, 42);
        rs_libp2p_cdr_buffer_write_int32(write_buffer, -1000);
        rs_libp2p_cdr_buffer_write_double(write_buffer, 3.14);
        rs_libp2p_cdr_buffer_write_bool(write_buffer, true);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut u8_result: u8 = 0;
        let mut i32_result: i32 = 0;
        let mut f64_result: f64 = 0.0;
        let mut bool_result: bool = false;

        rs_libp2p_cdr_buffer_read_uint8(read_buffer, &mut u8_result);
        rs_libp2p_cdr_buffer_read_int32(read_buffer, &mut i32_result);
        rs_libp2p_cdr_buffer_read_double(read_buffer, &mut f64_result);
        rs_libp2p_cdr_buffer_read_bool(read_buffer, &mut bool_result);

        assert_eq!(u8_result, 42);
        assert_eq!(i32_result, -1000);
        assert!((f64_result - 3.14).abs() < 0.0001);
        assert!(bool_result);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_buffer_free_null_pointer() {
        // Should not panic when freeing null pointer
        rs_libp2p_cdr_buffer_free(std::ptr::null_mut());
    }

    #[test]
    fn test_string_free_null_pointer() {
        // Should not panic when freeing null pointer
        rs_libp2p_cdr_buffer_free_string(std::ptr::null_mut());
    }

    #[test]
    fn test_write_read_float_special_values() {
        // Test zero
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_float(write_buffer, 0.0);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: f32 = 1.0;
        rs_libp2p_cdr_buffer_read_float(read_buffer, &mut result);
        assert_eq!(result, 0.0);
        rs_libp2p_cdr_buffer_free(read_buffer);

        // Test negative zero
        let write_buffer = create_write_buffer();
        rs_libp2p_cdr_buffer_write_float(write_buffer, -0.0);
        let bytes = get_buffer_bytes(write_buffer);
        rs_libp2p_cdr_buffer_free(write_buffer);

        let read_buffer = create_read_buffer(&bytes);
        let mut result: f32 = 1.0;
        rs_libp2p_cdr_buffer_read_float(read_buffer, &mut result);
        // -0.0 == 0.0 in floating point comparison
        assert_eq!(result, 0.0);
        rs_libp2p_cdr_buffer_free(read_buffer);
    }

    #[test]
    fn test_buffer_creation_and_destruction() {
        // Test that we can create and destroy buffers without memory issues
        for _ in 0..100 {
            let buffer = create_write_buffer();
            rs_libp2p_cdr_buffer_write_uint64(buffer, 12345);
            rs_libp2p_cdr_buffer_free(buffer);
        }
    }
}
