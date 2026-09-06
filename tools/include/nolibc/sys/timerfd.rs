//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/sys/timerfd.h
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
// timerfd definitions for NOLIBC
// Copyright (C) 2025 Thomas Weißschuh <thomas.weissschuh@linutronix.de>
//
// make sure to include all global symbols

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_timerfd_create, _arg: clockid, _arg: flags) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_timerfd_create(clockid, _arg: flags)) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_timerfd_gettime64, _arg: fd, _arg: curr_value) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_timerfd_gettime, _arg: fd, _arg: curr_value) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_timerfd_gettime(fd, _arg: curr_value)) -> return;
}

extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_timerfd_settime64, _arg: fd, _arg: flags, _arg: new_value, _arg: old_value) -> return;
}

extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_timerfd_settime, _arg: fd, _arg: flags, _arg: new_value, _arg: old_value) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_timerfd_settime(fd, _arg: flags, _arg: new_value, _arg: old_value)) -> return;
}
