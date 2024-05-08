// Copyright 2022 Esteve Fernandez All rights reserved.
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

#ifndef IMPL__RMW_LIBP2P_RS_HPP_
#define IMPL__RMW_LIBP2P_RS_HPP_

#ifdef __cplusplus
extern "C"
{
#endif

namespace rmw_libp2p_cpp
{
struct CustomSubscriptionHandle;

struct CustomSubscriptionInfo;
}

typedef struct rs_libp2p_custom_node rs_libp2p_custom_node_t;

typedef struct rs_libp2p_custom_publisher rs_libp2p_custom_publisher_t;

typedef struct rs_libp2p_custom_subscription rs_libp2p_custom_subscription_t;

typedef struct rs_libp2p_cdr_buffer rs_libp2p_cdr_buffer_t;

extern rs_libp2p_custom_node_t *
rs_libp2p_custom_node_new();

extern void
rs_libp2p_custom_node_free(rs_libp2p_custom_node_t *);

extern rs_libp2p_custom_publisher_t *
rs_libp2p_custom_publisher_new(rs_libp2p_custom_node_t *, const char *);

extern void
rs_libp2p_custom_publisher_free(rs_libp2p_custom_publisher_t *);

extern size_t
rs_libp2p_custom_publisher_get_gid(rs_libp2p_custom_publisher_t *, uint8_t *);

extern rs_libp2p_custom_subscription_t *
rs_libp2p_custom_subscription_new(
  rs_libp2p_custom_node_t *, const char *, const rmw_libp2p_cpp::CustomSubscriptionInfo *,
  void (*)(const rmw_libp2p_cpp::CustomSubscriptionHandle *, uint8_t *, const uintptr_t)
);

extern void
rs_libp2p_custom_subscription_free(rs_libp2p_custom_subscription_t *);

extern size_t
rs_libp2p_custom_subscription_get_gid(rs_libp2p_custom_subscription_t *, uint8_t *);

extern rs_libp2p_cdr_buffer_t *
rs_libp2p_cdr_buffer_write_new();

extern rs_libp2p_cdr_buffer_t *
rs_libp2p_cdr_buffer_read_new(const uint8_t *, size_t);

extern void
rs_libp2p_cdr_buffer_free(rs_libp2p_cdr_buffer_t *);

extern uint32_t rs_libp2p_custom_publisher_publish(
  rs_libp2p_custom_publisher_t *,
  const rs_libp2p_cdr_buffer *);

struct rmw_context_impl_s
{
  void * rs_event_loop_thread;
  bool is_shutdown;
  void * rs_local_key;
};

void * rs_rmw_init();

extern void
rs_libp2p_cdr_buffer_read_uint8(rs_libp2p_cdr_buffer_t *, uint8_t *);

extern void
rs_libp2p_cdr_buffer_read_uint16(rs_libp2p_cdr_buffer_t *, uint16_t *);

extern void
rs_libp2p_cdr_buffer_read_uint32(rs_libp2p_cdr_buffer_t *, uint32_t *);

extern void
rs_libp2p_cdr_buffer_read_uint64(rs_libp2p_cdr_buffer_t *, uint64_t *);

extern void
rs_libp2p_cdr_buffer_read_int8(rs_libp2p_cdr_buffer_t *, int8_t *);

extern void
rs_libp2p_cdr_buffer_read_int16(rs_libp2p_cdr_buffer_t *, int16_t *);

extern void
rs_libp2p_cdr_buffer_read_int32(rs_libp2p_cdr_buffer_t *, int32_t *);

extern void
rs_libp2p_cdr_buffer_read_int64(rs_libp2p_cdr_buffer_t *, int64_t *);

extern void
rs_libp2p_cdr_buffer_read_char(rs_libp2p_cdr_buffer_t *, char *);

extern void
rs_libp2p_cdr_buffer_read_char16(rs_libp2p_cdr_buffer_t *, char16_t *);

extern void
rs_libp2p_cdr_buffer_read_float(rs_libp2p_cdr_buffer_t *, float *);

extern void
rs_libp2p_cdr_buffer_read_double(rs_libp2p_cdr_buffer_t *, double *);

extern void
rs_libp2p_cdr_buffer_read_bool(rs_libp2p_cdr_buffer_t *, bool *);

extern void
rs_libp2p_cdr_buffer_read_string(rs_libp2p_cdr_buffer_t *, char **, size_t *);

extern void
rs_libp2p_cdr_buffer_free_string(char *);

extern void
rs_libp2p_cdr_buffer_read_u16string(rs_libp2p_cdr_buffer_t *, char16_t **, size_t *);

extern void
rs_libp2p_cdr_buffer_write_uint8(rs_libp2p_cdr_buffer_t *, uint8_t);

extern void
rs_libp2p_cdr_buffer_write_uint16(rs_libp2p_cdr_buffer_t *, uint16_t);

extern void
rs_libp2p_cdr_buffer_write_uint32(rs_libp2p_cdr_buffer_t *, uint32_t);

extern void
rs_libp2p_cdr_buffer_write_uint64(rs_libp2p_cdr_buffer_t *, uint64_t);

extern void
rs_libp2p_cdr_buffer_write_int8(rs_libp2p_cdr_buffer_t *, int8_t);

extern void
rs_libp2p_cdr_buffer_write_int16(rs_libp2p_cdr_buffer_t *, int16_t);

extern void
rs_libp2p_cdr_buffer_write_int32(rs_libp2p_cdr_buffer_t *, int32_t);

extern void
rs_libp2p_cdr_buffer_write_int64(rs_libp2p_cdr_buffer_t *, int64_t);

extern void
rs_libp2p_cdr_buffer_write_char(rs_libp2p_cdr_buffer_t *, char);

extern void
rs_libp2p_cdr_buffer_write_char16(rs_libp2p_cdr_buffer_t *, char16_t);

extern void
rs_libp2p_cdr_buffer_write_float(rs_libp2p_cdr_buffer_t *, float);

extern void
rs_libp2p_cdr_buffer_write_double(rs_libp2p_cdr_buffer_t *, double);

#ifdef __cplusplus
}
#endif

#endif  // IMPL__RMW_LIBP2P_RS_HPP_
