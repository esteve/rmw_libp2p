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

#ifndef RMW_LIBP2P_CPP__RMW_LIBP2P_RS_HPP_
#define RMW_LIBP2P_CPP__RMW_LIBP2P_RS_HPP_

#ifdef __cplusplus
extern "C"
{
#endif

  typedef struct rs_libp2p_custom_node rs_libp2p_custom_node_t;

  typedef struct rs_libp2p_custom_publisher rs_libp2p_custom_publisher_t;

  typedef struct rs_libp2p_cdr_buffer rs_libp2p_cdr_buffer_t;

  extern rs_libp2p_custom_node_t *
  rs_libp2p_custom_node_new(void);

  extern void
  rs_libp2p_custom_node_free(rs_libp2p_custom_node_t *);

  extern rs_libp2p_custom_publisher_t *
  rs_libp2p_custom_publisher_new(rs_libp2p_custom_node_t *, const char *);

  extern void
  rs_libp2p_custom_publisher_free(rs_libp2p_custom_publisher_t *);

  extern size_t
  rs_libp2p_custom_publisher_get_gid(rs_libp2p_custom_publisher_t *, uint8_t *);

  extern rs_libp2p_cdr_buffer_t *
  rs_libp2p_cdr_buffer_new(void);

  extern void
  rs_libp2p_cdr_buffer_free(rs_libp2p_cdr_buffer_t *);

  extern uint32_t rs_libp2p_custom_publisher_publish(rs_libp2p_custom_publisher_t *, rs_libp2p_cdr_buffer *);

  struct rmw_context_impl_s
  {
    void *rs_event_loop_thread;
    bool is_shutdown;
    void *rs_local_key;
  };

  void *rs_rmw_init();

#ifdef __cplusplus
}
#endif

#endif // RMW_LIBP2P_CPP__RMW_LIBP2P_RS_HPP_
