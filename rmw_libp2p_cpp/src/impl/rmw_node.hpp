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

#ifndef IMPL__RMW_NODE_HPP_
#define IMPL__RMW_NODE_HPP_

#include "rmw/rmw.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct rs_libp2p_custom_node rs_libp2p_custom_node_t;

extern rs_libp2p_custom_node_t * rs_libp2p_custom_node_new(void);

extern void rs_libp2p_custom_node_free(rs_libp2p_custom_node_t *);

#ifdef __cplusplus
}
#endif

#endif  // IMPL__RMW_NODE_HPP_
