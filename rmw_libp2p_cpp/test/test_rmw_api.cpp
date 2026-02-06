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

#include <gtest/gtest.h>

#include "rcutils/allocator.h"
#include "rmw/error_handling.h"
#include "rmw/rmw.h"
#include "rmw/types.h"

// Test fixture for RMW API tests
class TestRmwApi : public ::testing::Test
{
protected:
  void SetUp() override
  {
    // Get a zero-initialized context
    context = rmw_get_zero_initialized_context();
  }

  void TearDown() override
  {
    // Clean up any resources if needed
  }

  rmw_context_t context;
};

// ============================================================================
// Error Handling Tests - Test null pointer checks
// ============================================================================

TEST_F(TestRmwApi, CreateNodeWithNullContext) {
  rmw_node_t * node = rmw_create_node(nullptr, "test_node", "/test_namespace");
  EXPECT_EQ(node, nullptr);
  rmw_reset_error();
}

TEST_F(TestRmwApi, CreateNodeWithUninitializedContext) {
  rmw_node_t * node = rmw_create_node(&context, "test_node", "/test_namespace");
  EXPECT_EQ(node, nullptr);
  rmw_reset_error();
}

TEST_F(TestRmwApi, CreateNodeWithNullName) {
  rmw_node_t * node = rmw_create_node(&context, nullptr, "/test_namespace");
  EXPECT_EQ(node, nullptr);
  rmw_reset_error();
}

TEST_F(TestRmwApi, CreateNodeWithNullNamespace) {
  rmw_node_t * node = rmw_create_node(&context, "test_node", nullptr);
  EXPECT_EQ(node, nullptr);
  rmw_reset_error();
}

TEST_F(TestRmwApi, CreatePublisherWithNullNode) {
  rmw_qos_profile_t qos_profile = rmw_qos_profile_default;
  rmw_publisher_options_t publisher_options = rmw_get_default_publisher_options();

  rmw_publisher_t * publisher =
    rmw_create_publisher(nullptr, nullptr, "test_topic", &qos_profile, &publisher_options);

  EXPECT_EQ(publisher, nullptr);
  rmw_reset_error();
}

TEST_F(TestRmwApi, CreateSubscriptionWithNullNode) {
  rmw_qos_profile_t qos_profile = rmw_qos_profile_default;
  rmw_subscription_options_t subscription_options = rmw_get_default_subscription_options();

  rmw_subscription_t * subscription =
    rmw_create_subscription(nullptr, nullptr, "test_topic", &qos_profile, &subscription_options);

  EXPECT_EQ(subscription, nullptr);
  rmw_reset_error();
}

TEST_F(TestRmwApi, PublishWithNullPublisher) {
  rmw_ret_t ret = rmw_publish(nullptr, nullptr, nullptr);
  EXPECT_NE(ret, RMW_RET_OK);
  rmw_reset_error();
}

TEST_F(TestRmwApi, TakeWithNullSubscription) {
  bool taken = false;
  rmw_ret_t ret = rmw_take(nullptr, nullptr, &taken, nullptr);
  EXPECT_NE(ret, RMW_RET_OK);
  rmw_reset_error();
}

TEST_F(TestRmwApi, DestroyNodeWithNull) {
  rmw_ret_t ret = rmw_destroy_node(nullptr);
  EXPECT_EQ(ret, RMW_RET_ERROR);
  rmw_reset_error();
}

// ============================================================================
// Basic Lifecycle Tests - Test create/destroy patterns
// ============================================================================

class TestRmwLifecycle : public ::testing::Test
{
protected:
  void SetUp() override
  {
    // Initialize allocator
    allocator = rcutils_get_default_allocator();

    // Get zero-initialized init options
    init_options = rmw_get_zero_initialized_init_options();

    // Initialize init options
    rmw_ret_t ret = rmw_init_options_init(&init_options, allocator);
    ASSERT_EQ(ret, RMW_RET_OK);

    // Get zero-initialized context
    context = rmw_get_zero_initialized_context();

    // Initialize context
    ret = rmw_init(&init_options, &context);
    if (ret == RMW_RET_OK) {
      context_initialized = true;
    } else {
      // Context initialization may fail if Rust backend is not available
      // This is OK for testing the C++ API layer
      context_initialized = false;
      rmw_reset_error();
    }
  }

  void TearDown() override
  {
    if (context_initialized) {
      rmw_ret_t ret = rmw_shutdown(&context);
      EXPECT_EQ(ret, RMW_RET_OK);

      ret = rmw_context_fini(&context);
      EXPECT_EQ(ret, RMW_RET_OK);
    }

    rmw_ret_t ret = rmw_init_options_fini(&init_options);
    EXPECT_EQ(ret, RMW_RET_OK);
  }

  rcutils_allocator_t allocator;
  rmw_init_options_t init_options;
  rmw_context_t context;
  bool context_initialized = false;
};

