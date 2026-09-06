//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/qedr/verbs.h
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
extern "C" {
    pub fn qedr_query_pkey(ibdev: *mut ib_device, port: u32, index: u16, pkey: *mut u16) -> c_int;
}
extern "C" {
    pub fn qedr_alloc_ucontext(uctx: *mut ib_ucontext, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn qedr_dealloc_ucontext(uctx: *mut ib_ucontext);
}
extern "C" {
    pub fn qedr_mmap(ucontext: *mut ib_ucontext, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn qedr_mmap_free(rdma_entry: *mut rdma_user_mmap_entry);
}
extern "C" {
    pub fn qedr_alloc_pd(pd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn qedr_dealloc_pd(pd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn qedr_alloc_xrcd(ibxrcd: *mut ib_xrcd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn qedr_dealloc_xrcd(ibxrcd: *mut ib_xrcd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn qedr_destroy_cq(ibcq: *mut ib_cq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn qedr_arm_cq(ibcq: *mut ib_cq, flags: ib_cq_notify_flags) -> c_int;
}
extern "C" {
    pub fn qedr_destroy_qp(ibqp: *mut ib_qp, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn qedr_query_srq(ibsrq: *mut ib_srq, attr: *mut ib_srq_attr) -> c_int;
}
extern "C" {
    pub fn qedr_destroy_srq(ibsrq: *mut ib_srq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn qedr_destroy_ah(ibah: *mut ib_ah, flags: u32) -> c_int;
}
extern "C" {
    pub fn qedr_dereg_mr(ib_mr: *mut ib_mr, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn qedr_poll_cq(: *mut ib_cq, num_entries: c_int, wc: *mut ib_wc) -> c_int;
}
