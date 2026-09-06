//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/mlx5_user_ioctl_cmds.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


//
// Copyright (c) 2018, Mellanox Technologies inc.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_create_flow_action_attrs {
// This attribute belong to the driver namespace
    MLX5_IB_ATTR_CREATE_FLOW_ACTION_FLAGS = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_dm_methods {
    MLX5_IB_METHOD_DM_MAP_OP_ADDR  = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_METHOD_DM_QUERY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_dm_map_op_addr_attrs {
    MLX5_IB_ATTR_DM_MAP_OP_ADDR_REQ_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_DM_MAP_OP_ADDR_REQ_OP,
    MLX5_IB_ATTR_DM_MAP_OP_ADDR_RESP_START_OFFSET,
    MLX5_IB_ATTR_DM_MAP_OP_ADDR_RESP_PAGE_INDEX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_query_dm_attrs {
    MLX5_IB_ATTR_QUERY_DM_REQ_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_QUERY_DM_RESP_START_OFFSET,
    MLX5_IB_ATTR_QUERY_DM_RESP_PAGE_INDEX,
    MLX5_IB_ATTR_QUERY_DM_RESP_LENGTH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_alloc_dm_attrs {
    MLX5_IB_ATTR_ALLOC_DM_RESP_START_OFFSET = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_ALLOC_DM_RESP_PAGE_INDEX,
    MLX5_IB_ATTR_ALLOC_DM_REQ_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_methods {
    MLX5_IB_METHOD_DEVX_OTHER  = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_METHOD_DEVX_QUERY_UAR,
    MLX5_IB_METHOD_DEVX_QUERY_EQN,
    MLX5_IB_METHOD_DEVX_SUBSCRIBE_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_other_attrs {
    MLX5_IB_ATTR_DEVX_OTHER_CMD_IN = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_DEVX_OTHER_CMD_OUT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_obj_create_attrs {
    MLX5_IB_ATTR_DEVX_OBJ_CREATE_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_DEVX_OBJ_CREATE_CMD_IN,
    MLX5_IB_ATTR_DEVX_OBJ_CREATE_CMD_OUT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_query_uar_attrs {
    MLX5_IB_ATTR_DEVX_QUERY_UAR_USER_IDX = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_DEVX_QUERY_UAR_DEV_IDX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_obj_destroy_attrs {
    MLX5_IB_ATTR_DEVX_OBJ_DESTROY_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_obj_modify_attrs {
    MLX5_IB_ATTR_DEVX_OBJ_MODIFY_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_DEVX_OBJ_MODIFY_CMD_IN,
    MLX5_IB_ATTR_DEVX_OBJ_MODIFY_CMD_OUT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_obj_query_attrs {
    MLX5_IB_ATTR_DEVX_OBJ_QUERY_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_DEVX_OBJ_QUERY_CMD_IN,
    MLX5_IB_ATTR_DEVX_OBJ_QUERY_CMD_OUT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_obj_query_async_attrs {
    MLX5_IB_ATTR_DEVX_OBJ_QUERY_ASYNC_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_DEVX_OBJ_QUERY_ASYNC_CMD_IN,
    MLX5_IB_ATTR_DEVX_OBJ_QUERY_ASYNC_FD,
    MLX5_IB_ATTR_DEVX_OBJ_QUERY_ASYNC_WR_ID,
    MLX5_IB_ATTR_DEVX_OBJ_QUERY_ASYNC_OUT_LEN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_subscribe_event_attrs {
    MLX5_IB_ATTR_DEVX_SUBSCRIBE_EVENT_FD_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_DEVX_SUBSCRIBE_EVENT_OBJ_HANDLE,
    MLX5_IB_ATTR_DEVX_SUBSCRIBE_EVENT_TYPE_NUM_LIST,
    MLX5_IB_ATTR_DEVX_SUBSCRIBE_EVENT_FD_NUM,
    MLX5_IB_ATTR_DEVX_SUBSCRIBE_EVENT_COOKIE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_query_eqn_attrs {
    MLX5_IB_ATTR_DEVX_QUERY_EQN_USER_VEC = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_DEVX_QUERY_EQN_DEV_EQN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_obj_methods {
    MLX5_IB_METHOD_DEVX_OBJ_CREATE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_METHOD_DEVX_OBJ_DESTROY,
    MLX5_IB_METHOD_DEVX_OBJ_MODIFY,
    MLX5_IB_METHOD_DEVX_OBJ_QUERY,
    MLX5_IB_METHOD_DEVX_OBJ_ASYNC_QUERY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_var_alloc_attrs {
    MLX5_IB_ATTR_VAR_OBJ_ALLOC_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_VAR_OBJ_ALLOC_MMAP_OFFSET,
    MLX5_IB_ATTR_VAR_OBJ_ALLOC_MMAP_LENGTH,
    MLX5_IB_ATTR_VAR_OBJ_ALLOC_PAGE_ID,
    MLX5_IB_ATTR_VAR_OBJ_ALLOC_FLAGS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_var_obj_destroy_attrs {
    MLX5_IB_ATTR_VAR_OBJ_DESTROY_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_var_obj_methods {
    MLX5_IB_METHOD_VAR_OBJ_ALLOC = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_METHOD_VAR_OBJ_DESTROY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_uar_alloc_attrs {
    MLX5_IB_ATTR_UAR_OBJ_ALLOC_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_UAR_OBJ_ALLOC_TYPE,
    MLX5_IB_ATTR_UAR_OBJ_ALLOC_MMAP_OFFSET,
    MLX5_IB_ATTR_UAR_OBJ_ALLOC_MMAP_LENGTH,
    MLX5_IB_ATTR_UAR_OBJ_ALLOC_PAGE_ID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_uar_obj_destroy_attrs {
    MLX5_IB_ATTR_UAR_OBJ_DESTROY_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_uar_obj_methods {
    MLX5_IB_METHOD_UAR_OBJ_ALLOC = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_METHOD_UAR_OBJ_DESTROY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_umem_reg_attrs {
    MLX5_IB_ATTR_DEVX_UMEM_REG_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_DEVX_UMEM_REG_ADDR,
    MLX5_IB_ATTR_DEVX_UMEM_REG_LEN,
    MLX5_IB_ATTR_DEVX_UMEM_REG_ACCESS,
    MLX5_IB_ATTR_DEVX_UMEM_REG_OUT_ID,
    MLX5_IB_ATTR_DEVX_UMEM_REG_PGSZ_BITMAP,
    MLX5_IB_ATTR_DEVX_UMEM_REG_DMABUF_FD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_umem_dereg_attrs {
    MLX5_IB_ATTR_DEVX_UMEM_DEREG_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_pp_obj_methods {
    MLX5_IB_METHOD_PP_OBJ_ALLOC = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_METHOD_PP_OBJ_DESTROY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_pp_alloc_attrs {
    MLX5_IB_ATTR_PP_OBJ_ALLOC_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_PP_OBJ_ALLOC_CTX,
    MLX5_IB_ATTR_PP_OBJ_ALLOC_FLAGS,
    MLX5_IB_ATTR_PP_OBJ_ALLOC_INDEX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_pp_obj_destroy_attrs {
    MLX5_IB_ATTR_PP_OBJ_DESTROY_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_umem_methods {
    MLX5_IB_METHOD_DEVX_UMEM_REG = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_METHOD_DEVX_UMEM_DEREG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_async_cmd_fd_alloc_attrs {
    MLX5_IB_ATTR_DEVX_ASYNC_CMD_FD_ALLOC_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_async_event_fd_alloc_attrs {
    MLX5_IB_ATTR_DEVX_ASYNC_EVENT_FD_ALLOC_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_DEVX_ASYNC_EVENT_FD_ALLOC_FLAGS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_async_cmd_fd_methods {
    MLX5_IB_METHOD_DEVX_ASYNC_CMD_FD_ALLOC = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_devx_async_event_fd_methods {
    MLX5_IB_METHOD_DEVX_ASYNC_EVENT_FD_ALLOC = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_objects {
    MLX5_IB_OBJECT_DEVX = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_OBJECT_DEVX_OBJ,
    MLX5_IB_OBJECT_DEVX_UMEM,
    MLX5_IB_OBJECT_FLOW_MATCHER,
    MLX5_IB_OBJECT_DEVX_ASYNC_CMD_FD,
    MLX5_IB_OBJECT_DEVX_ASYNC_EVENT_FD,
    MLX5_IB_OBJECT_VAR,
    MLX5_IB_OBJECT_PP,
    MLX5_IB_OBJECT_UAR,
    MLX5_IB_OBJECT_STEERING_ANCHOR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_flow_matcher_create_attrs {
    MLX5_IB_ATTR_FLOW_MATCHER_CREATE_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_FLOW_MATCHER_MATCH_MASK,
    MLX5_IB_ATTR_FLOW_MATCHER_FLOW_TYPE,
    MLX5_IB_ATTR_FLOW_MATCHER_MATCH_CRITERIA,
    MLX5_IB_ATTR_FLOW_MATCHER_FLOW_FLAGS,
    MLX5_IB_ATTR_FLOW_MATCHER_FT_TYPE,
    MLX5_IB_ATTR_FLOW_MATCHER_IB_PORT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_flow_matcher_destroy_attrs {
    MLX5_IB_ATTR_FLOW_MATCHER_DESTROY_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_flow_matcher_methods {
    MLX5_IB_METHOD_FLOW_MATCHER_CREATE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_METHOD_FLOW_MATCHER_DESTROY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_flow_steering_anchor_create_attrs {
    MLX5_IB_ATTR_STEERING_ANCHOR_CREATE_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_STEERING_ANCHOR_FT_TYPE,
    MLX5_IB_ATTR_STEERING_ANCHOR_PRIORITY,
    MLX5_IB_ATTR_STEERING_ANCHOR_FT_ID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_flow_steering_anchor_destroy_attrs {
    MLX5_IB_ATTR_STEERING_ANCHOR_DESTROY_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_steering_anchor_methods {
    MLX5_IB_METHOD_STEERING_ANCHOR_CREATE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_METHOD_STEERING_ANCHOR_DESTROY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_device_query_context_attrs {
    MLX5_IB_ATTR_QUERY_CONTEXT_RESP_UCTX = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_create_cq_attrs {
    MLX5_IB_ATTR_CREATE_CQ_UAR_INDEX = UVERBS_ID_DRIVER_NS_WITH_UHW,
    MLX5_IB_ATTR_CREATE_CQ_DBR_BUF_UMEM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_create_qp_attrs {
    MLX5_IB_ATTR_CREATE_QP_DBR_BUF_UMEM = UVERBS_ID_DRIVER_NS_WITH_UHW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_create_srq_attrs {
    MLX5_IB_ATTR_CREATE_SRQ_DBR_BUF_UMEM = UVERBS_ID_DRIVER_NS_WITH_UHW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_reg_dmabuf_mr_attrs {
    MLX5_IB_ATTR_REG_DMABUF_MR_ACCESS_FLAGS = (1U << UVERBS_ID_NS_SHIFT),
}

pub const MLX5_IB_DW_MATCH_PARAM: c_uint = 0xA0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_match_params {
    pub match_params: [__u32; MLX5_IB_DW_MATCH_PARAM],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_flow_type {
    MLX5_IB_FLOW_TYPE_NORMAL,
    MLX5_IB_FLOW_TYPE_SNIFFER,
    MLX5_IB_FLOW_TYPE_ALL_DEFAULT,
    MLX5_IB_FLOW_TYPE_MC_DEFAULT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_create_flow_flags {
    MLX5_IB_ATTR_CREATE_FLOW_FLAGS_DEFAULT_MISS = 1 << 0,
    MLX5_IB_ATTR_CREATE_FLOW_FLAGS_DROP = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_create_flow_attrs {
    MLX5_IB_ATTR_CREATE_FLOW_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_CREATE_FLOW_MATCH_VALUE,
    MLX5_IB_ATTR_CREATE_FLOW_DEST_QP,
    MLX5_IB_ATTR_CREATE_FLOW_DEST_DEVX,
    MLX5_IB_ATTR_CREATE_FLOW_MATCHER,
    MLX5_IB_ATTR_CREATE_FLOW_ARR_FLOW_ACTIONS,
    MLX5_IB_ATTR_CREATE_FLOW_TAG,
    MLX5_IB_ATTR_CREATE_FLOW_ARR_COUNTERS_DEVX,
    MLX5_IB_ATTR_CREATE_FLOW_ARR_COUNTERS_DEVX_OFFSET,
    MLX5_IB_ATTR_CREATE_FLOW_FLAGS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_destroy_flow_attrs {
    MLX5_IB_ATTR_DESTROY_FLOW_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_flow_methods {
    MLX5_IB_METHOD_CREATE_FLOW = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_METHOD_DESTROY_FLOW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_flow_action_methods {
    MLX5_IB_METHOD_FLOW_ACTION_CREATE_MODIFY_HEADER = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_METHOD_FLOW_ACTION_CREATE_PACKET_REFORMAT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_create_flow_action_create_modify_header_attrs {
    MLX5_IB_ATTR_CREATE_MODIFY_HEADER_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_CREATE_MODIFY_HEADER_ACTIONS_PRM,
    MLX5_IB_ATTR_CREATE_MODIFY_HEADER_FT_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_create_flow_action_create_packet_reformat_attrs {
    MLX5_IB_ATTR_CREATE_PACKET_REFORMAT_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_CREATE_PACKET_REFORMAT_TYPE,
    MLX5_IB_ATTR_CREATE_PACKET_REFORMAT_FT_TYPE,
    MLX5_IB_ATTR_CREATE_PACKET_REFORMAT_DATA_BUF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_query_pd_attrs {
    MLX5_IB_ATTR_QUERY_PD_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_QUERY_PD_RESP_PDN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_pd_methods {
    MLX5_IB_METHOD_PD_QUERY = (1U << UVERBS_ID_NS_SHIFT),

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_device_methods {
    MLX5_IB_METHOD_QUERY_PORT = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_METHOD_GET_DATA_DIRECT_SYSFS_PATH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_query_port_attrs {
    MLX5_IB_ATTR_QUERY_PORT_PORT_NUM = (1U << UVERBS_ID_NS_SHIFT),
    MLX5_IB_ATTR_QUERY_PORT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_get_data_direct_sysfs_path_attrs {
    MLX5_IB_ATTR_GET_DATA_DIRECT_SYSFS_PATH = (1U << UVERBS_ID_NS_SHIFT),
}
