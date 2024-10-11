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

#include "rmw/impl/cpp/macros.hpp"
#include "rmw/rmw.h"

#include "impl/identifier.hpp"

#include "impl/rmw_libp2p_rs.hpp"
#include "impl/guard_condition.hpp"

extern "C"
{
rmw_guard_condition_t *
libp2p_c__rmw_create_guard_condition(rmw_context_t * context)
{
  RCUTILS_CHECK_ARGUMENT_FOR_NULL(context, nullptr);
  RMW_CHECK_TYPE_IDENTIFIERS_MATCH(
    init context,
    context->implementation_identifier,
    libp2p_identifier,
    return nullptr);

  rmw_guard_condition_t * guard_condition_handle = new rmw_guard_condition_t;
  guard_condition_handle->implementation_identifier = libp2p_identifier;
  guard_condition_handle->data = new GuardCondition();
  return guard_condition_handle;
}

rmw_ret_t
rmw_destroy_guard_condition(rmw_guard_condition_t * guard_condition)
{
  if (guard_condition) {
    delete static_cast<GuardCondition *>(guard_condition->data);
    delete guard_condition;
    return RMW_RET_OK;
  }

  return RMW_RET_ERROR;
}
}  // extern "C"
