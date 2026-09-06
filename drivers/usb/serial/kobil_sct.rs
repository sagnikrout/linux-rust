//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/kobil_sct.h
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
pub const SUSBCRequest_SetBaudRateParityAndStopBits: c_int = 1;
pub const SUSBCR_SBR_MASK: c_uint = 0xFF00;
pub const SUSBCR_SBR_1200: c_uint = 0x0100;
pub const SUSBCR_SBR_9600: c_uint = 0x0200;
pub const SUSBCR_SBR_19200: c_uint = 0x0400;
pub const SUSBCR_SBR_28800: c_uint = 0x0800;
pub const SUSBCR_SBR_38400: c_uint = 0x1000;
pub const SUSBCR_SBR_57600: c_uint = 0x2000;
pub const SUSBCR_SBR_115200: c_uint = 0x4000;
pub const SUSBCR_SPASB_MASK: c_uint = 0x0070;
pub const SUSBCR_SPASB_NoParity: c_uint = 0x0010;
pub const SUSBCR_SPASB_OddParity: c_uint = 0x0020;
pub const SUSBCR_SPASB_EvenParity: c_uint = 0x0040;
pub const SUSBCR_SPASB_STPMASK: c_uint = 0x0003;
pub const SUSBCR_SPASB_1StopBit: c_uint = 0x0001;
pub const SUSBCR_SPASB_2StopBits: c_uint = 0x0002;
pub const SUSBCRequest_SetStatusLinesOrQueues: c_int = 2;
pub const SUSBCR_SSL_SETRTS: c_uint = 0x0001;
pub const SUSBCR_SSL_CLRRTS: c_uint = 0x0002;
pub const SUSBCR_SSL_SETDTR: c_uint = 0x0004;
pub const SUSBCR_SSL_CLRDTR: c_uint = 0x0010;
// Kill the pending/current writes to the comm port.
pub const SUSBCR_SSL_PURGE_TXABORT: c_uint = 0x0100;
// Kill the pending/current reads to the comm port.
pub const SUSBCR_SSL_PURGE_RXABORT: c_uint = 0x0200;
// Kill the transmit queue if there.
pub const SUSBCR_SSL_PURGE_TXCLEAR: c_uint = 0x0400;
// Kill the typeahead buffer if there.
pub const SUSBCR_SSL_PURGE_RXCLEAR: c_uint = 0x0800;
pub const SUSBCRequest_GetStatusLineState: c_int = 4;
// Any Character received
pub const SUSBCR_GSL_RXCHAR: c_uint = 0x0001;
// Transmitt Queue Empty
pub const SUSBCR_GSL_TXEMPTY: c_uint = 0x0004;
// CTS changed state
pub const SUSBCR_GSL_CTS: c_uint = 0x0008;
// DSR changed state
pub const SUSBCR_GSL_DSR: c_uint = 0x0010;
// RLSD changed state
pub const SUSBCR_GSL_RLSD: c_uint = 0x0020;
// BREAK received
pub const SUSBCR_GSL_BREAK: c_uint = 0x0040;
// Line status error occurred
pub const SUSBCR_GSL_ERR: c_uint = 0x0080;
// Ring signal detected
pub const SUSBCR_GSL_RING: c_uint = 0x0100;
pub const SUSBCRequest_Misc: c_int = 8;
// use a predefined reset sequence
pub const SUSBCR_MSC_ResetReader: c_uint = 0x0001;
// use a predefined sequence to reset the internal queues
pub const SUSBCR_MSC_ResetAllQueues: c_uint = 0x0002;
pub const SUSBCRequest_GetMisc: c_uint = 0x10;
//
// get the firmware version from device, coded like this 0xHHLLBBPP with
// HH = Firmware Version High Byte
// LL = Firmware Version Low Byte
// BB = Build Number
// PP = Further Attributes
//
pub const SUSBCR_MSC_GetFWVersion: c_uint = 0x0001;
//
// get the hardware version from device coded like this 0xHHLLPPRR with
// HH = Software Version High Byte
// LL = Software Version Low Byte
// PP = Further Attributes
// RR = Reserved for the hardware ID
//
pub const SUSBCR_MSC_GetHWVersion: c_uint = 0x0002;
