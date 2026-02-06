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

#include "rmw/rmw.h"

#include "impl/serialization_format.hpp"

extern "C" {
const char * rmw_get_serialization_format()
{
  RCUTILS_LOG_DEBUG_NAMED("rmw_libp2p_cpp", "%s()", __FUNCTION__);

  return libp2p_serialization_format;
}
}  // extern "C"
