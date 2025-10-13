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

#include <iostream>

#include "rmw/allocators.h"
#include "rmw/error_handling.h"
#include "rmw/rmw.h"

#include "rcutils/logging_macros.h"

#include "rosidl_typesupport_introspection_cpp/identifier.hpp"

#include "rosidl_typesupport_introspection_c/identifier.h"

#include "impl/identifier.hpp"
#include "impl/custom_node_info.hpp"
#include "impl/custom_service_info.hpp"
#include "impl/custom_subscription_info.hpp"
#include "impl/listener.hpp"

#include "client_service_common.hpp"
#include "type_support_common.hpp"

rmw_service_t *
rmw_create_service(
  const rmw_node_t * node,
  const rosidl_service_type_support_t * type_supports,
  const char * service_name,
  const rmw_qos_profile_t * qos_policies)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s(node=%p,type_supports=%p,service_name=%s,"
    "qos_policies={history=%d,depth=%zu,reliability=%d,durability=%d})",
    __FUNCTION__, (void *)node, (void *)type_supports, service_name, qos_policies->history,
    qos_policies->depth, qos_policies->reliability, qos_policies->durability);

  // TODO(esteve): just boilerplate, services are not supported yet
  if (!node) {
    RMW_SET_ERROR_MSG("node handle is null");
    return nullptr;
  }

  if (node->implementation_identifier != libp2p_identifier) {
    RMW_SET_ERROR_MSG("node handle not from this implementation");
    return nullptr;
  }

  if (!service_name || strlen(service_name) == 0) {
    RMW_SET_ERROR_MSG("service topic is null or empty string");
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

  rmw_libp2p_cpp::CustomServiceInfo * info = nullptr;
  rmw_service_t * rmw_service = nullptr;

  info = new rmw_libp2p_cpp::CustomServiceInfo();
  info->node_ = node;
  info->typesupport_identifier_ = type_support->typesupport_identifier;
  info->request_info_ = new rmw_libp2p_cpp::CustomSubscriptionInfo;
  info->request_info_->node_ = node;
  info->request_info_->typesupport_identifier_ = type_support->typesupport_identifier;
  info->request_info_->qos_ = *qos_policies;

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

  if (!_get_registered_type(node_data->node_handle_, request_type_name, &info->request_info_->type_support_)) {
    info->request_info_->type_support_ = _create_request_type_support(type_support->data,
        info->typesupport_identifier_);
    _register_type(node_data->node_handle_, info->request_info_->type_support_, info->typesupport_identifier_);
  }

  if (!_get_registered_type(node_data->node_handle_, response_type_name, &info->response_type_support_)) {
    info->response_type_support_ = _create_response_type_support(type_support->data,
        info->typesupport_identifier_);
    _register_type(node_data->node_handle_, info->response_type_support_, info->typesupport_identifier_);
  }

  // TODO(esteve): delete Listener in the destructor
  info->listener_ = new rmw_libp2p_cpp::Listener;
  info->request_info_->listener_ = info->listener_;

  info->request_info_->subscription_handle_ = rs_libp2p_custom_subscription_new(
    node_data->node_handle_, service_name,
    info->request_info_, rmw_libp2p_cpp::Listener::on_publication);
  if (!info->request_info_->subscription_handle_) {
    RMW_SET_ERROR_MSG("failed to create libp2p subscription for service");
    goto fail;
  }

  rmw_service = rmw_service_allocate();
  if (!rmw_service) {
    RMW_SET_ERROR_MSG("failed to allocate memory for service");
    // TODO(esteve): replace with goto fail
    return nullptr;
  }

  rmw_service->implementation_identifier = libp2p_identifier;
  rmw_service->data = info;
  rmw_service->service_name = reinterpret_cast<const char *>(
    rmw_allocate(strlen(service_name) + 1));
  if (!rmw_service->service_name) {
    RMW_SET_ERROR_MSG("failed to allocate memory for service name");
    // TODO(esteve): replace with goto fail
    return nullptr;
  }
  memcpy(
    const_cast<char *>(rmw_service->service_name), service_name,
    strlen(service_name) + 1);

  {
    std::lock_guard<std::mutex> lock(node_data->services_mutex_);
    node_data->services_[service_name].insert(info);
  }

  return rmw_service;

fail:
  _delete_typesupport(info->request_info_->type_support_, info->typesupport_identifier_);
  if (info->request_info_->subscription_handle_) {
    rs_libp2p_custom_subscription_free(info->request_info_->subscription_handle_);
  }
  // if (info->request_type_support_handle_) {
  //   _unregister_type(impl->node_, info->request_type_support_handle_, info->typesupport_identifier_);
  // }

  _delete_typesupport(info->response_type_support_, info->typesupport_identifier_);
  // if (info->response_type_support_) {
  //   _unregister_type(impl->node_, info->response_type_support_handle_, info->typesupport_identifier_);
  // }
  delete info;

  if (rmw_service) {
    if (rmw_service->service_name) {
      rmw_free(const_cast<char *>(rmw_service->service_name));
    }
    rmw_service_free(rmw_service);
  }

  return nullptr;
}

rmw_ret_t
rmw_service_response_publisher_get_actual_qos(
  const rmw_service_t * service,
  rmw_qos_profile_t * qos_policies)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)service;
  (void)qos_policies;

  return RMW_RET_OK;
}

rmw_ret_t
rmw_service_request_subscription_get_actual_qos(
  const rmw_service_t * service,
  rmw_qos_profile_t * qos_policies)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)service;
  (void)qos_policies;

  return RMW_RET_OK;
}
