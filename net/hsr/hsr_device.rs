//! Automatically rewritten from C Header to Rust Module
//! Source: net/hsr/hsr_device.h
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
// Copyright 2011-2014 Autronica Fire and Security AS
//
// Author(s):
// 2011-2014 Arvid Brodin, arvid.brodin@alten.se
//
// include file for HSR and PRP.
//

extern "C" {
    pub fn hsr_del_ports(hsr: *mut hsr_priv);
}
extern "C" {
    pub fn hsr_dev_setup(dev: *mut net_device);
}
extern "C" {
    pub fn hsr_check_carrier_and_operstate(hsr: *mut hsr_priv);
}
extern "C" {
    pub fn hsr_get_max_mtu(hsr: *mut hsr_priv) -> c_int;
}
