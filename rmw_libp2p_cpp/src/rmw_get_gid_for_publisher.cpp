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

#include "rcutils/logging_macros.h"

#include "rmw/error_handling.h"
#include "rmw/rmw.h"

#include "impl/custom_publisher_info.hpp"
#include "impl/identifier.hpp"

extern "C"
{
RMW_PUBLIC
rmw_ret_t
libp2p_c__rmw_get_gid_for_publisher(const rmw_publisher_t * publisher, rmw_gid_t * gid)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s(publisher=%p,gid=%p)", __FUNCTION__, (void *)publisher, (void *)gid);

  if (!publisher) {
    RMW_SET_ERROR_MSG("publisher is null");
    return RMW_RET_ERROR;
  }

  if (publisher->implementation_identifier != libp2p_identifier) {
    RMW_SET_ERROR_MSG("publisher handle not from this implementation");
    return RMW_RET_ERROR;
  }

  if (!gid) {
    RMW_SET_ERROR_MSG("gid is null");
    return RMW_RET_ERROR;
  }

  auto info = static_cast<const rmw_libp2p_cpp::CustomPublisherInfo *>(publisher->data);
  if (!info || !info->publisher_handle_) {
    RMW_SET_ERROR_MSG("publisher info handle is null");
    return RMW_RET_ERROR;
  }

  constexpr size_t rs_uuid_size = sizeof(uint8_t) * 16u;

  gid->implementation_identifier = libp2p_identifier;
  static_assert(
    rs_uuid_size <= RMW_GID_STORAGE_SIZE,
    "RMW_GID_STORAGE_SIZE insufficient to store the rmw_libp2p_cpp GID implementation."
  );

  memset(gid->data, 0, RMW_GID_STORAGE_SIZE);
  const size_t ret = rs_libp2p_custom_publisher_get_gid(info->publisher_handle_, gid->data);
  if (ret == 0) {
    RMW_SET_ERROR_MSG("no guid found for publisher");
    return RMW_RET_ERROR;
  }
  return RMW_RET_OK;
}
}  // extern "C"
