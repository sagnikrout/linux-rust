//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/ocrdma-abi.h
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
// This file is part of the Emulex RoCE Device Driver for
// RoCE (RDMA over Converged Ethernet) adapters.
// Copyright (C) 2012-2015 Emulex. All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.emulex.com
//
// This software is available to you under a choice of one of two licenses.
// You may choose to be licensed under the terms of the GNU General Public
// License (GPL) Version 2, available from the file COPYING in the main
// directory of this source tree, or the BSD license below:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// - Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// - Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
// BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
// OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
// ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Contact Information:
// linux-drivers@emulex.com
//
// Emulex
// 3333 Susan Street
// Costa Mesa, CA 92626
//

pub const OCRDMA_ABI_VERSION: c_int = 2;
pub const OCRDMA_BE_ROCE_ABI_VERSION: c_int = 1;
// user kernel communication data structures.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_alloc_ucontext_resp {
    pub dev_id: __u32,
    pub wqe_size: __u32,
    pub max_inline_data: __u32,
    pub dpp_wqe_size: __u32,
    pub ah_tbl_page: __aligned_u64,
    pub ah_tbl_len: __u32,
    pub rqe_size: __u32,
    pub fw_ver: [__u8; 32],
// for future use/new features in progress
    pub rsvd1: __aligned_u64,
    pub rsvd2: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_alloc_pd_ureq {
    pub rsvd: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_alloc_pd_uresp {
    pub id: __u32,
    pub dpp_enabled: __u32,
    pub dpp_page_addr_hi: __u32,
    pub dpp_page_addr_lo: __u32,
    pub rsvd: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_cq_ureq {
    pub dpp_cq: __u32,
    pub /: *mut *mut __u32 rsvd; / pad,
}

pub const MAX_CQ_PAGES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_cq_uresp {
    pub cq_id: __u32,
    pub page_size: __u32,
    pub num_pages: __u32,
    pub max_hw_cqe: __u32,
    pub page_addr: [__aligned_u64; MAX_CQ_PAGES],
    pub db_page_addr: __aligned_u64,
    pub db_page_size: __u32,
    pub phase_change: __u32,
// for future use/new features in progress
    pub rsvd1: __aligned_u64,
    pub rsvd2: __aligned_u64,
}

pub const MAX_QP_PAGES: c_int = 8;
pub const MAX_UD_AV_PAGES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_qp_ureq {
    pub enable_dpp_cq: __u8,
    pub rsvd: __u8,
    pub dpp_cq_id: __u16,
    pub /: *mut *mut __u32 rsvd1; / pad,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_qp_uresp {
    pub qp_id: __u16,
    pub sq_dbid: __u16,
    pub rq_dbid: __u16,
    pub /: *mut *mut __u16 resv0; / pad,
    pub sq_page_size: __u32,
    pub rq_page_size: __u32,
    pub num_sq_pages: __u32,
    pub num_rq_pages: __u32,
    pub sq_page_addr: [__aligned_u64; MAX_QP_PAGES],
    pub rq_page_addr: [__aligned_u64; MAX_QP_PAGES],
    pub db_page_addr: __aligned_u64,
    pub db_page_size: __u32,
    pub dpp_credit: __u32,
    pub dpp_offset: __u32,
    pub num_wqe_allocated: __u32,
    pub num_rqe_allocated: __u32,
    pub db_sq_offset: __u32,
    pub db_rq_offset: __u32,
    pub db_shift: __u32,
    pub rsvd: [__aligned_u64; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_create_srq_uresp {
    pub rq_dbid: __u16,
    pub /: *mut *mut __u16 resv0; / pad,
    pub resv1: __u32,
    pub rq_page_size: __u32,
    pub num_rq_pages: __u32,
    pub rq_page_addr: [__aligned_u64; MAX_QP_PAGES],
    pub db_page_addr: __aligned_u64,
    pub db_page_size: __u32,
    pub num_rqe_allocated: __u32,
    pub db_rq_offset: __u32,
    pub db_shift: __u32,
    pub rsvd2: __aligned_u64,
    pub rsvd3: __aligned_u64,
}
