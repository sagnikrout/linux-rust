//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/bpf_filter.h
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
    pub fn __io_uring_run_bpf_filters(filters: *mut io_bpf_filter __rcu, req: *mut io_kiocb) -> c_int;
}
extern "C" {
    pub fn io_put_bpf_filters(res: *mut io_restriction);
}
extern "C" {
    pub fn io_bpf_filter_clone(dst: *mut io_restriction, src: *mut io_restriction);
}
extern "C" {
    pub fn __io_uring_run_bpf_filters(_arg: filters, _arg: req) -> return;
}

