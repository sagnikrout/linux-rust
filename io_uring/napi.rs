//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/napi.h
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

extern "C" {
    pub fn io_napi_init(ctx: *mut io_ring_ctx);
}
extern "C" {
    pub fn io_napi_free(ctx: *mut io_ring_ctx);
}
extern "C" {
    pub fn io_register_napi(ctx: *mut io_ring_ctx, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn io_unregister_napi(ctx: *mut io_ring_ctx, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn __io_napi_busy_loop(ctx: *mut io_ring_ctx, iowq: *mut io_wait_queue);
}
extern "C" {
    pub fn io_napi_sqpoll_busy_poll(ctx: *mut io_ring_ctx) -> c_int;
}
//
// io_napi_add() - Add napi id to the busy poll list
// @req: pointer to io_kiocb request
//
// Add the napi id of the socket to the napi busy poll list and hash table.
//

