//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/timeout.h
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
pub struct io_timeout_data {
    pub req: *mut io_kiocb,
    pub timer: hrtimer,
    pub time: ktime_t,
    pub mode: hrtimer_mode,
    pub flags: u32,
}

extern "C" {
    pub fn io_flush_timeouts(ctx: *mut io_ring_ctx) -> __cold void;
}
extern "C" {
    pub fn io_timeout_cancel(ctx: *mut io_ring_ctx, cd: *mut io_cancel_data) -> c_int;
}
extern "C" {
    pub fn io_queue_linked_timeout(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_disarm_next(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_timeout_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_link_timeout_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_timeout(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_timeout_remove_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_timeout_remove(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
