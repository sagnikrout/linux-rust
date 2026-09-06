//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/edid.h
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
// drivers/video/edid.h - EDID/DDC Header
//
// Based on:
// 1. XFree86 4.3.0, edid.h
// Copyright 1998 by Egbert Eich <Egbert.Eich@Physik.TU-Darmstadt.DE>
//
// 2. John Fremlin <vii@users.sourceforge.net> and
// Ani Joshi <ajoshi@unixbox.com>
//
// DDC is a Trademark of VESA (Video Electronics Standard Association).
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//
pub const EDID_LENGTH: c_uint = 0x80;
pub const EDID_HEADER: c_uint = 0x00;
pub const EDID_HEADER_END: c_uint = 0x07;
pub const ID_MANUFACTURER_NAME: c_uint = 0x08;
pub const ID_MANUFACTURER_NAME_END: c_uint = 0x09;
pub const ID_MODEL: c_uint = 0x0a;
pub const ID_SERIAL_NUMBER: c_uint = 0x0c;
pub const MANUFACTURE_WEEK: c_uint = 0x10;
pub const MANUFACTURE_YEAR: c_uint = 0x11;
pub const EDID_STRUCT_VERSION: c_uint = 0x12;
pub const EDID_STRUCT_REVISION: c_uint = 0x13;
pub const EDID_STRUCT_DISPLAY: c_uint = 0x14;
pub const DPMS_FLAGS: c_uint = 0x18;
pub const ESTABLISHED_TIMING_1: c_uint = 0x23;
pub const ESTABLISHED_TIMING_2: c_uint = 0x24;
pub const MANUFACTURERS_TIMINGS: c_uint = 0x25;
// standard timings supported
pub const STD_TIMING: c_int = 8;
pub const STD_TIMING_DESCRIPTION_SIZE: c_int = 2;
pub const STD_TIMING_DESCRIPTIONS_START: c_uint = 0x26;
pub const DETAILED_TIMING_DESCRIPTIONS_START: c_uint = 0x36;
pub const DETAILED_TIMING_DESCRIPTION_SIZE: c_int = 18;
pub const NO_DETAILED_TIMING_DESCRIPTIONS: c_int = 4;
pub const DETAILED_TIMING_DESCRIPTION_1: c_uint = 0x36;
pub const DETAILED_TIMING_DESCRIPTION_2: c_uint = 0x48;
pub const DETAILED_TIMING_DESCRIPTION_3: c_uint = 0x5a;
pub const DETAILED_TIMING_DESCRIPTION_4: c_uint = 0x6c;
pub const DESCRIPTOR_DATA: c_int = 5;

