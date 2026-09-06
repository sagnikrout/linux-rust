//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/hns-abi.h
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
// Copyright (c) 2016 Hisilicon Limited.
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
#[derive(Copy, Clone)]
pub struct hns_roce_ib_create_cq {
    pub buf_addr: __aligned_u64,
    pub db_addr: __aligned_u64,
    pub cqe_size: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_cq_cap_flags {
    HNS_ROCE_CQ_FLAG_RECORD_DB = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ib_create_cq_resp {
    pub /: *mut *mut __aligned_u64 cqn; / Only 32 bits used, 64 for compat,
    pub cap_flags: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_srq_cap_flags {
    HNS_ROCE_SRQ_CAP_RECORD_DB = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_srq_cap_flags_resp {
    HNS_ROCE_RSP_SRQ_CAP_RECORD_DB = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ib_create_srq {
    pub buf_addr: __aligned_u64,
    pub db_addr: __aligned_u64,
    pub que_addr: __aligned_u64,
    pub /: *mut *mut __u32 req_cap_flags; / Use enum hns_roce_srq_cap_flags,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ib_create_srq_resp {
    pub srqn: __u32,
    pub /: *mut *mut __u32 cap_flags; / Use enum hns_roce_srq_cap_flags,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_congest_type_flags {
    HNS_ROCE_CREATE_QP_FLAGS_DCQCN,
    HNS_ROCE_CREATE_QP_FLAGS_LDCP,
    HNS_ROCE_CREATE_QP_FLAGS_HC3,
    HNS_ROCE_CREATE_QP_FLAGS_DIP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_create_qp_comp_mask {
    HNS_ROCE_CREATE_QP_MASK_CONGEST_TYPE = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ib_create_qp {
    pub buf_addr: __aligned_u64,
    pub db_addr: __aligned_u64,
    pub log_sq_bb_count: __u8,
    pub log_sq_stride: __u8,
    pub sq_no_prefetch: __u8,
    pub reserved: [__u8; 5],
    pub sdb_addr: __aligned_u64,
    pub /: *mut *mut __aligned_u64 comp_mask; / Use enum hns_roce_create_qp_comp_mask,
    pub create_flags: __aligned_u64,
    pub cong_type_flags: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_qp_cap_flags {
    HNS_ROCE_QP_CAP_RQ_RECORD_DB = 1 << 0,
    HNS_ROCE_QP_CAP_SQ_RECORD_DB = 1 << 1,
    HNS_ROCE_QP_CAP_OWNER_DB = 1 << 2,
    HNS_ROCE_QP_CAP_DIRECT_WQE = 1 << 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ib_create_qp_resp {
    pub cap_flags: __aligned_u64,
    pub dwqe_mmap_key: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ib_modify_qp_resp {
    pub tc_mode: __u8,
    pub priority: __u8,
    pub reserved: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ib_alloc_ucontext_resp {
    pub qp_tab_size: __u32,
    pub cqe_size: __u32,
    pub srq_tab_size: __u32,
    pub reserved: __u32,
    pub config: __u32,
    pub max_inline_data: __u32,
    pub congest_type: __u8,
    pub reserved0: [__u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ib_alloc_ucontext {
    pub config: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ib_alloc_pd_resp {
    pub pdn: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ib_create_ah_resp {
    pub dmac: [__u8; 6],
    pub priority: __u8,
    pub tc_mode: __u8,
}
