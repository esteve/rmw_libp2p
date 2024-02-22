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

#include <iostream>

#include <mutex>

#include "rmw/allocators.h"
#include "rmw/error_handling.h"
#include "rmw/impl/cpp/macros.hpp"
#include "rmw/types.h"
#include "rmw/validate_full_topic_name.h"
#include "rmw/rmw.h"

#include "rcutils/logging_macros.h"

#include "rosidl_typesupport_introspection_cpp/identifier.hpp"

#include "rosidl_typesupport_introspection_c/identifier.h"

#include "rmw_libp2p_cpp/identifier.hpp"
#include "rmw_libp2p_cpp/custom_node_info.hpp"
#include "rmw_libp2p_cpp/custom_subscription_info.hpp"
#include "rmw_libp2p_cpp/Listener.hpp"

#include "type_support_common.hpp"

// Create and return an rmw subscriber
rmw_subscription_t *
rmw_create_subscription(
  const rmw_node_t * node,
  const rosidl_message_type_support_t * type_supports,
  const char * topic_name,
  const rmw_qos_profile_t * qos_policies,
  const rmw_subscription_options_t * subscription_options)
{
  RCUTILS_LOG_WARN_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  //   RCUTILS_LOG_WARN_NAMED(
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

  auto node_data = static_cast<CustomNodeInfo *>(node->data);
  if (!node_data) {
    RMW_SET_ERROR_MSG("node data is null");
    return nullptr;
  }

  if (!node_data->node_handle_) {
    RMW_SET_ERROR_MSG("node handle is null");
    return nullptr;
  }

  std::cout << "rmw_create_subscription: " << topic_name << std::endl;
  std::cout << "SUB: TRY TYPESUPPORT 1" << std::endl;
  const rosidl_message_type_support_t * type_support = get_message_typesupport_handle(
    type_supports, rosidl_typesupport_introspection_c__identifier);
  std::cout << "SUB: TRY TYPESUPPORT 2" << std::endl;
  if (!type_support) {
    std::cout << "SUB: TRY TYPESUPPORT 2a" << std::endl;
    type_support = get_message_typesupport_handle(
      type_supports, rosidl_typesupport_introspection_cpp::typesupport_identifier);
    if (!type_support) {
      std::cout << "SUB: TRY TYPESUPPORT 2aa" << std::endl;
      RMW_SET_ERROR_MSG("type support not from this implementation");
      return nullptr;
    }
  }

  std::cout << "SUB: TRY TYPESUPPORT 3" << std::endl;

  CustomSubscriptionInfo * info = nullptr;
  //   std::string dps_topic = _get_dps_topic_name(impl->domain_id_, topic_name);
  //   const char * topic = dps_topic.c_str();
  rmw_subscription_t * rmw_subscription = nullptr;
  //   rmw_libp2p_cpp::cdr::WriteCDRBuffer ser;
  //   DPS_Status ret;

  info = new CustomSubscriptionInfo();
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
  std::cout << "SUB: TRY TYPESUPPORT 4" << std::endl;

  info->qos_ = *qos_policies;
  /* Set to best-effort & volatile since QoS features are not supported by DPS at the moment. */
  info->qos_.history = RMW_QOS_POLICY_HISTORY_KEEP_LAST;
  info->qos_.durability = RMW_QOS_POLICY_DURABILITY_VOLATILE;
  info->qos_.reliability = RMW_QOS_POLICY_RELIABILITY_BEST_EFFORT;

  info->subscription_handle_ =
    rs_libp2p_custom_subscription_new(node_data->node_handle_, topic_name,
    info, Listener::on_publication);
  if (!info->subscription_handle_) {
    RMW_SET_ERROR_MSG("failed to create libp2p subscription");
    goto fail;
  }
  std::cout << "SUB: TRY TYPESUPPORT 5" << std::endl;

  // info->publication_ = DPS_CreatePublication(node_data->impl);
  // if (!info->publication_) {
  //   RMW_SET_ERROR_MSG("failed to create publication");
  //   goto fail;
  // }
  // ret = DPS_InitPublication(info->publication_, &topic, 1, DPS_TRUE, nullptr);
  // if (ret != DPS_OK) {
  //   RMW_SET_ERROR_MSG("failed to initialize publication");
  //   goto fail;
  // }

  rmw_subscription = rmw_subscription_allocate();
  if (!rmw_subscription) {
    RMW_SET_ERROR_MSG("failed to allocate subscription");
    goto fail;
  }
  std::cout << "SUB: TRY TYPESUPPORT 6" << std::endl;

  rmw_subscription->implementation_identifier = libp2p_identifier;
  rmw_subscription->data = info;
  rmw_subscription->topic_name = reinterpret_cast<char *>(
    rmw_allocate(strlen(topic_name) + 1));
  if (!rmw_subscription->topic_name) {
    RMW_SET_ERROR_MSG("failed to allocate memory for subscription topic name");
    goto fail;
  }
  memcpy(const_cast<char *>(rmw_subscription->topic_name), topic_name, strlen(topic_name) + 1);

  // info->discovery_name_ = dps_subscription_prefix + std::string(topic_name) +
  //   "&types=" + type_name;
  // if (_add_discovery_topic(impl, info->discovery_name_) != RMW_RET_OK) {
  //   goto fail;
  // }

  {
    std::lock_guard<std::mutex> lock(node_data->subscriptions_mutex_);
    node_data->subscriptions_[topic_name].insert(info);
  }

  std::cout << "SUB: TRY TYPESUPPORT 9" << std::endl;

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
rmw_ret_t
rmw_destroy_subscription(
  rmw_node_t * node,
  rmw_subscription_t * subscription)
{
  RCUTILS_LOG_WARN_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)subscription;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_subscription_get_actual_qos(
  const rmw_subscription_t * subscription,
  rmw_qos_profile_t * qos)
{
  RCUTILS_LOG_WARN_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscription;
  (void)qos;

  RMW_CHECK_ARGUMENT_FOR_NULL(subscription, RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_ARGUMENT_FOR_NULL(qos, RMW_RET_INVALID_ARGUMENT);

  auto info = static_cast<CustomSubscriptionInfo *>(subscription->data);
  *qos = info->qos_;
  return RMW_RET_OK;
}
