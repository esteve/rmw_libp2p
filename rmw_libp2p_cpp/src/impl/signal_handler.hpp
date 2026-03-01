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

#ifndef IMPL__SIGNAL_HANDLER_HPP_
#define IMPL__SIGNAL_HANDLER_HPP_

#include "impl/rmw_libp2p_rs.hpp"

namespace rmw_libp2p_cpp
{

/// Register a libp2p node for graceful shutdown on SIGINT.
///
/// This function adds the node to a global registry and installs
/// a SIGINT handler on first invocation. The handler will call
/// rs_libp2p_trigger_shutdown on all registered nodes.
///
/// @param node Pointer to the libp2p node to register.
void register_node_for_shutdown(rs_libp2p_custom_node_t * node);

/// Unregister a libp2p node from the shutdown handler.
///
/// This function removes the node from the global registry.
/// Should be called before the node is freed.
///
/// @param node Pointer to the libp2p node to unregister.
void unregister_node_for_shutdown(rs_libp2p_custom_node_t * node);

}  // namespace rmw_libp2p_cpp

#endif  // IMPL__SIGNAL_HANDLER_HPP_
