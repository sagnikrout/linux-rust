//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/jpeg.h
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
// JPEG markers
pub const JPEG_MARKER_TEM: c_uint = 0x01;
pub const JPEG_MARKER_SOF0: c_uint = 0xc0;
pub const JPEG_MARKER_DHT: c_uint = 0xc4;
pub const JPEG_MARKER_RST: c_uint = 0xd0;
pub const JPEG_MARKER_SOI: c_uint = 0xd8;
pub const JPEG_MARKER_EOI: c_uint = 0xd9;
pub const JPEG_MARKER_SOS: c_uint = 0xda;
pub const JPEG_MARKER_DQT: c_uint = 0xdb;
pub const JPEG_MARKER_DRI: c_uint = 0xdd;
pub const JPEG_MARKER_DHP: c_uint = 0xde;
pub const JPEG_MARKER_APP0: c_uint = 0xe0;
pub const JPEG_MARKER_COM: c_uint = 0xfe;
