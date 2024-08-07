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

#include "rmw/error_handling.h"

#include "ros_message_serialization.hpp"
#include "type_support_common.hpp"

#include "impl/cdr_buffer.hpp"

bool
_serialize_ros_message(
  const void * ros_message,
  rmw_libp2p_cpp::cdr::WriteCDRBuffer & ser,
  void * untyped_typesupport,
  const char * typesupport_identifier)
{
  if (using_introspection_c_typesupport(typesupport_identifier)) {
    auto typed_typesupport = static_cast<MessageTypeSupport_c *>(untyped_typesupport);
    return typed_typesupport->serializeROSmessage(ros_message, ser);
  } else if (using_introspection_cpp_typesupport(typesupport_identifier)) {
    auto typed_typesupport = static_cast<MessageTypeSupport_cpp *>(untyped_typesupport);
    return typed_typesupport->serializeROSmessage(ros_message, ser);
  }
  RMW_SET_ERROR_MSG("Unknown typesupport identifier");
  return false;
}

bool
_deserialize_ros_message(
  rmw_libp2p_cpp::cdr::ReadCDRBuffer & deser,
  void * ros_message,
  void * untyped_typesupport,
  const char * typesupport_identifier)
{
  if (using_introspection_c_typesupport(typesupport_identifier)) {
    auto typed_typesupport = static_cast<TypeSupport_c *>(untyped_typesupport);
    return typed_typesupport->deserializeROSmessage(deser, ros_message);
  } else if (using_introspection_cpp_typesupport(typesupport_identifier)) {
    auto typed_typesupport = static_cast<TypeSupport_cpp *>(untyped_typesupport);
    return typed_typesupport->deserializeROSmessage(deser, ros_message);
  }
  RMW_SET_ERROR_MSG("Unknown typesupport identifier");
  return false;
}
