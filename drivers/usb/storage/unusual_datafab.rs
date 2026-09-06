//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/storage/unusual_datafab.h
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
//
// Unusual Devices File for the Datafab USB Compact Flash reader
//

//
// The following Datafab-based devices may or may not work
// using the current driver...the 0xffff is arbitrary since I
// don't know what device versions exist for these guys.
//
// The 0xa003 and 0xa004 devices in particular I'm curious about.
// I'm told they exist but so far nobody has come forward to say that
// they work with this driver.  Given the success we've had getting
// other Datafab-based cards operational with this driver, I've decided
// to leave these two devices in the list.
//
// Reported by Josef Reisinger <josef.reisinger@netcologne.de>
// Submitted by Olaf Hering <olh@suse.de>
//
// Reported by Felix Moeller <felix@derklecks.de>
// in Germany this is sold by Hama with the productnumber 46952
// as "DualSlot CompactFlash(TM) & MStick Drive USB"
//
