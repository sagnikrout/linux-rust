//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mctp.h
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
// Management Component Transport Protocol (MCTP)
//
// Copyright (c) 2021 Code Construct
// Copyright (c) 2021 Google
//

pub type mctp_eid_t = __u8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_addr {
    pub s_addr: mctp_eid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_mctp {
    pub smctp_family: __kernel_sa_family_t,
    pub __smctp_pad0: __u16,
    pub smctp_network: c_uint,
    pub smctp_addr: mctp_addr,
    pub smctp_type: __u8,
    pub smctp_tag: __u8,
    pub __smctp_pad1: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_mctp_ext {
    pub smctp_base: sockaddr_mctp,
    pub smctp_ifindex: c_int,
    pub smctp_halen: __u8,
    pub __smctp_pad0: [__u8; 3],
    pub smctp_haddr: [__u8; MAX_ADDR_LEN],
}

// A "fully qualified" MCTP address, which includes the system-local network ID,
// required to uniquely resolve a routable EID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_fq_addr {
    pub net: c_uint,
    pub eid: mctp_eid_t,
}

pub const MCTP_NET_ANY: c_uint = 0x0;
pub const MCTP_ADDR_NULL: c_uint = 0x00;
pub const MCTP_ADDR_ANY: c_uint = 0xff;
pub const MCTP_TAG_MASK: c_uint = 0x07;
pub const MCTP_TAG_OWNER: c_uint = 0x08;
pub const MCTP_TAG_PREALLOC: c_uint = 0x10;
pub const MCTP_OPT_ADDR_EXT: c_int = 1;

// Deprecated: use mctp_ioc_tag_ctl2 / TAG2 ioctls instead, which defines the
// MCTP network ID as part of the allocated tag. Using this assumes the default
// net ID for allocated tags, which may not give correct behaviour on system
// with multiple networks configured.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_ioc_tag_ctl {
    pub peer_addr: mctp_eid_t,
// For SIOCMCTPALLOCTAG: must be passed as zero, kernel will
// populate with the allocated tag value. Returned tag value will
// always have TO and PREALLOC set.
//
// For SIOCMCTPDROPTAG: userspace provides tag value to drop, from
// a prior SIOCMCTPALLOCTAG call (and so must have TO and PREALLOC set).
//
    pub tag: __u8,
    pub flags: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_ioc_tag_ctl2 {
// Peer details: network ID, peer EID, local EID. All set by the
// caller.
//
// Local EID must be MCTP_ADDR_NULL or MCTP_ADDR_ANY in current
// kernels.
//
    pub net: c_uint,
    pub peer_addr: mctp_eid_t,
    pub local_addr: mctp_eid_t,
// Set by caller, but no flags defined currently. Must be 0
    pub flags: __u16,
// For SIOCMCTPALLOCTAG2: must be passed as zero, kernel will
// populate with the allocated tag value. Returned tag value will
// always have TO and PREALLOC set.
//
// For SIOCMCTPDROPTAG2: userspace provides tag value to drop, from
// a prior SIOCMCTPALLOCTAG2 call (and so must have TO and PREALLOC set).
//
    pub tag: __u8,
}
