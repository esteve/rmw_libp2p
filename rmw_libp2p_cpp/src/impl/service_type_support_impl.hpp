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

#ifndef IMPL__SERVICE_TYPE_SUPPORT_IMPL_HPP_
#define IMPL__SERVICE_TYPE_SUPPORT_IMPL_HPP_

#include <cassert>
#include <string>

#include "impl/service_type_support.hpp"
#include "rosidl_typesupport_introspection_cpp/field_types.hpp"

namespace rmw_libp2p_cpp
{

template<typename MembersType>
ServiceTypeSupport<MembersType>::ServiceTypeSupport(
  const MembersType * members)
: TypeSupport<MembersType>(members)
{
}

template<typename ServiceMembersType, typename MessageMembersType>
RequestTypeSupport<ServiceMembersType, MessageMembersType>::RequestTypeSupport(
  const ServiceMembersType * members)
: ServiceTypeSupport<MessageMembersType>(members->request_members_)
{
}

template<typename ServiceMembersType, typename MessageMembersType>
ResponseTypeSupport<ServiceMembersType, MessageMembersType>::ResponseTypeSupport(
  const ServiceMembersType * members)
: ServiceTypeSupport<MessageMembersType>(members->response_members_)
{
}

}  // namespace rmw_libp2p_cpp

#endif  // IMPL__SERVICE_TYPE_SUPPORT_IMPL_HPP_
