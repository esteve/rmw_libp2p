// Copyright 2024 Esteve Fernandez All rights reserved.
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

#ifndef IMPL__RMW_LIBP2P_RS_H_
#define IMPL__RMW_LIBP2P_RS_H_

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// This module contains the implementation of a custom node in the Libp2p network.
/// The `Libp2pCustomNode` struct represents a custom node and provides methods for creating and
/// interacting with the node.
/// The node uses the `RosNetworkBehaviour` struct as its network behavior, which combines the
/// `gossipsub` and `mdns` behaviors.
/// The node can publish messages to the network, subscribe to topics, and handle incoming
/// messages.
/// The node runs in its own thread and uses a `Swarm` instance to manage the network behavior.
/// The node also uses a `Queue` to store outgoing messages and a `HashMap` to store subscription
/// callbacks.
/// The `Libp2pCustomNode` struct provides methods for creating a new node, publishing messages,
/// and stopping the node.
/// The node is designed to be used in a multithreaded environment and provides thread-safe access
/// to its internal data structures.
struct Libp2pCustomNode;

/// Represents a custom publisher using the Libp2p protocol.
struct Libp2pCustomPublisher;

/// Represents a custom subscription in the Libp2p network.
///
/// This struct holds a unique identifier (UUID), a pointer to the associated `Libp2pCustomNode`,
/// the topic of the subscription,
/// and a queue for incoming messages.
///
/// # Fields
///
/// * `gid` - A unique identifier for this subscription.
/// * `node` - A raw pointer to the `Libp2pCustomNode` associated with this subscription. This is
///    needed to access the outgoing queue.
/// * `topic` - The topic of the subscription.
/// * `incoming_queue` - A thread-safe, unlimited queue for incoming messages. Each message is a
///    tuple of the topic and the message data.
///
/// # Safety
///
/// This struct is unsafe because it uses raw pointers.
struct Libp2pCustomSubscription;

template < typename T = void >
  struct Vec;

struct CustomSubscriptionHandle
{
  const void * ptr;
};

