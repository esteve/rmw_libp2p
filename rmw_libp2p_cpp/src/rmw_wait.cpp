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

#include "impl/custom_subscription_info.hpp"
#include "impl/custom_wait_set_info.hpp"
#include "impl/listener.hpp"

extern "C"
{
// helper function for wait
bool
check_wait_set_for_data(
  const rmw_subscriptions_t * subscriptions,
  const rmw_guard_conditions_t * guard_conditions,
  const rmw_services_t * services,
  const rmw_clients_t * clients)
{
  if (subscriptions) {
    for (size_t i = 0; i < subscriptions->subscriber_count; ++i) {
      void * data = subscriptions->subscribers[i];
      auto custom_subscriber_info = static_cast<rmw_libp2p_cpp::CustomSubscriptionInfo *>(data);
      if (custom_subscriber_info && custom_subscriber_info->listener_->has_data()) {
        return true;
      }
    }
  }

  return false;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_wait(
  rmw_subscriptions_t * subscriptions,
  rmw_guard_conditions_t * guard_conditions,
  rmw_services_t * services,
  rmw_clients_t * clients,
  rmw_events_t * events,
  rmw_wait_set_t * wait_set,
  const rmw_time_t * wait_timeout)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  if (events && events->event_count) {
    RMW_SET_ERROR_MSG("unimplemented");
    return RMW_RET_ERROR;
  }
  if (!wait_set) {
    RMW_SET_ERROR_MSG("wait set handle is null");
    return RMW_RET_ERROR;
  }
  rmw_libp2p_cpp::CustomWaitsetInfo * wait_set_info =
    static_cast<rmw_libp2p_cpp::CustomWaitsetInfo *>(wait_set->data);
  if (!wait_set_info) {
    RMW_SET_ERROR_MSG("Waitset info struct is null");
    return RMW_RET_ERROR;
  }
  std::mutex * condition_mutex = &wait_set_info->condition_mutex;
  std::condition_variable * condition_variable = &wait_set_info->condition;
  if (!condition_mutex) {
    RMW_SET_ERROR_MSG("Mutex for wait set was null");
    return RMW_RET_ERROR;
  }
  if (!condition_variable) {
    RMW_SET_ERROR_MSG("Condition variable for wait set was null");
    return RMW_RET_ERROR;
  }

  if (subscriptions) {
    for (size_t i = 0; i < subscriptions->subscriber_count; ++i) {
      void * data = subscriptions->subscribers[i];
      auto custom_subscriber_info = static_cast<rmw_libp2p_cpp::CustomSubscriptionInfo *>(data);
      custom_subscriber_info->listener_->attach_condition(condition_mutex, condition_variable);
    }
  }

  // This mutex prevents any of the listeners
  // to change the internal state and notify the condition
  // between the call to has_data() / hasTriggered() and wait()
  // otherwise the decision to wait might be incorrect
  std::unique_lock<std::mutex> lock(*condition_mutex);

  bool has_data = check_wait_set_for_data(
    subscriptions, guard_conditions, services, clients);
  auto predicate = [subscriptions, guard_conditions, services, clients]() {
      return check_wait_set_for_data(subscriptions, guard_conditions, services, clients);
    };

  bool timeout = false;
  if (!has_data) {
    if (!wait_timeout) {
      condition_variable->wait(lock, predicate);
    } else if (wait_timeout->sec > 0 || wait_timeout->nsec > 0) {
      auto n = std::chrono::duration_cast<std::chrono::nanoseconds>(
        std::chrono::seconds(wait_timeout->sec));
      n += std::chrono::nanoseconds(wait_timeout->nsec);
      timeout = !condition_variable->wait_for(lock, n, predicate);
    } else {
      timeout = true;
    }
  }

  // Unlock the condition variable mutex to prevent deadlocks that can occur if
  // a listener triggers while the condition variable is being detached.
  // Listeners will no longer be prevented from changing their internal state,
  // but that should not cause issues (if a listener has data / has triggered
  // after we check, it will be caught on the next call to this function).
  lock.unlock();

  if (subscriptions) {
    for (size_t i = 0; i < subscriptions->subscriber_count; ++i) {
      void * data = subscriptions->subscribers[i];
      auto custom_subscriber_info = static_cast<rmw_libp2p_cpp::CustomSubscriptionInfo *>(data);
      custom_subscriber_info->listener_->detach_condition();
      if (!custom_subscriber_info->listener_->has_data()) {
        subscriptions->subscribers[i] = 0;
      }
    }
  }

  return timeout ? RMW_RET_TIMEOUT : RMW_RET_OK;
}
}  // extern "C"
