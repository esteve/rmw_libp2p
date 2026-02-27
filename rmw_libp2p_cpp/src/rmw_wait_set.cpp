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

#include "rmw/allocators.h"
#include "rmw/error_handling.h"
#include "rmw/rmw.h"
#include "rmw/impl/cpp/macros.hpp"

#include "impl/identifier.hpp"
#include "impl/custom_wait_set_info.hpp"

extern "C"
{
rmw_wait_set_t *
rmw_create_wait_set(rmw_context_t * context, size_t max_conditions)
{
  (void)context;
  (void)max_conditions;
  rmw_wait_set_t * wait_set = rmw_wait_set_allocate();
  rmw_libp2p_cpp::CustomWaitsetInfo * wait_set_info = nullptr;

  if (!wait_set) {
    RMW_SET_ERROR_MSG("failed to allocate wait set");
    goto fail;
  }
  wait_set->implementation_identifier = libp2p_identifier;
  wait_set->data = rmw_allocate(sizeof(rmw_libp2p_cpp::CustomWaitsetInfo));

  wait_set_info = static_cast<rmw_libp2p_cpp::CustomWaitsetInfo *>(wait_set->data);
  RMW_TRY_PLACEMENT_NEW(
    wait_set_info, wait_set_info, goto fail, rmw_libp2p_cpp::CustomWaitsetInfo,
  )
  if (!wait_set_info) {
    RMW_SET_ERROR_MSG("failed to construct wait set info struct");
    goto fail;
  }

  return wait_set;

fail:
  if (wait_set) {
    if (wait_set->data) {
      RMW_TRY_DESTRUCTOR_FROM_WITHIN_FAILURE(
        wait_set_info->~CustomWaitsetInfo(), wait_set_info)
      rmw_free(wait_set->data);
    }
    rmw_wait_set_free(wait_set);
  }
  return nullptr;
}

rmw_ret_t
rmw_destroy_wait_set(rmw_wait_set_t * wait_set)
{
  if (!wait_set) {
    RMW_SET_ERROR_MSG("wait set handle is null");
    return RMW_RET_ERROR;
  }
  RMW_CHECK_TYPE_IDENTIFIERS_MATCH(
    wait set handle,
    wait_set->implementation_identifier, libp2p_identifier,
    return RMW_RET_ERROR);

  auto result = RMW_RET_OK;
  auto wait_set_info = static_cast<rmw_libp2p_cpp::CustomWaitsetInfo *>(wait_set->data);
  if (!wait_set_info) {
    RMW_SET_ERROR_MSG("wait set info is null");
    return RMW_RET_ERROR;
  }
  std::mutex * condition_mutex = &wait_set_info->condition_mutex;
  if (!condition_mutex) {
    RMW_SET_ERROR_MSG("wait set mutex is null");
    return RMW_RET_ERROR;
  }

  if (wait_set->data) {
    if (wait_set_info) {
      RMW_TRY_DESTRUCTOR(
        wait_set_info->~CustomWaitsetInfo(), wait_set_info, result = RMW_RET_ERROR)
    }
    rmw_free(wait_set->data);
  }
  rmw_wait_set_free(wait_set);
  return result;
}
}  // extern "C"
