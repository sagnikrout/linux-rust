//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/storage/unusual_uas.h
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
// Driver for USB Attached SCSI devices - Unusual Devices File
//
// (c) 2013 Hans de Goede <hdegoede@redhat.com>
//
// Based on the same file for the usb-storage driver, which is:
// (c) 2000-2002 Matthew Dharm (mdharm-usb@one-eyed-alien.net)
// (c) 2000 Adam J. Richter (adam@yggdrasil.com), Yggdrasil Computing, Inc.
//
// IMPORTANT NOTE: This file must be included in another file which defines
// a UNUSUAL_DEV macro before this file is included.
//
// If you edit this file, please try to keep it sorted first by VendorID,
// then by ProductID.
//
// If you want to add an entry for this file, be sure to include the
// following information:
// - a patch that adds the entry for your device, including your
// email address right above the entry (plus maybe a brief
// explanation of the reason for the entry),
// - lsusb -v output for the device
// Send your submission to Hans de Goede <hdegoede@redhat.com>
// and don't forget to CC: the USB development list <linux-usb@vger.kernel.org>
//
// Reported-by: Till Dörges <doerges@pre-sense.de>
//
// Initially Reported-by: Julian Groß <julian.g@posteo.de>
// Further reports David C. Partridge <david.partridge@perdrix.co.uk>
//
// Reported-by: Julian Sikorski <belegdol@gmail.com>
// Reported-by: Zhihong Zhou <zhouzhihong@greatwall.com.cn>
// Reported-by: Hongling Zeng <zenghongling@kylinos.cn>
//
// Apricorn USB3 dongle sometimes returns "USBSUSBSUSBS" in response to SCSI
// commands in UAS mode.  Observed with the 1.28 firmware; are there others?
//
// Reported-by: Tom Hu <huxiaoying@kylinos.cn>
// Reported-by: David Webb <djw@noc.ac.uk>
// Reported-by: Oliver Neukum <oneukum@suse.com>
// Reported-by: Benjamin Tissoires <benjamin.tissoires@redhat.com>
// Reported-by: Tom Arild Naess <tanaess@gmail.com>
// Reported-by: Claudio Bizzarri <claudio.bizzarri@gmail.com>
// Reported-by: David Kozub <zub@linux.fjfi.cvut.cz>
// Reported by: Yaroslav Furman <yaro330@gmail.com>
// Reported-by: Sam Burkels <sam@1a38.nl>
// Reported-by: Thinh Nguyen <thinhn@synopsys.com>
// Reported-by: Hongling Zeng <zenghongling@kylinos.cn>
// Reported-by: Hans de Goede <hdegoede@redhat.com>
// Reported-by: Icenowy Zheng <icenowy@aosc.io>
//
// Initially Reported-by: Takeo Nakayama <javhera@gmx.com>
// UAS Ignore Reported by Steven Ellis <sellis@redhat.com>
//
// Reported-by: Hans de Goede <hdegoede@redhat.com>
// Reported-by: Richard Henderson <rth@redhat.com>
// "G-DRIVE" external HDD hangs on write without these.
// Patch submitted by Alexander Kappner <agk@godking.net>
//
