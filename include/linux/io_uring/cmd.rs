//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/io_uring/cmd.h
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

// only top 8 bits of sqe->uring_cmd_flags for kernel internal use

// io_uring_cmd is being issued again

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_cmd {
    pub file: *mut file,
    pub sqe: *const io_uring_sqe,
    pub cmd_op: u32,
    pub flags: u32,
    pub /: *mut *mut u8 pdu[32]; / available inline for free use,
    pub unused: [u8; 8],
}

//
// Completes the request, i.e. posts an io_uring CQE and deallocates @ioucmd
// and the corresponding io_uring request.
//
// Note: the caller should never hard code @issue_flags and is only allowed
// to pass the mask provided by the core io_uring code.
//
// Note: the caller should never hard code @issue_flags and only use the
// mask provided by the core io_uring code.
//
// Execute the request from a blocking context
extern "C" {
    pub fn io_uring_cmd_issue_blocking(ioucmd: *mut io_uring_cmd);
}
//
// Select a buffer from the provided buffer group for multishot uring_cmd.
// Returns the selected buffer address and size.
//
// Complete a multishot uring_cmd event. This will post a CQE to the completion
// queue and update the provided buffer.
//

extern "C" {
    pub fn io_kiocb_to_cmd(_arg: tw_req.req, io_uring_cmd: struct) -> return;
}
// task_work executor checks the deferred list completion

// users must follow the IOU_F_TWQ_LAZY_WAKE semantics
//
// Return uring_cmd's context reference as its context handle for driver to
// track per-context resource, such as registered kernel IO buffer
//
extern "C" {
    pub fn __io_uring_cmd_done(_arg: ioucmd, _arg: ret, _arg: 0, _arg: issue_flags, _arg: false) -> return;
}
extern "C" {
    pub fn __io_uring_cmd_done(_arg: ioucmd, _arg: ret, _arg: res2, _arg: issue_flags, _arg: true) -> return;
}
