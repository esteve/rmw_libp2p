// Copyright 2024 Esteve Fernandez All rights reserved.
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

#include <chrono>
#include <cstdio>
#include <cstring>
#include <thread>

#include "rcutils/allocator.h"
#include "rcutils/logging.h"
#include "rcutils/strdup.h"
#include "rmw/error_handling.h"
#include "rmw/rmw.h"
#include "rmw/types.h"

/// Manual integration test for SIGINT handling.
///
/// This program creates an RMW node using rmw_libp2p_cpp and runs
/// indefinitely until it receives SIGINT (Ctrl+C). It demonstrates
/// that the signal handler correctly triggers graceful shutdown of
/// the libp2p node.
///
/// To run:
///   1. Build: colcon build --packages-select rmw_libp2p_cpp
///   2. Source: source install/setup.bash
///   3. Run: ./install/rmw_libp2p_cpp/lib/rmw_libp2p_cpp/manual_test_sigint
///   4. Press Ctrl+C to trigger shutdown
///
/// Expected behavior:
///   - Program prints "Node running, press Ctrl+C to stop..."
///   - Program prints heartbeat messages every 2 seconds
///   - When Ctrl+C is pressed, program prints "SIGINT received..."
///   - Program shuts down gracefully within 1-2 seconds
///   - No error messages or segfaults

int main(int argc, char ** argv)
{
  (void)argc;
  (void)argv;

  // Initialize logging
  rcutils_logging_set_default_logger_level(RCUTILS_LOG_SEVERITY_INFO);

  printf("=== RMW libp2p SIGINT Manual Integration Test ===\n");
  printf("This test verifies end-to-end SIGINT handling.\n");
  printf("Press Ctrl+C to trigger graceful shutdown.\n\n");
  fflush(stdout);

  // Get zero-initialized structures
  rmw_init_options_t init_options = rmw_get_zero_initialized_init_options();
  rmw_context_t context = rmw_get_zero_initialized_context();

  // Get allocator
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  // Initialize the init_options
  rmw_ret_t ret = rmw_init_options_init(&init_options, allocator);
  if (ret != RMW_RET_OK) {
    fprintf(stderr, "Failed to initialize init_options: %s\n", rmw_get_error_string().str);
    return 1;
  }

  // Set enclave (required by rmw_init)
  const char * enclave = "/";
  init_options.enclave = rcutils_strdup(enclave, allocator);
  if (!init_options.enclave) {
    fprintf(stderr, "Failed to allocate enclave string\n");
    ret = rmw_init_options_fini(&init_options);
    (void)ret;  // Ignore errors during cleanup
    return 1;
  }

  // Initialize the RMW context
  ret = rmw_init(&init_options, &context);
  if (ret != RMW_RET_OK) {
    fprintf(stderr, "Failed to initialize context: %s\n", rmw_get_error_string().str);
    ret = rmw_init_options_fini(&init_options);
    (void)ret;  // Ignore errors during cleanup
    return 1;
  }

  // Create a node
  rmw_node_t * node = rmw_create_node(
    &context,
    "test_sigint_node",
    "/test_namespace");

  if (!node) {
    fprintf(stderr, "Failed to create node: %s\n", rmw_get_error_string().str);
    ret = rmw_shutdown(&context);
    (void)ret;  // Ignore errors during cleanup
    ret = rmw_context_fini(&context);
    (void)ret;  // Ignore errors during cleanup
    ret = rmw_init_options_fini(&init_options);
    (void)ret;  // Ignore errors during cleanup
    return 1;
  }

  printf("Node created successfully: %s in namespace %s\n",
    node->name, node->namespace_);
  printf("Node running, press Ctrl+C to stop...\n\n");
  fflush(stdout);

  // Run indefinitely until SIGINT
  // Print a heartbeat message every 2 seconds to show the program is alive
  int heartbeat_count = 0;
  while (true) {
    std::this_thread::sleep_for(std::chrono::seconds(2));
    heartbeat_count++;
    printf("[Heartbeat %d] Node is running (waiting for SIGINT)...\n", heartbeat_count);
    fflush(stdout);
  }

  // Note: The code below is unreachable in normal operation because SIGINT
  // will terminate the process. However, if the signal handler is modified
  // to set a flag instead of exiting, this cleanup code would execute.

  // Clean up (unreachable due to SIGINT, but included for completeness)
  printf("\nShutting down...\n");

  ret = rmw_destroy_node(node);
  if (ret != RMW_RET_OK) {
    fprintf(stderr, "Failed to destroy node: %s\n", rmw_get_error_string().str);
  }

  ret = rmw_shutdown(&context);
  if (ret != RMW_RET_OK) {
    fprintf(stderr, "Failed to shutdown context: %s\n", rmw_get_error_string().str);
  }

  ret = rmw_context_fini(&context);
  if (ret != RMW_RET_OK) {
    fprintf(stderr, "Failed to finalize context: %s\n", rmw_get_error_string().str);
  }

  ret = rmw_init_options_fini(&init_options);
  if (ret != RMW_RET_OK) {
    fprintf(stderr, "Failed to finalize init_options: %s\n", rmw_get_error_string().str);
  }

  printf("Cleanup complete.\n");

  return 0;
}
