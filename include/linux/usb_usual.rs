//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb_usual.h
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
// Interface to the libusual.
//
// Copyright (c) 2005 Pete Zaitcev <zaitcev@redhat.com>
// Copyright (c) 1999-2002 Matthew Dharm (mdharm-usb@one-eyed-alien.net)
// Copyright (c) 1999 Michael Gee (michael@linuxspecific.com)
//
// We should do this for cleanliness... But other usb_foo.h do not do this.
// #include <linux/usb.h>
//
// The flags field, which we store in usb_device_id.driver_info.
// It is compatible with the old usb-storage flags in lower 24 bits.
//
// Static flag definitions.  We use this roundabout technique so that the
// proc_info() routine can automatically display a message for each flag.
//

// allow access to only LUN 0 */		\
// unusual_devs entry is necessary */		\
// supports multiple targets */			\
// INQUIRY response needs faking */		\
// READ CAPACITY response too big */		\
// reported residue is wrong */			\
// Uses 32-byte CBW length */			\
// PREVENT/ALLOW not supported */		\
// Need delay after Command phase */		\
// Don't check for write-protect */		\
// Sets max_sectors to 64    */			\
// Don't claim device */			\
// sometimes sizes is too big */		\
// Sets max_sectors to arch min */		\
// Ignore tag mismatch in bulk operations */    \
// Sane Sense (> 18 bytes) */			\
// READ CAPACITY response is correct */		\
// Bad Sense (never more than 18 bytes) */	\
// cannot handle READ_DISC_INFO */		\
// cannot handle READ_CAPACITY_16 */		\
// Initial READ(10) (and others) must be retried */	\
// Write Cache status is not available */	\
// cannot handle READ_CAPACITY_10 */		\
// Device advertises UAS but it is broken */	\
// Cannot handle FUA in WRITE or READ CDBs */	\
// Cannot handle ATA_12 or ATA_16 CDBs */	\
// Cannot handle MI_REPORT_SUPPORTED_OPERATION_CODES */	\
// Sets max_sectors to 240 */			\
// Cannot handle REPORT_LUNS */			\
// lies about caching, so always sync */	\
// Cannot handle WRITE_SAME */			\
// Do REQUEST_SENSE after SYNCHRONIZE_CACHE */	\

extern "C" {
    pub fn usb_usual_ignore_device(intf: *mut usb_interface) -> c_int;
}
