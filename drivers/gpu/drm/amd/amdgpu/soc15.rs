//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/soc15.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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

pub const SOC15_FLUSH_GPU_TLB_NUM_WREG: c_int = 6;
pub const SOC15_FLUSH_GPU_TLB_NUM_REG_WAIT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc15_reg_golden {
    pub hwip: u32,
    pub instance: u32,
    pub segment: u32,
    pub reg: u32,
    pub and_mask: u32,
    pub or_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc15_reg_rlcg {
    pub hwip: u32,
    pub instance: u32,
    pub segment: u32,
    pub reg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc15_reg {
    pub hwip: u32,
    pub inst: u32,
    pub seg: u32,
    pub reg_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc15_reg_entry {
    pub hwip: u32,
    pub inst: u32,
    pub seg: u32,
    pub reg_offset: u32,
    pub reg_value: u32,
    pub se_num: u32,
    pub instance: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc15_allowed_register_entry {
    pub hwip: u32,
    pub inst: u32,
    pub seg: u32,
    pub reg_offset: u32,
    pub grbm_indexed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc15_ras_field_entry {
    pub name: *const c_char,
    pub hwip: u32,
    pub inst: u32,
    pub seg: u32,
    pub reg_offset: u32,
    pub sec_count_mask: u32,
    pub sec_count_shift: u32,
    pub ded_count_mask: u32,
    pub ded_count_shift: u32,
}

// Over ride the instance id

extern "C" {
    pub fn soc15_set_virt_ops(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn vega10_reg_base_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn vega20_reg_base_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn arct_reg_base_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn aldebaran_reg_base_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn aqua_vanjaram_init_soc_config(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn vega10_doorbell_index_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn vega20_doorbell_index_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn aqua_vanjaram_doorbell_index_init(adev: *mut amdgpu_device);
}
