#ifndef RMW_LIBP2P_CPP__RMW_NODE_HPP_
#define RMW_LIBP2P_CPP__RMW_NODE_HPP_

#include "rmw/rmw.h"

#ifdef __cplusplus
extern "C"
{
#endif

typedef struct rs_libp2p_custom_node rs_libp2p_custom_node_t;

extern rs_libp2p_custom_node_t *
rs_libp2p_custom_node_new(void);

extern void
rs_libp2p_custom_node_free(rs_libp2p_custom_node_t *);

#ifdef __cplusplus
}
#endif

#endif // RMW_LIBP2P_CPP__RMW_NODE_HPP_
