//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/rpl.h
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
// IPv6 RPL-SR implementation
//
// Author:
// (C) 2020 Alexander Aring <alex.aring@gmail.com>
//

//
// RPL SR Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_rpl_sr_hdr {
    pub nexthdr: __u8,
    pub hdrlen: __u8,
    pub type: __u8,
    pub segments_left: __u8,

    pub addr): __DECLARE_FLEX_ARRAY(struct in6_addr,,
    pub data): __DECLARE_FLEX_ARRAY(__u8,,
    pub segments: },
    pub __attribute__((packed)): },

