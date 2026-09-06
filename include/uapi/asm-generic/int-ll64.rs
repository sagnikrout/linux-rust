//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/int-ll64.h
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
//
// asm-generic/int-ll64.h
//
// Integer declarations for architectures which use "long long"
// for 64-bit types.
//

//
// __xx is ok: it doesn't pollute the POSIX namespace. Use these in the
// header files exported to user space
//
pub type __s8 = __signed__ char;
pub type __u8 = c_uchar;
pub type __s16 = __signed__ short;
pub type __u16 = c_ushort;
pub type __s32 = __signed__ int;
pub type __u32 = c_uint;

pub type __s64 = __signed__ long long;
pub type __u64 = c_ulonglong;

