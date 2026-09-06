//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/unwind_user_types.h
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
// Unwind types, listed in priority order: lower numbers are attempted first if
// available.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum unwind_user_type_bits {
    UNWIND_USER_TYPE_FP_BIT =		0,

    NR_UNWIND_USER_TYPE_BITS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum unwind_user_type {
// Type "none" for the start of stack walk iteration.
    UNWIND_USER_TYPE_NONE =			0,
    UNWIND_USER_TYPE_FP =			BIT(UNWIND_USER_TYPE_FP_BIT),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwind_stacktrace {
    pub nr: c_uint,
    pub entries: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwind_user_frame {
    pub cfa_off: i32,
    pub ra_off: i32,
    pub fp_off: i32,
    pub use_fp: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwind_user_state {
    pub ip: c_ulong,
    pub sp: c_ulong,
    pub fp: c_ulong,
    pub ws: c_uint,
    pub current_type: unwind_user_type,
    pub available_types: c_uint,
    pub topmost: bool,
    pub done: bool,
}
