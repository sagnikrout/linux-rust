//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/types.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

extern "C" {
    pub fn __attribute__(_arg: (aligned(16))) -> typedef __signed__ __int128 __s128;
}
extern "C" {
    pub fn __attribute__(_arg: (aligned(16))) -> typedef unsigned __int128 __u128;
}

//
// Below are truly Linux-specific types that should never collide with
// any application/library that wants linux/types.h.
//
// sparse defines __CHECKER__; see Documentation/dev-tools/sparse.rst

// Macro flag: #define 

// The kernel doesn't use this legacy form, but user space does

pub type __le16 = __u16 ;
pub type __be16 = __u16 ;
pub type __le32 = __u32 ;
pub type __be32 = __u32 ;
pub type __le64 = __u64 ;
pub type __be64 = __u64 ;
pub type __sum16 = __u16 ;
pub type __wsum = __u32 ;
//
// aligned_u64 should be used in defining kernel<->userspace ABIs to avoid
// common 32/64-bit compat problems.
// 64-bit values align to 4-byte boundaries on x86_32 (and possibly other
// architectures) and to 8-byte boundaries on 64-bit architectures.  The new
// aligned_64 type enforces 8-byte alignment so that structs containing
// aligned_64 values have the same alignment on 32-bit and 64-bit architectures.
// No conversions are necessary between 32-bit user-space and a 64-bit kernel.
//

pub type __poll_t = unsigned ;

