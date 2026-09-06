//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/core/otg_productlist.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (C) 2004 Texas Instruments
//
// This OTG and Embedded Host list is "Targeted Peripheral List".
// It should mostly use of USB_DEVICE() or USB_DEVICE_VER() entries..
//
// YOU _SHOULD_ CHANGE THIS LIST TO MATCH YOUR PRODUCT AND ITS TESTING!
//
// hubs are optional in OTG, but very handy ...

// FIXME actually, printers are NOT supposed to use device classes;
// they're supposed to use interface classes...
//

// Linux-USB CDC Ethernet gadget
// Linux-USB CDC Ethernet + RNDIS gadget

// gadget zero, for testing

// HNP test device is _never_ targeted (see OTG spec 6.6.6)
// OTG PET device is always targeted (see OTG 2.0 ECN 6.4.2)
// NOTE: can't use usb_match_id() since interface caches
// aren't set up yet. this is cut/paste from that code.
//
// No need to test id->bcdDevice_lo != 0, since 0 is never
// add other match criteria here ...
// OTG MESSAGE: report errors here, customize to match your product
