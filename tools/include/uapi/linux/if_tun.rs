//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/if_tun.h
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
// Universal TUN/TAP device driver.
// Copyright (C) 1999-2000 Maxim Krasnyansky <max_mk@yahoo.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//

// Read queue size
pub const TUN_READQ_SIZE: c_int = 500;
// TUN device type flags: deprecated. Use IFF_TUN/IFF_TAP instead.

pub const TUN_TYPE_MASK: c_uint = 0x000f;
// Ioctl defines

// The TUNSETVNETBE and TUNGETVNETBE ioctls are for cross-endian support on
// little-endian hosts. Not all kernel configurations support them, but all
// configurations that support SET also support GET.
//

// TUNSETIFF ifr flags
pub const IFF_TUN: c_uint = 0x0001;
pub const IFF_TAP: c_uint = 0x0002;
pub const IFF_NAPI: c_uint = 0x0010;
pub const IFF_NAPI_FRAGS: c_uint = 0x0020;
// Stop the queue instead of dropping when the internal ring is full, so an
// attached qdisc applies backpressure instead of being bypassed.
//
pub const IFF_BACKPRESSURE: c_uint = 0x0080;
pub const IFF_NO_PI: c_uint = 0x1000;
// This flag has no real effect
pub const IFF_ONE_QUEUE: c_uint = 0x2000;
pub const IFF_VNET_HDR: c_uint = 0x4000;
pub const IFF_TUN_EXCL: c_uint = 0x8000;
pub const IFF_MULTI_QUEUE: c_uint = 0x0100;
pub const IFF_ATTACH_QUEUE: c_uint = 0x0200;
pub const IFF_DETACH_QUEUE: c_uint = 0x0400;
// read-only flag
pub const IFF_PERSIST: c_uint = 0x0800;
pub const IFF_NOFILTER: c_uint = 0x1000;
// Socket options
pub const TUN_TX_TIMESTAMP: c_int = 1;
// Features for GSO (TUNSETOFFLOAD).
pub const TUN_F_CSUM: c_uint = 0x01	/* You can hand me unchecksummed packets. */;
pub const TUN_F_TSO4: c_uint = 0x02	/* I can handle TSO for IPv4 packets */;
pub const TUN_F_TSO6: c_uint = 0x04	/* I can handle TSO for IPv6 packets */;
pub const TUN_F_TSO_ECN: c_uint = 0x08	/* I can handle TSO with ECN bits. */;
pub const TUN_F_UFO: c_uint = 0x10	/* I can handle UFO packets */;
// Protocol info prepended to the packets (when IFF_NO_PI is not set)
pub const TUN_PKT_STRIP: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tun_pi {
    pub flags: __u16,
    pub proto: __be16,
}

//
// Filter spec (used for SETXXFILTER ioctls)
// This stuff is applicable only to the TAP (Ethernet) devices.
// If the count is zero the filter is disabled and the driver accepts
// all packets (promisc mode).
// If the filter is enabled in order to accept broadcast packets
// broadcast addr must be explicitly included in the addr list.
//
pub const TUN_FLT_ALLMULTI: c_uint = 0x0001 /* Accept all multicast packets */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tun_filter {
    pub /: *mut *mut __u16 flags; / TUN_FLT_ flags see above,
    pub /: *mut *mut __u16 count; / Number of addresses,
    pub addr: [__u8; ][ETH_ALEN],
}
