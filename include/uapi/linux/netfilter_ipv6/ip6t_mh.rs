//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_ipv6/ip6t_mh.h
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

// MH matching stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6t_mh {
    pub /: *mut *mut __u8 types[2]; / MH type range,
    pub /: *mut *mut __u8 invflags; / Inverse flags,
}

// Values for "invflags" field in struct ip6t_mh.
pub const IP6T_MH_INV_TYPE: c_uint = 0x01	/* Invert the sense of type. */;
pub const IP6T_MH_INV_MASK: c_uint = 0x01	/* All possible flags. */;
