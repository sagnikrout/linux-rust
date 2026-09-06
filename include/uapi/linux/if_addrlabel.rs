//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_addrlabel.h
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
// if_addrlabel.h - netlink interface for address labels
//
// Copyright (C)2007 USAGI/WIDE Project,  All Rights Reserved.
//
// Authors:
// YOSHIFUJI Hideaki @ USAGI/WIDE <yoshfuji@linux-ipv6.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifaddrlblmsg {
    pub /: *mut *mut __u8 ifal_family; / Address family,
    pub /: *mut *mut __u8 __ifal_reserved; / Reserved,
    pub /: *mut *mut __u8 ifal_prefixlen; / Prefix length,
    pub /: *mut *mut __u8 ifal_flags; / Flags,
    pub /: *mut *mut __u32 ifal_index; / Link index,
    pub /: *mut *mut __u32 ifal_seq; / sequence number,
}

