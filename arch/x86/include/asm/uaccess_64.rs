//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/uaccess_64.h
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
// User space memory access functions
//

//
// Mask out tag bits from the address.
//

//
// Masking the user address is an alternative to a conditional
// user_access_begin that can avoid the fencing. This only works
// for dense accesses starting at the address.
//

//
// User pointers can have tag bits on x86-64.  This scheme tolerates
// arbitrary values in those bits rather then masking them off.
//
// Enforce two rules:
// 1. 'ptr' must be in the user part of the address space
// 2. 'ptr+size' must not overflow into kernel addresses
//
// Note that we always have at least one guard page between the
// max user address and the non-canonical gap, allowing us to
// ignore small sizes entirely.
//
// In fact, we could probably remove the size check entirely, since
// any kernel accesses will be in increasing address order starting
// at 'ptr'.
//
// That's a separate optimization, for now just handle the small
// constant case.
//
extern "C" {
    pub fn valid_user_address(_arg: ptr) -> return;
}

//
// Copy To/From Userspace
//
// Handles exceptions in both to and from, but doesn't do access_ok
//
// If CPU has FSRM feature, use 'rep movs'.
// Otherwise, use rep_movs_alternative.
//
extern "C" {
    pub fn copy_user_generic(_arg: dst, )src: *mut ( void, _arg: size) -> return;
}
extern "C" {
    pub fn copy_user_generic()dst: *mut ( void, _arg: src, _arg: size) -> return;
}

extern "C" {
    pub fn copy_to_nontemporal(dst: *mut c_void, src: *const c_void, size: usize) -> usize;
}
extern "C" {
    pub fn copy_user_flushcache(dst: *mut c_void, src: *const void __user, size: usize) -> usize;
}
extern "C" {
    pub fn copy_user_flushcache(_arg: dst, _arg: src, _arg: size) -> return;
}
//
// Zero Userspace.
//
// No memory constraint because it doesn't change any memory gcc
// knows about.
//
extern "C" {
    pub fn __clear_user(_arg: to, _arg: n) -> return;
}
