//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/jpeg_v1_0.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
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
extern "C" {
    pub fn jpeg_v1_0_early_init(ip_block: *mut amdgpu_ip_block) -> c_int;
}
extern "C" {
    pub fn jpeg_v1_0_sw_init(ip_block: *mut amdgpu_ip_block) -> c_int;
}
extern "C" {
    pub fn jpeg_v1_0_sw_fini(ip_block: *mut amdgpu_ip_block);
}
extern "C" {
    pub fn jpeg_v1_0_start(adev: *mut amdgpu_device, mode: c_int);
}
pub const JPEG_V1_REG_RANGE_START: c_uint = 0x8000;
pub const JPEG_V1_REG_RANGE_END: c_uint = 0x803f;
pub const JPEG_V1_LMI_JPEG_WRITE_64BIT_BAR_HIGH: c_uint = 0x8238;
pub const JPEG_V1_LMI_JPEG_WRITE_64BIT_BAR_LOW: c_uint = 0x8239;
pub const JPEG_V1_LMI_JPEG_READ_64BIT_BAR_HIGH: c_uint = 0x825a;
pub const JPEG_V1_LMI_JPEG_READ_64BIT_BAR_LOW: c_uint = 0x825b;
pub const JPEG_V1_REG_CTX_INDEX: c_uint = 0x8328;
pub const JPEG_V1_REG_CTX_DATA: c_uint = 0x8329;
pub const JPEG_V1_REG_SOFT_RESET: c_uint = 0x83a0;
