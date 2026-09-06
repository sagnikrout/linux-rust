//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/af_ieee802154.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// IEEE 802.15.4 interface for userspace
//
// Copyright 2007, 2008 Siemens AG
//
// Written by:
// Sergey Lapin <slapin@ossfans.org>
// Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
//

// RESERVED = 0x01,
// address length, octets
pub const IEEE802154_ADDR_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_addr_sa {
    pub addr_type: c_int,
    pub pan_id: u16,
    pub hwaddr: [u8; IEEE802154_ADDR_LEN],
    pub short_addr: u16,
}

pub const IEEE802154_PANID_BROADCAST: c_uint = 0xffff;
pub const IEEE802154_ADDR_BROADCAST: c_uint = 0xffff;
pub const IEEE802154_ADDR_UNDEF: c_uint = 0xfffe;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_ieee802154 {
    pub /: *mut *mut sa_family_t family; / AF_IEEE802154,
    pub addr: ieee802154_addr_sa,
}

// get/setsockopt
pub const SOL_IEEE802154: c_int = 0;
pub const WPAN_WANTACK: c_int = 0;
pub const WPAN_SECURITY: c_int = 1;
pub const WPAN_SECURITY_LEVEL: c_int = 2;
pub const WPAN_WANTLQI: c_int = 3;
pub const WPAN_SECURITY_DEFAULT: c_int = 0;
pub const WPAN_SECURITY_OFF: c_int = 1;
pub const WPAN_SECURITY_ON: c_int = 2;

