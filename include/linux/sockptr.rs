//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sockptr.h
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
// Copyright (c) 2020 Christoph Hellwig.
//
// Support for "universal" pointers that can point to either kernel or userspace
// memory.
//

extern "C" {
    pub fn copy_from_user(_arg: dst, offset: src.user +, _arg: size) -> return;
}
// Deprecated.
// This is unsafe, unless caller checked user provided optlen.
// Prefer copy_safe_from_sockptr() instead.
//
// Returns 0 for success, or number of bytes not copied on error.
//
extern "C" {
    pub fn copy_from_sockptr_offset(_arg: dst, _arg: src, _arg: 0, _arg: size) -> return;
}
//
// copy_safe_from_sockptr: copy a struct from sockptr
// @dst:   Destination address, in kernel space. This buffer must be @ksize
// bytes long.
// @ksize: Size of @dst struct.
// @optval: Source address. (in user or kernel space)
// @optlen: Size of @optval data.
//
// Returns:
// * -EINVAL: @optlen < @ksize
// * -EFAULT: access to userspace failed.
// * 0 : @ksize bytes were copied
//
extern "C" {
    pub fn copy_struct_from_user(_arg: dst, _arg: ksize, _arg: src.user, _arg: usize) -> return;
}
extern "C" {
    pub fn copy_struct_from_bounce_buffer(_arg: dst, _arg: ksize, _arg: src.kernel, _arg: usize) -> return;
}
extern "C" {
    pub fn copy_to_user(offset: dst.user +, _arg: src, _arg: size) -> return;
}
extern "C" {
    pub fn copy_to_sockptr_offset(_arg: dst, _arg: 0, _arg: src, _arg: size) -> return;
}
extern "C" {
    pub fn copy_struct_to_user(_arg: dst.user, _arg: usize, _arg: src, _arg: ksize, _arg: ignored_trailing) -> return;
}
extern "C" {
    pub fn copy_struct_to_bounce_buffer(_arg: dst.kernel, _arg: usize, _arg: src, _arg: ksize, _arg: ignored_trailing) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOMEM) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EFAULT) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOMEM) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EFAULT) -> return;
}

extern "C" {
    pub fn strncpy_from_user(_arg: dst, _arg: src.user, _arg: count) -> return;
}
extern "C" {
    pub fn check_zeroed_user(offset: src.user +, _arg: size) -> return;
}
