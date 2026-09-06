//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ralink/rt2x00/rt2800usb.h
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
// 8051 firmware image.
//

pub const FIRMWARE_IMAGE_BASE: c_uint = 0x3000;
//
// DMA descriptor defines.
//

//
// TX Info structure
//
// Word0
// WIV: Wireless Info Valid. 1: Driver filled WI,  0: DMA needs to copy WI
// QSEL: Select on-chip FIFO ID for 2nd-stage output scheduler.
// 0:MGMT, 1:HCCA 2:EDCA
// USB_DMA_NEXT_VALID: Used ONLY in USB bulk Aggregation, NextValid
// DMA_TX_BURST: used ONLY in USB bulk Aggregation.
// Force USB DMA transmit frame from current selected endpoint
//

//
// RX Info structure
//
// Word 0
//

//
// RX descriptor format for RX Ring.
//
// Word0
// UNICAST_TO_ME: This RX frame is unicast to me.
// MULTICAST: This is a multicast frame.
// BROADCAST: This is a broadcast frame.
// MY_BSS: this frame belongs to the same BSSID.
// CRC_ERROR: CRC error.
// CIPHER_ERROR: 0: decryption okay, 1:ICV error, 2:MIC error, 3:KEY not valid.
// AMSDU: rx with 802.3 header, not 802.11 header.
//

