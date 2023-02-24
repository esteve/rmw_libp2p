use std::ffi::CString;
use std::io::Cursor;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_new() -> *mut Cursor<Vec<u8>> {
    let libp2p2_cdr_buffer = Cursor::new(Vec::<u8>::new());
    Box::into_raw(Box::new(libp2p2_cdr_buffer))
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_free(ptr: *mut Cursor<Vec<u8>>) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_uint64(ptr: *mut Cursor<Vec<u8>>, n: *mut u64) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u64, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_uint32(ptr: *mut Cursor<Vec<u8>>, n: *mut u32) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u32, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_uint16(ptr: *mut Cursor<Vec<u8>>, n: *mut u16) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u16, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_uint8(ptr: *mut Cursor<Vec<u8>>, n: *mut u8) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u8, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_int64(ptr: *mut Cursor<Vec<u8>>, n: *mut i64) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, i64, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_int32(ptr: *mut Cursor<Vec<u8>>, n: *mut i32) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, i32, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_int16(ptr: *mut Cursor<Vec<u8>>, n: *mut i16) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, i16, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_int8(ptr: *mut Cursor<Vec<u8>>, n: *mut i8) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, i8, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_char(ptr: *mut Cursor<Vec<u8>>, n: *mut c_char) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, c_char, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_char16(ptr: *mut Cursor<Vec<u8>>, n: *mut u16) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u16, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_float(ptr: *mut Cursor<Vec<u8>>, n: *mut f32) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, f32, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_double(ptr: *mut Cursor<Vec<u8>>, n: *mut f64) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, f64, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_bool(ptr: *mut Cursor<Vec<u8>>, n: *mut bool) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, bool, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_string(
    ptr: *mut Cursor<Vec<u8>>,
    s: *mut *const c_char,
    size: *mut usize,
) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let cs = cdr::deserialize_from::<_, CString, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    unsafe {
        *size = cs.as_bytes().len();

        if *size != 0 {
            *s = cs.into_raw();
        }
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_u16string(
    ptr: *mut Cursor<Vec<u8>>,
    s: *mut *const u16,
    size: *mut usize,
) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let cs = cdr::deserialize_from::<_, Vec<u16>, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    unsafe {
        *size = cs.len();

        if *size != 0 {
            *s = cs.as_ptr();
        }
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_uint64(ptr: *mut Cursor<Vec<u8>>, n: u64) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_uint32(ptr: *mut Cursor<Vec<u8>>, n: u32) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_uint16(ptr: *mut Cursor<Vec<u8>>, n: u16) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_uint8(ptr: *mut Cursor<Vec<u8>>, n: u8) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_int64(ptr: *mut Cursor<Vec<u8>>, n: i64) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_int32(ptr: *mut Cursor<Vec<u8>>, n: i32) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_int16(ptr: *mut Cursor<Vec<u8>>, n: i16) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_int8(ptr: *mut Cursor<Vec<u8>>, n: i8) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_char(ptr: *mut Cursor<Vec<u8>>, n: c_char) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_char16(ptr: *mut Cursor<Vec<u8>>, n: u16) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_float(ptr: *mut Cursor<Vec<u8>>, n: f32) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_double(ptr: *mut Cursor<Vec<u8>>, n: f64) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_bool(ptr: *mut Cursor<Vec<u8>>, n: bool) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}
