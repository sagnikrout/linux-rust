//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/display/drm_scdc.h
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
// Copyright (c) 2015 NVIDIA Corporation. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sub license,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
pub const SCDC_SINK_VERSION: c_uint = 0x01;
pub const SCDC_SOURCE_VERSION: c_uint = 0x02;
pub const SCDC_UPDATE_0: c_uint = 0x10;

pub const SCDC_UPDATE_1: c_uint = 0x11;
pub const SCDC_TMDS_CONFIG: c_uint = 0x20;

pub const SCDC_SCRAMBLER_STATUS: c_uint = 0x21;

pub const SCDC_CONFIG_0: c_uint = 0x30;

pub const SCDC_STATUS_FLAGS_0: c_uint = 0x40;

pub const SCDC_STATUS_FLAGS_1: c_uint = 0x41;
pub const SCDC_ERR_DET_0_L: c_uint = 0x50;
pub const SCDC_ERR_DET_0_H: c_uint = 0x51;
pub const SCDC_ERR_DET_1_L: c_uint = 0x52;
pub const SCDC_ERR_DET_1_H: c_uint = 0x53;
pub const SCDC_ERR_DET_2_L: c_uint = 0x54;
pub const SCDC_ERR_DET_2_H: c_uint = 0x55;

pub const SCDC_ERR_DET_CHECKSUM: c_uint = 0x56;
pub const SCDC_TEST_CONFIG_0: c_uint = 0xc0;

pub const SCDC_MANUFACTURER_IEEE_OUI: c_uint = 0xd0;
pub const SCDC_MANUFACTURER_IEEE_OUI_SIZE: c_int = 3;
pub const SCDC_DEVICE_ID: c_uint = 0xd3;
pub const SCDC_DEVICE_ID_SIZE: c_int = 8;
pub const SCDC_DEVICE_HARDWARE_REVISION: c_uint = 0xdb;

pub const SCDC_DEVICE_SOFTWARE_MAJOR_REVISION: c_uint = 0xdc;
pub const SCDC_DEVICE_SOFTWARE_MINOR_REVISION: c_uint = 0xdd;
pub const SCDC_MANUFACTURER_SPECIFIC: c_uint = 0xde;
pub const SCDC_MANUFACTURER_SPECIFIC_SIZE: c_int = 34;
