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

#ifndef ROS_MESSAGE_SERIALIZATION_HPP_
#define ROS_MESSAGE_SERIALIZATION_HPP_

#include "impl/cdr_buffer.hpp"
#include "impl/rmw_libp2p_rs.hpp"

bool
_serialize_ros_message(
  const void * ros_message,
  rmw_libp2p_cpp::cdr::WriteCDRBuffer & ser,
  void * untyped_members,
  const char * typesupport_identifier);

bool
_deserialize_ros_message(
  rmw_libp2p_cpp::cdr::ReadCDRBuffer & deser,
  void * ros_message,
  void * untyped_members,
  const char * typesupport_identifier);

#endif  // ROS_MESSAGE_SERIALIZATION_HPP_
