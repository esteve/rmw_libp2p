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

#ifndef RMW_LIBP2P_CPP__CUSTOM_NODE_INFO_HPP_
#define RMW_LIBP2P_CPP__CUSTOM_NODE_INFO_HPP_

#include <map>
#include <mutex>
#include <set>

#include "rmw/rmw.h"
#include "rmw_libp2p_cpp/custom_publisher_info.hpp"
#include "rmw_libp2p_cpp/custom_subscription_info.hpp"
#include "rmw_libp2p_cpp/rmw_libp2p_rs.hpp"

typedef struct CustomNodeInfo
{
  rs_libp2p_custom_node_t * node_handle_;
  rmw_guard_condition_t * graph_guard_condition_;
  std::mutex publishers_mutex_;
  std::map<std::string, std::set<CustomPublisherInfo *>> publishers_;
  std::mutex subscriptions_mutex_;
  std::map<std::string, std::set<CustomSubscriptionInfo *>> subscriptions_;
} CustomNodeInfo;

#endif  // RMW_LIBP2P_CPP__CUSTOM_NODE_INFO_HPP_
