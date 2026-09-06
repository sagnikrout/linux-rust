//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/firewire.h
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


// SPDX-License-Identifier: GPL-2.0

// Pseudo L2 address
pub const FWNET_ALEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub union fwnet_hwaddr {
    pub u: [u8; FWNET_ALEN],
// "Hardware address" defined in RFC2734/RF3146
    pub /: *mut *mut __be64 uniq_id; / EUI-64,
    pub /: *mut *mut u8 max_rec; / max packet size,
    pub /: *mut *mut u8 sspd; / max speed,
    pub /: *mut *mut u8 fifo[6]; / FIFO addr,
    pub uc: } __packed,
}

// Pseudo L2 Header
pub const FWNET_HLEN: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwnet_header {
    pub /: *mut *mut u8 h_dest[FWNET_ALEN]; / destination address,
    pub /: *mut *mut __be16 h_proto; / packet type ID field,
    pub __packed: },
