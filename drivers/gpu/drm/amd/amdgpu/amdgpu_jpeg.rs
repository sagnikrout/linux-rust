//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_jpeg.h
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

pub const AMDGPU_MAX_JPEG_INSTANCES: c_int = 4;
pub const AMDGPU_MAX_JPEG_RINGS: c_int = 10;
pub const AMDGPU_MAX_JPEG_RINGS_4_0_3: c_int = 8;
pub const JPEG_REG_RANGE_START: c_uint = 0x4000;
pub const JPEG_REG_RANGE_END: c_uint = 0x41c2;
pub const JPEG_ATOMIC_RANGE_START: c_uint = 0x4120;
pub const JPEG_ATOMIC_RANGE_END: c_uint = 0x412A;

// adev->jpeg.inst[inst_idx].dpg_sram_curr_addr++ =		\

// adev->jpeg.inst[inst_idx].dpg_sram_curr_addr++ = offset;	\
// adev->jpeg.inst[inst_idx].dpg_sram_curr_addr++ = value;	\
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_jpeg_caps {
    AMDGPU_JPEG_RRMT_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_jpeg_reg {
    pub jpeg_pitch: [unsigned; AMDGPU_MAX_JPEG_RINGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_jpeg_inst {
    pub ring_dec: [amdgpu_ring; AMDGPU_MAX_JPEG_RINGS],
    pub irq: amdgpu_irq_src,
    pub ras_poison_irq: amdgpu_irq_src,
    pub external: amdgpu_jpeg_reg,
    pub dpg_sram_bo: *mut amdgpu_bo,
    pub pause_state: dpg_pause_state,
    pub dpg_sram_cpu_addr: *mut c_void,
    pub dpg_sram_gpu_addr: u64,
    pub dpg_sram_curr_addr: *mut u32,
    pub aid_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_jpeg_ras {
    pub ras_block: amdgpu_ras_block_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_jpeg {
    pub num_jpeg_inst: u8,
    pub inst: [amdgpu_jpeg_inst; AMDGPU_MAX_JPEG_INSTANCES],
    pub num_jpeg_rings: unsigned,
    pub internal: amdgpu_jpeg_reg,
    pub harvest_config: unsigned,
    pub idle_work: delayed_work,
    pub cur_state: amd_powergating_state,
    pub jpeg_pg_lock: mutex,
    pub total_submission_cnt: core::sync::atomic::AtomicI32,
    pub ras_if: *mut ras_common_if,
    pub ras: *mut amdgpu_jpeg_ras,
    pub inst_mask: u16,
    pub num_inst_per_aid: u8,
    pub indirect_sram: bool,
    pub supported_reset: u32,
    pub caps: u32,
    pub ip_dump: *mut u32,
    pub reg_count: u32,
    pub reg_list: *const amdgpu_hwip_reg_entry,
    pub disable_uq: bool,
    pub disable_kq: bool,
}

extern "C" {
    pub fn amdgpu_jpeg_sw_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_jpeg_sw_fini(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_jpeg_suspend(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_jpeg_resume(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_jpeg_ring_begin_use(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_jpeg_ring_end_use(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_jpeg_dec_ring_test_ring(ring: *mut amdgpu_ring) -> c_int;
}
extern "C" {
    pub fn amdgpu_jpeg_dec_ring_test_ib(ring: *mut amdgpu_ring, timeout: c_long) -> c_int;
}
extern "C" {
    pub fn amdgpu_jpeg_ras_sw_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_debugfs_jpeg_sched_mask_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_jpeg_sysfs_reset_mask_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_jpeg_sysfs_reset_mask_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_jpeg_dump_ip_state(ip_block: *mut amdgpu_ip_block);
}
extern "C" {
    pub fn amdgpu_jpeg_print_ip_state(ip_block: *mut amdgpu_ip_block, p: *mut drm_printer);
}
