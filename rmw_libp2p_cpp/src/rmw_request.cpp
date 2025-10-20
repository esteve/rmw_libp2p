// Copyright 2025 Esteve Fernandez All rights reserved.
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

#include "rmw/error_handling.h"
#include "rmw/rmw.h"

#include "rcutils/logging_macros.h"

#include "impl/cdr_buffer.hpp"
#include "impl/identifier.hpp"
#include "impl/custom_client_info.hpp"
#include "impl/custom_node_info.hpp"
#include "impl/custom_service_info.hpp"
#include "ros_message_serialization.hpp"

extern "C"
{
RMW_PUBLIC
rmw_ret_t
rmw_take_request(
  const rmw_service_t * service,
  rmw_service_info_t  * request_header,
  void * ros_request,
  bool * taken)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s(service=%p,request_header=%p,ros_request=%p,taken=%p)", __FUNCTION__, (void *)service,
    (void *)request_header, ros_request, (void *)taken);

  assert(service);
  assert(request_header);
  assert(ros_request);
  assert(taken);

  *taken = false;

  if (service->implementation_identifier != libp2p_identifier) {
    RMW_SET_ERROR_MSG("publisher handle not from this implementation");
    return RMW_RET_ERROR;
  }

  rmw_libp2p_cpp::CustomServiceInfo * info = static_cast<rmw_libp2p_cpp::CustomServiceInfo *>(service->data);
  RCUTILS_CHECK_FOR_NULL_WITH_MSG(info, "custom service info is null",
    return RMW_RET_ERROR);
  rmw_libp2p_cpp::CustomNodeInfo * node_data = static_cast<rmw_libp2p_cpp::CustomNodeInfo *>(info->node_->data);
  RCUTILS_CHECK_FOR_NULL_WITH_MSG(node_data, "custom node info is null",
    return RMW_RET_ERROR);

  uint8_t * message = nullptr;
  uintptr_t length = 0;

  if (info->listener_->take_next_data(&message, length)) {
    rmw_libp2p_cpp::cdr::ReadCDRBuffer buffer(message, length);

    std::cout << "rmw_take_request: received message of length " << length << std::endl;
    std::cout << "rmw_take_request: deserializing request" << std::endl;
    uint64_t secs = 0;
    uint32_t usecs = 0;
    buffer >> secs;
    buffer >> usecs;
    request_header->source_timestamp = secs * 1000000000ull + usecs * 1000ull;
    std::cout << "rmw_take_request: timestamp " << request_header->source_timestamp << std::endl;

    // Get header
    memset(request_header->request_id.writer_guid, 0, RMW_GID_STORAGE_SIZE);
    for(int i = 0; i < 16; ++i) {
      std::cout << "PRE rmw_take_request: read guid byte " << i << " value " << static_cast<int16_t>(request_header->request_id.writer_guid[i]) << std::endl;
      int8_t value = 0;
      buffer >> value;
      std::cout << "POST 1 rmw_take_request: read guid byte " << i << " value " << static_cast<int16_t>(value) << std::endl;
      request_header->request_id.writer_guid[i] = value;
      std::cout << "POST 2 rmw_take_request: read guid byte " << i << " value " << static_cast<int16_t>(request_header->request_id.writer_guid[i]) << std::endl;
    }
    buffer >> request_header->request_id.sequence_number;

    std::cout << "RECEIVED SEQUENCE NUMBER TAKE REQUEST: " << request_header->request_id.sequence_number << std::endl;

    // char uuid_str[37] = {};
    // unsigned long uuid_data1 = *reinterpret_cast<unsigned long*>(request_header->request_id.writer_guid);
    // unsigned short uuid_data2 = *reinterpret_cast<unsigned short*>(request_header->request_id.writer_guid + 4);
    // unsigned short uuid_data3 = *reinterpret_cast<unsigned short*>(request_header->request_id.writer_guid + 6);
    // sprintf(uuid_str, 
    // "%08x-%04x-%04x-%02x%02x-%02x%02x%02x%02x%02x%02x",
    //     uuid_data1, uuid_data2, uuid_data3,
    //     request_header->request_id.writer_guid[8], request_header->request_id.writer_guid[9], request_header->request_id.writer_guid[10], request_header->request_id.writer_guid[11], request_header->request_id.writer_guid[12], request_header->request_id.writer_guid[13], request_header->request_id.writer_guid[14], request_header->request_id.writer_guid[15]
    // );

    char uuid_str[37] = {};
    sprintf(uuid_str,
      "%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x",
      static_cast<uint8_t>(request_header->request_id.writer_guid[0]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[1]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[2]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[3]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[4]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[5]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[6]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[7]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[8]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[9]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[10]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[11]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[12]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[13]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[14]),
      static_cast<uint8_t>(request_header->request_id.writer_guid[15])
    );

    std::cout << "rmw_take_request: received request from guid: " << uuid_str
              << " with sequence number: " << request_header->request_id.sequence_number << std::endl;


    std::string topic_name(info->service_name_ + std::string("/response/") + uuid_str);

    rs_libp2p_custom_publisher_t * pub = rs_libp2p_custom_publisher_new(node_data->node_handle_, topic_name.c_str());

    std::cout << "rmw_take_request: received request from guid: " << uuid_str
              << " with sequence number: " << request_header->request_id.sequence_number << std::endl;
    // const size_t ret = rs_libp2p_custom_publisher_get_gid(pub, request_header->request_id.writer_guid);
    // request_header->request_id.sequence_number = rs_libp2p_custom_publisher_get_sequence_number(pub);

    _deserialize_ros_message(
      buffer, ros_request, info->request_subscription_->type_support_,
      info->typesupport_identifier_);

    *taken = true;

    info->requests_.emplace(std::make_pair(request_header->request_id, std::move(pub)));
  }

  return RMW_RET_OK;
}

