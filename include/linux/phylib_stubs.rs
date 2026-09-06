//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/phylib_stubs.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Stubs for the Network PHY library
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phylib_stubs {
    pub config): *mut kernel_hwtstamp_config,
    pub extack): *mut netlink_ext_ack,
    pub phydev_stats): *mut ethtool_phy_stats,
    pub link_stats): *mut ethtool_link_ext_stats,
}

// phylib_register_stubs() and phylib_unregister_stubs()
// also run under rtnl_lock().
//
// phylib_register_stubs() and phylib_unregister_stubs()
// also run under rtnl_lock().
//

