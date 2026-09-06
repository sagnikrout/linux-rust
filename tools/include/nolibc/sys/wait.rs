//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/sys/wait.h
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
// wait definitions for NOLIBC
// Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
//
// make sure to include all global symbols

//
// pid_t wait(int *status);
// pid_t waitpid(pid_t pid, int *status, int options);
// int waitid(idtype_t idtype, id_t id, siginfo_t *infop, int options);
//
extern "C" {
    pub fn __nolibc_syscall5(_arg: __NR_waitid, _arg: which, _arg: pid, _arg: infop, _arg: options, _arg: rusage) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_waitid(which, _arg: pid, _arg: infop, _arg: options, _arg: NULL)) -> return;
}
// status = 0;
// status = (info.si_status & 0xff) << 8;
// status = info.si_status & 0x7f;
// status = (info.si_status & 0x7f) | 0x80;
// status = (info.si_status << 8) + 0x7f;
// status = 0xffff;
extern "C" {
    pub fn waitpid(_arg: -1, _arg: status, _arg: 0) -> return;
}
