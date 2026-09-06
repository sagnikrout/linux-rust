//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/irdma-abi.h
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


// SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB
//
// Copyright (c) 2006 - 2021 Intel Corporation.  All rights reserved.
// Copyright (c) 2005 Topspin Communications.  All rights reserved.
// Copyright (c) 2005 Cisco Systems.  All rights reserved.
// Copyright (c) 2005 Open Grid Computing, Inc. All rights reserved.
//

// irdma must support legacy GEN_1 i40iw kernel
// and user-space whose last ABI ver is 5
//
pub const IRDMA_ABI_VER: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_memreg_type {
    IRDMA_MEMREG_TYPE_MEM  = 0,
    IRDMA_MEMREG_TYPE_QP   = 1,
    IRDMA_MEMREG_TYPE_CQ   = 2,
    IRDMA_MEMREG_TYPE_SRQ  = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_alloc_ucontext_req {
    pub rsvd32: __u32,
    pub userspace_ver: __u8,
    pub rsvd8: [__u8; 3],
    pub comp_mask: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_alloc_ucontext_resp {
    pub max_pds: __u32,
    pub max_qps: __u32,
    pub /: *mut *mut __u32 wq_size; / size of the WQs (SQ+RQ) in the mmaped area,
    pub kernel_ver: __u8,
    pub rsvd: [__u8; 3],
    pub feature_flags: __aligned_u64,
    pub db_mmap_key: __aligned_u64,
    pub max_hw_wq_frags: __u32,
    pub max_hw_read_sges: __u32,
    pub max_hw_inline: __u32,
    pub max_hw_rq_quanta: __u32,
    pub max_hw_wq_quanta: __u32,
    pub min_hw_cq_size: __u32,
    pub max_hw_cq_size: __u32,
    pub max_hw_sq_chunk: __u16,
    pub hw_rev: __u8,
    pub rsvd2: __u8,
    pub comp_mask: __aligned_u64,
    pub min_hw_wq_size: __u16,
    pub revd3: [__u8; 2],
    pub max_hw_srq_quanta: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_alloc_pd_resp {
    pub pd_id: __u32,
    pub rsvd: [__u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_resize_cq_req {
    pub user_cq_buffer: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_create_cq_req {
    pub user_cq_buf: __aligned_u64,
    pub user_shadow_area: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_create_srq_req {
    pub user_srq_buf: __aligned_u64,
    pub user_shadow_area: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_create_srq_resp {
    pub srq_id: __u32,
    pub srq_size: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_create_qp_req {
    pub user_wqe_bufs: __aligned_u64,
    pub user_compl_ctx: __aligned_u64,
    pub legacy_dontuse: [__aligned_u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_mem_reg_req {
    pub /: *mut *mut __u16 reg_type; / enum irdma_memreg_type,
    pub cq_pages: __u16,
    pub rq_pages: __u16,
    pub sq_pages: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_modify_qp_req {
    pub sq_flush: __u8,
    pub rq_flush: __u8,
    pub rsvd: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_create_cq_resp {
    pub cq_id: __u32,
    pub cq_size: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_create_qp_resp {
    pub qp_id: __u32,
    pub actual_sq_size: __u32,
    pub actual_rq_size: __u32,
    pub irdma_drv_opt: __u32,
    pub push_idx: __u16,
    pub lsmm: __u8,
    pub rsvd: __u8,
    pub qp_caps: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_modify_qp_resp {
    pub push_wqe_mmap_key: __aligned_u64,
    pub push_db_mmap_key: __aligned_u64,
    pub push_offset: __u16,
    pub push_valid: __u8,
    pub rsvd: [__u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_create_ah_resp {
    pub ah_id: __u32,
    pub rsvd: [__u8; 4],
}
