//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firewire/packet-header-definitions.h
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
// packet-header-definitions.h - The definitions of header fields for IEEE 1394 packet.
//
// Copyright (c) 2024 Takashi Sakamoto

pub const ASYNC_HEADER_QUADLET_COUNT: c_int = 4;
pub const ASYNC_HEADER_Q0_DESTINATION_SHIFT: c_int = 16;
pub const ASYNC_HEADER_Q0_DESTINATION_MASK: c_uint = 0xffff0000;
pub const ASYNC_HEADER_Q0_TLABEL_SHIFT: c_int = 10;
pub const ASYNC_HEADER_Q0_TLABEL_MASK: c_uint = 0x0000fc00;
pub const ASYNC_HEADER_Q0_RETRY_SHIFT: c_int = 8;
pub const ASYNC_HEADER_Q0_RETRY_MASK: c_uint = 0x00000300;
pub const ASYNC_HEADER_Q0_TCODE_SHIFT: c_int = 4;
pub const ASYNC_HEADER_Q0_TCODE_MASK: c_uint = 0x000000f0;
pub const ASYNC_HEADER_Q0_PRIORITY_SHIFT: c_int = 0;
pub const ASYNC_HEADER_Q0_PRIORITY_MASK: c_uint = 0x0000000f;
pub const ASYNC_HEADER_Q1_SOURCE_SHIFT: c_int = 16;
pub const ASYNC_HEADER_Q1_SOURCE_MASK: c_uint = 0xffff0000;
pub const ASYNC_HEADER_Q1_RCODE_SHIFT: c_int = 12;
pub const ASYNC_HEADER_Q1_RCODE_MASK: c_uint = 0x0000f000;
pub const ASYNC_HEADER_Q1_RCODE_SHIFT: c_int = 12;
pub const ASYNC_HEADER_Q1_RCODE_MASK: c_uint = 0x0000f000;
pub const ASYNC_HEADER_Q1_OFFSET_HIGH_SHIFT: c_int = 0;
pub const ASYNC_HEADER_Q1_OFFSET_HIGH_MASK: c_uint = 0x0000ffff;
pub const ASYNC_HEADER_Q3_DATA_LENGTH_SHIFT: c_int = 16;
pub const ASYNC_HEADER_Q3_DATA_LENGTH_MASK: c_uint = 0xffff0000;
pub const ASYNC_HEADER_Q3_EXTENDED_TCODE_SHIFT: c_int = 0;
pub const ASYNC_HEADER_Q3_EXTENDED_TCODE_MASK: c_uint = 0x0000ffff;
pub const ISOC_HEADER_DATA_LENGTH_SHIFT: c_int = 16;
pub const ISOC_HEADER_DATA_LENGTH_MASK: c_uint = 0xffff0000;
pub const ISOC_HEADER_TAG_SHIFT: c_int = 14;
pub const ISOC_HEADER_TAG_MASK: c_uint = 0x0000c000;
pub const ISOC_HEADER_CHANNEL_SHIFT: c_int = 8;
pub const ISOC_HEADER_CHANNEL_MASK: c_uint = 0x00003f00;
pub const ISOC_HEADER_TCODE_SHIFT: c_int = 4;
pub const ISOC_HEADER_TCODE_MASK: c_uint = 0x000000f0;
pub const ISOC_HEADER_SY_SHIFT: c_int = 0;
pub const ISOC_HEADER_SY_MASK: c_uint = 0x0000000f;
// header &= ~ISOC_HEADER_DATA_LENGTH_MASK;
// header |= (((u32)data_length) << ISOC_HEADER_DATA_LENGTH_SHIFT) & ISOC_HEADER_DATA_LENGTH_MASK;
// header &= ~ISOC_HEADER_TAG_MASK;
// header |= (((u32)tag) << ISOC_HEADER_TAG_SHIFT) & ISOC_HEADER_TAG_MASK;
// header &= ~ISOC_HEADER_CHANNEL_MASK;
// header |= (((u32)channel) << ISOC_HEADER_CHANNEL_SHIFT) & ISOC_HEADER_CHANNEL_MASK;
// header &= ~ISOC_HEADER_TCODE_MASK;
// header |= (((u32)tcode) << ISOC_HEADER_TCODE_SHIFT) & ISOC_HEADER_TCODE_MASK;
// header &= ~ISOC_HEADER_SY_MASK;
// header |= (((u32)sy) << ISOC_HEADER_SY_SHIFT) & ISOC_HEADER_SY_MASK;
