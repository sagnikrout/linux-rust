//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ipconfig.h
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
//
// Copyright (C) 1997 Martin Mares
//
// Automatic IP Layer Configuration
//
// The following are initdata:

// bits in ic_proto_{enabled,used}
pub const IC_PROTO: c_uint = 0xFF	/* Protocols mask: */;
pub const IC_BOOTP: c_uint = 0x01	/*   BOOTP (or DHCP, see below) */;
pub const IC_RARP: c_uint = 0x02	/*   RARP */;
pub const IC_USE_DHCP: c_uint = 0x100	/* If on, use DHCP instead of BOOTP */;
