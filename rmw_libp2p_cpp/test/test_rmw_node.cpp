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

class TestRmwNode : public ::testing::Test
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

// Test rmw_create_node with valid arguments
TEST_F(TestRmwNode, CreateNodeValid)
{
  rmw_node_t * node = rmw_create_node(&context_, "test_node", "/test_namespace");
  ASSERT_NE(node, nullptr);

  EXPECT_STREQ(node->name, "test_node");
  EXPECT_STREQ(node->namespace_, "/test_namespace");
  EXPECT_NE(node->implementation_identifier, nullptr);
  EXPECT_EQ(node->context, &context_);

  rmw_ret_t ret = rmw_destroy_node(node);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_create_node with null context
TEST_F(TestRmwNode, CreateNodeNullContext)
{
  rmw_node_t * node = rmw_create_node(nullptr, "test_node", "/test_namespace");
  EXPECT_EQ(node, nullptr);
}

// Test rmw_create_node with null name
TEST_F(TestRmwNode, CreateNodeNullName)
{
  rmw_node_t * node = rmw_create_node(&context_, nullptr, "/test_namespace");
  EXPECT_EQ(node, nullptr);
}

// Test rmw_create_node with null namespace
TEST_F(TestRmwNode, CreateNodeNullNamespace)
{
  rmw_node_t * node = rmw_create_node(&context_, "test_node", nullptr);
  EXPECT_EQ(node, nullptr);
}

// Test rmw_create_node with empty name and namespace
TEST_F(TestRmwNode, CreateNodeEmptyNameAndNamespace)
{
  rmw_node_t * node = rmw_create_node(&context_, "", "");
  ASSERT_NE(node, nullptr);

  EXPECT_STREQ(node->name, "");
  EXPECT_STREQ(node->namespace_, "");

  rmw_ret_t ret = rmw_destroy_node(node);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test rmw_destroy_node with null node
TEST_F(TestRmwNode, DestroyNodeNull)
{
  rmw_ret_t ret = rmw_destroy_node(nullptr);
  EXPECT_EQ(ret, RMW_RET_ERROR);
}

// Test rmw_node_get_graph_guard_condition with valid node
TEST_F(TestRmwNode, GetGraphGuardConditionValid)
{
  rmw_node_t * node = rmw_create_node(&context_, "test_node", "/test_namespace");
  ASSERT_NE(node, nullptr);

  const rmw_guard_condition_t * gc = rmw_node_get_graph_guard_condition(node);
  EXPECT_NE(gc, nullptr);

  rmw_ret_t ret = rmw_destroy_node(node);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test creating multiple nodes
TEST_F(TestRmwNode, CreateMultipleNodes)
{
  rmw_node_t * node1 = rmw_create_node(&context_, "node1", "/ns1");
  rmw_node_t * node2 = rmw_create_node(&context_, "node2", "/ns2");
  rmw_node_t * node3 = rmw_create_node(&context_, "node3", "/ns3");

  ASSERT_NE(node1, nullptr);
  ASSERT_NE(node2, nullptr);
  ASSERT_NE(node3, nullptr);

  EXPECT_NE(node1, node2);
  EXPECT_NE(node2, node3);
  EXPECT_NE(node1, node3);

  EXPECT_STREQ(node1->name, "node1");
  EXPECT_STREQ(node2->name, "node2");
  EXPECT_STREQ(node3->name, "node3");

  rmw_ret_t ret = rmw_destroy_node(node1);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_destroy_node(node2);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_destroy_node(node3);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test creating nodes with same name in different namespaces
TEST_F(TestRmwNode, CreateNodesWithSameNameDifferentNamespace)
{
  rmw_node_t * node1 = rmw_create_node(&context_, "test_node", "/ns1");
  rmw_node_t * node2 = rmw_create_node(&context_, "test_node", "/ns2");

  ASSERT_NE(node1, nullptr);
  ASSERT_NE(node2, nullptr);

  EXPECT_STREQ(node1->name, "test_node");
  EXPECT_STREQ(node2->name, "test_node");
  EXPECT_STRNE(node1->namespace_, node2->namespace_);

  rmw_ret_t ret = rmw_destroy_node(node1);
  EXPECT_EQ(ret, RMW_RET_OK);
  ret = rmw_destroy_node(node2);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test node lifecycle with long name
TEST_F(TestRmwNode, CreateNodeWithLongName)
{
  std::string long_name(256, 'a');
  std::string long_ns(256, 'b');

  rmw_node_t * node = rmw_create_node(&context_, long_name.c_str(), long_ns.c_str());
  ASSERT_NE(node, nullptr);

  EXPECT_STREQ(node->name, long_name.c_str());
  EXPECT_STREQ(node->namespace_, long_ns.c_str());

  rmw_ret_t ret = rmw_destroy_node(node);
  EXPECT_EQ(ret, RMW_RET_OK);
}

// Test node with special characters in name
TEST_F(TestRmwNode, CreateNodeWithSpecialCharacters)
{
  rmw_node_t * node = rmw_create_node(&context_, "test_node_123", "/test/nested/namespace");
  ASSERT_NE(node, nullptr);

  EXPECT_STREQ(node->name, "test_node_123");
  EXPECT_STREQ(node->namespace_, "/test/nested/namespace");

  rmw_ret_t ret = rmw_destroy_node(node);
  EXPECT_EQ(ret, RMW_RET_OK);
}

int main(int argc, char ** argv)
{
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
