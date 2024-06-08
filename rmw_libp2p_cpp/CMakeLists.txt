cmake_minimum_required(VERSION 3.5)
project(rmw_libp2p_cpp)

if(NOT CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 14)
endif()
if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
  set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -Wl,--no-undefined")
elseif(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -Wl,-undefined,error")
endif()

include(FetchContent)

fetchcontent_declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.4
)

fetchcontent_makeavailable(Corrosion)

find_package(ZLIB REQUIRED)


find_package(ament_cmake_ros REQUIRED)

find_package(rcpputils REQUIRED)
find_package(rcutils REQUIRED)

find_package(rmw REQUIRED)
find_package(rosidl_typesupport_introspection_c REQUIRED)
find_package(rosidl_typesupport_introspection_cpp REQUIRED)

add_library(rmw_libp2p_cpp
  src/rmw_guard_condition.cpp
  src/rmw_get_gid_for_publisher.cpp
  src/rmw_get_implementation_identifier.cpp
  src/rmw_get_serialization_format.cpp
  src/rmw_guard_condition.cpp
  src/rmw_init.cpp
  src/rmw_libp2p.cpp
  src/rmw_node.cpp
  src/rmw_publish.cpp
  src/rmw_publisher.cpp
  src/rmw_service.cpp
  src/rmw_subscription.cpp
  src/rmw_take.cpp
  src/rmw_trigger_guard_condition.cpp
  src/rmw_wait.cpp
  src/rmw_wait_set.cpp
  src/ros_message_serialization.cpp
  src/serialization_format.cpp
  src/type_support_common.cpp
)
target_include_directories(rmw_libp2p_cpp
  PRIVATE src
)

corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml)

target_link_libraries(rmw_libp2p_rs INTERFACE ZLIB::ZLIB)

target_link_libraries(rmw_libp2p_cpp rmw_libp2p_rs)

# Add the definitions, include directories and libraries of packages
# to a target
ament_target_dependencies(rmw_libp2p_cpp
  "rcpputils"
  "rcutils"
  "rosidl_typesupport_introspection_c"
  "rosidl_typesupport_introspection_cpp"
  "rmw"
)

# Configures a library which implements the rmw interface with custom
# settings (the custom settings are all related to library symbol
# visibility)
configure_rmw_library(rmw_libp2p_cpp)

# Export libraries to downstream packages
ament_export_libraries(rmw_libp2p_cpp)

# Export dependencies to downstream packages
ament_export_dependencies(rosidl_typesupport_introspection_cpp)
ament_export_dependencies(rosidl_typesupport_introspection_c)
ament_export_dependencies(rcpputils)
ament_export_dependencies(rcutils)
ament_export_dependencies(rmw)

# Register the current package as a ROS middleware implementation
# <language:typesupport> tuples where language is the language of the
# typesupport package and typesupport is the name of the package
register_rmw_implementation(
  "c:rosidl_typesupport_c:rosidl_typesupport_introspection_c"
  "cpp:rosidl_typesupport_cpp:rosidl_typesupport_introspection_cpp")

install(
  TARGETS rmw_libp2p_cpp
  ARCHIVE DESTINATION lib
  LIBRARY DESTINATION lib
  RUNTIME DESTINATION bin
)

if(BUILD_TESTING)
  find_package(ament_lint_auto REQUIRED)
  ament_lint_auto_find_test_dependencies()
endif()

ament_export_include_directories(include)
ament_export_libraries(rmw_libp2p_cpp)
ament_package()
