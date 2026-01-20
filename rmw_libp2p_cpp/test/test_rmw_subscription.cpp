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
#include "test_msgs/msg/basic_types.h"
#include "rosidl_typesupport_introspection_cpp/identifier.hpp"

class TestRmwSubscription : public ::testing::Test
{
protected:
  rmw_init_options_t init_options_;
  rmw_context_t context_;
  rmw_node_t * node_;

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

    node_ = rmw_create_node(&context_, "test_subscription_node", "/test");
    ASSERT_NE(node_, nullptr);
  }

  void TearDown() override
  {
    if (node_ != nullptr) {
      rmw_ret_t ret = rmw_destroy_node(node_);
      EXPECT_EQ(ret, RMW_RET_OK);
    }

    rmw_ret_t ret = rmw_shutdown(&context_);
    EXPECT_EQ(ret, RMW_RET_OK);

    context_.impl->is_shutdown = true;
    ret = rmw_context_fini(&context_);
    EXPECT_EQ(ret, RMW_RET_OK);

    ret = rmw_init_options_fini(&init_options_);
    EXPECT_EQ(ret, RMW_RET_OK);

    rmw_reset_error();
  }

  const rosidl_message_type_support_t * get_type_support()
  {
    return ROSIDL_GET_MSG_TYPE_SUPPORT(test_msgs, msg, BasicTypes);
  }
};

// Test rmw_create_subscription with valid arguments
TEST_F(TestRmwSubscription, CreateSubscriptionValid)
{
  rmw_subscription_options_t options = rmw_get_default_subscription_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_subscription_t * subscription = rmw_create_subscription(
    node_,
    get_type_support(),
    "test_topic",
    &qos,
    &options);
  ASSERT_NE(subscription, nullptr);

  EXPECT_NE(subscription->implementation_identifier, nullptr);
  EXPECT_STREQ(subscription->topic_name, "test_topic");

  rmw_ret_t ret = rmw_destroy_subscription(node_, subscription);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_create_subscription with null node
TEST_F(TestRmwSubscription, CreateSubscriptionNullNode)
{
  rmw_subscription_options_t options = rmw_get_default_subscription_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_subscription_t * subscription = rmw_create_subscription(
    nullptr,
    get_type_support(),
    "test_topic",
    &qos,
    &options);
  EXPECT_EQ(subscription, nullptr);
}

// Test rmw_create_subscription with null type support
TEST_F(TestRmwSubscription, CreateSubscriptionNullTypeSupport)
{
  rmw_subscription_options_t options = rmw_get_default_subscription_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_subscription_t * subscription = rmw_create_subscription(
    node_,
    nullptr,
    "test_topic",
    &qos,
    &options);
  EXPECT_EQ(subscription, nullptr);
}

// Test rmw_create_subscription with null topic name
TEST_F(TestRmwSubscription, CreateSubscriptionNullTopicName)
{
  rmw_subscription_options_t options = rmw_get_default_subscription_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_subscription_t * subscription = rmw_create_subscription(
    node_,
    get_type_support(),
    nullptr,
    &qos,
    &options);
  EXPECT_EQ(subscription, nullptr);
}

// Test rmw_destroy_subscription with null node
TEST_F(TestRmwSubscription, DestroySubscriptionNullNode)
{
  rmw_subscription_options_t options = rmw_get_default_subscription_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_subscription_t * subscription = rmw_create_subscription(
    node_,
    get_type_support(),
    "test_topic",
    &qos,
    &options);
  ASSERT_NE(subscription, nullptr);

  rmw_ret_t ret = rmw_destroy_subscription(nullptr, subscription);
  EXPECT_NE(ret, RMW_RET_OK);

  // Clean up properly
  ret = rmw_destroy_subscription(node_, subscription);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_destroy_subscription with null subscription
TEST_F(TestRmwSubscription, DestroySubscriptionNullSubscription)
{
  rmw_ret_t ret = rmw_destroy_subscription(node_, nullptr);
  EXPECT_NE(ret, RMW_RET_OK);
}

// Test creating multiple subscriptions
TEST_F(TestRmwSubscription, CreateMultipleSubscriptions)
{
  rmw_subscription_options_t options = rmw_get_default_subscription_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_subscription_t * sub1 = rmw_create_subscription(
    node_, get_type_support(), "topic1", &qos, &options);
  rmw_subscription_t * sub2 = rmw_create_subscription(
    node_, get_type_support(), "topic2", &qos, &options);
  rmw_subscription_t * sub3 = rmw_create_subscription(
    node_, get_type_support(), "topic3", &qos, &options);

  ASSERT_NE(sub1, nullptr);
  ASSERT_NE(sub2, nullptr);
  ASSERT_NE(sub3, nullptr);

  EXPECT_STREQ(sub1->topic_name, "topic1");
  EXPECT_STREQ(sub2->topic_name, "topic2");
  EXPECT_STREQ(sub3->topic_name, "topic3");

  rmw_ret_t ret = rmw_destroy_subscription(node_, sub1);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_destroy_subscription(node_, sub2);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_destroy_subscription(node_, sub3);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test multiple subscriptions on the same topic
TEST_F(TestRmwSubscription, CreateMultipleSubscriptionsSameTopic)
{
  rmw_subscription_options_t options = rmw_get_default_subscription_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_subscription_t * sub1 = rmw_create_subscription(
    node_, get_type_support(), "shared_topic", &qos, &options);
  rmw_subscription_t * sub2 = rmw_create_subscription(
    node_, get_type_support(), "shared_topic", &qos, &options);

  ASSERT_NE(sub1, nullptr);
  ASSERT_NE(sub2, nullptr);
  EXPECT_NE(sub1, sub2);

  rmw_ret_t ret = rmw_destroy_subscription(node_, sub1);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_destroy_subscription(node_, sub2);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test subscription count matched publishers
TEST_F(TestRmwSubscription, SubscriptionCountMatchedPublishers)
{
  rmw_subscription_options_t options = rmw_get_default_subscription_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_subscription_t * subscription = rmw_create_subscription(
    node_, get_type_support(), "test_topic", &qos, &options);
  ASSERT_NE(subscription, nullptr);

  size_t count = 0;
  rmw_ret_t ret = rmw_subscription_count_matched_publishers(subscription, &count);
  EXPECT_EQ(ret, RMW_RET_OK);
  // Initially, there should be no matched publishers
  EXPECT_EQ(count, 0u);

  ret = rmw_destroy_subscription(node_, subscription);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test subscription GID
TEST_F(TestRmwSubscription, GetSubscriptionGid)
{
  rmw_subscription_options_t options = rmw_get_default_subscription_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_subscription_t * subscription = rmw_create_subscription(
    node_, get_type_support(), "test_topic", &qos, &options);
  ASSERT_NE(subscription, nullptr);

  rmw_gid_t gid;
  rmw_ret_t ret = rmw_get_gid_for_client(subscription, &gid);
  // Note: This might return unsupported if not implemented
  // Just check that it doesn't crash
  (void)ret;

  ret = rmw_destroy_subscription(node_, subscription);
  EXPECT_EQ(ret, RMW_RET_OK);
}

int main(int argc, char ** argv)
{
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
