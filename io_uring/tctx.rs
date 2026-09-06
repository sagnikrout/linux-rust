//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/tctx.h
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
pub struct io_tctx_node {
    pub ctx_node: list_head,
    pub task: *mut task_struct,
    pub ctx: *mut io_ring_ctx,
}

extern "C" {
    pub fn io_uring_del_tctx_node(index: c_ulong);
}
extern "C" {
    pub fn __io_uring_add_tctx_node(ctx: *mut io_ring_ctx) -> c_int;
}
extern "C" {
    pub fn __io_uring_add_tctx_node_from_submit(ctx: *mut io_ring_ctx) -> c_int;
}
extern "C" {
    pub fn io_uring_clean_tctx(tctx: *mut io_uring_task);
}
extern "C" {
    pub fn io_uring_free_tctx(tsk: *mut task_struct);
}
extern "C" {
    pub fn io_uring_unreg_ringfd();
}
//
// Note that this task has used io_uring. We use it for cancelation purposes.
//
extern "C" {
    pub fn __io_uring_add_tctx_node_from_submit(_arg: ctx) -> return;
}
