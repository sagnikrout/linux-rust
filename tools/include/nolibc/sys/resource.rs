//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/sys/resource.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// Resource definitions for NOLIBC
// Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
//
// make sure to include all global symbols

//
// int getrlimit(int resource, struct rlimit *rlim);
// int setrlimit(int resource, const struct rlimit *rlim);
//
extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_prlimit64, _arg: pid, _arg: resource, _arg: new_limit, _arg: old_limit) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_prlimit64(0, _arg: resource, _arg: &rlim64, _arg: NULL)) -> return;
}
