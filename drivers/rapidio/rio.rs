//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/rapidio/rio.h
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
// RapidIO interconnect services
//
// Copyright 2005 MontaVista Software, Inc.
// Matt Porter <mporter@kernel.crashing.org>
//

pub const RIO_MAX_CHK_RETRY: c_int = 3;

// Functions internal to the RIO core code
extern "C" {
    pub fn rio_unlock_device(port: *mut rio_mport, destid: u16, hopcount: u8) -> c_int;
}
extern "C" {
    pub fn rio_route_clr_table(rdev: *mut rio_dev, table: u16, lock: c_int) -> c_int;
}
extern "C" {
    pub fn rio_set_port_lockout(rdev: *mut rio_dev, pnum: u32, lock: c_int) -> c_int;
}
extern "C" {
    pub fn rio_add_net(net: *mut rio_net) -> c_int;
}
extern "C" {
    pub fn rio_free_net(net: *mut rio_net);
}
extern "C" {
    pub fn rio_add_device(rdev: *mut rio_dev) -> c_int;
}
extern "C" {
    pub fn rio_del_device(rdev: *mut rio_dev, state: rio_device_state);
}
extern "C" {
    pub fn rio_register_scan(mport_id: c_int, scan_ops: *mut rio_scan) -> c_int;
}
extern "C" {
    pub fn rio_attach_device(rdev: *mut rio_dev);
}
extern "C" {
    pub fn rio_mport_scan(mport_id: c_int) -> c_int;
}
// Structures internal to the RIO core code

