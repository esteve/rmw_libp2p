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

rmw_ret_t
rmw_send_response(
  const rmw_service_t * service,
  rmw_request_id_t * request_header,
  void * ros_response)
{


  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s(service=%p,request_header=%p,ros_response=%p)", __FUNCTION__, (void *)service,
    (void *)request_header, ros_response);

  assert(service);
  assert(request_header);
  assert(ros_response);

  rmw_ret_t returned_value = RMW_RET_ERROR;

  if (service->implementation_identifier != libp2p_identifier) {
    RMW_SET_ERROR_MSG("service handle not from this implementation");
    return RMW_RET_ERROR;
  }

  auto info = static_cast<rmw_libp2p_cpp::CustomServiceInfo *>(service->data);
  assert(info);

  auto request = info->requests_.find(*request_header);
  if (request == info->requests_.end()) {
    RMW_SET_ERROR_MSG("cannot find request");
    return RMW_RET_ERROR;
  }

  rs_libp2p_custom_publisher_t * pub = std::move(request->second);
  rmw_libp2p_cpp::cdr::WriteCDRBuffer ser;

  // Get header
  rmw_gid_t request_guid;
  memset(request_guid.data, 0, RMW_GID_STORAGE_SIZE);
  const size_t ret = rs_libp2p_custom_publisher_get_gid(
    pub, request_guid.data);
  if (ret == 0) {
    RMW_SET_ERROR_MSG("no guid found for publisher");
    return RMW_RET_ERROR;
  }

  for (int i = 0; i < 16; ++i) {
    ser << static_cast<int8_t>(request_guid.data[i]);
  }

  int64_t seq_num = request_header->sequence_number;
  ser << seq_num;

  if (_serialize_ros_message(
      ros_response, ser, info->response_type_support_,
      info->typesupport_identifier_))
  {
    uint32_t status = rs_libp2p_custom_publisher_publish(pub, ser.data());
    if (status == 0) {
      returned_value = RMW_RET_OK;
    } else {
      RMW_SET_ERROR_MSG("cannot send response");
    }
  } else {
    RMW_SET_ERROR_MSG("cannot serialize data");
  }

  info->requests_.erase(request);
  return returned_value;
}

rmw_ret_t
rmw_take_response(
  const rmw_client_t * client,
  rmw_service_info_t * request_header,
  void * ros_response,
  bool * taken)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s(client=%p,request_header=%p,ros_request=%p,taken=%p)", __FUNCTION__, (void *)client,
    (void *)request_header, ros_response, (void *)taken);

  assert(client);
  assert(request_header);
  assert(ros_response);
  assert(taken);

  *taken = false;

  if (client->implementation_identifier != libp2p_identifier) {
    RMW_SET_ERROR_MSG("publisher handle not from this implementation");
    return RMW_RET_ERROR;
  }

  auto * info = static_cast<rmw_libp2p_cpp::CustomClientInfo *>(client->data);
  assert(info);

  uint8_t * message = nullptr;
  uintptr_t length = 0;

  if (info->listener_->take_next_data(&message, length)) {
    rmw_libp2p_cpp::cdr::ReadCDRBuffer buffer(message, length);

    uint64_t secs = 0;
    uint32_t usecs = 0;
    buffer >> secs;
    buffer >> usecs;
    request_header->source_timestamp = secs * 1000000000ull + usecs * 1000ull;

    // Get header
    memset(request_header->request_id.writer_guid, 0, RMW_GID_STORAGE_SIZE);
    for (int i = 0; i < 16; ++i) {
      int8_t value = 0;
      buffer >> value;
      request_header->request_id.writer_guid[i] = value;
    }
    buffer >> request_header->request_id.sequence_number;


    _deserialize_ros_message(
      buffer, ros_response, info->response_subscription_->type_support_,
      info->typesupport_identifier_);

    *taken = true;
  }

  return RMW_RET_OK;
}


}  // extern "C"
