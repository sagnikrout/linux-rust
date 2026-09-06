//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/8xx/mpc885ads.h
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


//
// A collection of structures, addresses, and values associated with
// the Freescale MPC885ADS board.
// Copied from the FADS stuff.
//
// Author: MontaVista Software, Inc.
// source@mvista.com
//
// 2005 (c) MontaVista Software, Inc.  This file is licensed under the
// terms of the GNU General Public License version 2.  This program is licensed
// "as is" without any warranty of any kind, whether express or implied.
//

// Bits of interest in the BCSRs.
//

pub const BCSR5_MII2_EN: c_uint = 0x40;
pub const BCSR5_MII2_RST: c_uint = 0x20;
pub const BCSR5_T1_RST: c_uint = 0x10;
pub const BCSR5_ATM155_RST: c_uint = 0x08;
pub const BCSR5_ATM25_RST: c_uint = 0x04;
pub const BCSR5_MII1_EN: c_uint = 0x02;
pub const BCSR5_MII1_RST: c_uint = 0x01;

