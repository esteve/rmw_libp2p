// Copyright 2022 Esteve Fernandez
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
#include "rmw/init.h"
#include "rmw/rmw.h"

#include "rmw/impl/cpp/macros.hpp"

#include "rcutils/logging_macros.h"

#include "rmw_libp2p_cpp/identifier.hpp"

const char *
rmw_get_implementation_identifier()
{
  return libp2p_identifier;
}

const char *
rmw_get_serialization_format()
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_get_serialization_format");

  return nullptr;
}

// Initialize the middleware with the given options, and yielding an context.
//
// rmw_context_t Doc: http://docs.ros2.org/latest/api/rmw/structrmw__context__t.html
rmw_ret_t
rmw_init(
  const rmw_init_options_t * options,
  rmw_context_t * context)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_init");
  (void)options;
  (void)context;

  return RMW_RET_ERROR;
}

// Create a node and return a handle to that node.
//
// rmw_node_t Doc: http://docs.ros2.org/latest/api/rmw/structrmw__node__t.html
rmw_node_t *
rmw_create_node(
  rmw_context_t * context,
  const char * name,
  const char * namespace_,
  size_t domain_id,
  bool localhost_only)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_create_node");
  (void)context;
  (void)name;
  (void)namespace_;
  (void)domain_id;
  (void)localhost_only;

  return nullptr;
}

// Shutdown the middleware for a given context.
rmw_ret_t
rmw_shutdown(
  rmw_context_t * context)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_shutdown");
  (void)context;

  return RMW_RET_ERROR;
}

// Initialize given init_options with the default values
// and implementation specific values.
//
// rmw_init_options_t Doc: http://docs.ros2.org/latest/api/rmw/structrmw__init__options__t.html
//
// Note: You should call rmw_get_zero_initialized_init_options()
// to get a zero initialized rmw_init_options_t struct first
rmw_ret_t
rmw_init_options_init(
  rmw_init_options_t * init_options,
  rcutils_allocator_t allocator)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_init_options_init");
  (void)init_options;
  (void)allocator;

  return RMW_RET_ERROR;
}

// Finalize a context. (Cleanup and deallocation.)
rmw_ret_t
rmw_context_fini(
  rmw_context_t * context)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_context_fini");
  (void)context;

  return RMW_RET_ERROR;
}

// Finalize a given node handle, reclaim the resources, and deallocate the node handle.
rmw_ret_t
rmw_destroy_node(
  rmw_node_t * node)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_destroy_node");
  (void)node;

  return RMW_RET_ERROR;
}

// Copy the given source init options to the destination init options.
rmw_ret_t
rmw_init_options_copy(
  const rmw_init_options_t * src,
  rmw_init_options_t * dst)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_init_options_copy");
  (void)src;
  (void)dst;

  return RMW_RET_ERROR;
}

// Finalize the given init_options. (Cleanup and deallocation.)
rmw_ret_t
rmw_init_options_fini(
  rmw_init_options_t * init_options)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_init_options_fini");
  (void)init_options;

  return RMW_RET_ERROR;
}

// Create and return an rmw subscriber
rmw_subscription_t *
rmw_create_subscription(
  const rmw_node_t * node,
  const rosidl_message_type_support_t * type_supports,
  const char * topic_name,
  const rmw_qos_profile_t * qos_profile,
  const rmw_subscription_options_t * subscription_options)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_create_subscription");
  (void)node;
  (void)type_supports;
  (void)topic_name;
  (void)qos_profile;
  (void)subscription_options;

  return nullptr;
}

// Destroy and deallocate an RMW subscription
rmw_ret_t
rmw_destroy_subscription(
  rmw_node_t * node,
  rmw_subscription_t * subscription)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_destroy_subscription");
  (void)node;
  (void)subscription;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_subscription_get_actual_qos(
  const rmw_subscription_t * subscription,
  rmw_qos_profile_t * qos_profile)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_subscription_get_actual_qos");
  (void)subscription;
  (void)qos_profile;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_subscription_count_matched_publishers(
  const rmw_subscription_t * subscription,
  size_t * count)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_subscription_count_matched_publishers");
  (void)subscription;
  (void)count;

  return RMW_RET_ERROR;
}

// Create and return an rmw publisher.
rmw_publisher_t *
rmw_create_publisher(
  const rmw_node_t * node,
  const rosidl_message_type_support_t * type_supports,
  const char * topic_name,
  const rmw_qos_profile_t * qos_profile,
  const rmw_publisher_options_t * publisher_options)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_create_publisher");
  (void)node;
  (void)type_supports;
  (void)topic_name;
  (void)qos_profile;
  (void)publisher_options;

  return nullptr;
}

