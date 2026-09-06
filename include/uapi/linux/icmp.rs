//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/icmp.h
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
// Definitions for the ICMP protocol.
//
// Version:	@(#)icmp.h	1.0.3	04/28/93
//
// Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

pub const NR_ICMP_TYPES: c_int = 18;
// Codes for UNREACH.

pub const ICMP_NET_UNKNOWN: c_int = 6;
pub const ICMP_HOST_UNKNOWN: c_int = 7;
pub const ICMP_HOST_ISOLATED: c_int = 8;
pub const ICMP_NET_ANO: c_int = 9;
pub const ICMP_HOST_ANO: c_int = 10;
pub const ICMP_NET_UNR_TOS: c_int = 11;
pub const ICMP_HOST_UNR_TOS: c_int = 12;

// Codes for REDIRECT.

// Codes for TIME_EXCEEDED.

// Codes for EXT_ECHO (PROBE)
pub const ICMP_EXT_ECHO: c_int = 42;
pub const ICMP_EXT_ECHOREPLY: c_int = 43;

// Constants for EXT_ECHO (PROBE)

pub const ICMP_EXT_ECHO_CTYPE_NAME: c_int = 1;
pub const ICMP_EXT_ECHO_CTYPE_INDEX: c_int = 2;
pub const ICMP_EXT_ECHO_CTYPE_ADDR: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmphdr {
    pub type: __u8,
    pub code: __u8,
    pub checksum: __sum16,
    pub id: __be16,
    pub sequence: __be16,
    pub echo: },
    pub gateway: __be32,
    pub __unused: __be16,
    pub mtu: __be16,
    pub frag: },
    pub reserved: [__u8; 4],
    pub un: },
}

//
// constants for (set|get)sockopt
//
pub const ICMP_FILTER: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_filter {
    pub data: __u32,
}

// RFC 4884 extension struct: one per message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_ext_hdr {

    pub reserved2: __u8,
    pub checksum: __sum16,
}

// RFC 4884 extension object header: one for each object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_extobj_hdr {
    pub length: __be16,
    pub class_num: __u8,
    pub class_type: __u8,
}

// RFC 8335: 2.1 Header for c-type 3 payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_ext_echo_ctype3_hdr {
    pub afi: __be16,
    pub addrlen: __u8,
    pub reserved: __u8,
}

// RFC 8335: 2.1 Interface Identification Object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_ext_echo_iio {
    pub extobj_hdr: icmp_extobj_hdr,
    pub name: [c_char; IFNAMSIZ],
    pub ifindex: __be32,
    pub ctype3_hdr: icmp_ext_echo_ctype3_hdr,
    pub ipv4_addr: __be32,
    pub ipv6_addr: in6_addr,
    pub ip_addr: },
    pub addr: },
    pub ident: },
}
