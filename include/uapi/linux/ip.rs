//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ip.h
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
// Definitions for the IP protocol.
//
// Version:	@(#)ip.h	1.0.2	04/28/93
//
// Authors:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

pub const IPTOS_TOS_MASK: c_uint = 0x1E;

pub const IPTOS_LOWDELAY: c_uint = 0x10;
pub const IPTOS_THROUGHPUT: c_uint = 0x08;
pub const IPTOS_RELIABILITY: c_uint = 0x04;
pub const IPTOS_MINCOST: c_uint = 0x02;
pub const IPTOS_PREC_MASK: c_uint = 0xE0;

pub const IPTOS_PREC_NETCONTROL: c_uint = 0xe0;
pub const IPTOS_PREC_INTERNETCONTROL: c_uint = 0xc0;
pub const IPTOS_PREC_CRITIC_ECP: c_uint = 0xa0;
pub const IPTOS_PREC_FLASHOVERRIDE: c_uint = 0x80;
pub const IPTOS_PREC_FLASH: c_uint = 0x60;
pub const IPTOS_PREC_IMMEDIATE: c_uint = 0x40;
pub const IPTOS_PREC_PRIORITY: c_uint = 0x20;
pub const IPTOS_PREC_ROUTINE: c_uint = 0x00;
// IP options
pub const IPOPT_COPY: c_uint = 0x80;
pub const IPOPT_CLASS_MASK: c_uint = 0x60;
pub const IPOPT_NUMBER_MASK: c_uint = 0x1f;

pub const IPOPT_CONTROL: c_uint = 0x00;
pub const IPOPT_RESERVED1: c_uint = 0x20;
pub const IPOPT_MEASUREMENT: c_uint = 0x40;
pub const IPOPT_RESERVED2: c_uint = 0x60;

pub const IPVERSION: c_int = 4;
pub const MAXTTL: c_int = 255;
pub const IPDEFTTL: c_int = 64;
pub const IPOPT_OPTVAL: c_int = 0;
pub const IPOPT_OLEN: c_int = 1;
pub const IPOPT_OFFSET: c_int = 2;
pub const IPOPT_MINOFF: c_int = 4;
pub const MAX_IPOPTLEN: c_int = 40;

pub const IPV4_BEET_PHMAXLEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iphdr {

    pub tos: __u8,
    pub tot_len: __be16,
    pub id: __be16,
    pub frag_off: __be16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __sum16,
    pub saddr: __be32,
    pub daddr: __be32,
// The options start here.
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_auth_hdr {
    pub nexthdr: __u8,
    pub /: *mut *mut __u8 hdrlen; / This one is measured in 32 bit units!,
    pub reserved: __be16,
    pub spi: __be32,
    pub /: *mut *mut __be32 seq_no; / Sequence number,
    pub /: *mut *mut __u8 auth_data[]; / Variable len but >=4. Mind the 64 bit alignment!,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_esp_hdr {
    pub spi: __be32,
    pub /: *mut *mut __be32 seq_no; / Sequence number,
    pub /: *mut *mut __u8 enc_data[]; / Variable len but >=8. Mind the 64 bit alignment!,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_comp_hdr {
    pub nexthdr: __u8,
    pub flags: __u8,
    pub cpi: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_beet_phdr {
    pub nexthdr: __u8,
    pub hdrlen: __u8,
    pub padlen: __u8,
    pub reserved: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_iptfs_hdr {
    pub /: *mut *mut *mut __u8 subtype; / 0: basic, 1: CC,
    pub flags: __u8,
    pub block_offset: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_iptfs_cc_hdr {
    pub /: *mut *mut *mut __u8 subtype; / 0: basic, 1: CC,
    pub flags: __u8,
    pub block_offset: __be16,
    pub loss_rate: __be32,
    pub rtt_adelay_xdelay: __be64,
    pub tval: __be32,
    pub techo: __be32,
}

// index values for the variables in ipv4_devconf

