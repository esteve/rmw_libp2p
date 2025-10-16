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

#include "rmw/allocators.h"
#include "rmw/error_handling.h"
#include "rmw/types.h"
#include "rmw/validate_full_topic_name.h"
#include "rmw/rmw.h"

#include "rmw/impl/cpp/macros.hpp"

#include "rcutils/allocator.h"
#include "rcutils/logging_macros.h"
#include "rcutils/strdup.h"

#include "rosidl_typesupport_introspection_cpp/identifier.hpp"

#include "rosidl_typesupport_introspection_c/identifier.h"

#include "impl/identifier.hpp"
#include "impl/custom_node_info.hpp"
#include "impl/custom_publisher_info.hpp"

#include "type_support_common.hpp"

// Create and return an rmw publisher.
rmw_publisher_t *
rmw_create_publisher(
  const rmw_node_t * node,
  const rosidl_message_type_support_t * type_supports,
  const char * topic_name,
  const rmw_qos_profile_t * qos_policies,
  const rmw_publisher_options_t * publisher_options)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

//   RCUTILS_LOG_DEBUG_NAMED(
//     "rmw_libp2p_cpp",
//     "%s(node=%p,type_supports=%p,topic_name=%s,"
//     "qos_policies={history=%s,depth=%zu,reliability=%s,durability=%s},publisher_options=%p)",
//     __FUNCTION__, reinterpret_cast<const void *>(node),
//     reinterpret_cast<const void *>(type_supports), topic_name,
//     qos_history_string(qos_policies->history), qos_policies->depth,
//     qos_reliability_string(qos_policies->reliability),
//     qos_durability_string(qos_policies->durability),
//     reinterpret_cast<const void *>(publisher_options));

  if (!node) {
    RMW_SET_ERROR_MSG("node handle is null");
    return nullptr;
  }

  if (node->implementation_identifier != libp2p_identifier) {
    RMW_SET_ERROR_MSG("node handle not from this implementation");
    return nullptr;
  }

  if (!topic_name || strlen(topic_name) == 0) {
    RMW_SET_ERROR_MSG("publisher topic is null or empty string");
    return nullptr;
  }

  if (!qos_policies) {
    RMW_SET_ERROR_MSG("qos_policies is null");
    return nullptr;
  }

  auto node_data = static_cast<rmw_libp2p_cpp::CustomNodeInfo *>(node->data);
  if (!node_data) {
    RMW_SET_ERROR_MSG("node data is null");
    return nullptr;
  }

  if (!node_data->node_handle_) {
    RMW_SET_ERROR_MSG("node handle is null");
    return nullptr;
  }

  std::cout << "PUB FOOOBAR" << std::endl;

  const rosidl_message_type_support_t * type_support = get_message_typesupport_handle(
    type_supports, rosidl_typesupport_introspection_c__identifier);
  if (!type_support) {
    std::cout << "PUB NO TS 1" << std::endl;
    type_support = get_message_typesupport_handle(
      type_supports, rosidl_typesupport_introspection_cpp::typesupport_identifier);
    if (!type_support) {
      std::cout << "PUB NO TS 2" << std::endl;

      RMW_SET_ERROR_MSG("type support not from this implementation");
      return nullptr;
    }
  }

  std::cout << "PUB YES TS 1 ???" << std::endl;

  rmw_libp2p_cpp::CustomPublisherInfo * info = nullptr;
  rmw_publisher_t * rmw_publisher = nullptr;

  info = new rmw_libp2p_cpp::CustomPublisherInfo();
  info->node_ = node;
  info->typesupport_identifier_ = type_support->typesupport_identifier;

  std::string type_name = _create_type_name(
    type_support->data, info->typesupport_identifier_);
  if (!_get_registered_type(node_data->node_handle_, type_name, &info->type_support_)) {
    info->type_support_ = _create_message_type_support(
      type_support->data,
      info->typesupport_identifier_);
    _register_type(node_data->node_handle_, info->type_support_, info->typesupport_identifier_);
  }

  info->qos_ = *qos_policies;
  // TODO(esteve): Set to best-effort & volatile since QoS features are not supported
  info->qos_.history = RMW_QOS_POLICY_HISTORY_KEEP_LAST;
  info->qos_.durability = RMW_QOS_POLICY_DURABILITY_VOLATILE;
  info->qos_.reliability = RMW_QOS_POLICY_RELIABILITY_BEST_EFFORT;

  info->publisher_handle_ = rs_libp2p_custom_publisher_new(node_data->node_handle_, topic_name);
  if (!info->publisher_handle_) {
    RMW_SET_ERROR_MSG("failed to create libp2p publisher");
    goto fail;
  }

  rmw_publisher = rmw_publisher_allocate();
  if (!rmw_publisher) {
    RMW_SET_ERROR_MSG("failed to allocate publisher");
    goto fail;
  }

  rmw_publisher->implementation_identifier = libp2p_identifier;
  rmw_publisher->data = info;
  rmw_publisher->topic_name = reinterpret_cast<char *>(
    rmw_allocate(strlen(topic_name) + 1));
  if (!rmw_publisher->topic_name) {
    RMW_SET_ERROR_MSG("failed to allocate memory for publisher topic name");
    goto fail;
  }
  memcpy(const_cast<char *>(rmw_publisher->topic_name), topic_name, strlen(topic_name) + 1);

  {
    std::lock_guard<std::mutex> lock(node_data->publishers_mutex_);
    node_data->publishers_[topic_name].insert(info);
  }

  return rmw_publisher;

fail:
  _delete_typesupport(info->type_support_, info->typesupport_identifier_);
  if (info->publisher_handle_) {
    rs_libp2p_custom_publisher_free(info->publisher_handle_);
  }
  delete info;

  if (rmw_publisher) {
    if (rmw_publisher->topic_name) {
      rmw_free(const_cast<char *>(rmw_publisher->topic_name));
    }
    rmw_publisher_free(rmw_publisher);
  }

  return nullptr;
}

// Destroy and deallocate an rmw publisher.
rmw_ret_t
rmw_destroy_publisher(
  rmw_node_t * node,
  rmw_publisher_t * publisher)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)publisher;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_publisher_get_actual_qos(
  const rmw_publisher_t * publisher,
  rmw_qos_profile_t * qos)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  RMW_CHECK_ARGUMENT_FOR_NULL(publisher, RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_ARGUMENT_FOR_NULL(qos, RMW_RET_INVALID_ARGUMENT);

  auto info = static_cast<rmw_libp2p_cpp::CustomPublisherInfo *>(publisher->data);
  *qos = info->qos_;
  return RMW_RET_OK;
}
