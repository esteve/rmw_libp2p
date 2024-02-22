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

#ifndef RMW_LIBP2P_CPP__LISTENER_HPP_
#define RMW_LIBP2P_CPP__LISTENER_HPP_

#include <atomic>
#include <condition_variable>
#include <memory>
#include <mutex>
#include <queue>
#include <utility>
#include <vector>

#include "rcutils/logging_macros.h"

struct CustomSubscriptionInfo;

struct CustomSubscriptionHandle {
  CustomSubscriptionInfo * custom_subscription_info;
};

class Listener
{
public:
  Listener()
  : condition_mutex_(nullptr), condition_variable_(nullptr)
  {
  }

  static void
  on_publication(const CustomSubscriptionHandle * subscription_handle, uint8_t * message,
  uintptr_t length)
  {
    CustomSubscriptionInfo * subscription_impl = static_cast<CustomSubscriptionInfo *>(subscription_handle->custom_subscription_info);

    RCUTILS_LOG_WARN_NAMED(
      "rmw_libp2p_cpp",
      "%s(node_impl=%p)",
      __FUNCTION__, (void *)subscription_impl);

    // Listener * listener = subscription_impl->listener_;
    // std::lock_guard<std::mutex> lock(listener->internal_mutex_);

    // if (listener->condition_mutex_) {
    //   std::unique_lock<std::mutex> clock(*listener->condition_mutex_);
    //   // the change to data_ needs to be mutually exclusive with rmw_wait()
    //   // which checks has_data() and decides if wait() needs to be called
    //   listener->message_queue_.push(std::move(message));
    //   clock.unlock();
    //   listener->condition_variable_->notify_one();
    // } else {
    //   listener->message_queue_.push(std::move(message));
    // }
  }

  void
  attach_condition(std::mutex * condition_mutex, std::condition_variable * condition_variable)
  {
    std::lock_guard<std::mutex> lock(internal_mutex_);
    condition_mutex_ = condition_mutex;
    condition_variable_ = condition_variable;
  }


  void
  detach_condition()
  {
    std::lock_guard<std::mutex> lock(internal_mutex_);
    condition_mutex_ = nullptr;
    condition_variable_ = nullptr;
  }

  bool
  has_data()
  {
    return message_queue_.size() > 0;
  }

private:
  std::mutex internal_mutex_;
  std::queue<uint8_t *> message_queue_;
  std::mutex * condition_mutex_;
  std::condition_variable * condition_variable_;
};

#endif  // RMW_LIBP2P_CPP__LISTENER_HPP_
