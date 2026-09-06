//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/storage/uas-detect.h
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
// ASMedia has a number of usb3 to sata bridge chips, at the time of
// this writing the following versions exist:
// ASM1051 - no uas support version
// ASM1051 - with broken (*) uas support
// ASM1053 - with working uas support, but problems with large xfers
// ASM1153 - with working uas support
//
// Devices with these chips re-use a number of device-ids over the
// entire line, so the device-id is useless to determine if we're
// dealing with an ASM1051 (which we want to avoid).
//
// The ASM1153 can be identified by config.MaxPower == 0,
// where as the ASM105x models have config.MaxPower == 36.
//
// Differentiating between the ASM1053 and ASM1051 is trickier, when
// connected over USB-3 we can look at the number of streams supported,
// ASM1051 supports 32 streams, where as early ASM1053 versions support
// 16 streams, newer ASM1053-s also support 32 streams, but have a
// different prod-id.
//
// (*) ASM1051 chips do work with UAS with some disks (with the
// US_FL_NO_REPORT_OPCODES quirk), but are broken with other disks
//
// ASM1153, do nothing
// No streams info, assume ASM1051
// Possibly an ASM1051, disable uas
// ASM1053, these have issues with large transfers
// All Seagate disk enclosures have broken ATA pass-through support
//
// RTL9210-based enclosure from HIKSEMI, MD202 reportedly have issues
// with UAS.  This isn't distinguishable with just idVendor and
// idProduct, use manufacturer and product too.
//
// Reported-by: Hongling Zeng <zenghongling@kylinos.cn>
//
// flags_ret = flags;
