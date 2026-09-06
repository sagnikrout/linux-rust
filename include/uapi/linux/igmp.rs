//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/igmp.h
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
// Linux NET3:	Internet Group Management Protocol  [IGMP]
//
// Authors:
// Alan Cox <alan@lxorguk.ukuu.org.uk>
//
// Extended to talk the BSD extended IGMP protocol of mrouted 3.6
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

//
// IGMP protocol structures
//
// Header in on cable format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igmphdr {
    pub type: __u8,
    pub /: *mut *mut __u8 code; / For newer IGMP,
    pub csum: __sum16,
    pub group: __be32,
}

// V3 group record types [grec_type]
pub const IGMPV3_MODE_IS_INCLUDE: c_int = 1;
pub const IGMPV3_MODE_IS_EXCLUDE: c_int = 2;
pub const IGMPV3_CHANGE_TO_INCLUDE: c_int = 3;
pub const IGMPV3_CHANGE_TO_EXCLUDE: c_int = 4;
pub const IGMPV3_ALLOW_NEW_SOURCES: c_int = 5;
pub const IGMPV3_BLOCK_OLD_SOURCES: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igmpv3_grec {
    pub grec_type: __u8,
    pub grec_auxwords: __u8,
    pub grec_nsrcs: __be16,
    pub grec_mca: __be32,
    pub grec_src: [__be32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igmpv3_report {
    pub type: __u8,
    pub resv1: __u8,
    pub csum: __sum16,
    pub resv2: __be16,
    pub ngrec: __be16,
    pub grec: [igmpv3_grec; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igmpv3_query {
    pub type: __u8,
    pub code: __u8,
    pub csum: __sum16,
    pub group: __be32,

    pub qqic: __u8,
    pub nsrcs: __be16,
    pub srcs: [__be32; ],
}

pub const IGMP_HOST_MEMBERSHIP_QUERY: c_uint = 0x11	/* From RFC1112 */;
pub const IGMP_HOST_MEMBERSHIP_REPORT: c_uint = 0x12	/* Ditto */;
pub const IGMP_DVMRP: c_uint = 0x13	/* DVMRP routing */;
pub const IGMP_PIM: c_uint = 0x14	/* PIM routing */;
pub const IGMP_TRACE: c_uint = 0x15;
pub const IGMPV2_HOST_MEMBERSHIP_REPORT: c_uint = 0x16	/* V2 version of 0x12 */;
pub const IGMP_HOST_LEAVE_MESSAGE: c_uint = 0x17;
pub const IGMPV3_HOST_MEMBERSHIP_REPORT: c_uint = 0x22	/* V3 version of 0x12 */;
pub const IGMP_MTRACE_RESP: c_uint = 0x1e;
pub const IGMP_MTRACE: c_uint = 0x1f;
pub const IGMP_MRDISC_ADV: c_uint = 0x30	/* From RFC4286 */;
//
// Use the BSD names for these for compatibility
//
pub const IGMP_DELAYING_MEMBER: c_uint = 0x01;
pub const IGMP_IDLE_MEMBER: c_uint = 0x02;
pub const IGMP_LAZY_MEMBER: c_uint = 0x03;
pub const IGMP_SLEEPING_MEMBER: c_uint = 0x04;
pub const IGMP_AWAKENING_MEMBER: c_uint = 0x05;
pub const IGMP_MINLEN: c_int = 8;

// query (in seconds)

// specifies time in 10th of seconds

// message in this period of time,
// revert to IGMP v2 router.

//
// struct for keeping the multicast list in
//
