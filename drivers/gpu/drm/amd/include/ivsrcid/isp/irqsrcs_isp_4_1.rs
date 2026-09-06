//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/ivsrcid/isp/irqsrcs_isp_4_1.h
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
// Copyright 2024 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
pub const ISP_4_1__SRCID__ISP_SEMA_WAIT_FAIL_TIMEOUT: c_uint = 0x12	// Semaphore wait fail timeout;
pub const ISP_4_1__SRCID__ISP_SEMA_WAIT_INCOMPLETE_TIMEOUT: c_uint = 0x13	// Semaphore wait incomplete timeout;
pub const ISP_4_1__SRCID__ISP_SEMA_SIGNAL_INCOMPLETE_TIMEOUT: c_uint = 0x14	// Semaphore signal incomplete timeout;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_BASE5_CHANGED: c_uint = 0x15	// Ringbuffer base5 address changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_WPT5: c_uint = 0x16	// Ringbuffer write point 5 changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_BASE6_CHANGED: c_uint = 0x17	// Ringbuffer base6 address changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_WPT6: c_uint = 0x18	// Ringbuffer write point 6 changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_BASE7_CHANGED: c_uint = 0x19	// Ringbuffer base7 address changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_WPT7: c_uint = 0x1A	// Ringbuffer write point 7 changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_BASE8_CHANGED: c_uint = 0x1B	// Ringbuffer base8 address changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_WPT8: c_uint = 0x1C	// Ringbuffer write point 8 changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_BASE9_CHANGED: c_uint = 0x00    // Ringbuffer base9 address changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_WPT9: c_uint = 0x01    // Ringbuffer write point 9 changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_BASE10_CHANGED: c_uint = 0x02    // Ringbuffer base10 address changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_WPT10: c_uint = 0x03    // Ringbuffer write point 10 changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_BASE11_CHANGED: c_uint = 0x04    // Ringbuffer base11 address changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_WPT11: c_uint = 0x05    // Ringbuffer write point 11 changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_BASE12_CHANGED: c_uint = 0x06    // Ringbuffer base12 address changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_WPT12: c_uint = 0x07    // Ringbuffer write point 12 changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_BASE13_CHANGED: c_uint = 0x08    // Ringbuffer base13 address changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_WPT13: c_uint = 0x09    // Ringbuffer write point 13 changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_BASE14_CHANGED: c_uint = 0x0A    // Ringbuffer base14 address changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_WPT14: c_uint = 0x0B    // Ringbuffer write point 14 changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_BASE15_CHANGED: c_uint = 0x0C    // Ringbuffer base15 address changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_WPT15: c_uint = 0x0D    // Ringbuffer write point 15 changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_BASE16_CHANGED: c_uint = 0x0E    // Ringbuffer base16 address changed;
pub const ISP_4_1__SRCID__ISP_RINGBUFFER_WPT16: c_uint = 0x0F    // Ringbuffer write point 16 changed;
pub const ISP_4_1__SRCID__ISP_MIPI0: c_uint = 0x29	// MIPI0 interrupt;
pub const ISP_4_1__SRCID__ISP_MIPI1: c_uint = 0x2A	// MIPI1 interrupt;
pub const ISP_4_1__SRCID__ISP_I2C0: c_uint = 0x2B	// I2C0 PAD interrupt;
pub const ISP_4_1__SRCID__ISP_I2C1: c_uint = 0x2C	// I2C1 PAD interrupt;
pub const ISP_4_1__SRCID__ISP_FLASH0: c_uint = 0x2D	// Flash0 interrupt;
pub const ISP_4_1__SRCID__ISP_FLASH1: c_uint = 0x2E	// Flash1 interrupt;
pub const ISP_4_1__SRCID__ISP_DEBUG: c_uint = 0x2F	// Debug information;
