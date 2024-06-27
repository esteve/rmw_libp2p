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

#include "rcutils/strdup.h"

#include "rcpputils/scope_exit.hpp"

#include "impl/identifier.hpp"

#include "impl/rmw_libp2p_rs.hpp"

extern "C"
{
// Initialize given init_options with the default values
// and implementation specific values.
//
// rmw_init_options_t Doc: http://docs.ros2.org/latest/api/rmw/structrmw__init__options__t.html
//
// Note: You should call rmw_get_zero_initialized_init_options()
// to get a zero initialized rmw_init_options_t struct first
rmw_ret_t
libp2p_c__rmw_init_options_init(
  rmw_init_options_t * init_options,
  rcutils_allocator_t allocator)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  RMW_CHECK_ARGUMENT_FOR_NULL(init_options, RMW_RET_INVALID_ARGUMENT);
  RCUTILS_CHECK_ALLOCATOR(&allocator, return RMW_RET_INVALID_ARGUMENT);
  if (nullptr != init_options->implementation_identifier) {
    RMW_SET_ERROR_MSG("expected zero-initialized init_options");
    return RMW_RET_INVALID_ARGUMENT;
  }
  init_options->instance_id = 0;
  init_options->implementation_identifier = libp2p_identifier;
  init_options->allocator = allocator;
  init_options->impl = nullptr;
  init_options->enclave = NULL;
  init_options->domain_id = RMW_DEFAULT_DOMAIN_ID;
  init_options->security_options = rmw_get_default_security_options();
  init_options->localhost_only = RMW_LOCALHOST_ONLY_DEFAULT;
  return RMW_RET_OK;
}

// Copy the given source init options to the destination init options.
rmw_ret_t
libp2p_c__rmw_init_options_copy(
  const rmw_init_options_t * src,
  rmw_init_options_t * dst)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  RMW_CHECK_ARGUMENT_FOR_NULL(src, RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_ARGUMENT_FOR_NULL(dst, RMW_RET_INVALID_ARGUMENT);
  if (NULL == src->implementation_identifier) {
    RMW_SET_ERROR_MSG("expected initialized dst");
    return RMW_RET_INVALID_ARGUMENT;
  }
  RMW_CHECK_TYPE_IDENTIFIERS_MATCH(
    src,
    src->implementation_identifier,
    libp2p_identifier,
    return RMW_RET_INCORRECT_RMW_IMPLEMENTATION);
  if (NULL != dst->implementation_identifier) {
    RMW_SET_ERROR_MSG("expected zero-initialized dst");
    return RMW_RET_INVALID_ARGUMENT;
  }
  const rcutils_allocator_t * allocator = &src->allocator;
  RCUTILS_CHECK_ALLOCATOR(allocator, return RMW_RET_INVALID_ARGUMENT);

  rmw_init_options_t tmp = *src;
  tmp.enclave = rcutils_strdup(tmp.enclave, *allocator);
  if (NULL != src->enclave && NULL == tmp.enclave) {
    return RMW_RET_BAD_ALLOC;
  }
  tmp.security_options = rmw_get_zero_initialized_security_options();
  rmw_ret_t ret =
    rmw_security_options_copy(&src->security_options, allocator, &tmp.security_options);
  if (RMW_RET_OK != ret) {
    allocator->deallocate(tmp.enclave, allocator->state);
    return ret;
  }
  *dst = tmp;
  return RMW_RET_OK;
}

// Finalize the given init_options. (Cleanup and deallocation.)
rmw_ret_t
libp2p_c__rmw_init_options_fini(
  rmw_init_options_t * init_options)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  RMW_CHECK_ARGUMENT_FOR_NULL(init_options, RMW_RET_INVALID_ARGUMENT);
  RCUTILS_CHECK_ALLOCATOR(&(init_options->allocator), return RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_TYPE_IDENTIFIERS_MATCH(
    init_options,
    init_options->implementation_identifier,
    libp2p_identifier,
    return RMW_RET_INCORRECT_RMW_IMPLEMENTATION);
  *init_options = rmw_get_zero_initialized_init_options();
  return RMW_RET_OK;
}