extern "C" {

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
void rs_libp2p_cdr_buffer_free(Cursor < Vec < uint8_t >> * ptr);

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
Cursor < Vec < uint8_t >> *rs_libp2p_cdr_buffer_read_new(const uint8_t * data, uintptr_t length);

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
void rs_libp2p_cdr_buffer_read_uint64(Cursor < Vec < uint8_t >> * ptr, uint64_t * n);

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
void rs_libp2p_cdr_buffer_read_uint32(Cursor < Vec < uint8_t >> * ptr, uint32_t * n);

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
void rs_libp2p_cdr_buffer_read_uint16(Cursor < Vec < uint8_t >> * ptr, uint16_t * n);

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
void rs_libp2p_cdr_buffer_read_uint8(Cursor < Vec < uint8_t >> * ptr, uint8_t * n);

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
void rs_libp2p_cdr_buffer_read_int64(Cursor < Vec < uint8_t >> * ptr, int64_t * n);

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
void rs_libp2p_cdr_buffer_read_int32(Cursor < Vec < uint8_t >> * ptr, int32_t * n);

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
void rs_libp2p_cdr_buffer_read_int16(Cursor < Vec < uint8_t >> * ptr, int16_t * n);

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
void rs_libp2p_cdr_buffer_read_int8(Cursor < Vec < uint8_t >> * ptr, int8_t * n);

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
void rs_libp2p_cdr_buffer_read_char(Cursor < Vec < uint8_t >> * ptr, char * n);

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
void rs_libp2p_cdr_buffer_read_char16(Cursor < Vec < uint8_t >> * ptr, uint16_t * n);

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
void rs_libp2p_cdr_buffer_read_float(Cursor < Vec < uint8_t >> * ptr, float * n);

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
void rs_libp2p_cdr_buffer_read_double(Cursor < Vec < uint8_t >> * ptr, double * n);

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
void rs_libp2p_cdr_buffer_read_bool(Cursor < Vec < uint8_t >> * ptr, bool * n);

/// Deserializes a `CString` from a `Cursor<Vec<u8>>` and stores the raw pointer and length of the
/// string.
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
void rs_libp2p_cdr_buffer_read_string(
  Cursor < Vec < uint8_t >> * ptr,
  const char ** s,
  uintptr_t * size);

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
void rs_libp2p_cdr_buffer_free_string(char * s);

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
void rs_libp2p_cdr_buffer_read_u16string(
  Cursor < Vec < uint8_t >> * ptr,
  const uint16_t ** s,
  uintptr_t * size);

/// Creates a new `Cursor<Vec<u8>>` to write to.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Returns
///
/// A raw pointer to a `Cursor<Vec<u8>>`.
Cursor < Vec < uint8_t >> *rs_libp2p_cdr_buffer_write_new();

/// Writes a `u64` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `u64` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into`
/// function.
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
void rs_libp2p_cdr_buffer_write_uint64(
  Cursor < Vec < uint8_t >> * ptr,
  uint64_t n);

/// Writes a `u32` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `u32` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into`
/// function.
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
void rs_libp2p_cdr_buffer_write_uint32(
  Cursor < Vec < uint8_t >> * ptr,
  uint32_t n);

/// Writes a `u16` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `u16` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into`
/// function.
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
void rs_libp2p_cdr_buffer_write_uint16(
  Cursor < Vec < uint8_t >> * ptr,
  uint16_t n);

/// Writes a `u8` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `u8` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into`
/// function.
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
void rs_libp2p_cdr_buffer_write_uint8(
  Cursor < Vec < uint8_t >> * ptr,
  uint8_t n);

/// Writes a `i64` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `i64` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into`
/// function.
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
void rs_libp2p_cdr_buffer_write_int64(
  Cursor < Vec < uint8_t >> * ptr,
  int64_t n);

/// Writes a `i32` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `i32` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into`
/// function.
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
void rs_libp2p_cdr_buffer_write_int32(
  Cursor < Vec < uint8_t >> * ptr,
  int32_t n);

/// Writes a `i16` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `i16` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into`
/// function.
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
void rs_libp2p_cdr_buffer_write_int16(
  Cursor < Vec < uint8_t >> * ptr,
  int16_t n);

/// Writes a `i8` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `i8` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into`
/// function.
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
void rs_libp2p_cdr_buffer_write_int8(
  Cursor < Vec < uint8_t >> * ptr,
  int8_t n);

/// Writes a `c_char` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `c_char` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into`
/// function.
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
void rs_libp2p_cdr_buffer_write_char(
  Cursor < Vec < uint8_t >> * ptr,
  char n);

/// Writes a 16-bit `char` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `u16` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into`
/// function.
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
void rs_libp2p_cdr_buffer_write_char16(
  Cursor < Vec < uint8_t >> * ptr,
  uint16_t n);

/// Writes a `float` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `f32` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into`
/// function.
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
void rs_libp2p_cdr_buffer_write_float(
  Cursor < Vec < uint8_t >> * ptr,
  float n);

/// Writes a `double` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `f64` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into`
/// function.
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
void rs_libp2p_cdr_buffer_write_double(
  Cursor < Vec < uint8_t >> * ptr,
  double n);

/// Writes a `bool` to a `Cursor<Vec<u8>>`.
///
/// This function serializes a `bool` into a `Cursor<Vec<u8>>` using the `cdr::serialize_into`
/// function.
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
void rs_libp2p_cdr_buffer_write_bool(
  Cursor < Vec < uint8_t >> * ptr,
  bool n);

/// Creates a new `Libp2pCustomNode`.
///
/// This function allocates a `Libp2pCustomNode` on the heap, then returns a raw pointer to the
/// heap-allocated object.
///
/// # Safety
///
/// This function is unsafe because it returns a raw pointer to a heap-allocated object. The
/// caller is responsible for freeing this memory.
///
/// # Returns
///
/// A raw pointer to a `Libp2pCustomNode`.
Libp2pCustomNode * rs_libp2p_custom_node_new();

/// Frees a `Libp2pCustomNode` from memory.
///
/// This function takes a raw pointer to a `Libp2pCustomNode`, converts it back into a `Box`, and
/// then drops the `Box`, freeing the memory.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Libp2pCustomNode`.
void rs_libp2p_custom_node_free(Libp2pCustomNode * ptr);

/// Creates a new `Libp2pCustomPublisher`.
///
/// This function takes a raw pointer to a `Libp2pCustomNode` and a raw pointer to a C string
/// representing the topic.
/// It then creates a new `Libp2pCustomPublisher` for the given node and topic, and returns a raw
/// pointer to the heap-allocated publisher.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers and calls unsafe functions.
///
/// # Arguments
///
/// * `ptr_node` - A raw pointer to a `Libp2pCustomNode`.
/// * `topic_str_ptr` - A raw pointer to a C string representing the topic.
///
/// # Returns
///
/// A raw pointer to a `Libp2pCustomPublisher`.
///
/// # Panics
///
/// This function will panic if `topic_str_ptr` is null or if it does not point to a valid
/// null-terminated string.
Libp2pCustomPublisher * rs_libp2p_custom_publisher_new(
  Libp2pCustomNode * ptr_node,
  const char * topic_str_ptr);

/// Frees a `Libp2pCustomPublisher` from memory.
///
/// This function takes a raw pointer to a `Libp2pCustomPublisher`, converts it back into a `Box`,
/// and then drops the `Box`, freeing the memory.
/// If the provided pointer is null, the function returns immediately.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Libp2pCustomPublisher`.
void rs_libp2p_custom_publisher_free(Libp2pCustomPublisher * ptr);

/// Gets the GID of a `Libp2pCustomPublisher`.
///
/// This function takes a raw pointer to a `Libp2pCustomPublisher` and a raw pointer to a buffer.
/// It then copies the bytes of the GID of the publisher into the buffer and returns the number
/// of bytes copied.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers and calls unsafe functions.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Libp2pCustomPublisher`.
/// * `buf` - A raw pointer to a buffer where the GID bytes will be copied.
///
/// # Returns
///
/// The number of bytes copied into the buffer.
///
/// # Panics
///
/// This function will panic if `ptr` is null.
uintptr_t rs_libp2p_custom_publisher_get_gid(
  Libp2pCustomPublisher * ptr,
  unsigned char * buf);

/// Publishes a message using a `Libp2pCustomPublisher`.
///
/// This function takes raw pointers to a `Libp2pCustomPublisher` and a `Cursor<Vec<u8>>`.
/// It then publishes the contents of the `Cursor<Vec<u8>>` using the `Libp2pCustomPublisher`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers and calls unsafe functions.
///
/// # Arguments
///
/// * `ptr_publisher` - A raw pointer to a `Libp2pCustomPublisher`.
/// * `ptr_buffer` - A raw pointer to a `Cursor<Vec<u8>>` containing the message to publish.
///
/// # Returns
///
/// Currently, this function always returns 0.
///
/// # Panics
///
/// This function will panic if either `ptr_publisher` or `ptr_buffer` is null.
uintptr_t rs_libp2p_custom_publisher_publish(
  Libp2pCustomPublisher * ptr_publisher,
  const Cursor < Vec < uint8_t >> * ptr_buffer);

/// Creates a new `Libp2pCustomSubscription`.
///
/// This function takes a raw pointer to a `Libp2pCustomNode`, a raw pointer to a C string
/// representing the topic, a `CustomSubscriptionHandle`, and a callback function.
/// It then creates a new `Libp2pCustomSubscription` for the given node and topic, and returns a
/// raw pointer to the heap-allocated subscription.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers and calls unsafe functions.
///
/// # Arguments
///
/// * `ptr_node` - A raw pointer to a `Libp2pCustomNode`.
/// * `topic_str_ptr` - A raw pointer to a C string representing the topic.
/// * `obj` - A `CustomSubscriptionHandle` associated with the new subscription.
/// * `callback` - A callback function to be called when a new message is published to the topic.
///
/// # Returns
///
/// A raw pointer to a `Libp2pCustomSubscription`.
///
/// # Panics
///
/// This function will panic if `topic_str_ptr` is null or if it does not point to a valid
/// null-terminated string.
Libp2pCustomSubscription * rs_libp2p_custom_subscription_new(
  Libp2pCustomNode * ptr_node,
  const char * topic_str_ptr,
  CustomSubscriptionHandle obj,
  void (* callback)(
    const CustomSubscriptionHandle *,
    uint8_t *,
    uintptr_t len));

/// Frees a `Libp2pCustomSubscription` from memory.
///
/// This function takes a raw pointer to a `Libp2pCustomSubscription`, converts it back into a
/// `Box`, and then drops the `Box`, freeing the memory.
/// If the provided pointer is null, the function returns immediately.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr_subscription` - A raw pointer to a `Libp2pCustomSubscription`.
///
/// # Panics
///
/// This function will panic if the provided pointer has been previously deallocated or was not
/// returned by `rs_libp2p_custom_subscription_new`.
void rs_libp2p_custom_subscription_free(Libp2pCustomSubscription * ptr_subscription);

/// Gets the GID of a `Libp2pCustomSubscription`.
///
/// This function takes a raw pointer to a `Libp2pCustomSubscription` and a raw pointer to a
/// buffer.
/// It then copies the bytes of the GID of the subscription into the buffer and returns the number
/// of bytes copied.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers and calls unsafe functions.
///
/// # Arguments
///
/// * `ptr_subscription` - A raw pointer to a `Libp2pCustomSubscription`.
/// * `buf` - A raw pointer to a buffer where the GID bytes will be copied.
///
/// # Returns
///
/// The number of bytes copied into the buffer.
///
/// # Panics
///
/// This function will panic if `ptr_subscription` is null.
uintptr_t rs_libp2p_custom_subscription_get_gid(
  Libp2pCustomSubscription * ptr_subscription,
  unsigned char * buf);

}  // extern "C"

#endif  // IMPL__RMW_LIBP2P_RS_H_
