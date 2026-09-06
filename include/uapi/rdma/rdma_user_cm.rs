//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/rdma_user_cm.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB)
//
// Copyright (c) 2005-2006 Intel Corporation.  All rights reserved.
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

pub const RDMA_USER_CM_ABI_VERSION: c_int = 4;
pub const RDMA_MAX_PRIVATE_DATA: c_int = 256;
// See IBTA Annex A11, servies ID bytes 4 & 5
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_ucm_port_space {
    RDMA_PS_IPOIB = 0x0002,
    RDMA_PS_IB    = 0x013F,
    RDMA_PS_TCP   = 0x0106,
    RDMA_PS_UDP   = 0x0111,
}

//
// command ABI structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_cmd_hdr {
    pub cmd: __u32,
    pub in: __u16,
    pub out: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_create_id {
    pub uid: __aligned_u64,
    pub response: __aligned_u64,
    pub /: *mut *mut __u16 ps; / use enum rdma_ucm_port_space,
    pub qp_type: __u8,
    pub reserved: [__u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_create_id_resp {
    pub id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_destroy_id {
    pub response: __aligned_u64,
    pub id: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_destroy_id_resp {
    pub events_reported: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_bind_ip {
    pub response: __aligned_u64,
    pub addr: sockaddr_in6,
    pub id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_bind {
    pub id: __u32,
    pub addr_size: __u16,
    pub reserved: __u16,
    pub addr: __kernel_sockaddr_storage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_resolve_ip {
    pub src_addr: sockaddr_in6,
    pub dst_addr: sockaddr_in6,
    pub id: __u32,
    pub timeout_ms: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_resolve_addr {
    pub id: __u32,
    pub timeout_ms: __u32,
    pub src_size: __u16,
    pub dst_size: __u16,
    pub reserved: __u32,
    pub src_addr: __kernel_sockaddr_storage,
    pub dst_addr: __kernel_sockaddr_storage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_resolve_route {
    pub id: __u32,
    pub timeout_ms: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_query {
    pub response: __aligned_u64,
    pub id: __u32,
    pub option: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_query_route_resp {
    pub node_guid: __aligned_u64,
    pub ib_route: [ib_user_path_rec; 2],
    pub src_addr: sockaddr_in6,
    pub dst_addr: sockaddr_in6,
    pub num_paths: __u32,
    pub port_num: __u8,
    pub reserved: [__u8; 3],
    pub ibdev_index: __u32,
    pub reserved1: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_query_addr_resp {
    pub node_guid: __aligned_u64,
    pub port_num: __u8,
    pub reserved: __u8,
    pub pkey: __u16,
    pub src_size: __u16,
    pub dst_size: __u16,
    pub src_addr: __kernel_sockaddr_storage,
    pub dst_addr: __kernel_sockaddr_storage,
    pub ibdev_index: __u32,
    pub reserved1: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_query_path_resp {
    pub num_paths: __u32,
    pub reserved: __u32,
    pub path_data: [ib_path_rec_data; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_query_ib_service_resp {
    pub num_service_recs: __u32,
    pub reserved: __u32,
    pub recs: [ib_user_service_rec; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_conn_param {
    pub qp_num: __u32,
    pub qkey: __u32,
    pub private_data: [__u8; RDMA_MAX_PRIVATE_DATA],
    pub private_data_len: __u8,
    pub srq: __u8,
    pub responder_resources: __u8,
    pub initiator_depth: __u8,
    pub flow_control: __u8,
    pub retry_count: __u8,
    pub rnr_retry_count: __u8,
    pub valid: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_ud_param {
    pub qp_num: __u32,
    pub qkey: __u32,
    pub ah_attr: ib_uverbs_ah_attr,
    pub private_data: [__u8; RDMA_MAX_PRIVATE_DATA],
    pub private_data_len: __u8,
    pub reserved: [__u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_ece {
    pub vendor_id: __u32,
    pub attr_mod: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_connect {
    pub conn_param: rdma_ucm_conn_param,
    pub id: __u32,
    pub reserved: __u32,
    pub ece: rdma_ucm_ece,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_listen {
    pub id: __u32,
    pub backlog: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_accept {
    pub uid: __aligned_u64,
    pub conn_param: rdma_ucm_conn_param,
    pub id: __u32,
    pub reserved: __u32,
    pub ece: rdma_ucm_ece,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_reject {
    pub id: __u32,
    pub private_data_len: __u8,
    pub reason: __u8,
    pub reserved: [__u8; 2],
    pub private_data: [__u8; RDMA_MAX_PRIVATE_DATA],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_disconnect {
    pub id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_init_qp_attr {
    pub response: __aligned_u64,
    pub id: __u32,
    pub qp_state: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_notify {
    pub id: __u32,
    pub event: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_join_ip_mcast {
    pub /: *mut *mut __aligned_u64 response; / rdma_ucm_create_id_resp,
    pub uid: __aligned_u64,
    pub addr: sockaddr_in6,
    pub id: __u32,
}

// Multicast join flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_join_mcast {
    pub /: *mut *mut __aligned_u64 response; / rdma_ucma_create_id_resp,
    pub uid: __aligned_u64,
    pub id: __u32,
    pub addr_size: __u16,
    pub join_flags: __u16,
    pub addr: __kernel_sockaddr_storage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_get_event {
    pub response: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_event_resp {
    pub uid: __aligned_u64,
    pub id: __u32,
    pub event: __u32,
    pub status: __u32,
//
// NOTE: This union is not aligned to 8 bytes so none of the union
// members may contain a u64 or anything with higher alignment than 4.
//
    pub conn: rdma_ucm_conn_param,
    pub ud: rdma_ucm_ud_param,
    pub arg32: [__u32; 2],
    pub param: },
    pub reserved: __u32,
    pub ece: rdma_ucm_ece,
}

// Option levels
// Option details
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_set_option {
    pub optval: __aligned_u64,
    pub id: __u32,
    pub level: __u32,
    pub optname: __u32,
    pub optlen: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_migrate_id {
    pub response: __aligned_u64,
    pub id: __u32,
    pub fd: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_migrate_resp {
    pub events_reported: __u32,
}

pub const RDMA_USER_CM_IB_SERVICE_NAME_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_ib_service {
    pub service_id: __aligned_u64,
    pub service_name: [__u8; RDMA_USER_CM_IB_SERVICE_NAME_SIZE],
    pub flags: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_resolve_ib_service {
    pub id: __u32,
    pub reserved: __u32,
    pub ibs: rdma_ucm_ib_service,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ucm_write_cm_event {
    pub id: __u32,
    pub reserved: __u32,
    pub event: __u32,
    pub status: __u32,
    pub conn: rdma_ucm_conn_param,
    pub ud: rdma_ucm_ud_param,
    pub arg: __u64,
    pub param: },
}
