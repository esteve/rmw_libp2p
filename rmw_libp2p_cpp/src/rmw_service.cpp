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

extern "C"
{
rmw_service_t *
libp2p_c__rmw_create_service(
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

  auto impl = static_cast<rmw_libp2p_cpp::CustomNodeInfo *>(node->data);
  if (!impl) {
    RMW_SET_ERROR_MSG("node impl is null");
    return nullptr;
  }

  rmw_service_t * rmw_service = nullptr;

  rmw_service = rmw_service_allocate();
  if (!rmw_service) {
    RMW_SET_ERROR_MSG("failed to allocate memory for service");
    // TODO(esteve): replace with goto fail
    return nullptr;
  }

  rmw_service->implementation_identifier = libp2p_identifier;
//   rmw_service->data = info;
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

  return rmw_service;
}

rmw_ret_t
libp2p_c__rmw_service_response_publisher_get_actual_qos(
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
libp2p_c__rmw_service_request_subscription_get_actual_qos(
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
}  // extern "C"