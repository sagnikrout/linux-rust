//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/poll.h
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

pub const IO_POLL_ALLOC_CACHE_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_poll {
    pub file: *mut file,
    pub head: *mut wait_queue_head,
    pub events: __poll_t,
    pub retries: c_int,
    pub wait: wait_queue_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct async_poll {
    pub poll: io_poll,
    pub double_poll: *mut io_poll,
}

//
// Must only be called inside issue_flags & IO_URING_F_MULTISHOT, or
// potentially other cases where we already "own" this poll request.
//
extern "C" {
    pub fn io_poll_add_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_poll_add(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_poll_remove_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_poll_remove(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_arm_apoll(req: *mut io_kiocb, issue_flags: unsigned, mask: __poll_t) -> c_int;
}
extern "C" {
    pub fn io_arm_poll_handler(req: *mut io_kiocb, issue_flags: unsigned) -> c_int;
}
extern "C" {
    pub fn io_poll_task_func(tw_req: io_tw_req, tw: io_tw_token_t);
}
