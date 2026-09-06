//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/icmp.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for the ICMP protocol.
//
// Version:	@(#)icmp.h	1.0.3	04/28/93
//
// Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//

// RFC 4884
pub const ICMP_EXT_ORIG_DGRAM_MIN_LEN: c_int = 128;
pub const ICMP_EXT_VERSION_2: c_int = 2;
// ICMP Extension Object Classes

// Interface Information Object - RFC 5837

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_ext_iio_name_subobj {
    pub len: u8,
    pub name: [c_char; IFNAMSIZ],
}

// RFC 5837 - Incoming IP Interface Role
// Add new constants above. Used by "icmp_errors_extension_mask"
// sysctl.
//
