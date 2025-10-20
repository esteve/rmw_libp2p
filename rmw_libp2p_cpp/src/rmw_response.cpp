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

  if (_serialize_ros_message(ros_response, ser, info->response_type_support_,
    info->typesupport_identifier_))
  {
    uint32_t status = rs_libp2p_custom_publisher_publish(pub, ser.data());
    std::cout << "rmw_send_response: publish status " << status << std::endl;
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

}  // extern "C"