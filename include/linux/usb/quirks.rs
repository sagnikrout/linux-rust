//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/quirks.h
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
// This file holds the definitions of quirks found in USB devices.
// Only quirks that affect the whole device, not an interface,
// belong here.
//
// string descriptors must not be fetched using a 255-byte read

// device can't resume correctly so reset it instead

// device can't handle Set-Interface requests

// device can't handle its Configuration or Interface strings

// device can't be reset(e.g morph devices), don't use reset

// device has more interface descriptions than the bNumInterfaces count,

// device needs a pause during initialization, after we read the device

//
// For high speed and super speed interrupt endpoints, the USB 2.0 and
// USB 3.0 spec require the interval in microframes
// (1 microframe = 125 microseconds) to be calculated as
// interval = 2 ^ (bInterval-1).
//
// Devices with this quirk report their bInterval as the result of this
// calculation instead of the exponent variable used in the calculation.
//

// device can't handle device_qualifier descriptor requests

// device generates spurious wakeup, ignore remote wakeup capability

// device can't handle Link Power Management

//
// Device reports its bInterval as linear frames instead of the
// USB 2.0 calculation.
//

//
// Device needs to be disconnected before suspend to prevent spurious
// wakeup.
//

// Device needs a pause after every control message.

// Hub needs extra delay after resetting its port.

// device has endpoints that should be ignored

// short SET_ADDRESS request timeout

// skip BOS descriptor request

// Device claims zero configurations, forcing to 1

// Use a 255 bytes config descriptor request mirroring windows behavior

