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

#ifndef TYPE_SUPPORT_COMMON_HPP_
#define TYPE_SUPPORT_COMMON_HPP_

#include <sstream>
#include <string>

#include "rmw/error_handling.h"

#include "impl/message_type_support.hpp"
#include "impl/service_type_support.hpp"

#include "impl/rmw_libp2p_rs.hpp"

using MessageTypeSupport_c =
  rmw_libp2p_cpp::MessageTypeSupport<rosidl_typesupport_introspection_c__MessageMembers>;
using MessageTypeSupport_cpp =
  rmw_libp2p_cpp::MessageTypeSupport<rosidl_typesupport_introspection_cpp::MessageMembers>;
using TypeSupport_c =
  rmw_libp2p_cpp::TypeSupport<rosidl_typesupport_introspection_c__MessageMembers>;
using TypeSupport_cpp =
  rmw_libp2p_cpp::TypeSupport<rosidl_typesupport_introspection_cpp::MessageMembers>;

using RequestTypeSupport_c = rmw_libp2p_cpp::RequestTypeSupport<
  rosidl_typesupport_introspection_c__ServiceMembers,
  rosidl_typesupport_introspection_c__MessageMembers
>;
using RequestTypeSupport_cpp = rmw_libp2p_cpp::RequestTypeSupport<
  rosidl_typesupport_introspection_cpp::ServiceMembers,
  rosidl_typesupport_introspection_cpp::MessageMembers
>;

using ResponseTypeSupport_c = rmw_libp2p_cpp::ResponseTypeSupport<
  rosidl_typesupport_introspection_c__ServiceMembers,
  rosidl_typesupport_introspection_c__MessageMembers
>;
using ResponseTypeSupport_cpp = rmw_libp2p_cpp::ResponseTypeSupport<
  rosidl_typesupport_introspection_cpp::ServiceMembers,
  rosidl_typesupport_introspection_cpp::MessageMembers
>;

bool
using_introspection_c_typesupport(const char * typesupport_identifier);

bool
using_introspection_cpp_typesupport(const char * typesupport_identifier);

template<typename MembersType>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_LOCAL
inline std::string
_create_type_name(
  const void * untyped_members)
{
  auto members = static_cast<const MembersType *>(untyped_members);
  if (!members) {
    RMW_SET_ERROR_MSG("members handle is null");
    return "";
  }

  std::ostringstream ss;
  std::string message_namespace(members->message_namespace_);
  std::string message_name(members->message_name_);
  if (!message_namespace.empty()) {
    ss << message_namespace << "::";
  }
  ss << "libp2p_::" << message_name << "_";
  return ss.str();
}

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_LOCAL
inline std::string
_create_type_name(
  const void * untyped_members,
  const char * typesupport)
{
  if (using_introspection_c_typesupport(typesupport)) {
    return _create_type_name<rosidl_typesupport_introspection_c__MessageMembers>(
      untyped_members);
  } else if (using_introspection_cpp_typesupport(typesupport)) {
    return _create_type_name<rosidl_typesupport_introspection_cpp::MessageMembers>(
      untyped_members);
  }
  RMW_SET_ERROR_MSG("Unknown typesupport identifier");
  return "";
}

void *
_create_message_type_support(const void * untyped_members, const char * typesupport_identifier);

void *
_create_request_type_support(const void * untyped_members, const char * typesupport_identifier);

void *
_create_response_type_support(const void * untyped_members, const char * typesupport_identifier);

void
_register_type(
  rs_libp2p_custom_node_t * node,
  void * untyped_typesupport,
  const char * typesupport_identifier);

void
_unregister_type(
  rs_libp2p_custom_node_t * node,
  void * untyped_typesupport,
  const char * typesupport_identifier);

bool
_get_registered_type(
  rs_libp2p_custom_node_t * node,
  const std::string & type_name,
  void ** untyped_typesupport);

void
_delete_typesupport(void * untyped_typesupport, const char * typesupport_identifier);

#endif  // TYPE_SUPPORT_COMMON_HPP_
