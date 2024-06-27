// Copyright 2023 Esteve Fernandez All rights reserved.
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
#include "impl/custom_subscription_info.hpp"
#include "impl/listener.hpp"
#include "ros_message_serialization.hpp"

extern "C"
{
rmw_ret_t
_take(
  const rmw_subscription_t * subscription,
  void * ros_message,
  bool * taken,
  rmw_message_info_t * message_info)
{
  *taken = false;

  if (subscription->implementation_identifier != libp2p_identifier) {
    RMW_SET_ERROR_MSG("publisher handle not from this implementation");
    return RMW_RET_ERROR;
  }

  rmw_libp2p_cpp::CustomSubscriptionInfo * info =
    static_cast<rmw_libp2p_cpp::CustomSubscriptionInfo *>(subscription->data);
  RCUTILS_CHECK_FOR_NULL_WITH_MSG(info, "custom subscription info is null", return RMW_RET_ERROR);

  uint8_t * message = nullptr;
  uintptr_t length = 0;

  if (info->listener_->take_next_data(&message, length)) {
    rmw_libp2p_cpp::cdr::ReadCDRBuffer buffer(message, length);
    _deserialize_ros_message(
      buffer, ros_message, info->type_support_,
      info->typesupport_identifier_);
    *taken = true;
  }

  return RMW_RET_OK;
}

rmw_ret_t
libp2p_c__rmw_take_with_info(
  const rmw_subscription_t * subscription,
  void * ros_message,
  bool * taken,
  rmw_message_info_t * message_info,
  rmw_subscription_allocation_t * allocation)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s(subscription=%p,ros_message=%p,taken=%p,message_info=%p,allocation=%p)", __FUNCTION__,
    (void *)subscription, ros_message, (void *)taken, (void *)message_info, (void *)allocation);

  RCUTILS_CHECK_FOR_NULL_WITH_MSG(
    subscription, "subscription pointer is null", return RMW_RET_ERROR);
  RCUTILS_CHECK_FOR_NULL_WITH_MSG(
    ros_message, "ros_message pointer is null", return RMW_RET_ERROR);
  RCUTILS_CHECK_FOR_NULL_WITH_MSG(
    taken, "boolean flag for taken is null", return RMW_RET_ERROR);
  RCUTILS_CHECK_FOR_NULL_WITH_MSG(
    message_info, "message info pointer is null", return RMW_RET_ERROR);

  return _take(subscription, ros_message, taken, message_info);
}
}  // extern "C"