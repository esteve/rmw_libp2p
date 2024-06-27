bindgen \
    --allowlist-file /opt/ros/humble/include/rmw/rmw/rmw.h \
    --generate functions \
    --no-layout-tests \
    --no-doc-comments \
    --merge-extern-blocks /opt/ros/humble/include/rmw/rmw/rmw.h \
    -o bindings.rs \
    -- \
    -I /opt/ros/humble/include/rmw \
    -I /opt/ros/humble/include/rcutils/ \
    -I /opt/ros/humble/include/rosidl_runtime_c \
    -I /opt/ros/humble/include/rosidl_typesupport_interface/