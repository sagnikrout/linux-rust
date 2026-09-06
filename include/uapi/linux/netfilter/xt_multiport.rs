//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_multiport.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xt_multiport_flags {
    XT_MULTIPORT_SOURCE,
    XT_MULTIPORT_DESTINATION,
    XT_MULTIPORT_EITHER
}

pub const XT_MULTI_PORTS: c_int = 15;
// Must fit inside union xt_matchinfo: 16 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_multiport {
    pub /: *mut *mut __u8 flags; / Type of comparison,
    pub /: *mut *mut __u8 count; / Number of ports,
    pub /: *mut *mut __u16 ports[XT_MULTI_PORTS]; / Ports,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_multiport_v1 {
    pub /: *mut *mut __u8 flags; / Type of comparison,
    pub /: *mut *mut __u8 count; / Number of ports,
    pub /: *mut *mut __u16 ports[XT_MULTI_PORTS]; / Ports,
    pub /: *mut *mut __u8 pflags[XT_MULTI_PORTS]; / Port flags,
    pub /: *mut *mut __u8 invert; / Invert flag,
}
