//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/lmedm04.h
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


// SPDX-License-Identifier: GPL-2.0-only
// DVB USB compliant linux driver for
//
// DM04/QQBOX DVB-S USB BOX	LME2510C + SHARP:BS2F7HZ7395
// LME2510C + LG TDQY-P001F
// LME2510 + LG TDQY-P001F
//
// MVB7395 (LME2510C+SHARP:BS2F7HZ7395)
// SHARP:BS2F7HZ7395 = (STV0288+Sharp IX2505V)
//
// MVB001F (LME2510+LGTDQT-P001F)
// LG TDQY - P001F =(TDA8263 + TDA10086H)
//
// MVB0001F (LME2510C+LGTDQT-P001F)
//
// see Documentation/driver-api/media/drivers/dvb-usb.rst for more information
//
// Streamer &  PID
//
// Note:	These commands do not actually stop the streaming
// but form some kind of packet filtering/stream count
// or tuning related functions.
// 06 XX
// offset 1 = 00 Enable Streaming
//
// PID
// 03 XX XX  ----> reg number ---> setting....20 XX
// offset 1 = length
// offset 2 = start of data
// end byte -1 = 20
// end byte = clear pid always a0, other wise 9c, 9a ??
//

// LNB Voltage
// 07 XX XX
// offset 1 = 01
// offset 2 = 00=Voltage low 01=Voltage high
//
// LNB Power
// 03 01 XX
// offset 2 = 00=ON 01=OFF
//

// Initial stv0288 settings for 7395 Frontend
