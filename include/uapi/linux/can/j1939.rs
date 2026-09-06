//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/can/j1939.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// j1939.h
//
// Copyright (c) 2010-2011 EIA Electronics
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

pub const J1939_MAX_UNICAST_ADDR: c_uint = 0xfd;
pub const J1939_IDLE_ADDR: c_uint = 0xfe;
pub const J1939_NO_ADDR: c_uint = 0xff		/* == broadcast or no addr */;
pub const J1939_NO_NAME: c_int = 0;
pub const J1939_PGN_REQUEST: c_uint = 0x0ea00		/* Request PG */;
pub const J1939_PGN_ADDRESS_CLAIMED: c_uint = 0x0ee00	/* Address Claimed */;
pub const J1939_PGN_ADDRESS_COMMANDED: c_uint = 0x0fed8	/* Commanded Address */;
pub const J1939_PGN_PDU1_MAX: c_uint = 0x3ff00;
pub const J1939_PGN_MAX: c_uint = 0x3ffff;
pub const J1939_NO_PGN: c_uint = 0x40000;
// J1939 Parameter Group Number
//
// bit 0-7	: PDU Specific (PS)
// bit 8-15	: PDU Format (PF)
// bit 16	: Data Page (DP)
// bit 17	: Reserved (R)
// bit 19-31	: set to zero
//
pub type pgn_t = __u32;
// J1939 Priority
//
// bit 0-2	: Priority (P)
// bit 3-7	: set to zero
//
pub type priority_t = __u8;
// J1939 NAME
//
// bit 0-20	: Identity Number
// bit 21-31	: Manufacturer Code
// bit 32-34	: ECU Instance
// bit 35-39	: Function Instance
// bit 40-47	: Function
// bit 48	: Reserved
// bit 49-55	: Vehicle System
// bit 56-59	: Vehicle System Instance
// bit 60-62	: Industry Group
// bit 63	: Arbitrary Address Capable
//
pub type name_t = __u64;
// J1939 socket options

#[repr(C)]
#[derive(Copy, Clone)]
pub struct j1939_filter {
    pub name: name_t,
    pub name_mask: name_t,
    pub pgn: pgn_t,
    pub pgn_mask: pgn_t,
    pub addr: __u8,
    pub addr_mask: __u8,
}

