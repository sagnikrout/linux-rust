//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_vpe.h
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
// Copyright 2022 Advanced Micro Devices, Inc.
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

pub const AMDGPU_MAX_VPE_INSTANCES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpe_funcs {
    pub offset): *mut *mut *mut uint32_t (get_reg_offset)(struct amdgpu_vpe vpe, uint32_t inst, uint32_t,
    pub vpe): *mut *mut int (set_regs)(struct amdgpu_vpe,
    pub vpe): *mut *mut int (irq_init)(struct amdgpu_vpe,
    pub vpe): *mut *mut int (init_microcode)(struct amdgpu_vpe,
    pub vpe): *mut *mut int (load_microcode)(struct amdgpu_vpe,
    pub vpe): *mut *mut int (ring_init)(struct amdgpu_vpe,
    pub vpe): *mut *mut int (ring_start)(struct amdgpu_vpe,
    pub vpe): *mut *mut int (ring_stop)(struct amdgpu_vpe,
    pub vpe): *mut *mut int (ring_fini)(struct amdgpu_vpe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpe_regs {
    pub queue0_rb_rptr_lo: u32,
    pub queue0_rb_rptr_hi: u32,
    pub queue0_rb_wptr_lo: u32,
    pub queue0_rb_wptr_hi: u32,
    pub queue0_preempt: u32,
    pub dpm_enable: u32,
    pub dpm_pratio: u32,
    pub dpm_request_interval: u32,
    pub dpm_decision_threshold: u32,
    pub dpm_busy_clamp_threshold: u32,
    pub dpm_idle_clamp_threshold: u32,
    pub dpm_request_lv: u32,
    pub context_indicator: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vpe {
    pub ring: amdgpu_ring,
    pub trap_irq: amdgpu_irq_src,
    pub funcs: *const vpe_funcs,
    pub regs: vpe_regs,
    pub fw: *const firmware,
    pub fw_version: u32,
    pub feature_version: u32,
    pub cmdbuf_obj: *mut amdgpu_bo,
    pub cmdbuf_gpu_addr: u64,
    pub cmdbuf_cpu_addr: *mut u32,
    pub idle_work: delayed_work,
    pub context_started: bool,
    pub num_instances: u32,
    pub collaborate_mode: bool,
    pub supported_reset: u32,
}

extern "C" {
    pub fn amdgpu_vpe_psp_update_sram(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_vpe_init_microcode(vpe: *mut amdgpu_vpe) -> c_int;
}
extern "C" {
    pub fn amdgpu_vpe_ring_init(vpe: *mut amdgpu_vpe) -> c_int;
}
extern "C" {
    pub fn amdgpu_vpe_ring_fini(vpe: *mut amdgpu_vpe) -> c_int;
}
extern "C" {
    pub fn amdgpu_vpe_configure_dpm(vpe: *mut amdgpu_vpe) -> c_int;
}
extern "C" {
    pub fn amdgpu_vpe_sysfs_reset_mask_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_vpe_sysfs_reset_mask_init(adev: *mut amdgpu_device) -> c_int;
}

