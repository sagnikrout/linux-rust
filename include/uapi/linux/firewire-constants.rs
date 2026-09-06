//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/firewire-constants.h
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


//
// IEEE 1394 constants.
//
// Copyright (C) 2005-2007  Kristian Hoegsberg <krh@bitplanet.net>
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
pub const TCODE_WRITE_QUADLET_REQUEST: c_uint = 0x0;
pub const TCODE_WRITE_BLOCK_REQUEST: c_uint = 0x1;
pub const TCODE_WRITE_RESPONSE: c_uint = 0x2;
pub const TCODE_READ_QUADLET_REQUEST: c_uint = 0x4;
pub const TCODE_READ_BLOCK_REQUEST: c_uint = 0x5;
pub const TCODE_READ_QUADLET_RESPONSE: c_uint = 0x6;
pub const TCODE_READ_BLOCK_RESPONSE: c_uint = 0x7;
pub const TCODE_CYCLE_START: c_uint = 0x8;
pub const TCODE_LOCK_REQUEST: c_uint = 0x9;
pub const TCODE_STREAM_DATA: c_uint = 0xa;
pub const TCODE_LOCK_RESPONSE: c_uint = 0xb;
pub const EXTCODE_MASK_SWAP: c_uint = 0x1;
pub const EXTCODE_COMPARE_SWAP: c_uint = 0x2;
pub const EXTCODE_FETCH_ADD: c_uint = 0x3;
pub const EXTCODE_LITTLE_ADD: c_uint = 0x4;
pub const EXTCODE_BOUNDED_ADD: c_uint = 0x5;
pub const EXTCODE_WRAP_ADD: c_uint = 0x6;
pub const EXTCODE_VENDOR_DEPENDENT: c_uint = 0x7;
// Linux firewire-core (Juju) specific tcodes

pub const RCODE_COMPLETE: c_uint = 0x0;
pub const RCODE_CONFLICT_ERROR: c_uint = 0x4;
pub const RCODE_DATA_ERROR: c_uint = 0x5;
pub const RCODE_TYPE_ERROR: c_uint = 0x6;
pub const RCODE_ADDRESS_ERROR: c_uint = 0x7;
// Linux firewire-core (Juju) specific rcodes
pub const RCODE_SEND_ERROR: c_uint = 0x10;
pub const RCODE_CANCELLED: c_uint = 0x11;
pub const RCODE_BUSY: c_uint = 0x12;
pub const RCODE_GENERATION: c_uint = 0x13;
pub const RCODE_NO_ACK: c_uint = 0x14;
pub const SCODE_100: c_uint = 0x0;
pub const SCODE_200: c_uint = 0x1;
pub const SCODE_400: c_uint = 0x2;
pub const SCODE_800: c_uint = 0x3;
pub const SCODE_1600: c_uint = 0x4;
pub const SCODE_3200: c_uint = 0x5;
pub const SCODE_BETA: c_uint = 0x3;
pub const ACK_COMPLETE: c_uint = 0x1;
pub const ACK_PENDING: c_uint = 0x2;
pub const ACK_BUSY_X: c_uint = 0x4;
pub const ACK_BUSY_A: c_uint = 0x5;
pub const ACK_BUSY_B: c_uint = 0x6;
pub const ACK_DATA_ERROR: c_uint = 0xd;
pub const ACK_TYPE_ERROR: c_uint = 0xe;
pub const RETRY_1: c_uint = 0x00;
pub const RETRY_X: c_uint = 0x01;
pub const RETRY_A: c_uint = 0x02;
pub const RETRY_B: c_uint = 0x03;
