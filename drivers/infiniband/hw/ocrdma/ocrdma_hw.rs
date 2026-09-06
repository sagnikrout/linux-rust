//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/ocrdma/ocrdma_hw.h
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

// (dst_ptr + i) = cpu_to_le32p(src_ptr + i);

// (dst_ptr + i) = le32_to_cpu(*(src_ptr + i));

// (dst_ptr + i) = cpu_to_le32p(src_ptr + i);

// (dst_ptr + i) = le32_to_cpu(*(src_ptr + i));

extern "C" {
    pub fn ocrdma_init_hw(: *mut ocrdma_dev) -> c_int;
}
extern "C" {
    pub fn ocrdma_cleanup_hw(: *mut ocrdma_dev);
}
extern "C" {
    pub fn get_ibqp_state(qps: ocrdma_qp_state) -> ib_qp_state;
}
// verbs specific mailbox commands
extern "C" {
    pub fn ocrdma_mbx_alloc_pd(: *mut ocrdma_dev, : *mut ocrdma_pd) -> c_int;
}
extern "C" {
    pub fn ocrdma_mbx_dealloc_pd(: *mut ocrdma_dev, : *mut ocrdma_pd) -> c_int;
}
extern "C" {
    pub fn ocrdma_mbx_dealloc_lkey(: *mut ocrdma_dev, fmr: c_int, lkey: u32) -> c_int;
}
extern "C" {
    pub fn ocrdma_mbx_destroy_cq(dev: *mut ocrdma_dev, cq: *mut ocrdma_cq);
}
extern "C" {
    pub fn ocrdma_mbx_destroy_qp(: *mut ocrdma_dev, : *mut ocrdma_qp) -> c_int;
}
extern "C" {
    pub fn ocrdma_mbx_modify_srq(: *mut ocrdma_srq, : *mut ib_srq_attr) -> c_int;
}
extern "C" {
    pub fn ocrdma_mbx_query_srq(: *mut ocrdma_srq, : *mut ib_srq_attr) -> c_int;
}
extern "C" {
    pub fn ocrdma_mbx_destroy_srq(dev: *mut ocrdma_dev, srq: *mut ocrdma_srq);
}
extern "C" {
    pub fn ocrdma_alloc_av(dev: *mut ocrdma_dev, ah: *mut ocrdma_ah) -> c_int;
}
extern "C" {
    pub fn ocrdma_free_av(dev: *mut ocrdma_dev, ah: *mut ocrdma_ah);
}
extern "C" {
    pub fn ocrdma_is_qp_in_sq_flushlist(: *mut ocrdma_cq, : *mut ocrdma_qp) -> bool;
}
extern "C" {
    pub fn ocrdma_is_qp_in_rq_flushlist(: *mut ocrdma_cq, : *mut ocrdma_qp) -> bool;
}
extern "C" {
    pub fn ocrdma_flush_qp(: *mut ocrdma_qp);
}
extern "C" {
    pub fn ocrdma_get_irq(dev: *mut ocrdma_dev, eq: *mut ocrdma_eq) -> c_int;
}
extern "C" {
    pub fn ocrdma_mbx_rdma_stats(: *mut ocrdma_dev, reset: bool) -> c_int;
}
extern "C" {
    pub fn ocrdma_init_service_level(: *mut ocrdma_dev);
}
extern "C" {
    pub fn ocrdma_alloc_pd_pool(dev: *mut ocrdma_dev);
}
extern "C" {
    pub fn ocrdma_free_pd_range(dev: *mut ocrdma_dev);
}
extern "C" {
    pub fn ocrdma_update_link_state(dev: *mut ocrdma_dev, lstate: u8);
}
