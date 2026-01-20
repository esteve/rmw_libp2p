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

class TestRmwGuardCondition : public ::testing::Test
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

// Test rmw_create_guard_condition with valid context
TEST_F(TestRmwGuardCondition, CreateGuardConditionValid)
{
  rmw_guard_condition_t * gc = rmw_create_guard_condition(&context_);
  ASSERT_NE(gc, nullptr);

  EXPECT_NE(gc->implementation_identifier, nullptr);
  EXPECT_NE(gc->data, nullptr);

  rmw_ret_t ret = rmw_destroy_guard_condition(gc);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_create_guard_condition with null context
TEST_F(TestRmwGuardCondition, CreateGuardConditionNullContext)
{
  rmw_guard_condition_t * gc = rmw_create_guard_condition(nullptr);
  EXPECT_EQ(gc, nullptr);
}

// Test rmw_destroy_guard_condition with null guard condition
TEST_F(TestRmwGuardCondition, DestroyGuardConditionNull)
{
  rmw_ret_t ret = rmw_destroy_guard_condition(nullptr);
  EXPECT_EQ(ret, RMW_RET_ERROR);
}

// Test creating multiple guard conditions
TEST_F(TestRmwGuardCondition, CreateMultipleGuardConditions)
{
  rmw_guard_condition_t * gc1 = rmw_create_guard_condition(&context_);
  rmw_guard_condition_t * gc2 = rmw_create_guard_condition(&context_);
  rmw_guard_condition_t * gc3 = rmw_create_guard_condition(&context_);

  ASSERT_NE(gc1, nullptr);
  ASSERT_NE(gc2, nullptr);
  ASSERT_NE(gc3, nullptr);

  // Each guard condition should be different
  EXPECT_NE(gc1, gc2);
  EXPECT_NE(gc2, gc3);
  EXPECT_NE(gc1, gc3);

  rmw_ret_t ret = rmw_destroy_guard_condition(gc1);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_destroy_guard_condition(gc2);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_destroy_guard_condition(gc3);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_trigger_guard_condition with valid guard condition
TEST_F(TestRmwGuardCondition, TriggerGuardConditionValid)
{
  rmw_guard_condition_t * gc = rmw_create_guard_condition(&context_);
  ASSERT_NE(gc, nullptr);

  rmw_ret_t ret = rmw_trigger_guard_condition(gc);
  EXPECT_EQ(ret, RMW_RET_OK);

  ret = rmw_destroy_guard_condition(gc);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_trigger_guard_condition with null guard condition
TEST_F(TestRmwGuardCondition, TriggerGuardConditionNull)
{
  rmw_ret_t ret = rmw_trigger_guard_condition(nullptr);
  EXPECT_EQ(ret, RMW_RET_INVALID_ARGUMENT);
}

// Test triggering the same guard condition multiple times
TEST_F(TestRmwGuardCondition, TriggerGuardConditionMultipleTimes)
{
  rmw_guard_condition_t * gc = rmw_create_guard_condition(&context_);
  ASSERT_NE(gc, nullptr);

  for (int i = 0; i < 10; ++i) {
    rmw_ret_t ret = rmw_trigger_guard_condition(gc);
    EXPECT_EQ(ret, RMW_RET_OK);
  }

  rmw_ret_t ret = rmw_destroy_guard_condition(gc);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test guard condition lifecycle
TEST_F(TestRmwGuardCondition, GuardConditionLifecycle)
{
  // Create, trigger, and destroy in a loop
  for (int i = 0; i < 5; ++i) {
    rmw_guard_condition_t * gc = rmw_create_guard_condition(&context_);
    ASSERT_NE(gc, nullptr);

    rmw_ret_t ret = rmw_trigger_guard_condition(gc);
    EXPECT_EQ(ret, RMW_RET_OK);

    ret = rmw_destroy_guard_condition(gc);
    EXPECT_EQ(ret, RMW_RET_OK);
  }
}

int main(int argc, char ** argv)
{
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
