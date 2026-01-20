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

#include <gtest/gtest.h>
#include <thread>
#include <chrono>

#include "impl/guard_condition.hpp"

class TestGuardConditionClass : public ::testing::Test
{
protected:
  void SetUp() override
  {
  }

  void TearDown() override
  {
  }
};

// Test default construction
TEST_F(TestGuardConditionClass, DefaultConstruction)
{
  GuardCondition gc;
  EXPECT_FALSE(gc.hasTriggered());
}

// Test trigger sets has_triggered flag
TEST_F(TestGuardConditionClass, TriggerSetsFlag)
{
  GuardCondition gc;
  EXPECT_FALSE(gc.hasTriggered());

  gc.trigger();
  EXPECT_TRUE(gc.hasTriggered());
}

// Test getHasTriggered returns true and resets flag
TEST_F(TestGuardConditionClass, GetHasTriggeredResets)
{
  GuardCondition gc;

  gc.trigger();
  EXPECT_TRUE(gc.hasTriggered());

  // getHasTriggered should return true and reset the flag
  bool was_triggered = gc.getHasTriggered();
  EXPECT_TRUE(was_triggered);

  // After getHasTriggered, the flag should be false
  EXPECT_FALSE(gc.hasTriggered());
  EXPECT_FALSE(gc.getHasTriggered());
}

// Test multiple triggers
TEST_F(TestGuardConditionClass, MultipleTriggers)
{
  GuardCondition gc;

  gc.trigger();
  gc.trigger();
  gc.trigger();

  EXPECT_TRUE(gc.hasTriggered());

  // Only one getHasTriggered should succeed
  EXPECT_TRUE(gc.getHasTriggered());
  EXPECT_FALSE(gc.getHasTriggered());
}

// Test attach and detach condition
TEST_F(TestGuardConditionClass, AttachDetachCondition)
{
  GuardCondition gc;
  std::mutex mutex;
  std::condition_variable cv;

  gc.attachCondition(&mutex, &cv);

  // Should still work with condition attached
  gc.trigger();
  EXPECT_TRUE(gc.hasTriggered());

  gc.detachCondition();

  // Should still work after detach
  gc.getHasTriggered();  // Reset
  gc.trigger();
  EXPECT_TRUE(gc.hasTriggered());
}

// Test trigger with attached condition notifies
TEST_F(TestGuardConditionClass, TriggerNotifiesCondition)
{
  GuardCondition gc;
  std::mutex mutex;
  std::condition_variable cv;
  bool notified = false;

  gc.attachCondition(&mutex, &cv);

  std::thread waiter([&]() {
    std::unique_lock<std::mutex> lock(mutex);
    if (cv.wait_for(lock, std::chrono::milliseconds(1000), [&]() {
      return gc.hasTriggered();
    })) {
      notified = true;
    }
  });

  // Give the waiter thread time to start waiting
  std::this_thread::sleep_for(std::chrono::milliseconds(50));

  gc.trigger();

  waiter.join();

  EXPECT_TRUE(notified);
  EXPECT_TRUE(gc.hasTriggered());

  gc.detachCondition();
}

// Test thread safety of trigger
TEST_F(TestGuardConditionClass, ThreadSafetyOfTrigger)
{
  GuardCondition gc;

  std::vector<std::thread> threads;
  const int num_threads = 10;
  const int triggers_per_thread = 100;

  for (int i = 0; i < num_threads; ++i) {
    threads.emplace_back([&gc, triggers_per_thread]() {
      for (int j = 0; j < triggers_per_thread; ++j) {
        gc.trigger();
        std::this_thread::yield();
      }
    });
  }

  for (auto & t : threads) {
    t.join();
  }

  // After all triggers, should be triggered
  EXPECT_TRUE(gc.hasTriggered());
}

// Test thread safety of attach/detach
TEST_F(TestGuardConditionClass, ThreadSafetyOfAttachDetach)
{
  GuardCondition gc;
  std::mutex mutex;
  std::condition_variable cv;

  std::atomic<bool> running{true};

  std::thread trigger_thread([&]() {
    while (running) {
      gc.trigger();
      std::this_thread::yield();
    }
  });

  std::thread attach_thread([&]() {
    while (running) {
      gc.attachCondition(&mutex, &cv);
      std::this_thread::yield();
      gc.detachCondition();
      std::this_thread::yield();
    }
  });

  // Run for a short time
  std::this_thread::sleep_for(std::chrono::milliseconds(100));
  running = false;

  trigger_thread.join();
  attach_thread.join();

  // Cleanup - make sure condition is detached
  gc.detachCondition();
}

// Test hasTriggered vs getHasTriggered semantics
TEST_F(TestGuardConditionClass, HasTriggeredVsGetHasTriggered)
{
  GuardCondition gc;

  // Initial state
  EXPECT_FALSE(gc.hasTriggered());
  EXPECT_FALSE(gc.getHasTriggered());

  gc.trigger();

  // hasTriggered should not reset
  EXPECT_TRUE(gc.hasTriggered());
  EXPECT_TRUE(gc.hasTriggered());
  EXPECT_TRUE(gc.hasTriggered());

  // getHasTriggered should reset
  EXPECT_TRUE(gc.getHasTriggered());
  EXPECT_FALSE(gc.hasTriggered());
  EXPECT_FALSE(gc.getHasTriggered());
}

// Test condition variable notification timing
TEST_F(TestGuardConditionClass, ConditionVariableNotificationTiming)
{
  GuardCondition gc;
  std::mutex mutex;
  std::condition_variable cv;

  gc.attachCondition(&mutex, &cv);

  auto start = std::chrono::steady_clock::now();
  bool timed_out = false;

  std::thread waiter([&]() {
    std::unique_lock<std::mutex> lock(mutex);
    if (!cv.wait_for(lock, std::chrono::milliseconds(500), [&]() {
      return gc.hasTriggered();
    })) {
      timed_out = true;
    }
  });

  // Trigger after a short delay
  std::this_thread::sleep_for(std::chrono::milliseconds(50));
  gc.trigger();

  waiter.join();

  auto end = std::chrono::steady_clock::now();
  auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);

  EXPECT_FALSE(timed_out);
  // Should complete much faster than the 500ms timeout
  EXPECT_LT(duration.count(), 300);

  gc.detachCondition();
}

int main(int argc, char ** argv)
{
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
