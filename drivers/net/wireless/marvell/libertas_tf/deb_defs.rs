//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/libertas_tf/deb_defs.h
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
// This header file contains global constant/enum definitions,
// global variable declaration.
//

// Macro flag: #define DEBUG
// Macro flag: #define PROC_DEBUG

pub const LBTF_DEB_ENTER: c_uint = 0x00000001;
pub const LBTF_DEB_LEAVE: c_uint = 0x00000002;
pub const LBTF_DEB_MAIN: c_uint = 0x00000004;
pub const LBTF_DEB_NET: c_uint = 0x00000008;
pub const LBTF_DEB_MESH: c_uint = 0x00000010;
pub const LBTF_DEB_WEXT: c_uint = 0x00000020;
pub const LBTF_DEB_IOCTL: c_uint = 0x00000040;
pub const LBTF_DEB_SCAN: c_uint = 0x00000080;
pub const LBTF_DEB_ASSOC: c_uint = 0x00000100;
pub const LBTF_DEB_JOIN: c_uint = 0x00000200;
pub const LBTF_DEB_11D: c_uint = 0x00000400;
pub const LBTF_DEB_DEBUGFS: c_uint = 0x00000800;
pub const LBTF_DEB_ETHTOOL: c_uint = 0x00001000;
pub const LBTF_DEB_HOST: c_uint = 0x00002000;
pub const LBTF_DEB_CMD: c_uint = 0x00004000;
pub const LBTF_DEB_RX: c_uint = 0x00008000;
pub const LBTF_DEB_TX: c_uint = 0x00010000;
pub const LBTF_DEB_USB: c_uint = 0x00020000;
pub const LBTF_DEB_CS: c_uint = 0x00040000;
pub const LBTF_DEB_FW: c_uint = 0x00080000;
pub const LBTF_DEB_THREAD: c_uint = 0x00100000;
pub const LBTF_DEB_HEX: c_uint = 0x00200000;
pub const LBTF_DEB_SDIO: c_uint = 0x00400000;
pub const LBTF_DEB_MACOPS: c_uint = 0x00800000;

