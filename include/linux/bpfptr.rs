//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bpfptr.h
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
// A pointer that can point to either kernel or userspace memory.

pub type bpfptr_t = sockptr_t;
extern "C" {
    pub fn KERNEL_BPFPTR(addr: *mut *mut (void) (uintptr_t)) -> return;
}
extern "C" {
    pub fn USER_BPFPTR(_arg: u64_to_user_ptr(addr)) -> return;
}
extern "C" {
    pub fn copy_from_user(_arg: dst, offset: src.user +, _arg: size) -> return;
}
extern "C" {
    pub fn copy_from_kernel_nofault(_arg: dst, offset: src.kernel +, _arg: size) -> return;
}
extern "C" {
    pub fn copy_from_bpfptr_offset(_arg: dst, _arg: src, _arg: 0, _arg: size) -> return;
}
extern "C" {
    pub fn copy_to_sockptr_offset(dst: (sockptr_t), _arg: offset, _arg: src, _arg: size) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOMEM) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EFAULT) -> return;
}

extern "C" {
    pub fn strncpy_from_kernel_nofault(_arg: dst, _arg: src.kernel, _arg: count) -> return;
}
extern "C" {
    pub fn strncpy_from_user(_arg: dst, _arg: src.user, _arg: count) -> return;
}
