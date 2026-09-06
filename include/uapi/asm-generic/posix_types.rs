//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/posix_types.h
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
// This file is generally used by user-level software, so you need to
// be a little careful about namespace pollution etc.
//
// First the types that are often defined in different ways across
// architectures, so that you can override them.
//

pub type __kernel_long_t = c_long;
pub type __kernel_ulong_t = c_ulong;

pub type __kernel_ino_t = __kernel_ulong_t;

pub type __kernel_mode_t = c_uint;

pub type __kernel_pid_t = c_int;

pub type __kernel_ipc_pid_t = c_int;

pub type __kernel_uid_t = c_uint;
pub type __kernel_gid_t = c_uint;

pub type __kernel_suseconds_t = __kernel_long_t;

pub type __kernel_daddr_t = c_int;

pub type __kernel_uid32_t = c_uint;
pub type __kernel_gid32_t = c_uint;

pub type __kernel_old_uid_t = __kernel_uid_t;
pub type __kernel_old_gid_t = __kernel_gid_t;

pub type __kernel_old_dev_t = c_uint;

//
// Most 32 bit architectures use "unsigned int" size_t,
// and all 64 bit architectures use "unsigned long" size_t.
//

pub type __kernel_size_t = c_uint;
pub type __kernel_ssize_t = c_int;
pub type __kernel_ptrdiff_t = c_int;

pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;

//
// anything below here should be completely generic
//
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = c_longlong;
pub type __kernel_uoff_t = c_ulonglong;
pub type __kernel_old_time_t = __kernel_long_t;
pub type __kernel_time_t = __kernel_long_t;

pub type __kernel_time64_t = c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = c_int;
pub type __kernel_clockid_t = c_int;
pub type __kernel_caddr_t = *mut c_char;
pub type __kernel_uid16_t = c_ushort;
pub type __kernel_gid16_t = c_ushort;
