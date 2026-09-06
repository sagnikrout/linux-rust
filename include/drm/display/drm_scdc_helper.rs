//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/display/drm_scdc_helper.h
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

//
// drm_scdc_readb - read a single byte from SCDC
// @adapter: I2C adapter
// @offset: offset of register to read
// @value: return location for the register value
//
// Reads a single byte from SCDC. This is a convenience wrapper around the
// drm_scdc_read() function.
//
// Returns:
// 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn drm_scdc_read(_arg: adapter, _arg: offset, _arg: value, _arg: *mut sizeof(value)) -> return;
}
//
// drm_scdc_writeb - write a single byte to SCDC
// @adapter: I2C adapter
// @offset: offset of register to read
// @value: return location for the register value
//
// Writes a single byte to SCDC. This is a convenience wrapper around the
// drm_scdc_write() function.
//
// Returns:
// 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn drm_scdc_write(_arg: adapter, _arg: offset, _arg: &value, _arg: sizeof(value)) -> return;
}
extern "C" {
    pub fn drm_scdc_get_scrambling_status(connector: *mut drm_connector) -> bool;
}
extern "C" {
    pub fn drm_scdc_set_scrambling(connector: *mut drm_connector, enable: bool) -> bool;
}
extern "C" {
    pub fn drm_scdc_set_high_tmds_clock_ratio(connector: *mut drm_connector, set: bool) -> bool;
}
