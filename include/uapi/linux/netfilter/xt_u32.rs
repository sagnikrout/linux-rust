//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_u32.h
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
pub const _XT_U32_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xt_u32_ops {
    XT_U32_AND,
    XT_U32_LEFTSH,
    XT_U32_RIGHTSH,
    XT_U32_AT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_u32_location_element {
    pub number: __u32,
    pub nextop: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_u32_value_element {
    pub min: __u32,
    pub max: __u32,
}

//
// Any way to allow for an arbitrary number of elements?
// For now, I settle with a limit of 10 each.
//
pub const XT_U32_MAXSIZE: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_u32_test {
    pub location: [xt_u32_location_element; XT_U32_MAXSIZE+1],
    pub value: [xt_u32_value_element; XT_U32_MAXSIZE+1],
    pub nnums: __u8,
    pub nvalues: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_u32 {
    pub tests: [xt_u32_test; XT_U32_MAXSIZE+1],
    pub ntests: __u8,
    pub invert: __u8,
}
