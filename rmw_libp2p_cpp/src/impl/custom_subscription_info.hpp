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

#ifndef IMPL__CUSTOM_SUBSCRIPTION_INFO_HPP_
#define IMPL__CUSTOM_SUBSCRIPTION_INFO_HPP_

#include <atomic>
#include <queue>
#include <set>
#include <string>

#include "rmw/rmw.h"

#include "impl/rmw_libp2p_rs.hpp"

namespace rmw_libp2p_cpp
{
struct Listener;

typedef struct CustomSubscriptionInfo
{
  const rmw_node_t * node_;
  rmw_libp2p_cpp::Listener * listener_;
  void * type_support_;
  const char * typesupport_identifier_;
  rmw_qos_profile_t qos_;
  rs_libp2p_custom_subscription_t * subscription_handle_;
} CustomSubscriptionInfo;
}  // namespace rmw_libp2p_cpp
#endif  // IMPL__CUSTOM_SUBSCRIPTION_INFO_HPP_
