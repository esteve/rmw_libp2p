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
#include "rmw/impl/cpp/macros.hpp"
#include "rmw/rmw.h"
#include "rmw/types.h"
#include "rmw/validate_full_topic_name.h"

#include "rcutils/logging_macros.h"

#include "rosidl_typesupport_introspection_cpp/identifier.hpp"

#include "rosidl_typesupport_introspection_c/identifier.h"

#include "impl/custom_node_info.hpp"
#include "impl/custom_subscription_info.hpp"
#include "impl/identifier.hpp"
#include "impl/listener.hpp"

#include "type_support_common.hpp"

// Create and return an rmw subscriber
rmw_subscription_t * rmw_create_subscription(
  const rmw_node_t * node,
  const rosidl_message_type_support_t * type_supports,
  const char * topic_name,
  const rmw_qos_profile_t * qos_policies,
  const rmw_subscription_options_t * subscription_options)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "%s()", __FUNCTION__);

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

  const rosidl_message_type_support_t * type_support =
    get_message_typesupport_handle(type_supports, rosidl_typesupport_introspection_c__identifier);
  if (!type_support) {
    type_support = get_message_typesupport_handle(
      type_supports, rosidl_typesupport_introspection_cpp::typesupport_identifier);
    if (!type_support) {
      RMW_SET_ERROR_MSG("type support not from this implementation");
      return nullptr;
    }
  }

  rmw_libp2p_cpp::CustomSubscriptionInfo * info = nullptr;
  rmw_subscription_t * rmw_subscription = nullptr;

  info = new rmw_libp2p_cpp::CustomSubscriptionInfo();
  info->node_ = node;
  info->typesupport_identifier_ = type_support->typesupport_identifier;

  std::string type_name = _create_type_name(type_support->data, info->typesupport_identifier_);
  if (!_get_registered_type(node_data->node_handle_, type_name, &info->type_support_)) {
    info->type_support_ =
      _create_message_type_support(type_support->data, info->typesupport_identifier_);
    _register_type(node_data->node_handle_, info->type_support_, info->typesupport_identifier_);
  }

  info->qos_ = *qos_policies;
  // TODO(esteve): Set to best-effort & volatile since QoS features are not supported
  info->qos_.history = RMW_QOS_POLICY_HISTORY_KEEP_LAST;
  info->qos_.durability = RMW_QOS_POLICY_DURABILITY_VOLATILE;
  info->qos_.reliability = RMW_QOS_POLICY_RELIABILITY_BEST_EFFORT;

  // TODO(esteve): delete Listener in the destructor
  info->listener_ = new rmw_libp2p_cpp::Listener;

  info->subscription_handle_ = rs_libp2p_custom_subscription_new(
    node_data->node_handle_, topic_name, info, rmw_libp2p_cpp::Listener::on_publication);
  if (!info->subscription_handle_) {
    RMW_SET_ERROR_MSG("failed to create libp2p subscription");
    goto fail;
  }

  rmw_subscription = rmw_subscription_allocate();
  if (!rmw_subscription) {
    RMW_SET_ERROR_MSG("failed to allocate subscription");
    goto fail;
  }

  rmw_subscription->implementation_identifier = libp2p_identifier;
  rmw_subscription->data = info;
  rmw_subscription->topic_name = reinterpret_cast<char *>(rmw_allocate(strlen(topic_name) + 1));
  if (!rmw_subscription->topic_name) {
    RMW_SET_ERROR_MSG("failed to allocate memory for subscription topic name");
    goto fail;
  }
  memcpy(const_cast<char *>(rmw_subscription->topic_name), topic_name, strlen(topic_name) + 1);

  {
    std::lock_guard<std::mutex> lock(node_data->subscriptions_mutex_);
    node_data->subscriptions_[topic_name].insert(info);
  }

  return rmw_subscription;

fail:
  _delete_typesupport(info->type_support_, info->typesupport_identifier_);
  if (info->subscription_handle_) {
    rs_libp2p_custom_subscription_free(info->subscription_handle_);
  }
  delete info;

  if (rmw_subscription) {
    if (rmw_subscription->topic_name) {
      rmw_free(const_cast<char *>(rmw_subscription->topic_name));
    }
    rmw_subscription_free(rmw_subscription);
  }

  return nullptr;
}

// Destroy and deallocate an RMW subscription
rmw_ret_t rmw_destroy_subscription(rmw_node_t * node, rmw_subscription_t * subscription)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "%s()", __FUNCTION__);

  (void)node;
  (void)subscription;

  // return RMW_RET_ERROR;
  return RMW_RET_OK;
}

rmw_ret_t rmw_subscription_get_actual_qos(
  const rmw_subscription_t * subscription,
  rmw_qos_profile_t * qos)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "%s()", __FUNCTION__);

  (void)subscription;
  (void)qos;

  RMW_CHECK_ARGUMENT_FOR_NULL(subscription, RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_ARGUMENT_FOR_NULL(qos, RMW_RET_INVALID_ARGUMENT);

  auto info = static_cast<rmw_libp2p_cpp::CustomSubscriptionInfo *>(subscription->data);
  *qos = info->qos_;
  return RMW_RET_OK;
}
