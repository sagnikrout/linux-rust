//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_fddi.h
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
// operating system.  INET is implemented using the BSD Socket
// interface as the means of communication with the user level.
//
// Global definitions for the ANSI FDDI interface.
//
// Version:	@(#)if_fddi.h	1.0.3	Oct  6 2018
//
// Author:	Lawrence V. Stefani, <stefani@yahoo.com>
// Maintainer:	Maciej W. Rozycki, <macro@orcam.me.uk>
//
// if_fddi.h is based on previous if_ether.h and if_tr.h work by
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
// Donald Becker, <becker@super.org>
// Alan Cox, <alan@lxorguk.ukuu.org.uk>
// Steve Whitehouse, <gw7rrm@eeshack3.swan.ac.uk>
// Peter De Schrijver, <stud11@cc4.kuleuven.ac.be>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

//
// Define max and min legal sizes.  The frame sizes do not include
// 4 byte FCS/CRC (frame check sequence).
//

// Define FDDI Frame Control (FC) Byte masks
pub const FDDI_FC_K_CLASS_MASK: c_uint = 0x80	/* class bit */;
pub const FDDI_FC_K_CLASS_SYNC: c_uint = 0x80;
pub const FDDI_FC_K_CLASS_ASYNC: c_uint = 0x00;
pub const FDDI_FC_K_ALEN_MASK: c_uint = 0x40	/* address length bit */;
pub const FDDI_FC_K_ALEN_48: c_uint = 0x40;
pub const FDDI_FC_K_ALEN_16: c_uint = 0x00;
pub const FDDI_FC_K_FORMAT_MASK: c_uint = 0x30	/* format bits */;
pub const FDDI_FC_K_FORMAT_FUTURE: c_uint = 0x30;
pub const FDDI_FC_K_FORMAT_IMPLEMENTOR: c_uint = 0x20;
pub const FDDI_FC_K_FORMAT_LLC: c_uint = 0x10;
pub const FDDI_FC_K_FORMAT_MANAGEMENT: c_uint = 0x00;
pub const FDDI_FC_K_CONTROL_MASK: c_uint = 0x0f	/* control bits */;
// Define FDDI Frame Control (FC) Byte specific values
pub const FDDI_FC_K_VOID: c_uint = 0x00;
pub const FDDI_FC_K_NON_RESTRICTED_TOKEN: c_uint = 0x80;
pub const FDDI_FC_K_RESTRICTED_TOKEN: c_uint = 0xC0;
pub const FDDI_FC_K_SMT_MIN: c_uint = 0x41;
pub const FDDI_FC_K_SMT_MAX: c_uint = 0x4F;
pub const FDDI_FC_K_MAC_MIN: c_uint = 0xC1;
pub const FDDI_FC_K_MAC_MAX: c_uint = 0xCF;
pub const FDDI_FC_K_ASYNC_LLC_MIN: c_uint = 0x50;
pub const FDDI_FC_K_ASYNC_LLC_DEF: c_uint = 0x54;
pub const FDDI_FC_K_ASYNC_LLC_MAX: c_uint = 0x5F;
pub const FDDI_FC_K_SYNC_LLC_MIN: c_uint = 0xD0;
pub const FDDI_FC_K_SYNC_LLC_MAX: c_uint = 0xD7;
pub const FDDI_FC_K_IMPLEMENTOR_MIN: c_uint = 0x60;
pub const FDDI_FC_K_IMPLEMENTOR_MAX: c_uint = 0x6F;
pub const FDDI_FC_K_RESERVED_MIN: c_uint = 0x70;
pub const FDDI_FC_K_RESERVED_MAX: c_uint = 0x7F;
// Define LLC and SNAP constants
pub const FDDI_EXTENDED_SAP: c_uint = 0xAA;
pub const FDDI_UI_CMD: c_uint = 0x03;
// Define 802.2 Type 1 header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fddi_8022_1_hdr {
    pub /: *mut *mut __u8 dsap; / destination service access point,
    pub /: *mut *mut __u8 ssap; / source service access point,
    pub /: *mut *mut __u8 ctrl; / control byte #1,
    pub __attribute__((packed)): },
// Define 802.2 Type 2 header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fddi_8022_2_hdr {
    pub /: *mut *mut __u8 dsap; / destination service access point,
    pub /: *mut *mut __u8 ssap; / source service access point,
    pub /: *mut *mut __u8 ctrl_1; / control byte #1,
    pub /: *mut *mut __u8 ctrl_2; / control byte #2,
    pub __attribute__((packed)): },
// Define 802.2 SNAP header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fddi_snap_hdr {
    pub /: *mut *mut __u8 dsap; / always 0xAA,
    pub /: *mut *mut __u8 ssap; / always 0xAA,
    pub /: *mut *mut __u8 ctrl; / always 0x03,
    pub /: *mut *mut __u8 oui[FDDI_K_OUI_LEN]; / organizational universal id,
    pub /: *mut *mut __be16 ethertype; / packet type ID field,
    pub __attribute__((packed)): },
// Define FDDI LLC frame header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fddihdr {
    pub /: *mut *mut __u8 fc; / frame control,
    pub /: *mut *mut __u8 daddr[FDDI_K_ALEN]; / destination address,
    pub /: *mut *mut __u8 saddr[FDDI_K_ALEN]; / source address,
    pub llc_8022_1: fddi_8022_1_hdr,
    pub llc_8022_2: fddi_8022_2_hdr,
    pub llc_snap: fddi_snap_hdr,
    pub hdr: },
    pub __attribute__((packed)): },
