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

#ifndef RMW_LIBP2P_CPP__TYPES__GUARD_CONDITION_HPP_
#define RMW_LIBP2P_CPP__TYPES__GUARD_CONDITION_HPP_

#include <array>
#include <atomic>
#include <cassert>
#include <condition_variable>
#include <mutex>
#include <utility>

#include "rcpputils/thread_safety_annotations.hpp"

class GuardCondition
{
public:
  GuardCondition()
  : hasTriggered_(false),
    condition_mutex_(nullptr), conditionVariable_(nullptr) {}

  void
  trigger()
  {
    std::lock_guard<std::mutex> lock(internalMutex_);

    if (condition_mutex_) {
      std::unique_lock<std::mutex> clock(*condition_mutex_);
      // the change to hasTriggered_ needs to be mutually exclusive with
      // rmw_wait() which checks hasTriggered() and decides if wait() needs to
      // be called
      hasTriggered_ = true;
      clock.unlock();
      conditionVariable_->notify_one();
    } else {
      hasTriggered_ = true;
    }
  }

  void
  attachCondition(std::mutex * condition_mutex, std::condition_variable * conditionVariable)
  {
    std::lock_guard<std::mutex> lock(internalMutex_);
    condition_mutex_ = condition_mutex;
    conditionVariable_ = conditionVariable;
  }

  void
  detachCondition()
  {
    std::lock_guard<std::mutex> lock(internalMutex_);
    condition_mutex_ = nullptr;
    conditionVariable_ = nullptr;
  }

  bool
  hasTriggered()
  {
    return hasTriggered_;
  }

  bool
  getHasTriggered()
  {
    return hasTriggered_.exchange(false);
  }

private:
  std::mutex internalMutex_;
  std::atomic_bool hasTriggered_;
  std::mutex * condition_mutex_ RCPPUTILS_TSA_GUARDED_BY(internalMutex_);
  std::condition_variable * conditionVariable_ RCPPUTILS_TSA_GUARDED_BY(internalMutex_);
};

#endif  // RMW_LIBP2P_CPP__TYPES__GUARD_CONDITION_HPP_
