//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/circ_buf.h
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
// See Documentation/core-api/circular-buffers.rst for more information.
//
pub const _LINUX_CIRC_BUF_H: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct circ_buf {
    pub buf: *mut c_char,
    pub head: c_int,
    pub tail: c_int,
}

// Return count in buffer.

// Return space available, 0..size-1.  We always leave one free char

// Return count up to the end of the buffer.  Carefully avoid

// Return space available up to the end of the buffer.

