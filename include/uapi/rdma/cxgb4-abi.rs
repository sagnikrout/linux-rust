//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/cxgb4-abi.h
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
// Copyright (c) 2009-2010 Chelsio, Inc. All rights reserved.
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

pub const C4IW_UVERBS_ABI_VERSION: c_int = 3;
//
// Make sure that all structs defined in this file remain laid out so
// that they pack the same way on 32-bit and 64-bit architectures (to
// avoid incompatibility between 32-bit userspace and 64-bit kernels).
// In particular do not use pointer types -- pass pointers in __aligned_u64
// instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_create_cq {
    pub flags: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_create_cq_resp {
    pub key: __aligned_u64,
    pub gts_key: __aligned_u64,
    pub memsize: __aligned_u64,
    pub cqid: __u32,
    pub size: __u32,
    pub qid_mask: __u32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_create_qp_resp {
    pub ma_sync_key: __aligned_u64,
    pub sq_key: __aligned_u64,
    pub rq_key: __aligned_u64,
    pub sq_db_gts_key: __aligned_u64,
    pub rq_db_gts_key: __aligned_u64,
    pub sq_memsize: __aligned_u64,
    pub rq_memsize: __aligned_u64,
    pub sqid: __u32,
    pub rqid: __u32,
    pub sq_size: __u32,
    pub rq_size: __u32,
    pub qid_mask: __u32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_create_srq_resp {
    pub srq_key: __aligned_u64,
    pub srq_db_gts_key: __aligned_u64,
    pub srq_memsize: __aligned_u64,
    pub srqid: __u32,
    pub srq_size: __u32,
    pub rqt_abs_idx: __u32,
    pub qid_mask: __u32,
    pub flags: __u32,
    pub /: *mut *mut __u32 reserved; / explicit padding,
}

// HW supports SRQ_LIMIT_REACHED event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_alloc_ucontext_resp {
    pub status_page_key: __aligned_u64,
    pub status_page_size: __u32,
    pub /: *mut *mut __u32 reserved; / explicit padding (optional for i386),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4iw_alloc_pd_resp {
    pub pdid: __u32,
}
