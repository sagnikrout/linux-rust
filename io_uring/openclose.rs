//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/openclose.h
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
    pub fn io_openat_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_openat(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_open_cleanup(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_openat_bpf_populate(bctx: *mut io_uring_bpf_ctx, req: *mut io_kiocb);
}
extern "C" {
    pub fn io_openat2_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_openat2(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_close_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_close(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_pipe_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_pipe(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_install_fixed_fd_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_install_fixed_fd(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
