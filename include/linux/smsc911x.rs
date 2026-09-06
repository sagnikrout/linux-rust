//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/smsc911x.h
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
// Copyright (C) 2004-2008 SMSC
// Copyright (C) 2005-2008 ARM
//

// platform_device configuration data, should be assigned to
// the platform_device's dev.platform_data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smsc911x_platform_config {
    pub irq_polarity: c_uint,
    pub irq_type: c_uint,
    pub flags: c_uint,
    pub shift: c_uint,
    pub phy_interface: phy_interface_t,
    pub mac: [c_uchar; ETH_ALEN],
}

// Constants for platform_device irq polarity configuration
pub const SMSC911X_IRQ_POLARITY_ACTIVE_LOW: c_int = 0;
pub const SMSC911X_IRQ_POLARITY_ACTIVE_HIGH: c_int = 1;
// Constants for platform_device irq type configuration
pub const SMSC911X_IRQ_TYPE_OPEN_DRAIN: c_int = 0;
pub const SMSC911X_IRQ_TYPE_PUSH_PULL: c_int = 1;
// Constants for flags

//
// SMSC911X_SWAP_FIFO:
// Enables software byte swap for fifo data. Should only be used as a
// "last resort" in the case of big endian mode on boards with incorrectly
// routed data bus to older devices such as LAN9118. Newer devices such as
// LAN9221 can handle this in hardware, there are registers to control
// this swapping but the driver doesn't currently use them.
//

