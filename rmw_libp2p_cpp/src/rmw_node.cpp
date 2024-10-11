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

#include <mutex>

#include "rcutils/logging_macros.h"
#include "rcutils/strdup.h"

#include "rcpputils/scope_exit.hpp"

#include "rmw/allocators.h"
#include "rmw/error_handling.h"
#include "rmw/impl/cpp/macros.hpp"
#include "rmw/rmw.h"
#include "rmw/validate_namespace.h"
#include "rmw/validate_node_name.h"

#include "impl/identifier.hpp"
#include "impl/custom_node_info.hpp"
#include "impl/rmw_libp2p_rs.hpp"

extern "C"
{
// TODO(esteve): cleanup
rmw_guard_condition_t *
libp2p_c__rmw_create_guard_condition(rmw_context_t * context);

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_destroy_node(
  rmw_node_t * node);


// Create a node and return a handle to that node.
//
// rmw_node_t Doc: http://docs.ros2.org/latest/api/rmw/structrmw__node__t.html
RMW_PUBLIC
rmw_node_t *
libp2p_c__rmw_create_node(
  rmw_context_t * context,
  const char * name,
  const char * namespace_)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s(name=%s,namespace_=%s)",
    __FUNCTION__, name, namespace_);

  // ASSERTIONS ================================================================
  RMW_CHECK_ARGUMENT_FOR_NULL(context, nullptr);
  RMW_CHECK_TYPE_IDENTIFIERS_MATCH(
    context,
    context->implementation_identifier,
    libp2p_identifier,
    return nullptr);
  RMW_CHECK_FOR_NULL_WITH_MSG(context->impl, "expected initialized context", return nullptr);
  RMW_CHECK_FOR_NULL_WITH_MSG(name, "name is null", return nullptr);
  RMW_CHECK_FOR_NULL_WITH_MSG(namespace_, "namespace is null", return nullptr);

  if (context->impl->is_shutdown) {
    RMW_SET_ERROR_MSG("context has been shutdown");
    return nullptr;
  }

  // Declare everything before beginning to create things.
  rmw_libp2p_cpp::CustomNodeInfo * node_impl = nullptr;
  rmw_node_t * node_handle = nullptr;

  node_handle = rmw_node_allocate();
  if (!node_handle) {
    RMW_SET_ERROR_MSG("failed to allocate rmw_node_t");
    goto fail;
  }
  node_handle->implementation_identifier = libp2p_identifier;

  node_handle->name =
    static_cast<const char *>(rmw_allocate(sizeof(char) * strlen(name) + 1));
  if (!node_handle->name) {
    RMW_SET_ERROR_MSG("failed to allocate memory");
    goto fail;
  }
  memcpy(const_cast<char *>(node_handle->name), name, strlen(name) + 1);
  node_handle->namespace_ =
    static_cast<const char *>(rmw_allocate(sizeof(char) * strlen(namespace_) + 1));
  if (!node_handle->namespace_) {
    RMW_SET_ERROR_MSG("failed to allocate memory");
    goto fail;
  }
  memcpy(const_cast<char *>(node_handle->namespace_), namespace_, strlen(namespace_) + 1);

  try {
    node_impl = new rmw_libp2p_cpp::CustomNodeInfo();
  } catch (std::bad_alloc &) {
    RMW_SET_ERROR_MSG("failed to allocate node impl struct");
    goto fail;
  }
  node_handle->data = node_impl;

  node_impl->graph_guard_condition_ = libp2p_c__rmw_create_guard_condition(context);
  if (!node_impl->graph_guard_condition_) {
    // error already set
    goto fail;
  }

  node_impl->node_handle_ = rs_libp2p_custom_node_new();
  if (!node_impl->node_handle_) {
    RMW_SET_ERROR_MSG("failed to allocate libp2p node");
    goto fail;
  }

  // Assign ROS context
  node_handle->context = context;

  return node_handle;

fail:
  rmw_ret_t ret = libp2p_c__rmw_destroy_node(node_handle);
  if (ret != RMW_RET_OK) {
    RCUTILS_LOG_ERROR_NAMED(
      "rmw_libp2p_cpp",
      "failed to destroy node during error handling");
  }
  return nullptr;
}

// Finalize a given node handle, reclaim the resources, and deallocate the node handle.
RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_destroy_node(
  rmw_node_t * node)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s(node=%p)", __FUNCTION__, (void *)node);

  if (!node) {
    RMW_SET_ERROR_MSG("node handle is null");
    return RMW_RET_ERROR;
  }

  if (node->implementation_identifier != libp2p_identifier) {
    RMW_SET_ERROR_MSG("node handle not from this implementation");
    return RMW_RET_ERROR;
  }

  auto impl = static_cast<rmw_libp2p_cpp::CustomNodeInfo *>(node->data);
  if (impl) {
    if (impl->node_handle_) {
      rs_libp2p_custom_node_free(impl->node_handle_);
    }
    if (impl->graph_guard_condition_) {
      rmw_ret_t ret = rmw_destroy_guard_condition(impl->graph_guard_condition_);
      if (ret != RMW_RET_OK) {
        RCUTILS_LOG_ERROR_NAMED(
          "rmw_libp2p_cpp",
          "failed to destroy guard condition");
      }
    }
    delete impl;
  }
  if (node->namespace_) {
    rmw_free(const_cast<char *>(node->namespace_));
  }
  if (node->name) {
    rmw_free(const_cast<char *>(node->name));
  }
  rmw_node_free(node);

  return RMW_RET_OK;
}

RMW_PUBLIC
const rmw_guard_condition_t *
libp2p_c__rmw_node_get_graph_guard_condition(const rmw_node_t * node)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s(node=%p)", __FUNCTION__, (void *)node);

  auto impl = static_cast<rmw_libp2p_cpp::CustomNodeInfo *>(node->data);
  if (!impl) {
    RMW_SET_ERROR_MSG("node impl is null");
    return nullptr;
  }
  return impl->graph_guard_condition_;
}
}  // extern "C"
