// Copyright 2023 Esteve Fernandez All rights reserved.
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

#include <condition_variable>
#include <mutex>

#include "rcutils/logging_macros.h"

#include "rmw/error_handling.h"
#include "rmw/rmw.h"

#include "rmw_libp2p_cpp/custom_subscription_info.hpp"
#include "rmw_libp2p_cpp/custom_wait_set_info.hpp"

rmw_ret_t
rmw_wait(
  rmw_subscriptions_t * subscriptions,
  rmw_guard_conditions_t * guard_conditions,
  rmw_services_t * services,
  rmw_clients_t * clients,
  rmw_events_t * events,
  rmw_wait_set_t * wait_set,
  const rmw_time_t * wait_timeout)
{
  RCUTILS_LOG_WARN_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscriptions;
  (void)guard_conditions;
  (void)services;
  (void)clients;
  (void)events;
  (void)wait_set;
  (void)wait_timeout;

  if (events && events->event_count) {
    RMW_SET_ERROR_MSG("unimplemented");
    return RMW_RET_ERROR;
  }
  if (!wait_set) {
    RMW_SET_ERROR_MSG("wait set handle is null");
    return RMW_RET_ERROR;
  }
  CustomWaitsetInfo * wait_set_info = static_cast<CustomWaitsetInfo *>(wait_set->data);
  if (!wait_set_info) {
    RMW_SET_ERROR_MSG("Waitset info struct is null");
    return RMW_RET_ERROR;
  }
  std::mutex * conditionMutex = &wait_set_info->condition_mutex;
  std::condition_variable * conditionVariable = &wait_set_info->condition;
  if (!conditionMutex) {
    RMW_SET_ERROR_MSG("Mutex for wait set was null");
    return RMW_RET_ERROR;
  }
  if (!conditionVariable) {
    RMW_SET_ERROR_MSG("Condition variable for wait set was null");
    return RMW_RET_ERROR;
  }

  // if (subscriptions) {
  //   for (size_t i = 0; i < subscriptions->subscriber_count; ++i) {
  //     void * data = subscriptions->subscribers[i];
  //     auto custom_subscriber_info = static_cast<CustomSubscriptionInfo *>(data);
  //     // Short circuiting out of this function is possible
  //     if (custom_subscriber_info && custom_subscriber_info->listener_->hasData()) {
  //       return true;
  //     }
  //   }
  // }

  // return RMW_RET_ERROR;
  // return RMW_RET_OK;
  return RMW_RET_TIMEOUT;
}
