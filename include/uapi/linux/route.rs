//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/route.h
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
// Global definitions for the IP router interface.
//
// Version:	@(#)route.h	1.0.3	05/27/93
//
// Authors:	Original taken from Berkeley UNIX 4.3, (c) UCB 1986-1988
// for the purposes of compatibility only.
//
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//
// Changes:
// Mike McLagan    :       Routing by source
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

// This structure gets passed by the SIOCADDRT and SIOCDELRT calls.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtentry {
    pub rt_pad1: c_ulong,
    pub /: *mut *mut sockaddr rt_dst; / target address,
    pub /: *mut *mut sockaddr rt_gateway; / gateway addr (RTF_GATEWAY),
    pub /: *mut *mut sockaddr rt_genmask; / target network mask (IP),
    pub rt_flags: c_ushort,
    pub rt_pad2: c_short,
    pub rt_pad3: c_ulong,
    pub rt_pad4: *mut c_void,
    pub /: *mut *mut short rt_metric; / +1 for binary compatibility!,
    pub /: *mut *mut *mut char __user rt_dev; / forcing the device at add,
    pub /: *mut *mut unsigned long rt_mtu; / per route MTU/Window,

    pub /: *mut *mut unsigned long rt_window; / Window clamping,
    pub /: *mut *mut unsigned short rt_irtt; / Initial RTT,
}

pub const RTF_UP: c_uint = 0x0001		/* route usable		  	*/;
pub const RTF_GATEWAY: c_uint = 0x0002		/* destination is a gateway	*/;
pub const RTF_HOST: c_uint = 0x0004		/* host entry (net otherwise)	*/;
pub const RTF_REINSTATE: c_uint = 0x0008		/* reinstate route after tmout	*/;
pub const RTF_DYNAMIC: c_uint = 0x0010		/* created dyn. (by redirect)	*/;
pub const RTF_MODIFIED: c_uint = 0x0020		/* modified dyn. (by redirect)	*/;
pub const RTF_MTU: c_uint = 0x0040		/* specific MTU for this route	*/;

pub const RTF_WINDOW: c_uint = 0x0080		/* per route window clamping	*/;
pub const RTF_IRTT: c_uint = 0x0100		/* Initial round trip time	*/;
pub const RTF_REJECT: c_uint = 0x0200		/* Reject route			*/;
//
// <linux/ipv6_route.h> uses RTF values >= 64k
//
