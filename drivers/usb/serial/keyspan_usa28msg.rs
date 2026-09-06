//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/keyspan_usa28msg.h
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


// SPDX-License-Identifier: BSD-3-Clause
//
extern "C" {
    pub fn bytes(62: 1 +, bytes: or 31 data) -> so the maximum length is 63;
}
//
// 19Q: 0x08:CTSflowControl 0x10:DSRflowControl
//
// bit defines in txState
pub const TX_OFF: c_uint = 0x01	// requested by host txOff command;
pub const TX_XOFF: c_uint = 0x02	// either real, or simulated by host;
// ie: the maximum length of an EZUSB endpoint buffer
pub const MAX_DATA_LEN: c_int = 64;
// the parity bytes have only one significant bit
pub const RX_PARITY_BIT: c_uint = 0x04;
pub const TX_PARITY_BIT: c_uint = 0x01;
// update status approx. 60 times a second (16.6666 ms)
pub const STATUS_UPDATE_INTERVAL: c_int = 16;
