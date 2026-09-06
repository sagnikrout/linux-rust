//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/phy-caps.h
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
// link caps internal header, for link modes <-> capabilities <-> interfaces
// conversions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_capabilities {
    pub speed: c_int,
    pub duplex: c_uint,
}

extern "C" {
    pub fn phy_caps_init() -> int __init;
}
extern "C" {
    pub fn phy_caps_linkmode_max_speed(max_speed: u32, linkmodes: *mut c_ulong);
}
extern "C" {
    pub fn phy_caps_valid(speed: c_int, duplex: c_int, linkmodes: *const c_ulong) -> bool;
}
extern "C" {
    pub fn phy_caps_linkmodes(caps: c_ulong, linkmodes: *mut c_ulong);
}
extern "C" {
    pub fn phy_caps_from_interface(interface: phy_interface_t) -> c_ulong;
}
extern "C" {
    pub fn phy_caps_mediums_from_linkmodes(linkmodes: *mut c_ulong) -> u32;
}
