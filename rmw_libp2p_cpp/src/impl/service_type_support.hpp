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

#ifndef IMPL__SERVICE_TYPE_SUPPORT_HPP_
#define IMPL__SERVICE_TYPE_SUPPORT_HPP_

#include <cassert>

#include "impl/type_support.hpp"
#include "rosidl_typesupport_introspection_cpp/field_types.hpp"

struct CustomServiceInfo;

namespace rmw_libp2p_cpp
{

template<typename MembersType>
class ServiceTypeSupport : public TypeSupport<MembersType>
{
protected:
  explicit ServiceTypeSupport(const MembersType * members);
};

template<typename ServiceMembersType, typename MessageMembersType>
class RequestTypeSupport : public ServiceTypeSupport<MessageMembersType>
{
public:
  explicit RequestTypeSupport(const ServiceMembersType * members);
};

template<typename ServiceMembersType, typename MessageMembersType>
class ResponseTypeSupport : public ServiceTypeSupport<MessageMembersType>
{
public:
  explicit ResponseTypeSupport(const ServiceMembersType * members);
};

}  // namespace rmw_libp2p_cpp

#include "impl/service_type_support_impl.hpp"

#endif  // IMPL__SERVICE_TYPE_SUPPORT_HPP_
