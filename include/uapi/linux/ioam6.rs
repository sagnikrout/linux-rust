//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ioam6.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// IPv6 IOAM implementation
//
// Author:
// Justin Iurman <justin.iurman@uliege.be>
//

//
// IPv6 IOAM Option Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioam6_hdr {
    pub opt_type: __u8,
    pub opt_len: __u8,
    pub /: *mut *mut __u8 :8; / reserved,
pub const IOAM6_TYPE_PREALLOC: c_int = 0;
    pub type: __u8,
    pub __attribute__((packed)): },
//
// IOAM Trace Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioam6_trace_hdr {
    pub namespace_id: __be16,

    pub /: *mut *mut :1; / unused,
    pub type_be32: __be32,
    pub /: *mut *mut :8; / reserved,
    pub type: },
}

pub const IOAM6_TRACE_DATA_SIZE_MAX: c_int = 244;
