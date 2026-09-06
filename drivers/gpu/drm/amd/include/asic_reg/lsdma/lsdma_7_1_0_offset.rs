//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/lsdma/lsdma_7_1_0_offset.h
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
// Copyright 2026 Advanced Micro Devices, Inc.
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

// Macro flag: #define _lsdma_7_1_0_OFFSET_HEADER
pub const regLSDMA_PIO_SRC_ADDR_LO: c_uint = 0x0080;
pub const regLSDMA_PIO_SRC_ADDR_LO_BASE_IDX: c_int = 0;
pub const regLSDMA_PIO_SRC_ADDR_HI: c_uint = 0x0081;
pub const regLSDMA_PIO_SRC_ADDR_HI_BASE_IDX: c_int = 0;
pub const regLSDMA_PIO_DST_ADDR_LO: c_uint = 0x0082;
pub const regLSDMA_PIO_DST_ADDR_LO_BASE_IDX: c_int = 0;
pub const regLSDMA_PIO_DST_ADDR_HI: c_uint = 0x0083;
pub const regLSDMA_PIO_DST_ADDR_HI_BASE_IDX: c_int = 0;
pub const regLSDMA_PIO_COMMAND: c_uint = 0x0084;
pub const regLSDMA_PIO_COMMAND_BASE_IDX: c_int = 0;
pub const regLSDMA_PIO_CONSTFILL_DATA: c_uint = 0x0085;
pub const regLSDMA_PIO_CONSTFILL_DATA_BASE_IDX: c_int = 0;
pub const regLSDMA_PIO_CONTROL: c_uint = 0x0086;
pub const regLSDMA_PIO_CONTROL_BASE_IDX: c_int = 0;
pub const regLSDMA_PIO_STATUS: c_uint = 0x008a;
pub const regLSDMA_PIO_STATUS_BASE_IDX: c_int = 0;
