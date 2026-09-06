//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_sdma.h
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
// Copyright 2018 Advanced Micro Devices, Inc.
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

// max number of IP instances
pub const AMDGPU_MAX_SDMA_INSTANCES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_sdma_irq {
    AMDGPU_SDMA_IRQ_INSTANCE0  = 0,
    AMDGPU_SDMA_IRQ_INSTANCE1,
    AMDGPU_SDMA_IRQ_INSTANCE2,
    AMDGPU_SDMA_IRQ_INSTANCE3,
    AMDGPU_SDMA_IRQ_INSTANCE4,
    AMDGPU_SDMA_IRQ_INSTANCE5,
    AMDGPU_SDMA_IRQ_INSTANCE6,
    AMDGPU_SDMA_IRQ_INSTANCE7,
    AMDGPU_SDMA_IRQ_INSTANCE8,
    AMDGPU_SDMA_IRQ_INSTANCE9,
    AMDGPU_SDMA_IRQ_INSTANCE10,
    AMDGPU_SDMA_IRQ_INSTANCE11,
    AMDGPU_SDMA_IRQ_INSTANCE12,
    AMDGPU_SDMA_IRQ_INSTANCE13,
    AMDGPU_SDMA_IRQ_INSTANCE14,
    AMDGPU_SDMA_IRQ_INSTANCE15,
    AMDGPU_SDMA_IRQ_LAST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_sdma_csa_info {
    pub size: u32,
    pub alignment: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_sdma_funcs {
    pub ring): *mut *mut int (stop_kernel_queue)(struct amdgpu_ring,
    pub ring): *mut *mut int (start_kernel_queue)(struct amdgpu_ring,
    pub instance_id): *mut *mut *mut int (soft_reset_kernel_queue)(struct amdgpu_device adev, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_sdma_instance {
// SDMA firmware
    pub fw: *const firmware,
    pub fw_version: u32,
    pub feature_version: u32,
    pub ring: amdgpu_ring,
    pub page: amdgpu_ring,
    pub burst_nop: bool,
    pub aid_id: u32,
    pub xcc_id: u32,
}

// track guilty state of GFX and PAGE queues
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_sdma_ras {
    pub ras_block: amdgpu_ras_block_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_sdma {
    pub instance: [amdgpu_sdma_instance; AMDGPU_MAX_SDMA_INSTANCES],
    pub trap_irq: amdgpu_irq_src,
    pub illegal_inst_irq: amdgpu_irq_src,
    pub fence_irq: amdgpu_irq_src,
    pub ecc_irq: amdgpu_irq_src,
    pub vm_hole_irq: amdgpu_irq_src,
    pub doorbell_invalid_irq: amdgpu_irq_src,
    pub pool_timeout_irq: amdgpu_irq_src,
    pub srbm_write_irq: amdgpu_irq_src,
    pub ctxt_empty_irq: amdgpu_irq_src,
    pub num_instances: c_int,
    pub sdma_mask: u32,
    pub num_inst_per_aid: c_int,
    pub num_inst_per_xcc: c_int,
}

//
// Provided by hw blocks that can move/clear data.  e.g., gfx or sdma
// But currently, we use sdma to move data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_buffer_funcs {
// maximum bytes in a single operation
    pub copy_max_bytes: u32,
// number of dw to reserve per operation
    pub copy_num_dw: unsigned,
// used for buffer migration
// src addr in bytes
// dst addr in bytes
// number of byte to transfer
    pub copy_flags): u32,
// maximum bytes in a single operation
    pub fill_max_bytes: u32,
// number of dw to reserve per operation
    pub fill_num_dw: unsigned,
// used for buffer clearing
// value to write to memory
// dst addr in bytes
// number of byte to fill
    pub byte_count): u32,
}

extern "C" {
    pub fn amdgpu_sdma_get_index_from_ring(ring: *mut amdgpu_ring, index: *mut u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_sdma_get_csa_mc_addr(ring: *mut amdgpu_ring, vmid: unsigned) -> u64;
}
extern "C" {
    pub fn amdgpu_sdma_ras_sw_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_debugfs_sdma_sched_mask_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_sdma_sysfs_reset_mask_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_sdma_sysfs_reset_mask_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_sdma_is_shared_inv_eng(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) -> bool;
}
