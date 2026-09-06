//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/kl5kusb105.h
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
// Definitions for the KLSI KL5KUSB105 serial port adapter
//
// vendor/product pairs that are known to contain this chipset
pub const PALMCONNECT_VID: c_uint = 0x0830;
pub const PALMCONNECT_PID: c_uint = 0x0080;
// Vendor commands:
// port table -- the chip supports up to 4 channels
// baud rates
// data bits
pub const kl5kusb105a_dtb_7: c_int = 7;
pub const kl5kusb105a_dtb_8: c_int = 8;
// requests:
pub const KL5KUSB105A_SIO_SET_DATA: c_int = 1;
pub const KL5KUSB105A_SIO_POLL: c_int = 2;
pub const KL5KUSB105A_SIO_CONFIGURE: c_int = 3;
// values used for request KL5KUSB105A_SIO_CONFIGURE
pub const KL5KUSB105A_SIO_CONFIGURE_READ_ON: c_int = 3;
pub const KL5KUSB105A_SIO_CONFIGURE_READ_OFF: c_int = 2;
// Interpretation of modem status lines
// These need sorting out by individually connecting pins and checking
// results. FIXME!
// When data is being sent we see 0x30 in the lower byte; this must
// contain DSR and CTS ...
//

pub const KL5KUSB105A_WANTS_TO_SEND: c_uint = 0x30;

// Macro flag: #define KL5KUSB105A_LE
// Macro flag: #define KL5KUSB105A_RTS
// Macro flag: #define KL5KUSB105A_ST
// Macro flag: #define KL5KUSB105A_SR

