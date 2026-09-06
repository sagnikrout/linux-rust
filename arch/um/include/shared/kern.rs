//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/shared/kern.h
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
// Copyright (C) 2000 Jeff Dike (jdike@karaya.com)
//
// These are all user-mode things which are convenient to call directly
// from kernel code and for which writing a wrapper is too much of a pain.
// The regular include files can't be included because this file is included
// only into kernel code, and user-space includes conflict with kernel
// includes.
//
extern "C" {
    pub fn printf(fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn pause() -> c_int;
}
extern "C" {
    pub fn exit(_arg: c_int);
}
