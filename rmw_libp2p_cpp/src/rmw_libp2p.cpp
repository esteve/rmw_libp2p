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

// Doc: http://docs.ros2.org/latest/api/rmw/init_8h.html

#include <cstring>

#include <memory>

#include "rmw/error_handling.h"
#include "rmw/event.h"
#include "rmw/features.h"
#include "rmw/get_network_flow_endpoints.h"
#include "rmw/get_node_info_and_types.h"
#include "rmw/get_service_names_and_types.h"
#include "rmw/get_topic_endpoint_info.h"
#include "rmw/get_topic_names_and_types.h"
#include "rmw/init.h"
#include "rmw/qos_profiles.h"
#include "rmw/rmw.h"

#include "rmw/impl/cpp/macros.hpp"

#include "rcutils/allocator.h"
#include "rcutils/logging_macros.h"
#include "rcutils/macros.h"
#include "rcutils/types.h"

#include "impl/identifier.hpp"

#include "impl/rmw_libp2p_rs.hpp"
#include "impl/custom_subscription_info.hpp"

extern "C"
{
RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_subscription_count_matched_publishers(
  const rmw_subscription_t * subscription,
  size_t * publisher_count)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscription;
  (void)publisher_count;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_publisher_event_init(
  rmw_event_t * event,
  const rmw_publisher_t * publisher,
  rmw_event_type_t event_type)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)event;
  (void)publisher;
  (void)event_type;

  return RMW_RET_UNSUPPORTED;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_subscription_event_init(
  rmw_event_t * event,
  const rmw_subscription_t * subscription,
  rmw_event_type_t event_type)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  return RMW_RET_UNSUPPORTED;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_take_loaned_message_with_info(
  const rmw_subscription_t * subscription,
  void ** loaned_message,
  bool * taken,
  rmw_message_info_t * message_info,
  rmw_subscription_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscription;
  (void)loaned_message;
  (void)taken;
  (void)message_info;
  (void)allocation;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_client_t *
libp2p_c__rmw_create_client(
  const rmw_node_t * node,
  const rosidl_service_type_support_t * type_support,
  const char * service_name,
  const rmw_qos_profile_t * qos_policies)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)type_support;
  (void)service_name;
  (void)qos_policies;

  return nullptr;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_service_server_is_available(
  const rmw_node_t * node,
  const rmw_client_t * client,
  bool * is_available)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)client;
  (void)is_available;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_send_response(
  const rmw_service_t * service,
  rmw_request_id_t * request_header,
  void * ros_response)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)service;
  (void)request_header;
  (void)ros_response;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_take_request(
  const rmw_service_t * service,
  rmw_service_info_t * request_header,
  void * ros_request,
  bool * taken)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)service;
  (void)request_header;
  (void)ros_request;
  (void)taken;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_destroy_service(
  rmw_node_t * node,
  rmw_service_t * service)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)service;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_take_response(
  const rmw_client_t * client,
  rmw_service_info_t * request_header,
  void * ros_response,
  bool * taken)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)client;
  (void)request_header;
  (void)ros_response;
  (void)taken;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_send_request(
  const rmw_client_t * client,
  const void * ros_request,
  int64_t * sequence_id)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)client;
  (void)ros_request;
  (void)sequence_id;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_destroy_client(
  rmw_node_t * node,
  rmw_client_t * client)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)client;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_return_loaned_message_from_subscription(
  const rmw_subscription_t * subscription,
  void * loaned_message)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscription;
  (void)loaned_message;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_borrow_loaned_message(
  const rmw_publisher_t * publisher,
  const rosidl_message_type_support_t * type_support,
  void ** ros_message)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)publisher;
  (void)type_support;
  (void)ros_message;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_client_request_publisher_get_actual_qos(
  const rmw_client_t * client,
  rmw_qos_profile_t * qos)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)client;
  (void)qos;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_client_response_subscription_get_actual_qos(
  const rmw_client_t * client,
  rmw_qos_profile_t * qos)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)client;
  (void)qos;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_client_set_on_new_response_callback(
  rmw_client_t * client,
  rmw_event_callback_t callback,
  const void * user_data)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)client;
  (void)callback;
  (void)user_data;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_compare_gids_equal(const rmw_gid_t * gid1, const rmw_gid_t * gid2, bool * result)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)gid1;
  (void)gid2;
  (void)result;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_init_publisher_allocation(
  const rosidl_message_type_support_t * type_support,
  const rosidl_runtime_c__Sequence__bound * message_bounds,
  rmw_publisher_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)type_support;
  (void)message_bounds;
  (void)allocation;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_fini_publisher_allocation(
  rmw_publisher_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)allocation;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_return_loaned_message_from_publisher(
  const rmw_publisher_t * publisher,
  void * loaned_message)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)publisher;
  (void)loaned_message;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_publisher_count_matched_subscriptions(
  const rmw_publisher_t * publisher,
  size_t * subscription_count)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)publisher;
  (void)subscription_count;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_publish_serialized_message(
  const rmw_publisher_t * publisher,
  const rmw_serialized_message_t * serialized_message,
  rmw_publisher_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)publisher;
  (void)serialized_message;
  (void)allocation;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_get_serialized_message_size(
  const rosidl_message_type_support_t * type_support,
  const rosidl_runtime_c__Sequence__bound * message_bounds,
  size_t * size)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)type_support;
  (void)message_bounds;
  (void)size;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_publisher_assert_liveliness(const rmw_publisher_t * publisher)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)publisher;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_publisher_wait_for_all_acked(
  const rmw_publisher_t * publisher,
  rmw_time_t wait_timeout)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)publisher;
  (void)wait_timeout;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_serialize(
  const void * ros_message,
  const rosidl_message_type_support_t * type_support,
  rmw_serialized_message_t * serialized_message)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)ros_message;
  (void)type_support;
  (void)serialized_message;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_deserialize(
  const rmw_serialized_message_t * serialized_message,
  const rosidl_message_type_support_t * type_support,
  void * ros_message)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)serialized_message;
  (void)type_support;
  (void)ros_message;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_init_subscription_allocation(
  const rosidl_message_type_support_t * type_support,
  const rosidl_runtime_c__Sequence__bound * message_bounds,
  rmw_subscription_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)type_support;
  (void)message_bounds;
  (void)allocation;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_fini_subscription_allocation(
  rmw_subscription_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)allocation;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_count_publishers(
  const rmw_node_t * node,
  const char * topic_name,
  size_t * count)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)topic_name;
  (void)count;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_count_subscribers(
  const rmw_node_t * node,
  const char * topic_name,
  size_t * count)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)topic_name;
  (void)count;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_get_node_names_with_enclaves(
  const rmw_node_t * node,
  rcutils_string_array_t * node_names,
  rcutils_string_array_t * node_namespaces,
  rcutils_string_array_t * enclaves)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)node_names;
  (void)node_namespaces;
  (void)enclaves;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_get_node_names(
  const rmw_node_t * node,
  rcutils_string_array_t * node_names,
  rcutils_string_array_t * node_namespaces)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)node_names;
  (void)node_namespaces;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_set_log_severity(rmw_log_severity_t severity)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)severity;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_subscription_set_on_new_message_callback(
  rmw_subscription_t * subscription,
  rmw_event_callback_t callback,
  const void * user_data)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscription;
  (void)callback;
  (void)user_data;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_service_set_on_new_request_callback(
  rmw_service_t * service,
  rmw_event_callback_t callback,
  const void * user_data)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)service;
  (void)callback;
  (void)user_data;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_event_set_callback(
  rmw_event_t * event,
  rmw_event_callback_t callback,
  const void * user_data)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)event;
  (void)callback;
  (void)user_data;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_take_loaned_message(
  const rmw_subscription_t * subscription,
  void ** loaned_message,
  bool * taken,
  rmw_subscription_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscription;
  (void)loaned_message;
  (void)taken;
  (void)allocation;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_take_serialized_message_with_info(
  const rmw_subscription_t * subscription,
  rmw_serialized_message_t * serialized_message,
  bool * taken,
  rmw_message_info_t * message_info,
  rmw_subscription_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscription;
  (void)serialized_message;
  (void)taken;
  (void)message_info;
  (void)allocation;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_take_serialized_message(
  const rmw_subscription_t * subscription,
  rmw_serialized_message_t * serialized_message,
  bool * taken,
  rmw_subscription_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscription;
  (void)serialized_message;
  (void)taken;
  (void)allocation;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_take_sequence(
  const rmw_subscription_t * subscription,
  size_t count,
  rmw_message_sequence_t * message_sequence,
  rmw_message_info_sequence_t * message_info_sequence,
  size_t * taken,
  rmw_subscription_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscription;
  (void)count;
  (void)message_sequence;
  (void)message_info_sequence;
  (void)taken;
  (void)allocation;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_subscription_get_content_filter(
  const rmw_subscription_t * subscription,
  rcutils_allocator_t * allocator,
  rmw_subscription_content_filter_options_t * options)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscription;
  (void)allocator;
  (void)options;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_subscription_set_content_filter(
  rmw_subscription_t * subscription,
  const rmw_subscription_content_filter_options_t * options)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscription;
  (void)options;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
bool
libp2p_c__rmw_feature_supported(rmw_feature_t feature)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)feature;

  return false;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_take_event(
  const rmw_event_t * event_handle,
  void * event_info,
  bool * taken)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)event_handle;
  (void)event_info;
  (void)taken;

  return RMW_RET_OK;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_get_client_names_and_types_by_node(
  const rmw_node_t * node,
  rcutils_allocator_t * allocator,
  const char * node_name,
  const char * node_namespace,
  rmw_names_and_types_t * service_names_and_types)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)allocator;
  (void)node_name;
  (void)node_namespace;
  (void)service_names_and_types;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_get_service_names_and_types_by_node(
  const rmw_node_t * node,
  rcutils_allocator_t * allocator,
  const char * node_name,
  const char * node_namespace,
  rmw_names_and_types_t * service_names_and_types)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)allocator;
  (void)node_name;
  (void)node_namespace;
  (void)service_names_and_types;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_get_publisher_names_and_types_by_node(
  const rmw_node_t * node,
  rcutils_allocator_t * allocator,
  const char * node_name,
  const char * node_namespace,
  bool no_demangle,
  rmw_names_and_types_t * topic_names_and_types)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)allocator;
  (void)node_name;
  (void)node_namespace;
  (void)no_demangle;
  (void)topic_names_and_types;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_get_subscriber_names_and_types_by_node(
  const rmw_node_t * node,
  rcutils_allocator_t * allocator,
  const char * node_name,
  const char * node_namespace,
  bool no_demangle,
  rmw_names_and_types_t * topic_names_and_types)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)allocator;
  (void)node_name;
  (void)node_namespace;
  (void)no_demangle;
  (void)topic_names_and_types;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_get_publishers_info_by_topic(
  const rmw_node_t * node,
  rcutils_allocator_t * allocator,
  const char * topic_name,
  bool no_mangle,
  rmw_topic_endpoint_info_array_t * publishers_info)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)allocator;
  (void)topic_name;
  (void)no_mangle;
  (void)publishers_info;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_get_service_names_and_types(
  const rmw_node_t * node,
  rcutils_allocator_t * allocator,
  rmw_names_and_types_t * service_names_and_types)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)allocator;
  (void)service_names_and_types;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_get_subscriptions_info_by_topic(
  const rmw_node_t * node,
  rcutils_allocator_t * allocator,
  const char * topic_name,
  bool no_mangle,
  rmw_topic_endpoint_info_array_t * subscriptions_info)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)allocator;
  (void)topic_name;
  (void)no_mangle;
  (void)subscriptions_info;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_get_topic_names_and_types(
  const rmw_node_t * node,
  rcutils_allocator_t * allocator,
  bool no_demangle,
  rmw_names_and_types_t * topic_names_and_types)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;
  (void)allocator;
  (void)no_demangle;
  (void)topic_names_and_types;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_publisher_get_network_flow_endpoints(
  const rmw_publisher_t * publisher,
  rcutils_allocator_t * allocator,
  rmw_network_flow_endpoint_array_t * network_flow_endpoint_array)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)publisher;
  (void)allocator;
  (void)network_flow_endpoint_array;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_qos_profile_check_compatible(
  const rmw_qos_profile_t publisher_profile,
  const rmw_qos_profile_t subscription_profile,
  rmw_qos_compatibility_type_t * compatibility,
  char * reason,
  size_t reason_size)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)publisher_profile;
  (void)subscription_profile;
  (void)compatibility;
  (void)reason;
  (void)reason_size;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_subscription_get_network_flow_endpoints(
  const rmw_subscription_t * subscription,
  rcutils_allocator_t * allocator,
  rmw_network_flow_endpoint_array_t * network_flow_endpoint_array)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscription;
  (void)allocator;
  (void)network_flow_endpoint_array;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_node_assert_liveliness(const rmw_node_t * node)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)node;

  return RMW_RET_ERROR;
}

RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_take(
        const rmw_subscription_t * subscription,
        void * ros_message,
        bool * taken,
        rmw_subscription_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  (void)subscription;
  (void)ros_message;
  (void)taken;
  (void)allocation;

  return RMW_RET_ERROR;
}

}  // extern "C"
