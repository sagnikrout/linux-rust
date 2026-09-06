//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netdevice.h
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
// Definitions for the Interfaces handler.
//
// Version:	@(#)dev.h	1.0.10	08/12/93
//
// Authors:	Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
// Corey Minyard <wf-rch!minyard@relay.EU.net>
// Donald J. Becker, <becker@cesdis.gsfc.nasa.gov>
// Alan Cox, <alan@lxorguk.ukuu.org.uk>
// Bjorn Ekwall. <bj0rn@blox.se>
// Pekka Riikonen <priikone@poseidon.pspt.fi>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//
// Moved to /usr/include/linux for NET3
//

// Initial net device group. All devices belong to group 0 by default.
pub const INIT_NETDEV_GROUP: c_int = 0;
// interface name assignment types (sysfs name_assign_type attribute)

// Media selection options.
// hardware address assignment types

// dev_set_mac_address()