// Initialize the middleware with the given options, and yielding an context.
//
// rmw_context_t Doc: http://docs.ros2.org/latest/api/rmw/structrmw__context__t.html
rmw_ret_t
libp2p_c__rmw_init(
  const rmw_init_options_t * options,
  rmw_context_t * context)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  RMW_CHECK_ARGUMENT_FOR_NULL(options, RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_ARGUMENT_FOR_NULL(context, RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_FOR_NULL_WITH_MSG(
    options->implementation_identifier,
    "init option is not initialized",
    return RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_FOR_NULL_WITH_MSG(
    options->enclave,
    "init options encalve is null",
    return RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_TYPE_IDENTIFIERS_MATCH(
    options,
    options->implementation_identifier,
    libp2p_identifier,
    return RMW_RET_INCORRECT_RMW_IMPLEMENTATION);

  if (context->implementation_identifier != nullptr) {
    RMW_SET_ERROR_MSG("context is not zero-initialized");
    return RMW_RET_INVALID_ARGUMENT;
  }

  auto restore_context = rcpputils::make_scope_exit(
    [context]() {*context = rmw_get_zero_initialized_context();});

  context->instance_id = options->instance_id;
  context->implementation_identifier = libp2p_identifier;
  context->actual_domain_id =
    RMW_DEFAULT_DOMAIN_ID == options->domain_id ? 0uL : options->domain_id;

  context->impl = new (std::nothrow) rmw_context_impl_t();
  if (nullptr == context->impl) {
    RMW_SET_ERROR_MSG("failed to allocate context impl");
    return RMW_RET_BAD_ALLOC;
  }

  auto cleanup_impl = rcpputils::make_scope_exit(
    [context]() {delete context->impl;});

  // context->impl->rs_event_loop_thread = rs_rmw_init();
  context->options = rmw_get_zero_initialized_init_options();
  rmw_ret_t ret = rmw_init_options_copy(options, &context->options);
  if (RMW_RET_OK != ret) {
    return ret;
  }

  ret = rcutils_logging_set_logger_level("rmw_libp2p2_cpp", RCUTILS_LOG_SEVERITY_INFO);
  if (RMW_RET_OK != ret) {
    return ret;
  }

  cleanup_impl.cancel();
  restore_context.cancel();
  return RMW_RET_OK;
}

// Shutdown the middleware for a given context.
rmw_ret_t
libp2p_c__rmw_shutdown(
  rmw_context_t * context)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  RMW_CHECK_ARGUMENT_FOR_NULL(context, RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_FOR_NULL_WITH_MSG(
    context->impl,
    "expected initialized context",
    return RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_TYPE_IDENTIFIERS_MATCH(
    context,
    context->implementation_identifier,
    libp2p_identifier,
    return RMW_RET_INCORRECT_RMW_IMPLEMENTATION);
  // TODO(esteve): stop event loop thread here
  // context->impl->event_loop_thread
  return RMW_RET_OK;
}

// Finalize a context. (Cleanup and deallocation)
rmw_ret_t
libp2p_c__rmw_context_fini(
  rmw_context_t * context)
{
  RCUTILS_LOG_DEBUG_NAMED(
    "rmw_libp2p_cpp",
    "%s()", __FUNCTION__);

  RMW_CHECK_ARGUMENT_FOR_NULL(context, RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_FOR_NULL_WITH_MSG(
    context->impl,
    "expected initialized context",
    return RMW_RET_INVALID_ARGUMENT);
  RMW_CHECK_TYPE_IDENTIFIERS_MATCH(
    context,
    context->implementation_identifier,
    libp2p_identifier,
    return RMW_RET_INCORRECT_RMW_IMPLEMENTATION);
  if (!context->impl->is_shutdown) {
    RCUTILS_SET_ERROR_MSG("context has not been shutdown");
    return RMW_RET_INVALID_ARGUMENT;
  }
  rmw_ret_t ret = rmw_init_options_fini(&context->options);
  delete context->impl;
  *context = rmw_get_zero_initialized_context();
  return ret;
}
}  // extern "C"
