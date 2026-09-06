//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/efa-abi.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-2-Clause)
//
// Copyright 2018-2026 Amazon.com, Inc. or its affiliates. All rights reserved.
//

//
// Increment this value if any changes that break userspace ABI
// compatibility are made.
//
pub const EFA_UVERBS_ABI_VERSION: c_int = 1;
//
// Keep structs aligned to 8 bytes.
// Keep reserved fields as arrays of __u8 named reserved_XXX where XXX is the
// hex bit offset of the field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ibv_alloc_ucontext_cmd {
    pub supported_caps: __u32,
    pub reserved_20: [__u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_ibv_user_cmds_supp_udata {
    EFA_USER_CMDS_SUPP_UDATA_QUERY_DEVICE = 1 << 0,
    EFA_USER_CMDS_SUPP_UDATA_CREATE_AH    = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ibv_alloc_ucontext_resp {
    pub comp_mask: __u32,
    pub cmds_supp_udata_mask: __u32,
    pub sub_cqs_per_cq: __u16,
    pub inline_buf_size: __u16,
    pub /: *mut *mut __u32 max_llq_size; / bytes,
    pub /: *mut *mut __u16 max_tx_batch; / units of 64 bytes,
    pub min_sq_wr: __u16,
    pub inline_buf_size_ex: __u16,
    pub reserved_b0: [__u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ibv_alloc_pd_resp {
    pub comp_mask: __u32,
    pub pdn: __u16,
    pub reserved_30: [__u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ibv_create_cq {
    pub comp_mask: __u32,
    pub cq_entry_size: __u32,
    pub num_sub_cqs: __u16,
    pub flags: __u8,
    pub reserved_58: [__u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ibv_create_cq_resp {
    pub comp_mask: __u32,
    pub reserved_20: [__u8; 4],
    pub q_mmap_key: __aligned_u64,
    pub q_mmap_size: __aligned_u64,
    pub cq_idx: __u16,
    pub reserved_d0: [__u8; 2],
    pub db_off: __u32,
    pub db_mmap_key: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ibv_create_qp {
    pub comp_mask: __u32,
    pub /: *mut *mut __u32 rq_ring_size; / bytes,
    pub /: *mut *mut __u32 sq_ring_size; / bytes,
    pub driver_qp_type: __u32,
    pub flags: __u16,
    pub sl: __u8,
    pub reserved_98: [__u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ibv_create_qp_resp {
    pub comp_mask: __u32,
// the offset inside the page of the rq db
    pub rq_db_offset: __u32,
// the offset inside the page of the sq db
    pub sq_db_offset: __u32,
// the offset inside the page of descriptors buffer
    pub llq_desc_offset: __u32,
    pub rq_mmap_key: __aligned_u64,
    pub rq_mmap_size: __aligned_u64,
    pub rq_db_mmap_key: __aligned_u64,
    pub sq_db_mmap_key: __aligned_u64,
    pub llq_desc_mmap_key: __aligned_u64,
    pub send_sub_cq_idx: __u16,
    pub recv_sub_cq_idx: __u16,
    pub reserved_1e0: [__u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ibv_create_ah_resp {
    pub comp_mask: __u32,
    pub efa_address_handle: __u16,
    pub reserved_30: [__u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ibv_ex_query_device_resp {
    pub comp_mask: __u32,
    pub max_sq_wr: __u32,
    pub max_rq_wr: __u32,
    pub max_sq_sge: __u16,
    pub max_rq_sge: __u16,
    pub max_rdma_size: __u32,
    pub device_caps: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_query_mr_attrs {
    EFA_IB_ATTR_QUERY_MR_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    EFA_IB_ATTR_QUERY_MR_RESP_IC_ID_VALIDITY,
    EFA_IB_ATTR_QUERY_MR_RESP_RECV_IC_ID,
    EFA_IB_ATTR_QUERY_MR_RESP_RDMA_READ_IC_ID,
    EFA_IB_ATTR_QUERY_MR_RESP_RDMA_RECV_IC_ID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_mr_methods {
    EFA_IB_METHOD_MR_QUERY = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_comp_cntr_create_attrs {
    EFA_IB_ATTR_CREATE_COMP_CNTR_COMP_BUFFER = (1U << UVERBS_ID_NS_SHIFT),
    EFA_IB_ATTR_CREATE_COMP_CNTR_ERR_BUFFER,
}
