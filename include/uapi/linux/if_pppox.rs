//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_pppox.h
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
// Linux PPP over X - Generic PPP transport layer sockets
// Linux PPP over Ethernet (PPPoE) Socket Implementation (RFC 2516)
//
// This file supplies definitions required by the PPP over Ethernet driver
// (pppox.c).  All version information wrt this file is located in pppox.c
//
// License:
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

// For user-space programs to pick up these definitions
// which they wouldn't get otherwise without defining __KERNEL__
//

pub const AF_PPPOX: c_int = 24;

//
// PPPoE addressing definition
//
pub type sid_t = __be16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pppoe_addr {
    pub /: *mut *mut sid_t sid; / Session identifier,
    pub /: *mut *mut unsigned char remote[ETH_ALEN]; / Remote address,
    pub /: *mut *mut char dev[IFNAMSIZ]; / Local device to use,
}

//
// PPTP addressing definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pptp_addr {
    pub call_id: __u16,
    pub sin_addr: in_addr,
}

//
// Protocols supported by AF_PPPOX
//

pub const PX_PROTO_PPTP: c_int = 2;
pub const PX_MAX_PROTO: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_pppox {
    pub /: *mut *mut __kernel_sa_family_t sa_family; / address family, AF_PPPOX,
    pub /: *mut *mut unsigned int sa_protocol; / protocol identifier,
    pub pppoe: pppoe_addr,
    pub pptp: pptp_addr,
    pub sa_addr: },
    pub __packed: },
// The use of the above union isn't viable because the size of this
// struct must stay fixed over time -- applications use sizeof(struct
// sockaddr_pppox) to fill it. We use a protocol specific sockaddr
// type instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_pppol2tp {
    pub /: *mut *mut __kernel_sa_family_t sa_family; / address family, AF_PPPOX,
    pub /: *mut *mut unsigned int sa_protocol; / protocol identifier,
    pub pppol2tp: pppol2tp_addr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_pppol2tpin6 {
    pub /: *mut *mut __kernel_sa_family_t sa_family; / address family, AF_PPPOX,
    pub /: *mut *mut unsigned int sa_protocol; / protocol identifier,
    pub pppol2tp: pppol2tpin6_addr,
    pub __packed: },
// The L2TPv3 protocol changes tunnel and session ids from 16 to 32
// bits. So we need a different sockaddr structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_pppol2tpv3 {
    pub /: *mut *mut __kernel_sa_family_t sa_family; / address family, AF_PPPOX,
    pub /: *mut *mut unsigned int sa_protocol; / protocol identifier,
    pub pppol2tp: pppol2tpv3_addr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_pppol2tpv3in6 {
    pub /: *mut *mut __kernel_sa_family_t sa_family; / address family, AF_PPPOX,
    pub /: *mut *mut unsigned int sa_protocol; / protocol identifier,
    pub pppol2tp: pppol2tpv3in6_addr,
    pub __packed: },
// Codes to identify message types
pub const PADI_CODE: c_uint = 0x09;
pub const PADO_CODE: c_uint = 0x07;
pub const PADR_CODE: c_uint = 0x19;
pub const PADS_CODE: c_uint = 0x65;
pub const PADT_CODE: c_uint = 0xa7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pppoe_tag {
    pub tag_type: __be16,
    pub tag_len: __be16,
    pub tag_data: [c_char; ],
// C attribute field omitted
// Tag identifiers

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pppoe_hdr {

    pub 4: __u8 type :,
    pub 4: __u8 ver :,

    pub 4: __u8 ver :,
    pub 4: __u8 type :,

    pub code: __u8,
    pub sid: __be16,
    pub length: __be16,
    pub tag: [pppoe_tag; ],
    pub __packed: },
// Length of entire PPPoE + PPP header
pub const PPPOE_SES_HLEN: c_int = 8;