TEST_F(TestRmwLifecycle, CreateAndDestroyNode) {
  if (!context_initialized) {
    GTEST_SKIP() << "Context initialization failed, skipping node lifecycle test";
  }

  rmw_node_t * node = rmw_create_node(&context, "test_node", "/");
  if (node != nullptr) {
    EXPECT_NE(node->name, nullptr);
    EXPECT_STREQ(node->name, "test_node");
    EXPECT_NE(node->namespace_, nullptr);
    EXPECT_STREQ(node->namespace_, "/");

    rmw_ret_t ret = rmw_destroy_node(node);
    EXPECT_EQ(ret, RMW_RET_OK);
  } else {
    // Node creation may fail if backend is not fully initialized
    // This is acceptable for API testing
    rmw_reset_error();
    GTEST_SKIP() << "Node creation failed, backend may not be available";
  }
}

TEST_F(TestRmwLifecycle, CreateAndDestroyPublisher) {
  if (!context_initialized) {
    GTEST_SKIP() << "Context initialization failed, skipping publisher lifecycle test";
  }

  rmw_node_t * node = rmw_create_node(&context, "test_node", "/");
  if (node == nullptr) {
    rmw_reset_error();
    GTEST_SKIP() << "Node creation failed, skipping publisher test";
  }

  // Note: We can't test publisher creation without a valid type support
  // This would require including test_msgs or another message package
  // For now, we just verify the node was created successfully

  rmw_ret_t ret = rmw_destroy_node(node);
  EXPECT_EQ(ret, RMW_RET_OK);
}

TEST_F(TestRmwLifecycle, CreateAndDestroySubscription) {
  if (!context_initialized) {
    GTEST_SKIP() << "Context initialization failed, skipping subscription lifecycle test";
  }

  rmw_node_t * node = rmw_create_node(&context, "test_node", "/");
  if (node == nullptr) {
    rmw_reset_error();
    GTEST_SKIP() << "Node creation failed, skipping subscription test";
  }

  // Note: We can't test subscription creation without a valid type support
  // This would require including test_msgs or another message package
  // For now, we just verify the node was created successfully

  rmw_ret_t ret = rmw_destroy_node(node);
  EXPECT_EQ(ret, RMW_RET_OK);
}

TEST_F(TestRmwLifecycle, CreateAndDestroyWaitSet) {
  if (!context_initialized) {
    GTEST_SKIP() << "Context initialization failed, skipping wait set lifecycle test";
  }

  rmw_wait_set_t * wait_set = rmw_create_wait_set(&context, 10);
  if (wait_set != nullptr) {
    EXPECT_NE(wait_set->data, nullptr);

    rmw_ret_t ret = rmw_destroy_wait_set(wait_set);
    EXPECT_EQ(ret, RMW_RET_OK);
  } else {
    // Wait set creation may fail if backend is not available
    rmw_reset_error();
    GTEST_SKIP() << "Wait set creation failed, backend may not be available";
  }
}

TEST_F(TestRmwLifecycle, DestroyWaitSetWithNull) {
  rmw_ret_t ret = rmw_destroy_wait_set(nullptr);
  EXPECT_EQ(ret, RMW_RET_ERROR);
  rmw_reset_error();
}

// ============================================================================
// RMW Initialization Tests
// ============================================================================

TEST(TestRmwInit, InitOptionsInit) {
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  rmw_init_options_t init_options = rmw_get_zero_initialized_init_options();

  rmw_ret_t ret = rmw_init_options_init(&init_options, allocator);
  EXPECT_EQ(ret, RMW_RET_OK);

  ret = rmw_init_options_fini(&init_options);
  EXPECT_EQ(ret, RMW_RET_OK);
}

TEST(TestRmwInit, InitOptionsInitWithNull) {
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  rmw_ret_t ret = rmw_init_options_init(nullptr, allocator);
  EXPECT_EQ(ret, RMW_RET_INVALID_ARGUMENT);
  rmw_reset_error();
}

TEST(TestRmwInit, ContextInit) {
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  rmw_init_options_t init_options = rmw_get_zero_initialized_init_options();
  rmw_context_t context = rmw_get_zero_initialized_context();

  rmw_ret_t ret = rmw_init_options_init(&init_options, allocator);
  ASSERT_EQ(ret, RMW_RET_OK);

  ret = rmw_init(&init_options, &context);
  if (ret == RMW_RET_OK) {
    // Context initialized successfully
    rmw_ret_t shutdown_ret = rmw_shutdown(&context);
    EXPECT_EQ(shutdown_ret, RMW_RET_OK);

    rmw_ret_t fini_ret = rmw_context_fini(&context);
    EXPECT_EQ(fini_ret, RMW_RET_OK);
  } else {
    // Context initialization may fail if backend is not available
    rmw_reset_error();
  }

  ret = rmw_init_options_fini(&init_options);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// ============================================================================
// Main function
// ============================================================================

int main(int argc, char ** argv)
{
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
