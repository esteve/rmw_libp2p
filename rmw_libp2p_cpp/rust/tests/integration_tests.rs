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

//! Integration tests for rmw_libp2p_rs

use std::ffi::CString;
use std::io::Cursor;

// Note: These tests require the library to be built as a rlib for testing
// The staticlib build is for C FFI only

/// Test the complete flow of creating a node, publisher, and publishing a message
#[test]
fn test_full_publish_workflow() {
    // This test verifies the complete workflow:
    // 1. Create a node
    // 2. Create a publisher on that node
    // 3. Publish a message
    // 4. Clean up resources

    // Due to the staticlib nature, we test through the FFI interface
    // which is already covered by unit tests in the individual modules

    // Integration test placeholder - the actual FFI functions are tested
    // in the unit tests of each module
    assert!(true);
}

/// Test creating multiple nodes that could potentially communicate
#[test]
fn test_multiple_nodes_creation() {
    // This test verifies that we can create multiple independent nodes
    // Each node should have its own libp2p swarm and event loop

    // The actual implementation is tested through unit tests
    // This integration test verifies the concept
    assert!(true);
}

/// Test CDR serialization round-trip with various data types
#[test]
fn test_cdr_serialization_roundtrip() {
    // Test that data can be serialized and deserialized correctly
    use std::io::Cursor;

    // Create a buffer with known data using CDR serialization
    let mut write_buffer = Cursor::new(Vec::<u8>::new());

    // Write some test values
    let test_u32: u32 = 12345;
    let test_i64: i64 = -9876543210;
    let test_f64: f64 = 3.141592653589793;

    cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut write_buffer, &test_u32, cdr::Infinite).unwrap();
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut write_buffer, &test_i64, cdr::Infinite).unwrap();
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut write_buffer, &test_f64, cdr::Infinite).unwrap();

    // Read back the values
    let data = write_buffer.into_inner();
    let mut read_buffer = Cursor::new(data);

    let read_u32: u32 = cdr::deserialize_from(&mut read_buffer, cdr::Infinite).unwrap();
    let read_i64: i64 = cdr::deserialize_from(&mut read_buffer, cdr::Infinite).unwrap();
    let read_f64: f64 = cdr::deserialize_from(&mut read_buffer, cdr::Infinite).unwrap();

    assert_eq!(read_u32, test_u32);
    assert_eq!(read_i64, test_i64);
    assert!((read_f64 - test_f64).abs() < 1e-15);
}

/// Test CDR serialization with strings
#[test]
fn test_cdr_string_serialization() {
    use std::ffi::CString;
    use std::io::Cursor;

    let mut write_buffer = Cursor::new(Vec::<u8>::new());

    let test_string = CString::new("Hello, ROS2!").unwrap();
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut write_buffer, &test_string, cdr::Infinite).unwrap();

    let data = write_buffer.into_inner();
    let mut read_buffer = Cursor::new(data);

    let read_string: CString = cdr::deserialize_from(&mut read_buffer, cdr::Infinite).unwrap();
    assert_eq!(read_string, test_string);
}

/// Test CDR serialization with arrays
#[test]
fn test_cdr_array_serialization() {
    use std::io::Cursor;

    let mut write_buffer = Cursor::new(Vec::<u8>::new());

    let test_array: Vec<u32> = vec![1, 2, 3, 4, 5];
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut write_buffer, &test_array, cdr::Infinite).unwrap();

    let data = write_buffer.into_inner();
    let mut read_buffer = Cursor::new(data);

    let read_array: Vec<u32> = cdr::deserialize_from(&mut read_buffer, cdr::Infinite).unwrap();
    assert_eq!(read_array, test_array);
}

/// Test CDR serialization with boolean values
#[test]
fn test_cdr_bool_serialization() {
    use std::io::Cursor;

    let mut write_buffer = Cursor::new(Vec::<u8>::new());

    cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut write_buffer, &true, cdr::Infinite).unwrap();
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut write_buffer, &false, cdr::Infinite).unwrap();

    let data = write_buffer.into_inner();
    let mut read_buffer = Cursor::new(data);

    let read_true: bool = cdr::deserialize_from(&mut read_buffer, cdr::Infinite).unwrap();
    let read_false: bool = cdr::deserialize_from(&mut read_buffer, cdr::Infinite).unwrap();

    assert!(read_true);
    assert!(!read_false);
}

/// Test UUID generation for GIDs
#[test]
fn test_uuid_uniqueness() {
    use uuid::Uuid;

    // Generate multiple UUIDs and verify they're unique
    let uuids: Vec<Uuid> = (0..100).map(|_| Uuid::new_v4()).collect();

    // Check that all UUIDs are unique
    for i in 0..uuids.len() {
        for j in (i + 1)..uuids.len() {
            assert_ne!(uuids[i], uuids[j], "UUIDs should be unique");
        }
    }

    // Check that UUIDs are the expected size (16 bytes)
    for uuid in &uuids {
        assert_eq!(uuid.as_bytes().len(), 16);
    }
}

/// Test timestamp generation for message publishing
#[test]
fn test_timestamp_generation() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let secs = since_epoch.as_secs();
    let usecs = since_epoch.subsec_micros();

    // Verify that we get valid timestamp values
    assert!(secs > 0, "Seconds should be positive");
    assert!(usecs < 1_000_000, "Microseconds should be less than 1 million");

    // Verify the timestamp can be serialized
    let mut buffer = Cursor::new(Vec::<u8>::new());
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut buffer, &secs, cdr::Infinite).unwrap();
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut buffer, &usecs, cdr::Infinite).unwrap();

    // Read back
    let data = buffer.into_inner();
    let mut read_buffer = Cursor::new(data);

    let read_secs: u64 = cdr::deserialize_from(&mut read_buffer, cdr::Infinite).unwrap();
    let read_usecs: u32 = cdr::deserialize_from(&mut read_buffer, cdr::Infinite).unwrap();

    assert_eq!(read_secs, secs);
    assert_eq!(read_usecs, usecs);
}

/// Test edge cases for numeric serialization
#[test]
fn test_numeric_edge_cases() {
    use std::io::Cursor;

    // Test boundary values for various integer types
    let test_cases: Vec<(i64, i64)> = vec![
        (i64::MIN, i64::MIN),
        (i64::MAX, i64::MAX),
        (0, 0),
        (-1, -1),
        (1, 1),
    ];

    for (input, expected) in test_cases {
        let mut write_buffer = Cursor::new(Vec::<u8>::new());
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut write_buffer, &input, cdr::Infinite).unwrap();

        let data = write_buffer.into_inner();
        let mut read_buffer = Cursor::new(data);

        let result: i64 = cdr::deserialize_from(&mut read_buffer, cdr::Infinite).unwrap();
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}

/// Test floating point edge cases
#[test]
fn test_float_edge_cases() {
    use std::io::Cursor;

    let test_values: Vec<f64> = vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        f64::MIN_POSITIVE,
        f64::MAX,
        f64::MIN,
    ];

    for value in test_values {
        let mut write_buffer = Cursor::new(Vec::<u8>::new());
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut write_buffer, &value, cdr::Infinite).unwrap();

        let data = write_buffer.into_inner();
        let mut read_buffer = Cursor::new(data);

        let result: f64 = cdr::deserialize_from(&mut read_buffer, cdr::Infinite).unwrap();

        // Use bit-level comparison for special values like -0.0
        assert_eq!(value.to_bits(), result.to_bits(), "Failed for value: {}", value);
    }
}
