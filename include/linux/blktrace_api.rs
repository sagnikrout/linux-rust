//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/blktrace_api.h
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


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_trace {
    pub version: c_int,
    pub trace_state: c_int,
    pub rchan: *mut rchan,
    pub sequence: *mut unsigned long __percpu,
    pub msg_data: *mut unsigned char __percpu,
    pub act_mask: u64,
    pub start_lba: u64,
    pub end_lba: u64,
    pub pid: u32,
    pub dev: u32,
    pub dir: *mut dentry,
    pub running_list: list_head,
    pub dropped: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn blk_trace_ioctl(: *mut block_device, _arg: unsigned, : *mut char __user) -> c_int;
}
extern "C" {
    pub fn blk_trace_shutdown(: *mut request_queue);
}
//
// blk_add_trace_msg - Add a (simple) message to the blktrace stream
// @q:		queue the io is for
// @fmt:	format to print message in
// args...	Variable argument list for format
//
// Description:
// Records a (simple) message onto the blktrace stream.
//
// NOTE: BLK_TN_MAX_MSG characters are output at most.
// NOTE: Can not use 'static inline' due to presence of var args...
//

pub const BLK_TN_MAX_MSG: c_int = 128;
extern "C" {
    pub fn blk_add_driver_data(rq: *mut request, data: *mut c_void, len: usize);
}
extern "C" {
    pub fn blk_trace_startstop(q: *mut request_queue, start: c_int) -> c_int;
}
extern "C" {
    pub fn blk_trace_remove(q: *mut request_queue) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_blk_user_trace_setup {
    pub name: [c_char; BLKTRACE_BDEV_SIZE],
    pub act_mask: u16,
    pub buf_size: u32,
    pub buf_nr: u32,
    pub start_lba: compat_u64,
    pub end_lba: compat_u64,
    pub pid: u32,
}

extern "C" {
    pub fn blk_fill_rwbs(rwbs: *mut c_char, opf: blk_opf_t);
}
//
// Tracing should ignore starting sector for passthrough requests and
// requests where starting sector didn't get set.
//
extern "C" {
    pub fn blk_rq_pos(_arg: rq) -> return;
}
extern "C" {
    pub fn blk_rq_is_passthrough(blk_rq_sectors(rq: rq) ? 0 :) -> return;
}
