//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_irq.h
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

pub const AMDGPU_MAX_IRQ_SRC_ID: c_uint = 0x100;
pub const AMDGPU_MAX_IRQ_CLIENT_ID: c_uint = 0x100;
pub const AMDGPU_IRQ_CLIENTID_LEGACY: c_int = 0;

pub const AMDGPU_IRQ_SRC_DATA_MAX_SIZE_DW: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_interrupt_state {
    AMDGPU_IRQ_STATE_DISABLE,
    AMDGPU_IRQ_STATE_ENABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_iv_entry {
    pub ih: *mut amdgpu_ih_ring,
    pub client_id: unsigned,
    pub src_id: unsigned,
    pub ring_id: unsigned,
    pub vmid: unsigned,
    pub vmid_src: unsigned,
    pub timestamp: u64,
    pub timestamp_src: unsigned,
    pub pasid: unsigned,
    pub node_id: unsigned,
    pub src_data: [unsigned; AMDGPU_IRQ_SRC_DATA_MAX_SIZE_DW],
    pub iv_entry: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_irq_src {
    pub num_types: unsigned,
    pub enabled_types: *mut core::sync::atomic::AtomicI32,
    pub funcs: *const amdgpu_irq_src_funcs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_irq_client {
    pub sources: *mut amdgpu_irq_src,
}

// provided by interrupt generating IP blocks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_irq_src_funcs {
    pub state): unsigned type, enum amdgpu_interrupt_state,
    pub entry): *mut amdgpu_iv_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_irq {
    pub installed: bool,
    pub irq: c_uint,
    pub lock: spinlock_t,
// interrupt sources
    pub client: [amdgpu_irq_client; AMDGPU_IRQ_CLIENTID_MAX],
// status, etc.
    pub /: *mut *mut bool msi_enabled; / msi enabled,
// interrupt rings
    pub ih_soft: amdgpu_ih_ring ih, ih1, ih2,,
    pub ih_funcs: *const amdgpu_ih_funcs,
    pub ih_soft_work: work_ih1_work, ih2_work,,
    pub self_irq: amdgpu_irq_src,
// gen irq stuff
    pub /: *mut *mut *mut irq_domain domain; / GPU irq controller domain,
    pub virq: [unsigned; AMDGPU_MAX_IRQ_SRC_ID],
    pub srbm_soft_reset: u32,
    pub retry_cam_doorbell_index: u32,
    pub retry_cam_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum interrupt_node_id_per_aid {
    AID0_NODEID = 0,
    XCD0_NODEID = 1,
    XCD1_NODEID = 2,
    AID1_NODEID = 4,
    XCD2_NODEID = 5,
    XCD3_NODEID = 6,
    AID2_NODEID = 8,
    XCD4_NODEID = 9,
    XCD5_NODEID = 10,
    AID3_NODEID = 12,
    XCD6_NODEID = 13,
    XCD7_NODEID = 14,
    NODEID_MAX,
}

extern "C" {
    pub fn amdgpu_irq_disable_all(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_irq_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_irq_fini_sw(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_irq_fini_hw(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_irq_gpu_reset_resume_helper(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_irq_add_domain(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_irq_remove_domain(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_irq_create_mapping(adev: *mut amdgpu_device, src_id: unsigned) -> unsigned;
}
extern "C" {
    pub fn amdgpu_restore_msix(adev: *mut amdgpu_device);
}
