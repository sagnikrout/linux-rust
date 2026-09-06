//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_isp.h
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


// SPDX-License-Identifier: MIT
//
// Copyright (C) 2024 Advanced Micro Devices, Inc. All rights reserved.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//

pub const ISP_REGS_OFFSET_END: c_uint = 0x629A4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_funcs {
    pub isp): *mut *mut int (hw_init)(struct amdgpu_isp,
    pub isp): *mut *mut int (hw_fini)(struct amdgpu_isp,
    pub isp): *mut *mut int (hw_suspend)(struct amdgpu_isp,
    pub isp): *mut *mut int (hw_resume)(struct amdgpu_isp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_isp {
    pub parent: *mut device,
    pub adev: *mut amdgpu_device,
    pub funcs: *const isp_funcs,
    pub isp_cell: *mut mfd_cell,
    pub isp_res: *mut resource,
    pub isp_i2c_res: *mut resource,
    pub isp_gpio_res: *mut resource,
    pub isp_pdata: *mut isp_platform_data,
    pub harvest_config: c_uint,
    pub fw: *const firmware,
    pub ispgpd: generic_pm_domain,
}
