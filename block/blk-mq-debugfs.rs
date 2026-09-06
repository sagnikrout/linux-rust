//! Automatically rewritten from C Header to Rust Module
//! Source: block/blk-mq-debugfs.h
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
pub struct blk_mq_debugfs_attr {
    pub name: *const c_char,
    pub mode: umode_t,
    pub ): *mut *mut *mut int (show)(void , struct seq_file,
    pub ): *const *const *const *const ssize_t (write)(void , char __user , size_t, loff_t,
// Set either .show or .seq_ops.
    pub seq_ops: *const seq_operations,
}

extern "C" {
    pub fn __blk_mq_debugfs_rq_show(m: *mut seq_file, rq: *mut request) -> c_int;
}
extern "C" {
    pub fn blk_mq_debugfs_rq_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn blk_mq_debugfs_register(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_debugfs_unregister_hctx(hctx: *mut blk_mq_hw_ctx);
}
extern "C" {
    pub fn blk_mq_debugfs_register_hctxs(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_debugfs_unregister_hctxs(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_debugfs_register_sched(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_debugfs_unregister_sched(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_debugfs_unregister_sched_hctx(hctx: *mut blk_mq_hw_ctx);
}
extern "C" {
    pub fn blk_mq_debugfs_register_rq_qos(q: *mut request_queue);
}

extern "C" {
    pub fn queue_zone_wplugs_show(data: *mut c_void, m: *mut seq_file) -> c_int;
}

