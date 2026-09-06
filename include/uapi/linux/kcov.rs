//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/kcov.h
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
// Argument for KCOV_REMOTE_ENABLE ioctl, see Documentation/dev-tools/kcov.rst
// and the comment before kcov_remote_start() for usage details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcov_remote_arg {
    pub /: *mut *mut __u32 trace_mode; / KCOV_TRACE_PC or KCOV_TRACE_CMP,
    pub /: *mut *mut __u32 area_size; / Length of coverage buffer in words,
    pub /: *mut *mut __u32 num_handles; / Size of handles array,
    pub common_handle: __aligned_u64,
    pub handles: [__aligned_u64; ],
}

pub const KCOV_REMOTE_MAX_HANDLES: c_uint = 0x100;

//
// Tracing coverage collection mode.
// Covered PCs are collected in a per-task buffer.
// In new KCOV version the mode is chosen by calling
// ioctl(fd, KCOV_ENABLE, mode). In older versions the mode argument
// was supposed to be 0 in such a call. So, for reasons of backward
// compatibility, we have chosen the value KCOV_TRACE_PC to be 0.
//
// Collecting comparison operands mode.
//
// The format for the types of collected comparisons.
//
// Bit 0 shows whether one of the arguments is a compile-time constant.
// Bits 1 & 2 contain log2 of the argument size, up to 8 bytes.
//

