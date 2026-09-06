//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_ipv6/ip6t_ipv6header.h
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
// ipv6header match - matches IPv6 packets based
// Original idea: Brad Chapman
// Rewritten by: Andras Kis-Szabo <kisza@sch.bme.hu>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6t_ipv6header_info {
    pub matchflags: __u8,
    pub invflags: __u8,
    pub modeflag: __u8,
}

pub const MASK_HOPOPTS: c_int = 128;
pub const MASK_DSTOPTS: c_int = 64;
pub const MASK_ROUTING: c_int = 32;
pub const MASK_FRAGMENT: c_int = 16;
pub const MASK_AH: c_int = 8;
pub const MASK_ESP: c_int = 4;
pub const MASK_NONE: c_int = 2;
pub const MASK_PROTO: c_int = 1;