RMW_PUBLIC
rmw_ret_t
rmw_send_request(
  const rmw_client_t * client,
  const void * ros_request,
  int64_t * sequence_id)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s(client=%p,ros_request=%p,sequence_id=%p)", __FUNCTION__, (void *)client, ros_request,
    (void *)sequence_id);

  RMW_CHECK_ARGUMENT_FOR_NULL(client, RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_ARGUMENT_FOR_NULL(ros_request, RMW_RET_INVALID_ARGUMENT);

  rmw_ret_t returned_value = RMW_RET_ERROR;

  if (client->implementation_identifier != libp2p_identifier) {
    RMW_SET_ERROR_MSG("node handle not from this implementation");
    return RMW_RET_ERROR;
  }

  auto info = static_cast<rmw_libp2p_cpp::CustomClientInfo *>(client->data);
  assert(info);

  rmw_libp2p_cpp::cdr::WriteCDRBuffer ser;

  // Get header
  rmw_gid_t request_guid;
  memset(request_guid.data, 0, RMW_GID_STORAGE_SIZE);
  const size_t ret = rs_libp2p_custom_publisher_get_gid(
    info->request_publisher_->publisher_handle_, request_guid.data);
  if (ret == 0) {
    RMW_SET_ERROR_MSG("no guid found for publisher");
    return RMW_RET_ERROR;
  }

  for(int i = 0; i < 16; ++i) {
    std::cout << "rmw_send_request: writing guid byte " << i << ": " << +static_cast<int8_t>(request_guid.data[i]) << std::endl;
    ser << static_cast<int8_t>(request_guid.data[i]);
  }

  int64_t seq_num = rs_libp2p_custom_publisher_get_sequence_number(info->request_publisher_->publisher_handle_);
  std::cout << "rmw_send_request: writing sequence number: " << seq_num << std::endl;
  ser << seq_num;

  if (_serialize_ros_message(ros_request, ser, info->request_publisher_->type_support_,
    info->typesupport_identifier_))
  {
    std::cout << "rmw_send_request: serialized message size " << std::endl;
    uint32_t status = rs_libp2p_custom_publisher_publish(info->request_publisher_->publisher_handle_, ser.data());
    std::cout << "rmw_send_request: publish status " << status << std::endl;
    if (status == 0) {  // TODO(esteve): replace with proper error codes
      *sequence_id = seq_num;
      returned_value = RMW_RET_OK;
    } else {
      RMW_SET_ERROR_MSG("cannot publish data");
    }
  } else {
    RMW_SET_ERROR_MSG("cannot serialize data");
  }

  return returned_value;
}
}  // extern "C"