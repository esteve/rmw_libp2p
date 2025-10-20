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
#include "rmw/rmw.h"

#include "rcutils/logging_macros.h"

#include "rosidl_typesupport_introspection_cpp/identifier.hpp"

#include "rosidl_typesupport_introspection_c/identifier.h"

#include "impl/identifier.hpp"
#include "impl/custom_node_info.hpp"
#include "impl/custom_client_info.hpp"
#include "impl/custom_subscription_info.hpp"
#include "impl/listener.hpp"

#include "client_service_common.hpp"
#include "type_support_common.hpp"

rmw_client_t *
rmw_create_client(
  const rmw_node_t * node,
  const rosidl_service_type_support_t * type_supports,
  const char * service_name,
  const rmw_qos_profile_t * qos_policies)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  if (!node) {
    RMW_SET_ERROR_MSG("node handle is null");
    return nullptr;
  }

  if (node->implementation_identifier != libp2p_identifier) {
    RMW_SET_ERROR_MSG("node handle not from this implementation");
    return nullptr;
  }

  if (!service_name || strlen(service_name) == 0) {
    RMW_SET_ERROR_MSG("client topic is null or empty string");
    return nullptr;
  }

  if (!qos_policies) {
    RMW_SET_ERROR_MSG("qos_profile is null");
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

  const rosidl_service_type_support_t * type_support = get_service_typesupport_handle(
  type_supports, rosidl_typesupport_introspection_c__identifier);
  if (!type_support) {
    type_support = get_service_typesupport_handle(
      type_supports, rosidl_typesupport_introspection_cpp::typesupport_identifier);
    if (!type_support) {
      RMW_SET_ERROR_MSG("type support not from this implementation");
      return nullptr;
    }
  }

  rmw_libp2p_cpp::CustomClientInfo * info = nullptr;
  rmw_client_t * rmw_client = nullptr;

  info = new rmw_libp2p_cpp::CustomClientInfo();
  info->node_ = node;

  info->typesupport_identifier_ = type_support->typesupport_identifier;
  info->request_publisher_ = new rmw_libp2p_cpp::CustomPublisherInfo();
  info->request_publisher_->node_ = node;
  info->request_publisher_->typesupport_identifier_ = type_support->typesupport_identifier;
  info->request_publisher_->qos_ = *qos_policies;

  info->response_subscription_ = new rmw_libp2p_cpp::CustomSubscriptionInfo();
  info->response_subscription_->node_ = node;
  info->response_subscription_->typesupport_identifier_ = type_support->typesupport_identifier;
  info->response_subscription_->qos_ = *qos_policies;

  const void * untyped_request_members;
  const void * untyped_response_members;

  untyped_request_members =
    get_request_ptr(type_support->data, info->typesupport_identifier_);
  untyped_response_members = get_response_ptr(type_support->data,
      info->typesupport_identifier_);

  std::string request_type_name = _create_type_name(untyped_request_members,
      info->typesupport_identifier_);

  std::string response_type_name = _create_type_name(untyped_response_members,
      info->typesupport_identifier_);

  if (!_get_registered_type(node_data->node_handle_, request_type_name, &info->request_publisher_->type_support_)) {
    info->request_publisher_->type_support_ = _create_request_type_support(type_support->data,
        info->typesupport_identifier_);
    _register_type(node_data->node_handle_, info->request_publisher_->type_support_, info->typesupport_identifier_);
  }

  if (!_get_registered_type(node_data->node_handle_, response_type_name, &info->response_subscription_->type_support_)) {
    info->response_subscription_->type_support_ = _create_response_type_support(type_support->data,
        info->typesupport_identifier_);
    _register_type(node_data->node_handle_, info->response_subscription_->type_support_, info->typesupport_identifier_);
  }

  // TODO(esteve): delete Listener in the destructor
  info->listener_ = new rmw_libp2p_cpp::Listener;
  info->response_subscription_->listener_ = info->listener_;
  info->service_name_ = service_name;
  info->request_publisher_->topic_name_ = std::string(service_name) + "/request";

  info->request_publisher_->publisher_handle_ = rs_libp2p_custom_publisher_new(node_data->node_handle_, info->request_publisher_->topic_name_.c_str());
  if (!info->request_publisher_->publisher_handle_) {
    RMW_SET_ERROR_MSG("failed to create libp2p publisher for service");
    // goto fail;
  }

  // Get header
  rmw_gid_t request_guid;
  memset(request_guid.data, 0, RMW_GID_STORAGE_SIZE);
  const size_t ret = rs_libp2p_custom_publisher_get_gid(
    info->request_publisher_->publisher_handle_, request_guid.data);
  // if (ret == 0) {
  //   RMW_SET_ERROR_MSG("no guid found for publisher");
  //   goto fail;
  // }

  char uuid_str[37] = {};
  sprintf(uuid_str,
    "%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x",
    static_cast<uint8_t>(request_guid.data[0]),
    static_cast<uint8_t>(request_guid.data[1]),
    static_cast<uint8_t>(request_guid.data[2]),
    static_cast<uint8_t>(request_guid.data[3]),
    static_cast<uint8_t>(request_guid.data[4]),
    static_cast<uint8_t>(request_guid.data[5]),
    static_cast<uint8_t>(request_guid.data[6]),
    static_cast<uint8_t>(request_guid.data[7]),
    static_cast<uint8_t>(request_guid.data[8]),
    static_cast<uint8_t>(request_guid.data[9]),
    static_cast<uint8_t>(request_guid.data[10]),
    static_cast<uint8_t>(request_guid.data[11]),
    static_cast<uint8_t>(request_guid.data[12]),
    static_cast<uint8_t>(request_guid.data[13]),
    static_cast<uint8_t>(request_guid.data[14]),
    static_cast<uint8_t>(request_guid.data[15])
  );

  std::cout << "rmw_client. publisher guid: " << uuid_str << std::endl;
  std::string topic_name = service_name + std::string("/response/") + uuid_str;
  info->discovery_name_ = topic_name;

  info->response_subscription_->subscription_handle_ = rs_libp2p_custom_subscription_new(
    node_data->node_handle_, info->discovery_name_.c_str(),
    info->response_subscription_, rmw_libp2p_cpp::Listener::on_publication);

  rmw_client = rmw_client_allocate();
  if (!rmw_client) {
    RMW_SET_ERROR_MSG("failed to allocate memory for client");
    goto fail;
  }
  rmw_client->implementation_identifier = libp2p_identifier;
  rmw_client->data = info;
  rmw_client->service_name = reinterpret_cast<const char *>(
    rmw_allocate(strlen(service_name) + 1));
  if (!rmw_client->service_name) {
    RMW_SET_ERROR_MSG("failed to allocate memory for client name");
    goto fail;
  }
  memcpy(const_cast<char *>(rmw_client->service_name), service_name, strlen(service_name) + 1);

  {
    std::lock_guard<std::mutex> lock(node_data->clients_mutex_);
    node_data->clients_[service_name].insert(info);
  }
  return rmw_client;

fail:
  if (node_data) {
    if (info->request_publisher_->type_support_) {
    //   _unregister_type(node_data->node_, info->request_type_support_, info->typesupport_identifier_);
    }

    if (info->response_subscription_->type_support_) {
    //   _unregister_type(node_data->node_, info->response_type_support_, info->typesupport_identifier_);
    }
  } else {
    RCUTILS_LOG_ERROR_NAMED(
      "rmw_libp2p_cpp",
      "leaking type support objects because node node_data is null");
  }

  if (info->request_publisher_->publisher_handle_) {
    rs_libp2p_custom_publisher_free(info->request_publisher_->publisher_handle_);
  }
  if (info->response_subscription_->subscription_handle_) {
    rs_libp2p_custom_subscription_free(info->response_subscription_->subscription_handle_);
  }

  delete info;

  if (rmw_client) {
    if (rmw_client->service_name) {
      rmw_free(const_cast<char *>(rmw_client->service_name));
    }
    rmw_client_free(rmw_client);
  }

  return nullptr;
}

rmw_ret_t
rmw_client_request_publisher_get_actual_qos(
  const rmw_client_t * client,
  rmw_qos_profile_t * qos)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  RMW_CHECK_ARGUMENT_FOR_NULL(client, RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_ARGUMENT_FOR_NULL(qos, RMW_RET_INVALID_ARGUMENT);

  auto info = static_cast<rmw_libp2p_cpp::CustomClientInfo *>(client->data);
  *qos = info->request_publisher_->qos_;
  return RMW_RET_OK;
}

rmw_ret_t
rmw_client_response_subscription_get_actual_qos(
  const rmw_client_t * client,
  rmw_qos_profile_t * qos)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  RMW_CHECK_ARGUMENT_FOR_NULL(client, RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_ARGUMENT_FOR_NULL(qos, RMW_RET_INVALID_ARGUMENT);

  auto info = static_cast<rmw_libp2p_cpp::CustomClientInfo *>(client->data);
  *qos = info->response_subscription_->qos_;
  return RMW_RET_OK;
}
