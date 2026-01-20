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

class TestRmwInit : public ::testing::Test
{
protected:
  void SetUp() override
  {
    rmw_reset_error();
  }

  void TearDown() override
  {
    rmw_reset_error();
  }
};

// Test rmw_init_options_init with valid arguments
TEST_F(TestRmwInit, InitOptionsInitValid)
{
  rmw_init_options_t init_options = rmw_get_zero_initialized_init_options();
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  rmw_ret_t ret = rmw_init_options_init(&init_options, allocator);
  EXPECT_EQ(ret, RMW_RET_OK);

  // Verify the options were initialized
  EXPECT_NE(init_options.implementation_identifier, nullptr);

  // Clean up
  ret = rmw_init_options_fini(&init_options);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_init_options_init with null init_options
TEST_F(TestRmwInit, InitOptionsInitNullOptions)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  rmw_ret_t ret = rmw_init_options_init(nullptr, allocator);
  EXPECT_EQ(ret, RMW_RET_INVALID_ARGUMENT);
}

// Test rmw_init_options_init with already initialized options
TEST_F(TestRmwInit, InitOptionsInitAlreadyInitialized)
{
  rmw_init_options_t init_options = rmw_get_zero_initialized_init_options();
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  rmw_ret_t ret = rmw_init_options_init(&init_options, allocator);
  EXPECT_EQ(ret, RMW_RET_OK);

  // Try to initialize again - should fail
  ret = rmw_init_options_init(&init_options, allocator);
  EXPECT_EQ(ret, RMW_RET_INVALID_ARGUMENT);

  // Clean up
  ret = rmw_init_options_fini(&init_options);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_init_options_copy with valid arguments
TEST_F(TestRmwInit, InitOptionsCopyValid)
{
  rmw_init_options_t src = rmw_get_zero_initialized_init_options();
  rmw_init_options_t dst = rmw_get_zero_initialized_init_options();
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  rmw_ret_t ret = rmw_init_options_init(&src, allocator);
  EXPECT_EQ(ret, RMW_RET_OK);

  // Set enclave to a non-null value for copy test
  src.enclave = rcutils_strdup("/test_enclave", allocator);
  ASSERT_NE(src.enclave, nullptr);

  ret = rmw_init_options_copy(&src, &dst);
  EXPECT_EQ(ret, RMW_RET_OK);

  // Verify the copy
  EXPECT_EQ(dst.implementation_identifier, src.implementation_identifier);
  EXPECT_STREQ(dst.enclave, src.enclave);

  // Clean up
  ret = rmw_init_options_fini(&src);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_init_options_fini(&dst);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_init_options_copy with null source
TEST_F(TestRmwInit, InitOptionsCopyNullSource)
{
  rmw_init_options_t dst = rmw_get_zero_initialized_init_options();

  rmw_ret_t ret = rmw_init_options_copy(nullptr, &dst);
  EXPECT_EQ(ret, RMW_RET_INVALID_ARGUMENT);
}

// Test rmw_init_options_copy with null destination
TEST_F(TestRmwInit, InitOptionsCopyNullDestination)
{
  rmw_init_options_t src = rmw_get_zero_initialized_init_options();
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  rmw_ret_t ret = rmw_init_options_init(&src, allocator);
  EXPECT_EQ(ret, RMW_RET_OK);

  src.enclave = rcutils_strdup("/test", allocator);

  ret = rmw_init_options_copy(&src, nullptr);
  EXPECT_EQ(ret, RMW_RET_INVALID_ARGUMENT);

  // Clean up
  ret = rmw_init_options_fini(&src);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_init_options_fini with null options
TEST_F(TestRmwInit, InitOptionsFiniNull)
{
  rmw_ret_t ret = rmw_init_options_fini(nullptr);
  EXPECT_EQ(ret, RMW_RET_INVALID_ARGUMENT);
}

// Test rmw_init with valid arguments
TEST_F(TestRmwInit, InitValid)
{
  rmw_init_options_t init_options = rmw_get_zero_initialized_init_options();
  rmw_context_t context = rmw_get_zero_initialized_context();
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  rmw_ret_t ret = rmw_init_options_init(&init_options, allocator);
  EXPECT_EQ(ret, RMW_RET_OK);

  init_options.enclave = rcutils_strdup("/", allocator);
  ASSERT_NE(init_options.enclave, nullptr);

  ret = rmw_init(&init_options, &context);
  EXPECT_EQ(ret, RMW_RET_OK);

  // Verify context was initialized
  EXPECT_NE(context.implementation_identifier, nullptr);
  EXPECT_NE(context.impl, nullptr);

  // Clean up
  ret = rmw_shutdown(&context);
  EXPECT_EQ(ret, RMW_RET_OK);

  context.impl->is_shutdown = true;
  ret = rmw_context_fini(&context);
  EXPECT_EQ(ret, RMW_RET_OK);

  ret = rmw_init_options_fini(&init_options);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_init with null options
TEST_F(TestRmwInit, InitNullOptions)
{
  rmw_context_t context = rmw_get_zero_initialized_context();

  rmw_ret_t ret = rmw_init(nullptr, &context);
  EXPECT_EQ(ret, RMW_RET_INVALID_ARGUMENT);
}

// Test rmw_init with null context
TEST_F(TestRmwInit, InitNullContext)
{
  rmw_init_options_t init_options = rmw_get_zero_initialized_init_options();
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  rmw_ret_t ret = rmw_init_options_init(&init_options, allocator);
  EXPECT_EQ(ret, RMW_RET_OK);

  init_options.enclave = rcutils_strdup("/", allocator);

  ret = rmw_init(&init_options, nullptr);
  EXPECT_EQ(ret, RMW_RET_INVALID_ARGUMENT);

  // Clean up
  ret = rmw_init_options_fini(&init_options);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_shutdown with null context
TEST_F(TestRmwInit, ShutdownNullContext)
{
  rmw_ret_t ret = rmw_shutdown(nullptr);
  EXPECT_EQ(ret, RMW_RET_INVALID_ARGUMENT);
}

// Test rmw_context_fini with null context
TEST_F(TestRmwInit, ContextFiniNull)
{
  rmw_ret_t ret = rmw_context_fini(nullptr);
  EXPECT_EQ(ret, RMW_RET_INVALID_ARGUMENT);
}

// Test full lifecycle: init -> shutdown -> fini
TEST_F(TestRmwInit, FullLifecycle)
{
  rmw_init_options_t init_options = rmw_get_zero_initialized_init_options();
  rmw_context_t context = rmw_get_zero_initialized_context();
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  // Initialize options
  rmw_ret_t ret = rmw_init_options_init(&init_options, allocator);
  EXPECT_EQ(ret, RMW_RET_OK);

  init_options.enclave = rcutils_strdup("/", allocator);
  ASSERT_NE(init_options.enclave, nullptr);

  // Initialize context
  ret = rmw_init(&init_options, &context);
  EXPECT_EQ(ret, RMW_RET_OK);

  // Shutdown
  ret = rmw_shutdown(&context);
  EXPECT_EQ(ret, RMW_RET_OK);

  // Mark as shutdown and finalize
  context.impl->is_shutdown = true;
  ret = rmw_context_fini(&context);
  EXPECT_EQ(ret, RMW_RET_OK);

  // Finalize options
  ret = rmw_init_options_fini(&init_options);
  EXPECT_EQ(ret, RMW_RET_OK);
}

int main(int argc, char ** argv)
{
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
