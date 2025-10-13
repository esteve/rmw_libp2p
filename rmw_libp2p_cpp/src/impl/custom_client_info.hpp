// Copyright 2025 Esteve Fernandez All rights reserved.
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

#ifndef IMPL__CUSTOM_CLIENT_INFO_HPP_
#define IMPL__CUSTOM_CLIENT_INFO_HPP_

#include <string>

#include "rmw/rmw.h"

#include "impl/rmw_libp2p_rs.hpp"

namespace rmw_libp2p_cpp
{
typedef struct CustomClientInfo
{
  void * request_type_support_;
  void * response_type_support_;
  rmw_libp2p_cpp::Listener * listener_;
  const rmw_node_t * node_;
  const char * typesupport_identifier_;
  std::string discovery_name_;
  rs_libp2p_custom_publisher_t * publisher_handle_;
} CustomClientInfo;
}  // namespace rmw_libp2p_cpp

#endif  // IMPL__CUSTOM_CLIENT_INFO_HPP_