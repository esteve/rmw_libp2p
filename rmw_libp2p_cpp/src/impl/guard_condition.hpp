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

#ifndef IMPL__GUARD_CONDITION_HPP_
#define IMPL__GUARD_CONDITION_HPP_

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
  : has_triggered_(false), condition_mutex_(nullptr), condition_variable_(nullptr)
  {
  }

  void trigger()
  {
    std::lock_guard<std::mutex> lock(internal_mutex_);

    if (condition_mutex_) {
      std::unique_lock<std::mutex> clock(*condition_mutex_);
      has_triggered_ = true;
      clock.unlock();
      condition_variable_->notify_one();
    } else {
      has_triggered_ = true;
    }
  }

  void attachCondition(std::mutex * condition_mutex, std::condition_variable * conditionVariable)
  {
    std::lock_guard<std::mutex> lock(internal_mutex_);
    condition_mutex_ = condition_mutex;
    condition_variable_ = conditionVariable;
  }

  void detachCondition()
  {
    std::lock_guard<std::mutex> lock(internal_mutex_);
    condition_mutex_ = nullptr;
    condition_variable_ = nullptr;
  }

  bool hasTriggered()
  {
    return has_triggered_;
  }

  bool getHasTriggered()
  {
    return has_triggered_.exchange(false);
  }

private:
  std::mutex internal_mutex_;
  std::atomic_bool has_triggered_;
  std::mutex * condition_mutex_ RCPPUTILS_TSA_GUARDED_BY(internal_mutex_);
  std::condition_variable * condition_variable_ RCPPUTILS_TSA_GUARDED_BY(internal_mutex_);
};

#endif  // IMPL__GUARD_CONDITION_HPP_
