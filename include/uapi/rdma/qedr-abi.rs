//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/qedr-abi.h
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
// QLogic qedr NIC Driver
// Copyright (c) 2015-2016  QLogic Corporation
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
// disclaimer in the documentation and /or other materials
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

// user kernel communication data structures.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qedr_alloc_ucontext_flags {
    QEDR_ALLOC_UCTX_EDPM_MODE	= 1 << 0,
    QEDR_ALLOC_UCTX_DB_REC		= 1 << 1,
    QEDR_SUPPORT_DPM_SIZES		= 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_alloc_ucontext_req {
    pub context_flags: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qedr_rdma_dpm_type {
    QEDR_DPM_TYPE_NONE		= 0,
    QEDR_DPM_TYPE_ROCE_ENHANCED	= 1 << 0,
    QEDR_DPM_TYPE_ROCE_LEGACY	= 1 << 1,
    QEDR_DPM_TYPE_IWARP_LEGACY	= 1 << 2,
    QEDR_DPM_TYPE_ROCE_EDPM_MODE	= 1 << 3,
    QEDR_DPM_SIZES_SET		= 1 << 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_alloc_ucontext_resp {
    pub db_pa: __aligned_u64,
    pub db_size: __u32,
    pub max_send_wr: __u32,
    pub max_recv_wr: __u32,
    pub max_srq_wr: __u32,
    pub sges_per_send_wr: __u32,
    pub sges_per_recv_wr: __u32,
    pub sges_per_srq_wr: __u32,
    pub max_cqes: __u32,
    pub dpm_flags: __u8,
    pub wids_enabled: __u8,
    pub wid_count: __u16,
    pub ldpm_limit_size: __u16,
    pub edpm_trans_size: __u8,
    pub reserved: __u8,
    pub edpm_limit_size: __u16,
    pub padding: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_alloc_pd_ureq {
    pub rsvd1: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_alloc_pd_uresp {
    pub pd_id: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_create_cq_ureq {
    pub addr: __aligned_u64,
    pub len: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_create_cq_uresp {
    pub db_offset: __u32,
    pub icid: __u16,
    pub reserved: __u16,
    pub db_rec_addr: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_create_qp_ureq {
    pub qp_handle_hi: __u32,
    pub qp_handle_lo: __u32,
// SQ
// user space virtual address of SQ buffer
    pub sq_addr: __aligned_u64,
// length of SQ buffer
    pub sq_len: __aligned_u64,
// RQ
// user space virtual address of RQ buffer
    pub rq_addr: __aligned_u64,
// length of RQ buffer
    pub rq_len: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_create_qp_uresp {
    pub qp_id: __u32,
    pub atomic_supported: __u32,
// SQ
    pub sq_db_offset: __u32,
    pub sq_icid: __u16,
// RQ
    pub rq_db_offset: __u32,
    pub rq_icid: __u16,
    pub rq_db2_offset: __u32,
    pub reserved: __u32,
// address of SQ doorbell recovery user entry
    pub sq_db_rec_addr: __aligned_u64,
// address of RQ doorbell recovery user entry
    pub rq_db_rec_addr: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_create_srq_ureq {
// user space virtual address of producer pair
    pub prod_pair_addr: __aligned_u64,
// user space virtual address of SRQ buffer
    pub srq_addr: __aligned_u64,
// length of SRQ buffer
    pub srq_len: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_create_srq_uresp {
    pub srq_id: __u16,
    pub reserved0: __u16,
    pub reserved1: __u32,
}

// doorbell recovery entry allocated and populated by userspace doorbelling
// entities and mapped to kernel. Kernel uses this to register doorbell
// information with doorbell drop recovery mechanism.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_user_db_rec {
    pub /: *mut *mut __aligned_u64 db_data; / doorbell data,
}
