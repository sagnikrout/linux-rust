//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/projid.h
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
// A set of types for the internal kernel types representing project ids.
//
// The types defined in this header allow distinguishing which project ids in
// the kernel are values used by userspace and which project id values are
// the internal kernel values.  With the addition of user namespaces the values
// can be different.  Using the type system makes it possible for the compiler
// to detect when we overlook these differences.
//

pub type projid_t = __kernel_uid32_t;

pub const OVERFLOW_PROJID: c_int = 65534;
extern "C" {
    pub fn __kprojid_val(__kprojid_val(right: left) ==) -> return;
}
extern "C" {
    pub fn __kprojid_val(__kprojid_val(right: left) <) -> return;
}

extern "C" {
    pub fn make_kprojid(from: *mut user_namespace, projid: projid_t) -> kprojid_t;
}
extern "C" {
    pub fn from_kprojid(to: *mut user_namespace, projid: kprojid_t) -> projid_t;
}
extern "C" {
    pub fn from_kprojid_munged(to: *mut user_namespace, projid: kprojid_t) -> projid_t;
}

extern "C" {
    pub fn KPROJIDT_INIT(_arg: projid) -> return;
}
extern "C" {
    pub fn __kprojid_val(_arg: kprojid) -> return;
}

