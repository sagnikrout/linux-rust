//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bsg-lib.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// BSG helper library
//
// Copyright (C) 2008   James Smart, Emulex Corporation
// Copyright (C) 2011   Red Hat, Inc.  All rights reserved.
// Copyright (C) 2011   Mike Christie
//

extern "C" {
    pub fn int(: *mut bsg_job_fn) (struct bsg_job) -> typedef;
}
extern "C" {
    pub fn blk_eh_timer_return(: *mut bsg_timeout_fn)(struct request) -> typedef enum;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsg_buffer {
    pub payload_len: c_uint,
    pub sg_cnt: c_int,
    pub sg_list: *mut scatterlist,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsg_job {
    pub dev: *mut device,
    pub kref: kref,
    pub timeout: c_uint,
// Transport/driver specific request/reply structs
    pub request: *mut c_void,
    pub reply: *mut c_void,
    pub request_len: c_uint,
    pub reply_len: c_uint,
//
// On entry : reply_len indicates the buffer size allocated for
// the reply.
//
// Upon completion : the message handler must set reply_len
// to indicates the size of the reply to be returned to the
// caller.
//
// DMA payloads for the request/response
    pub request_payload: bsg_buffer,
    pub reply_payload: bsg_buffer,
    pub result: c_int,
    pub reply_payload_rcv_len: c_uint,
// BIDI support
    pub bidi_rq: *mut request,
    pub bidi_bio: *mut bio,
    pub /: *mut *mut *mut void dd_data; / Used for driver-specific storage,
}

extern "C" {
    pub fn bsg_remove_queue(q: *mut request_queue);
}
extern "C" {
    pub fn bsg_job_put(job: *mut bsg_job);
}
extern "C" {
    pub fn bsg_job_get(job: *mut bsg_job) -> int __must_check;
}
