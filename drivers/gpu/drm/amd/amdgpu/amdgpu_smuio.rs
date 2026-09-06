//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_smuio.h
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
// Copyright 2020 Advanced Micro Devices, Inc.
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_pkg_type {
    AMDGPU_PKG_TYPE_APU = 2,
    AMDGPU_PKG_TYPE_CEM = 3,
    AMDGPU_PKG_TYPE_OAM = 4,
    AMDGPU_PKG_TYPE_BB  = 5,
    AMDGPU_PKG_TYPE_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_smuio_mcm_config_info {
    pub socket_id: c_int,
    pub die_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_smuio_funcs {
    pub adev): *mut *mut u32 (get_rom_index_offset)(struct amdgpu_device,
    pub adev): *mut *mut u32 (get_rom_data_offset)(struct amdgpu_device,
    pub enable): *mut *mut *mut void (update_rom_clock_gating)(struct amdgpu_device adev, bool,
    pub flags): *mut *mut *mut void (get_clock_gating_state)(struct amdgpu_device adev, u64,
    pub adev): *mut *mut u32 (get_die_id)(struct amdgpu_device,
    pub adev): *mut *mut u32 (get_socket_id)(struct amdgpu_device,
    pub adev): *mut *mut amdgpu_pkg_type (get_pkg_type)(struct amdgpu_device,
    pub adev): *mut *mut bool (is_host_gpu_xgmi_supported)(struct amdgpu_device,
    pub adev): *mut *mut bool (is_connected_with_ethernet_switch)(struct amdgpu_device,
    pub adev): *mut *mut bool (is_custom_hbm_supported)(struct amdgpu_device,
    pub adev): *mut *mut u64 (get_gpu_clock_counter)(struct amdgpu_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_smuio {
    pub funcs: *const amdgpu_smuio_funcs,
}
