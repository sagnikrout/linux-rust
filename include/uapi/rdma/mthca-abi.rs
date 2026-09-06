//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/mthca-abi.h
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
// Copyright (c) 2005 Topspin Communications.  All rights reserved.
// Copyright (c) 2005, 2006 Cisco Systems.  All rights reserved.
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

//
// Increment this value if any changes that break userspace ABI
// compatibility are made.
//
pub const MTHCA_UVERBS_ABI_VERSION: c_int = 1;
//
// Make sure that all structs defined in this file remain laid out so
// that they pack the same way on 32-bit and 64-bit architectures (to
// avoid incompatibility between 32-bit userspace and 64-bit kernels).
// In particular do not use pointer types -- pass pointers in __u64
// instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_alloc_ucontext_resp {
    pub qp_tab_size: __u32,
    pub uarc_size: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_alloc_pd_resp {
    pub pdn: __u32,
    pub reserved: __u32,
}

//
// Mark the memory region with a DMA attribute that causes
// in-flight DMA to be flushed when the region is written to:
//
pub const MTHCA_MR_DMASYNC: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_reg_mr {
    pub mr_attrs: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_create_cq {
    pub lkey: __u32,
    pub pdn: __u32,
    pub arm_db_page: __aligned_u64,
    pub set_db_page: __aligned_u64,
    pub arm_db_index: __u32,
    pub set_db_index: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_create_cq_resp {
    pub cqn: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_resize_cq {
    pub lkey: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_create_srq {
    pub lkey: __u32,
    pub db_index: __u32,
    pub db_page: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_create_srq_resp {
    pub srqn: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_create_qp {
    pub lkey: __u32,
    pub reserved: __u32,
    pub sq_db_page: __aligned_u64,
    pub rq_db_page: __aligned_u64,
    pub sq_db_index: __u32,
    pub rq_db_index: __u32,
}
