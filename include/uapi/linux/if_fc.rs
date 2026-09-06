//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_fc.h
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
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Global definitions for Fibre Channel.
//
// Version:	@(#)if_fc.h	0.0	11/20/98
//
// Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
// Donald Becker, <becker@super.org>
// Peter De Schrijver, <stud11@cc4.kuleuven.ac.be>
// Vineet Abraham, <vma@iol.unh.edu>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

// LLC and SNAP constants
pub const EXTENDED_SAP: c_uint = 0xAA;
pub const UI_CMD: c_uint = 0x03;
// This is NOT the Fibre Channel frame header. The FC frame header is
// constructed in the driver as the Tachyon needs certain fields in
// certains positions. So, it can't be generalized here.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fch_hdr {
    pub /: *mut *mut __u8 daddr[FC_ALEN]; / destination address,
    pub /: *mut *mut __u8 saddr[FC_ALEN]; / source address,
}

// This is a Fibre Channel LLC structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcllc {
    pub /: *mut *mut __u8 dsap; / destination SAP,
    pub /: *mut *mut __u8 ssap; / source SAP,
    pub /: *mut *mut __u8 llc; / LLC control field,
    pub /: *mut *mut __u8 protid[3]; / protocol id,
    pub /: *mut *mut __be16 ethertype; / ether type field,
}
