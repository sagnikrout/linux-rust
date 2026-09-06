//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_arcnet.h
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
// INET         An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Global definitions for the ARCnet interface.
//
// Authors:     David Woodhouse and Avery Pennarun
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

//
// These are the defined ARCnet Protocol ID's.
//
// CAP mode
// No macro but uses 1-8
// RFC1201 Protocol ID's

// Old RFC1051 Protocol ID's

// MS LanMan/WfWg "NDIS" encapsulation

// Unsupported/indirectly supported protocols

pub const ARC_P_DATAPOINT_MOUNT: c_int = 1;

pub const ARC_P_ATALK: c_uint = 0xDD;
// Hardware address length
pub const ARCNET_ALEN: c_int = 1;
//
// The RFC1201-specific components of an arcnet packet header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_rfc1201 {
    pub /: *mut *mut __u8 proto; / protocol ID field - varies,
    pub /: *mut *mut __u8 split_flag; / for use with split packets,
    pub /: *mut *mut __be16 sequence; / sequence number,
    pub bytes)*/: *mut *mut __u8 payload[]; / space remaining in packet (504,
}

pub const RFC1201_HDR_SIZE: c_int = 4;
//
// The RFC1051-specific components.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_rfc1051 {
    pub /: *mut *mut __u8 proto; / ARC_P_RFC1051_ARP/RFC1051_IP,
    pub /: *mut *mut __u8 payload[]; / 507 bytes,
}

pub const RFC1051_HDR_SIZE: c_int = 1;
//
// The ethernet-encap-specific components.  We have a real ethernet header
// and some data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_eth_encap {
    pub /: *mut *mut __u8 proto; / Always ARC_P_ETHER,
    pub /: *mut *mut ethhdr eth; / standard ethernet header (yuck!),
    pub /: *mut *mut __u8 payload[]; / 493 bytes,
}

pub const ETH_ENCAP_HDR_SIZE: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_cap {
    pub proto: __u8,
    pub cookie: [__u8; sizeof(int)],
// Actually NOT sent over the network
    pub ack: __u8,
    pub /: *mut *mut __u8 raw[0]; / 507 bytes,
    pub mes: },
}

//
// The data needed by the actual arcnet hardware.
//
// Now, in the real arcnet hardware, the third and fourth bytes are the
// 'offset' specification instead of the length, and the soft data is at
// the _end_ of the 512-byte buffer.  We hide this complexity inside the
// driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_hardware {
    pub /: *mut *mut __u8 source; / source ARCnet - filled in automagically,
    pub /: *mut *mut __u8 dest; / destination ARCnet - 0 for broadcast,
    pub /: *mut *mut __u8 offset[2]; / offset bytes (some weird semantics),
}

pub const ARC_HDR_SIZE: c_int = 4;
//
// This is an ARCnet frame header, as seen by the kernel (and userspace,
// when you do a raw packet capture).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct archdr {
// hardware requirements
    pub hard: arc_hardware,
// arcnet encapsulation-specific bits
    pub rfc1201: arc_rfc1201,
    pub rfc1051: arc_rfc1051,
    pub eth_encap: arc_eth_encap,
    pub cap: arc_cap,
    pub /: *mut *mut __u8 raw[0]; / 508 bytes,
    pub soft: },
}
