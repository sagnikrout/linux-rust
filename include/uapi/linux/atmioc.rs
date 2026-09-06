//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/atmioc.h
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
// atmioc.h - ranges for ATM-related ioctl numbers
// Written 1995-1999 by Werner Almesberger, EPFL LRC/ICA
//
// See https://icawww1.epfl.ch/linux-atm/magic.html for the complete list of
// "magic" ioctl numbers.
//

// everybody including atmioc.h will also need _IO{,R,W,WR}
pub const ATMIOC_PHYCOM: c_uint = 0x00 /* PHY device common ioctls, globally unique */;
pub const ATMIOC_PHYCOM_END: c_uint = 0x0f;
pub const ATMIOC_PHYTYP: c_uint = 0x10 /* PHY dev type ioctls, unique per PHY type */;
pub const ATMIOC_PHYTYP_END: c_uint = 0x2f;
pub const ATMIOC_PHYPRV: c_uint = 0x30 /* PHY dev private ioctls, unique per driver */;
pub const ATMIOC_PHYPRV_END: c_uint = 0x4f;
pub const ATMIOC_SARCOM: c_uint = 0x50 /* SAR device common ioctls, globally unique */;
pub const ATMIOC_SARCOM_END: c_uint = 0x50;
pub const ATMIOC_SARPRV: c_uint = 0x60 /* SAR dev private ioctls, unique per driver */;
pub const ATMIOC_SARPRV_END: c_uint = 0x7f;
pub const ATMIOC_ITF: c_uint = 0x80 /* Interface ioctls, globally unique */;
pub const ATMIOC_ITF_END: c_uint = 0x8f;
pub const ATMIOC_BACKEND: c_uint = 0x90 /* ATM generic backend ioctls, u. per backend */;
pub const ATMIOC_BACKEND_END: c_uint = 0xaf;
// 0xb0-0xbf: Reserved for future use
pub const ATMIOC_AREQUIPA: c_uint = 0xc0 /* Application requested IP over ATM, glob. u. */;
pub const ATMIOC_LANE: c_uint = 0xd0 /* LAN Emulation, globally unique */;
pub const ATMIOC_MPOA: c_uint = 0xd8 /* MPOA, globally unique */;
pub const ATMIOC_CLIP: c_uint = 0xe0 /* Classical IP over ATM control, globally u. */;
pub const ATMIOC_CLIP_END: c_uint = 0xef;
pub const ATMIOC_SPECIAL: c_uint = 0xf0 /* Special-purpose controls, globally unique */;
pub const ATMIOC_SPECIAL_END: c_uint = 0xff;
