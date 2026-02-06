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

#include <cassert>

#include "rcutils/logging_macros.h"

#include "rmw/error_handling.h"
#include "rmw/rmw.h"

#include "impl/cdr_buffer.hpp"
#include "impl/custom_publisher_info.hpp"
#include "impl/identifier.hpp"
#include "ros_message_serialization.hpp"

extern "C" {
rmw_ret_t rmw_publish(
  const rmw_publisher_t * publisher,
  const void * ros_message,
  rmw_publisher_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s(publisher=%p,ros_message=%p,allocation=%p)",
    __FUNCTION__,
    (void *)publisher,
    (void *)ros_message,
    (void *)allocation);

  rmw_ret_t returned_value = RMW_RET_ERROR;
  RCUTILS_CHECK_FOR_NULL_WITH_MSG(publisher, "publisher pointer is null", return RMW_RET_ERROR);
  RCUTILS_CHECK_FOR_NULL_WITH_MSG(ros_message, "ros_message pointer is null", return RMW_RET_ERROR);

  if (publisher->implementation_identifier != libp2p_identifier) {
    RMW_SET_ERROR_MSG("publisher handle not from this implementation");
    return RMW_RET_ERROR;
  }

  auto info = static_cast<rmw_libp2p_cpp::CustomPublisherInfo *>(publisher->data);
  assert(info);

  rmw_libp2p_cpp::cdr::WriteCDRBuffer ser;

  if (_serialize_ros_message(
      ros_message, ser, info->type_support_, info->typesupport_identifier_))
  {
    uint32_t status = rs_libp2p_custom_publisher_publish(info->publisher_handle_, ser.data());
    if (status == 0) {  // TODO(esteve): replace with proper error codes
      returned_value = RMW_RET_OK;
    } else {
      RMW_SET_ERROR_MSG("cannot publish data");
    }
  } else {
    RMW_SET_ERROR_MSG("cannot serialize data");
  }

  return returned_value;
}

rmw_ret_t rmw_publish_loaned_message(
  const rmw_publisher_t * publisher,
  void * ros_message,
  rmw_publisher_allocation_t * allocation)
{
  (void)publisher;
  (void)ros_message;
  (void)allocation;

  RMW_SET_ERROR_MSG("rmw_publish_loaned_message not implemented for rmw_libp2p_cpp");
  return RMW_RET_UNSUPPORTED;
}
}  // extern "C"
