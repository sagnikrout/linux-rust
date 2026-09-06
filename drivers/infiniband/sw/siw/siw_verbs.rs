//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/siw/siw_verbs.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Authors: Bernard Metzler <bmt@zurich.ibm.com>
// Copyright (c) 2008-2019, IBM Corporation

//
// siw_copy_sgl()
//
// Copy SGL from RDMA core representation to local
// representation.
//
extern "C" {
    pub fn siw_alloc_ucontext(base_ctx: *mut ib_ucontext, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn siw_dealloc_ucontext(base_ctx: *mut ib_ucontext);
}
extern "C" {
    pub fn siw_alloc_pd(base_pd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn siw_dealloc_pd(base_pd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn siw_destroy_qp(base_qp: *mut ib_qp, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn siw_destroy_cq(base_cq: *mut ib_cq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn siw_poll_cq(base_cq: *mut ib_cq, num_entries: c_int, wc: *mut ib_wc) -> c_int;
}
extern "C" {
    pub fn siw_req_notify_cq(base_cq: *mut ib_cq, flags: ib_cq_notify_flags) -> c_int;
}
extern "C" {
    pub fn siw_dereg_mr(base_mr: *mut ib_mr, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn siw_query_srq(base_srq: *mut ib_srq, attr: *mut ib_srq_attr) -> c_int;
}
extern "C" {
    pub fn siw_destroy_srq(base_srq: *mut ib_srq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn siw_mmap(ctx: *mut ib_ucontext, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn siw_mmap_free(rdma_entry: *mut rdma_user_mmap_entry);
}
extern "C" {
    pub fn siw_qp_event(qp: *mut siw_qp, type: ib_event_type);
}
extern "C" {
    pub fn siw_cq_event(cq: *mut siw_cq, type: ib_event_type);
}
extern "C" {
    pub fn siw_srq_event(srq: *mut siw_srq, type: ib_event_type);
}
extern "C" {
    pub fn siw_port_event(dev: *mut siw_device, port: u32, type: ib_event_type);
}
