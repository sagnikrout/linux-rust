//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_atombios.h
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
// Copyright 2014 Advanced Micro Devices, Inc.
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
#[derive(Copy, Clone)]
pub struct atom_clock_dividers {
    pub post_div: u32,

    pub 6: u32 reserved :,
    pub 12: u32 whole_fb_div :,
    pub 14: u32 frac_fb_div :,

    pub 14: u32 frac_fb_div :,
    pub 12: u32 whole_fb_div :,
    pub 6: u32 reserved :,

}

// added for CI
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_mpll_param {

    pub 8: u32 reserved :,
    pub 12: u32 clkfrac :,
    pub 12: u32 clkf :,

    pub 12: u32 clkf :,
    pub 12: u32 clkfrac :,
    pub 8: u32 reserved :,

}

pub const MEM_TYPE_GDDR5: c_uint = 0x50;
pub const MEM_TYPE_GDDR4: c_uint = 0x40;
pub const MEM_TYPE_GDDR3: c_uint = 0x30;
pub const MEM_TYPE_DDR2: c_uint = 0x20;
pub const MEM_TYPE_GDDR1: c_uint = 0x10;
pub const MEM_TYPE_DDR3: c_uint = 0xb0;
pub const MEM_TYPE_MASK: c_uint = 0xf0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_memory_info {
    pub mem_vendor: u8,
    pub mem_type: u8,
}

pub const MAX_AC_TIMING_ENTRIES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_memory_clock_range_table {
    pub num_entries: u8,
    pub rsv: [u8; 3],
    pub mclk: [u32; MAX_AC_TIMING_ENTRIES],
}

pub const VBIOS_MC_REGISTER_ARRAY_SIZE: c_int = 32;
pub const VBIOS_MAX_AC_TIMING_ENTRIES: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_mc_reg_entry {
    pub mclk_max: u32,
    pub mc_data: [u32; VBIOS_MC_REGISTER_ARRAY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_mc_register_address {
    pub s1: u16,
    pub pre_reg_data: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_mc_reg_table {
    pub last: u8,
    pub num_entries: u8,
    pub mc_reg_table_entry: [atom_mc_reg_entry; VBIOS_MAX_AC_TIMING_ENTRIES],
    pub mc_reg_address: [atom_mc_register_address; VBIOS_MC_REGISTER_ARRAY_SIZE],
}

pub const MAX_VOLTAGE_ENTRIES: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_voltage_table_entry {
    pub value: u16,
    pub smio_low: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_voltage_table {
    pub count: u32,
    pub mask_low: u32,
    pub phase_delay: u32,
    pub entries: [atom_voltage_table_entry; MAX_VOLTAGE_ENTRIES],
}

extern "C" {
    pub fn amdgpu_atombios_i2c_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_atombios_oem_i2c_init(adev: *mut amdgpu_device, i2c_id: u8);
}
extern "C" {
    pub fn amdgpu_atombios_has_dce_engine_info(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_atombios_get_connector_info_from_object_table(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_atombios_get_clock_info(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_atombios_get_gfx_info(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_atombios_get_vram_width(adev: *mut amdgpu_device) -> c_int;
}

extern "C" {
    pub fn amdgpu_atombios_has_gpu_virtualization_table(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_atombios_scratch_regs_lock(adev: *mut amdgpu_device, lock: bool);
}
extern "C" {
    pub fn amdgpu_atombios_scratch_need_asic_init(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_atombios_copy_swap(dst: *mut u8, src: *mut u8, num_bytes: u8, to_le: bool);
}
extern "C" {
    pub fn amdgpu_atombios_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_atombios_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_atombios_sysfs_init(adev: *mut amdgpu_device) -> c_int;
}
