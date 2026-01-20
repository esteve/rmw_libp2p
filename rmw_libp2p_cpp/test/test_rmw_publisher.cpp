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

class TestRmwPublisher : public ::testing::Test
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

    node_ = rmw_create_node(&context_, "test_publisher_node", "/test");
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

// Test rmw_create_publisher with valid arguments
TEST_F(TestRmwPublisher, CreatePublisherValid)
{
  rmw_publisher_options_t options = rmw_get_default_publisher_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_publisher_t * publisher = rmw_create_publisher(
    node_,
    get_type_support(),
    "test_topic",
    &qos,
    &options);
  ASSERT_NE(publisher, nullptr);

  EXPECT_NE(publisher->implementation_identifier, nullptr);
  EXPECT_STREQ(publisher->topic_name, "test_topic");

  rmw_ret_t ret = rmw_destroy_publisher(node_, publisher);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_create_publisher with null node
TEST_F(TestRmwPublisher, CreatePublisherNullNode)
{
  rmw_publisher_options_t options = rmw_get_default_publisher_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_publisher_t * publisher = rmw_create_publisher(
    nullptr,
    get_type_support(),
    "test_topic",
    &qos,
    &options);
  EXPECT_EQ(publisher, nullptr);
}

// Test rmw_create_publisher with null type support
TEST_F(TestRmwPublisher, CreatePublisherNullTypeSupport)
{
  rmw_publisher_options_t options = rmw_get_default_publisher_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_publisher_t * publisher = rmw_create_publisher(
    node_,
    nullptr,
    "test_topic",
    &qos,
    &options);
  EXPECT_EQ(publisher, nullptr);
}

// Test rmw_create_publisher with null topic name
TEST_F(TestRmwPublisher, CreatePublisherNullTopicName)
{
  rmw_publisher_options_t options = rmw_get_default_publisher_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_publisher_t * publisher = rmw_create_publisher(
    node_,
    get_type_support(),
    nullptr,
    &qos,
    &options);
  EXPECT_EQ(publisher, nullptr);
}

// Test rmw_destroy_publisher with null node
TEST_F(TestRmwPublisher, DestroyPublisherNullNode)
{
  rmw_publisher_options_t options = rmw_get_default_publisher_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_publisher_t * publisher = rmw_create_publisher(
    node_,
    get_type_support(),
    "test_topic",
    &qos,
    &options);
  ASSERT_NE(publisher, nullptr);

  rmw_ret_t ret = rmw_destroy_publisher(nullptr, publisher);
  EXPECT_NE(ret, RMW_RET_OK);

  // Clean up properly
  ret = rmw_destroy_publisher(node_, publisher);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_destroy_publisher with null publisher
TEST_F(TestRmwPublisher, DestroyPublisherNullPublisher)
{
  rmw_ret_t ret = rmw_destroy_publisher(node_, nullptr);
  EXPECT_NE(ret, RMW_RET_OK);
}

// Test creating multiple publishers
TEST_F(TestRmwPublisher, CreateMultiplePublishers)
{
  rmw_publisher_options_t options = rmw_get_default_publisher_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_publisher_t * pub1 = rmw_create_publisher(
    node_, get_type_support(), "topic1", &qos, &options);
  rmw_publisher_t * pub2 = rmw_create_publisher(
    node_, get_type_support(), "topic2", &qos, &options);
  rmw_publisher_t * pub3 = rmw_create_publisher(
    node_, get_type_support(), "topic3", &qos, &options);

  ASSERT_NE(pub1, nullptr);
  ASSERT_NE(pub2, nullptr);
  ASSERT_NE(pub3, nullptr);

  EXPECT_STREQ(pub1->topic_name, "topic1");
  EXPECT_STREQ(pub2->topic_name, "topic2");
  EXPECT_STREQ(pub3->topic_name, "topic3");

  rmw_ret_t ret = rmw_destroy_publisher(node_, pub1);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_destroy_publisher(node_, pub2);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_destroy_publisher(node_, pub3);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test multiple publishers on the same topic
TEST_F(TestRmwPublisher, CreateMultiplePublishersSameTopic)
{
  rmw_publisher_options_t options = rmw_get_default_publisher_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_publisher_t * pub1 = rmw_create_publisher(
    node_, get_type_support(), "shared_topic", &qos, &options);
  rmw_publisher_t * pub2 = rmw_create_publisher(
    node_, get_type_support(), "shared_topic", &qos, &options);

  ASSERT_NE(pub1, nullptr);
  ASSERT_NE(pub2, nullptr);
  EXPECT_NE(pub1, pub2);

  rmw_ret_t ret = rmw_destroy_publisher(node_, pub1);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_destroy_publisher(node_, pub2);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_get_gid_for_publisher
TEST_F(TestRmwPublisher, GetGidForPublisher)
{
  rmw_publisher_options_t options = rmw_get_default_publisher_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_publisher_t * publisher = rmw_create_publisher(
    node_, get_type_support(), "test_topic", &qos, &options);
  ASSERT_NE(publisher, nullptr);

  rmw_gid_t gid;
  rmw_ret_t ret = rmw_get_gid_for_publisher(publisher, &gid);
  EXPECT_EQ(ret, RMW_RET_OK);

  // GID should not be all zeros
  bool all_zeros = true;
  for (size_t i = 0; i < RMW_GID_STORAGE_SIZE; ++i) {
    if (gid.data[i] != 0) {
      all_zeros = false;
      break;
    }
  }
  EXPECT_FALSE(all_zeros);

  ret = rmw_destroy_publisher(node_, publisher);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test publisher count matched subscriptions
TEST_F(TestRmwPublisher, PublisherCountMatchedSubscriptions)
{
  rmw_publisher_options_t options = rmw_get_default_publisher_options();
  rmw_qos_profile_t qos = rmw_qos_profile_default;

  rmw_publisher_t * publisher = rmw_create_publisher(
    node_, get_type_support(), "test_topic", &qos, &options);
  ASSERT_NE(publisher, nullptr);

  size_t count = 0;
  rmw_ret_t ret = rmw_publisher_count_matched_subscriptions(publisher, &count);
  EXPECT_EQ(ret, RMW_RET_OK);
  // Initially, there should be no matched subscriptions
  EXPECT_EQ(count, 0u);

  ret = rmw_destroy_publisher(node_, publisher);
  EXPECT_EQ(ret, RMW_RET_OK);
}

int main(int argc, char ** argv)
{
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
