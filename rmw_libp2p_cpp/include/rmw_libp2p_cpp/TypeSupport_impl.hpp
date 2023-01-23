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

#ifndef RMW_LIBP2P_CPP__TYPESUPPORT_IMPL_HPP_
#define RMW_LIBP2P_CPP__TYPESUPPORT_IMPL_HPP_

#include <stdexcept>
#include <string>
#include <vector>

#include "rmw_libp2p_cpp/macros.hpp"

#include "rosidl_runtime_c/primitives_sequence_functions.h"

namespace rmw_libp2p_cpp
{

template<typename T>
struct GenericCSequence;

// multiple definitions of ambiguous primitive types
SPECIALIZE_GENERIC_C_SEQUENCE(bool, bool)
SPECIALIZE_GENERIC_C_SEQUENCE(byte, uint8_t)
SPECIALIZE_GENERIC_C_SEQUENCE(char, char)
SPECIALIZE_GENERIC_C_SEQUENCE(float32, float)
SPECIALIZE_GENERIC_C_SEQUENCE(float64, double)
SPECIALIZE_GENERIC_C_SEQUENCE(int8, int8_t)
SPECIALIZE_GENERIC_C_SEQUENCE(int16, int16_t)
SPECIALIZE_GENERIC_C_SEQUENCE(uint16, uint16_t)
SPECIALIZE_GENERIC_C_SEQUENCE(int32, int32_t)
SPECIALIZE_GENERIC_C_SEQUENCE(uint32, uint32_t)
SPECIALIZE_GENERIC_C_SEQUENCE(int64, int64_t)
SPECIALIZE_GENERIC_C_SEQUENCE(uint64, uint64_t)

typedef struct rosidl_runtime_c__void__Sequence
{
  void * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} rosidl_runtime_c__void__Sequence;

inline
bool
rosidl_runtime_c__void__Sequence__init(
  rosidl_runtime_c__void__Sequence * sequence, size_t size, size_t member_size)
{
  if (!sequence) {
    return false;
  }
  void * data = nullptr;
  if (size) {
    data = static_cast<void *>(calloc(size, member_size));
    if (!data) {
      return false;
    }
  }
  sequence->data = data;
  sequence->size = size;
  sequence->capacity = size;
  return true;
}

inline
void
rosidl_runtime_c__void__Sequence__fini(rosidl_runtime_c__void__Sequence * sequence)
{
  if (!sequence) {
    return;
  }
  if (sequence->data) {
    // ensure that data and capacity values are consistent
    assert(sequence->capacity > 0);
    // finalize all sequence elements
    free(sequence->data);
    sequence->data = nullptr;
    sequence->size = 0;
    sequence->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == sequence->size);
    assert(0 == sequence->capacity);
  }
}

template<typename MembersType>
TypeSupport<MembersType>::TypeSupport(const MembersType * members)
{
  assert(members);
  this->members_ = members;
}

template<typename MembersType>
bool TypeSupport<MembersType>::deserializeROSmessage(
  rs_libp2p_cdr_buffer * deser, const MembersType * members, void * ros_message, bool call_new)
{
  assert(members);
  assert(ros_message);

  size_t member_count = 0;
  // rs_deser.deserializeSequence(&member_count);
  if (member_count != members->member_count_) {
    throw std::runtime_error("failed to deserialize value");
  }

  for (uint32_t i = 0; i < members->member_count_; ++i) {
    const auto * member = members->members_ + i;
    void * field = static_cast<char *>(ros_message) + member->offset_;
    switch (member->type_id_) {
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_BOOLEAN:
    //     deserialize_field<bool>(member, field, deser, call_new);
    //     break;
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_OCTET:
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_UINT8:
    //     deserialize_field<uint8_t>(member, field, deser, call_new);
    //     break;
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_CHAR:
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_INT8:
    //     deserialize_field<char>(member, field, deser, call_new);
    //     break;
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_FLOAT:
    //     deserialize_field<float>(member, field, deser, call_new);
    //     break;
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_DOUBLE:
    //     deserialize_field<double>(member, field, deser, call_new);
    //     break;
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_INT16:
    //     deserialize_field<int16_t>(member, field, deser, call_new);
    //     break;
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_UINT16:
    //     deserialize_field<uint16_t>(member, field, deser, call_new);
    //     break;
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_INT32:
    //     deserialize_field<int32_t>(member, field, deser, call_new);
    //     break;
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_UINT32:
    //     deserialize_field<uint32_t>(member, field, deser, call_new);
    //     break;
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_INT64:
    //     deserialize_field<int64_t>(member, field, deser, call_new);
    //     break;
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_UINT64:
    //     deserialize_field<uint64_t>(member, field, deser, call_new);
    //     break;
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_STRING:
    //     deserialize_field<std::string>(member, field, deser, call_new);
    //     break;
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_WSTRING:
    //     deserialize_field<std::u16string>(member, field, deser, call_new);
    //     break;
    //   case ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE:
    //     {
    //       auto sub_members = (const MembersType *)member->members_->data;
    //       if (!member->is_array_) {
    //         deserializeROSmessage(deser, sub_members, field, call_new);
    //       } else {
    //         void * subros_message = nullptr;
    //         size_t array_size = 0;
    //         bool recall_new = call_new;

    //         array_size = get_submessage_sequence_deserialize(
    //           member, deser, field, subros_message, recall_new);

    //         for (size_t index = 0; index < array_size; ++index) {
    //           deserializeROSmessage(
    //             deser, sub_members, member->get_function(subros_message, index), recall_new);
    //         }
    //       }
    //     }
    //     break;
      default:
        throw std::runtime_error("unknown type");
    }
  }

  return true;
}

template<typename MembersType>
bool TypeSupport<MembersType>::serializeROSmessage(
  const void * ros_message, rs_libp2p_cdr_buffer * ser)
{
  assert(ros_message);

  if (members_->member_count_ != 0) {
    TypeSupport::serializeROSmessage(ser, members_, ros_message);
  }
  return true;
}

template<typename MembersType>
bool TypeSupport<MembersType>::deserializeROSmessage(
  rs_libp2p_cdr_buffer * deser, void * ros_message)
{
  assert(ros_message);

  if (members_->member_count_ != 0) {
    TypeSupport::deserializeROSmessage(deser, members_, ros_message, false);
  }

  return true;
}

}  // namespace rmw_libp2p_cpp

#endif  // RMW_LIBP2P_CPP__TYPESUPPORT_IMPL_HPP_
