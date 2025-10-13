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
#include "impl/custom_service_info.hpp"
#include "impl/custom_subscription_info.hpp"
#include "impl/listener.hpp"
#include "ros_message_serialization.hpp"

extern "C"
{
RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_take_request(
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

  uint8_t * message = nullptr;
  uintptr_t length = 0;

  pub = CREATE_PUBLISHER();

  if (info->listener_->take_next_data(&message, length)) {
    rmw_libp2p_cpp::cdr::ReadCDRBuffer buffer(message, length);

    _deserialize_ros_message(
      buffer, ros_request, info->request_type_support_,
      info->typesupport_identifier_);

    // Get header
    memset(request_header->request_id.writer_guid, 0, sizeof(
      request_header->request_id.writer_guid));
    const size_t ret = GET_GID_FROM_PUBLISHER(pub);
    COPY_GID_TO_WRITER_GUID(
      request_header->request_id.writer_guid, ret);
    request_header->request_id.sequence_number = GET_SEQUENCE_NUMBER_FROM_ROS_REQUEST(ros_request);

    *taken = true;

    info->requests_.emplace(std::make_pair(request_header->request_id, std::move(pub)));
  }

  return RMW_RET_OK;
}
}  // extern "C"