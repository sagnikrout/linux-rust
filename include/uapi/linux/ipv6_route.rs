//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ipv6_route.h
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
// Linux INET6 implementation
//
// Authors:
// Pedro Roque		<roque@di.fc.ul.pt>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

pub const RTF_DEFAULT: c_uint = 0x00010000	/* default - learned via ND	*/;
pub const RTF_ALLONLINK: c_uint = 0x00020000	/* (deprecated and will be removed);
pub const RTF_ADDRCONF: c_uint = 0x00040000	/* addrconf route - RA		*/;
pub const RTF_PREFIX_RT: c_uint = 0x00080000	/* A prefix only route - RA	*/;
pub const RTF_ANYCAST: c_uint = 0x00100000	/* Anycast			*/;
pub const RTF_NONEXTHOP: c_uint = 0x00200000	/* route with no nexthop	*/;
pub const RTF_EXPIRES: c_uint = 0x00400000;
pub const RTF_ROUTEINFO: c_uint = 0x00800000	/* route information - RA	*/;
pub const RTF_CACHE: c_uint = 0x01000000	/* read-only: can not be set by user */;
pub const RTF_FLOW: c_uint = 0x02000000	/* flow significant route	*/;
pub const RTF_POLICY: c_uint = 0x04000000	/* policy route			*/;

pub const RTF_PREF_MASK: c_uint = 0x18000000;
pub const RTF_PCPU: c_uint = 0x40000000	/* read-only: can not be set by user */;
pub const RTF_LOCAL: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_rtmsg {
    pub rtmsg_dst: in6_addr,
    pub rtmsg_src: in6_addr,
    pub rtmsg_gateway: in6_addr,
    pub rtmsg_type: __u32,
    pub rtmsg_dst_len: __u16,
    pub rtmsg_src_len: __u16,
    pub rtmsg_metric: __u32,
    pub rtmsg_info: c_ulong,
    pub rtmsg_flags: __u32,
    pub rtmsg_ifindex: c_int,
}

pub const RTMSG_NEWDEVICE: c_uint = 0x11;
pub const RTMSG_DELDEVICE: c_uint = 0x12;
pub const RTMSG_NEWROUTE: c_uint = 0x21;
pub const RTMSG_DELROUTE: c_uint = 0x22;
pub const IP6_RT_PRIO_USER: c_int = 1024;
pub const IP6_RT_PRIO_ADDRCONF: c_int = 256;
