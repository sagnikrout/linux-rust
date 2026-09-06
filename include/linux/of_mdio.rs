//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/of_mdio.h
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
// OF helpers for the MDIO (Ethernet PHY) API
//
// Copyright (c) 2009 Secret Lab Technologies, Ltd.
//

extern "C" {
    pub fn of_mdiobus_child_is_phy(child: *mut device_node) -> bool;
}
extern "C" {
    pub fn __of_mdiobus_register(_arg: mdio, _arg: np, _arg: THIS_MODULE) -> return;
}
extern "C" {
    pub fn __devm_of_mdiobus_register(_arg: dev, _arg: mdio, _arg: np, _arg: THIS_MODULE) -> return;
}
extern "C" {
    pub fn of_phy_register_fixed_link(np: *mut device_node) -> c_int;
}
extern "C" {
    pub fn of_phy_deregister_fixed_link(np: *mut device_node);
}
extern "C" {
    pub fn of_phy_is_fixed_link(np: *mut device_node) -> bool;
}
// A PHY must have a reg property in the range [0-31]

//
// Fall back to the non-DT function to register a bus.
// This way, we don't have to keep compat bits around in drivers.
//
extern "C" {
    pub fn mdiobus_register(_arg: mdio) -> return;
}
extern "C" {
    pub fn devm_mdiobus_register(_arg: dev, _arg: mdio) -> return;
}