// Destroy and deallocate an rmw publisher.
rmw_ret_t
rmw_destroy_publisher(
  rmw_node_t * node,
  rmw_publisher_t * publisher)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_destroy_publisher");
  (void)node;
  (void)publisher;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_publisher_get_actual_qos(
  const rmw_publisher_t * publisher,
  rmw_qos_profile_t * qos_profile)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_publisher_get_actual_qos");
  (void)publisher;
  (void)qos_profile;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_publisher_event_init(
  rmw_event_t * event,
  const rmw_publisher_t * publisher,
  rmw_event_type_t event_type)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_publisher_event_init");
  (void)event;
  (void)publisher;
  (void)event_type;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_publish(
  const rmw_publisher_t * publisher,
  const void * ros_message,
  rmw_publisher_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_publish");
  (void)publisher;
  (void)ros_message;
  (void)allocation;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_take(
  const rmw_subscription_t * subscription,
  void * ros_message,
  bool * taken,
  rmw_subscription_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_take");
  (void)subscription;
  (void)ros_message;
  (void)taken;
  (void)allocation;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_subscription_event_init(
  rmw_event_t * event,
  const rmw_subscription_t * subscription,
  rmw_event_type_t event_type)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_subscription_event_init");
  (void)event;
  (void)subscription;
  (void)event_type;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_take_with_info(
  const rmw_subscription_t * subscription,
  void * ros_message,
  bool * taken,
  rmw_message_info_t * message_info,
  rmw_subscription_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_take_with_info");
  (void)subscription;
  (void)ros_message;
  (void)taken;
  (void)message_info;
  (void)allocation;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_take_loaned_message_with_info(
  const rmw_subscription_t * subscription,
  void ** loaned_message,
  bool * taken,
  rmw_message_info_t * message_info,
  rmw_subscription_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_take_loaned_message_with_info");
  (void)subscription;
  (void)loaned_message;
  (void)taken;
  (void)message_info;
  (void)allocation;

  return RMW_RET_ERROR;
}

rmw_client_t *
rmw_create_client(
  const rmw_node_t * node,
  const rosidl_service_type_support_t * type_supports,
  const char * service_name,
  const rmw_qos_profile_t * qos_profile)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_create_client");
  (void)node;
  (void)type_supports;
  (void)service_name;
  (void)qos_profile;

  return nullptr;
}

rmw_service_t *
rmw_create_service(
  const rmw_node_t * node,
  const rosidl_service_type_support_t * type_supports,
  const char * service_name,
  const rmw_qos_profile_t * qos_profile)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_create_service");
  (void)node;
  (void)type_supports;
  (void)service_name;
  (void)qos_profile;

  return nullptr;
}

rmw_ret_t
rmw_service_server_is_available(
  const rmw_node_t * node,
  const rmw_client_t * client,
  bool * result)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_service_server_is_available");
  (void)node;
  (void)client;
  (void)result;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_get_gid_for_publisher(
  const rmw_publisher_t * publisher,
  rmw_gid_t * gid)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_get_gid_for_publisher");
  (void)publisher;
  (void)gid;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_destroy_wait_set(
  rmw_wait_set_t * wait_set)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_destroy_wait_set");
  (void)wait_set;

  return RMW_RET_ERROR;
}

rmw_wait_set_t *
rmw_create_wait_set(
  rmw_context_t * context,
  size_t max_conditions)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_create_wait_set");
  (void)context;
  (void)max_conditions;

  return nullptr;
}

rmw_ret_t
rmw_trigger_guard_condition(
  const rmw_guard_condition_t * guard_condition_handle)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_trigger_guard_condition");
  (void)guard_condition_handle;

  return RMW_RET_ERROR;
}

rmw_guard_condition_t *
rmw_create_guard_condition(
  rmw_context_t * context)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_create_guard_condition");
  (void)context;

  return nullptr;
}

rmw_ret_t
rmw_send_response(
  const rmw_service_t * service,
  rmw_request_id_t * request_header,
  void * ros_response)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_send_response");
  (void)service;
  (void)request_header;
  (void)ros_response;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_take_request(
  const rmw_service_t * service,
  rmw_service_info_t * request_header,
  void * ros_request,
  bool * taken)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_take_request");
  (void)service;
  (void)request_header;
  (void)ros_request;
  (void)taken;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_destroy_service(
  rmw_node_t * node,
  rmw_service_t * service)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_destroy_service");
  (void)node;
  (void)service;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_take_response(
  const rmw_client_t * client,
  rmw_service_info_t * request_header,
  void * ros_response,
  bool * taken)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_take_response");
  (void)client;
  (void)request_header;
  (void)ros_response;
  (void)taken;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_send_request(
  const rmw_client_t * client,
  const void * ros_request,
  int64_t * sequence_id)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_send_request");
  (void)client;
  (void)ros_request;
  (void)sequence_id;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_destroy_client(
  rmw_node_t * node,
  rmw_client_t * client)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_destroy_client");
  (void)node;
  (void)client;

  return RMW_RET_ERROR;
}

rmw_ret_t
rmw_return_loaned_message_from_subscription(
  const rmw_subscription_t * subscription,
  void * loaned_message)
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "rmw_return_loaned_message_from_subscription");
  (void)subscription;
  (void)loaned_message;

  return RMW_RET_ERROR;
}
