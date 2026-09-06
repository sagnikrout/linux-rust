//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/syscore_ops.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// syscore_ops.h - System core operations.
//
// Copyright (C) 2011 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscore_ops {
    pub data): *mut *mut int (suspend)(void,
    pub data): *mut *mut void (resume)(void,
    pub data): *mut *mut void (shutdown)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscore {
    pub node: list_head,
    pub ops: *const syscore_ops,
    pub data: *mut c_void,
}

extern "C" {
    pub fn register_syscore(syscore: *mut syscore);
}
extern "C" {
    pub fn unregister_syscore(syscore: *mut syscore);
}

extern "C" {
    pub fn syscore_suspend() -> c_int;
}
extern "C" {
    pub fn syscore_resume();
}

extern "C" {
    pub fn syscore_shutdown();
}
