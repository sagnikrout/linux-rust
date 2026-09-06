//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/shared/skas/mm_id.h
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
//
// Copyright (C) 2005 Jeff Dike (jdike@karaya.com)
//

pub const STUB_MAX_FDS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_id {
    pub pid: c_int,
    pub stack: c_ulong,
    pub syscall_data_len: c_int,
// Only used with SECCOMP mode
    pub sock: c_int,
    pub syscall_fd_num: c_int,
    pub syscall_fd_map: [c_int; STUB_MAX_FDS],
}

extern "C" {
    pub fn enter_turnstile(__acquires(__get_turnstile(mm_id): *mut *mut mm_id mm_id));
}
extern "C" {
    pub fn exit_turnstile(__releases(__get_turnstile(mm_id): *mut *mut mm_id mm_id));
}
extern "C" {
    pub fn notify_mm_kill(pid: c_int);
}
