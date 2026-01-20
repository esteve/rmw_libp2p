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

#include "rcutils/allocator.h"
#include "rmw/rmw.h"
#include "rmw/error_handling.h"

class TestRmwWait : public ::testing::Test
{
protected:
  rmw_init_options_t init_options_;
  rmw_context_t context_;

  void SetUp() override
  {
    rmw_reset_error();
    init_options_ = rmw_get_zero_initialized_init_options();
    context_ = rmw_get_zero_initialized_context();

    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    rmw_ret_t ret = rmw_init_options_init(&init_options_, allocator);
    ASSERT_EQ(ret, RMW_RET_OK);

    init_options_.enclave = rcutils_strdup("/", allocator);
    ASSERT_NE(init_options_.enclave, nullptr);

    ret = rmw_init(&init_options_, &context_);
    ASSERT_EQ(ret, RMW_RET_OK);
  }

  void TearDown() override
  {
    rmw_ret_t ret = rmw_shutdown(&context_);
    EXPECT_EQ(ret, RMW_RET_OK);

    context_.impl->is_shutdown = true;
    ret = rmw_context_fini(&context_);
    EXPECT_EQ(ret, RMW_RET_OK);

    ret = rmw_init_options_fini(&init_options_);
    EXPECT_EQ(ret, RMW_RET_OK);

    rmw_reset_error();
  }
};

// Test rmw_create_wait_set with valid arguments
TEST_F(TestRmwWait, CreateWaitSetValid)
{
  rmw_wait_set_t * wait_set = rmw_create_wait_set(&context_, 10);
  ASSERT_NE(wait_set, nullptr);

  EXPECT_NE(wait_set->implementation_identifier, nullptr);
  EXPECT_NE(wait_set->data, nullptr);

  rmw_ret_t ret = rmw_destroy_wait_set(wait_set);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_create_wait_set with null context
TEST_F(TestRmwWait, CreateWaitSetNullContext)
{
  rmw_wait_set_t * wait_set = rmw_create_wait_set(nullptr, 10);
  EXPECT_EQ(wait_set, nullptr);
}

// Test rmw_create_wait_set with zero max conditions
TEST_F(TestRmwWait, CreateWaitSetZeroMaxConditions)
{
  rmw_wait_set_t * wait_set = rmw_create_wait_set(&context_, 0);
  // Should still create a valid wait set
  ASSERT_NE(wait_set, nullptr);

  rmw_ret_t ret = rmw_destroy_wait_set(wait_set);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_destroy_wait_set with null wait set
TEST_F(TestRmwWait, DestroyWaitSetNull)
{
  rmw_ret_t ret = rmw_destroy_wait_set(nullptr);
  EXPECT_NE(ret, RMW_RET_OK);
}

// Test rmw_wait with timeout (immediate return)
TEST_F(TestRmwWait, WaitWithZeroTimeout)
{
  rmw_wait_set_t * wait_set = rmw_create_wait_set(&context_, 10);
  ASSERT_NE(wait_set, nullptr);

  rmw_time_t timeout = {0, 0};  // Zero timeout - should return immediately

  rmw_ret_t ret = rmw_wait(
    nullptr,  // subscriptions
    nullptr,  // guard_conditions
    nullptr,  // services
    nullptr,  // clients
    nullptr,  // events
    wait_set,
    &timeout);

  // With no conditions and zero timeout, should return timeout
  EXPECT_EQ(ret, RMW_RET_TIMEOUT);

  ret = rmw_destroy_wait_set(wait_set);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_wait with guard condition that is triggered
TEST_F(TestRmwWait, WaitWithTriggeredGuardCondition)
{
  rmw_wait_set_t * wait_set = rmw_create_wait_set(&context_, 10);
  ASSERT_NE(wait_set, nullptr);

  rmw_guard_condition_t * gc = rmw_create_guard_condition(&context_);
  ASSERT_NE(gc, nullptr);

  // Trigger the guard condition before waiting
  rmw_ret_t ret = rmw_trigger_guard_condition(gc);
  EXPECT_EQ(ret, RMW_RET_OK);

  rmw_guard_conditions_t guard_conditions;
  guard_conditions.guard_condition_count = 1;
  guard_conditions.guard_conditions = reinterpret_cast<void **>(&gc);

  rmw_time_t timeout = {0, 0};  // Zero timeout

  ret = rmw_wait(
    nullptr,
    &guard_conditions,
    nullptr,
    nullptr,
    nullptr,
    wait_set,
    &timeout);

  // The triggered guard condition should cause immediate return with OK
  EXPECT_EQ(ret, RMW_RET_OK);

  ret = rmw_destroy_guard_condition(gc);
  EXPECT_EQ(ret, RMW_RET_OK);

  ret = rmw_destroy_wait_set(wait_set);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test creating multiple wait sets
TEST_F(TestRmwWait, CreateMultipleWaitSets)
{
  rmw_wait_set_t * ws1 = rmw_create_wait_set(&context_, 5);
  rmw_wait_set_t * ws2 = rmw_create_wait_set(&context_, 10);
  rmw_wait_set_t * ws3 = rmw_create_wait_set(&context_, 15);

  ASSERT_NE(ws1, nullptr);
  ASSERT_NE(ws2, nullptr);
  ASSERT_NE(ws3, nullptr);

  EXPECT_NE(ws1, ws2);
  EXPECT_NE(ws2, ws3);
  EXPECT_NE(ws1, ws3);

  rmw_ret_t ret = rmw_destroy_wait_set(ws1);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_destroy_wait_set(ws2);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_destroy_wait_set(ws3);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test wait set lifecycle
TEST_F(TestRmwWait, WaitSetLifecycle)
{
  for (int i = 0; i < 5; ++i) {
    rmw_wait_set_t * wait_set = rmw_create_wait_set(&context_, 10);
    ASSERT_NE(wait_set, nullptr);

    rmw_time_t timeout = {0, 0};
    rmw_ret_t ret = rmw_wait(
      nullptr, nullptr, nullptr, nullptr, nullptr, wait_set, &timeout);
    EXPECT_EQ(ret, RMW_RET_TIMEOUT);

    ret = rmw_destroy_wait_set(wait_set);
    EXPECT_EQ(ret, RMW_RET_OK);
  }
}

// Test rmw_wait with null wait set
TEST_F(TestRmwWait, WaitWithNullWaitSet)
{
  rmw_time_t timeout = {0, 0};

  rmw_ret_t ret = rmw_wait(
    nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, &timeout);
  EXPECT_NE(ret, RMW_RET_OK);
}

// Test wait with short timeout
TEST_F(TestRmwWait, WaitWithShortTimeout)
{
  rmw_wait_set_t * wait_set = rmw_create_wait_set(&context_, 10);
  ASSERT_NE(wait_set, nullptr);

  // 1 millisecond timeout
  rmw_time_t timeout = {0, 1000000};

  rmw_ret_t ret = rmw_wait(
    nullptr, nullptr, nullptr, nullptr, nullptr, wait_set, &timeout);

  // Should timeout since there's nothing to wait on
  EXPECT_EQ(ret, RMW_RET_TIMEOUT);

  ret = rmw_destroy_wait_set(wait_set);
  EXPECT_EQ(ret, RMW_RET_OK);
}

int main(int argc, char ** argv)
{
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
