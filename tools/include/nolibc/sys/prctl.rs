//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/sys/prctl.h
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
// Prctl definitions for NOLIBC
// Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
//
// make sure to include all global symbols

//
// int prctl(int option, unsigned long arg2, unsigned long arg3,
// unsigned long arg4, unsigned long arg5);
//
extern "C" {
    pub fn __nolibc_syscall5(_arg: __NR_prctl, _arg: option, _arg: arg2, _arg: arg3, _arg: arg4, _arg: arg5) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_prctl(option, _arg: arg2, _arg: arg3, _arg: arg4, _arg: arg5)) -> return;
}
