//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_nbio.h
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
// amdgpu nbio functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nbio_hdp_flush_reg {
    pub ref_and_mask_cp0: u32,
    pub ref_and_mask_cp1: u32,
    pub ref_and_mask_cp2: u32,
    pub ref_and_mask_cp3: u32,
    pub ref_and_mask_cp4: u32,
    pub ref_and_mask_cp5: u32,
    pub ref_and_mask_cp6: u32,
    pub ref_and_mask_cp7: u32,
    pub ref_and_mask_cp8: u32,
    pub ref_and_mask_cp9: u32,
    pub ref_and_mask_sdma0: u32,
    pub ref_and_mask_sdma1: u32,
    pub ref_and_mask_sdma2: u32,
    pub ref_and_mask_sdma3: u32,
    pub ref_and_mask_sdma4: u32,
    pub ref_and_mask_sdma5: u32,
    pub ref_and_mask_sdma6: u32,
    pub ref_and_mask_sdma7: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_nbio_ras {
    pub ras_block: amdgpu_ras_block_object,
    pub adev): *mut *mut void (handle_ras_controller_intr_no_bifring)(struct amdgpu_device,
    pub adev): *mut *mut void (handle_ras_err_event_athub_intr_no_bifring)(struct amdgpu_device,
    pub adev): *mut *mut int (init_ras_controller_interrupt)(struct amdgpu_device,
    pub adev): *mut *mut int (init_ras_err_event_athub_interrupt)(struct amdgpu_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_nbio_funcs {
    pub hdp_flush_reg: *const nbio_hdp_flush_reg,
    pub adev): *mut *mut u32 (get_hdp_flush_req_offset)(struct amdgpu_device,
    pub adev): *mut *mut u32 (get_hdp_flush_done_offset)(struct amdgpu_device,
    pub adev): *mut *mut u32 (get_pcie_index_offset)(struct amdgpu_device,
    pub adev): *mut *mut u32 (get_pcie_data_offset)(struct amdgpu_device,
    pub adev): *mut *mut u32 (get_pcie_index_hi_offset)(struct amdgpu_device,
    pub adev): *mut *mut u32 (get_pcie_port_index_offset)(struct amdgpu_device,
    pub adev): *mut *mut u32 (get_pcie_port_data_offset)(struct amdgpu_device,
    pub adev): *mut *mut u32 (get_rev_id)(struct amdgpu_device,
    pub enable): *mut *mut *mut void (mc_access_enable)(struct amdgpu_device adev, bool,
    pub adev): *mut *mut u32 (get_memsize)(struct amdgpu_device,
    pub doorbell_size): bool use_doorbell, int doorbell_index, int,
    pub doorbell_size): bool use_doorbell, int doorbell_index, int,
    pub instance): int doorbell_index, int,
    pub adev): *mut *mut void (gc_doorbell_init)(struct amdgpu_device,
    pub enable): bool,
    pub enable): bool,
    pub doorbell_index): bool use_doorbell, int,
    pub enable): bool,
    pub enable): bool,
    pub enable): bool,
    pub flags): *mut u64,
    pub adev): *mut *mut void (ih_control)(struct amdgpu_device,
    pub adev): *mut *mut void (init_registers)(struct amdgpu_device,
    pub adev): *mut *mut void (remap_hdp_registers)(struct amdgpu_device,
    pub enable): bool,
    pub adev): *mut *mut void (program_aspm)(struct amdgpu_device,
    pub adev): *mut *mut void (apply_lc_spc_mode_wa)(struct amdgpu_device,
    pub adev): *mut *mut void (apply_l1_link_width_reconfig_wa)(struct amdgpu_device,
    pub adev): *mut *mut void (clear_doorbell_interrupt)(struct amdgpu_device,
    pub adev): *mut *mut u32 (get_rom_offset)(struct amdgpu_device,
    pub adev): *mut *mut int (get_compute_partition_mode)(struct amdgpu_device,
    pub supp_modes): *mut u32,
    pub adev): *mut *mut bool (is_nps_switch_requested)(struct amdgpu_device,
    pub adev): *mut *mut u64 (get_pcie_replay_count)(struct amdgpu_device,
    pub adev): *mut *mut void (set_reg_remap)(struct amdgpu_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_nbio {
    pub hdp_flush_reg: *const nbio_hdp_flush_reg,
    pub ras_controller_irq: amdgpu_irq_src,
    pub ras_err_event_athub_irq: amdgpu_irq_src,
    pub ras_if: *mut ras_common_if,
    pub funcs: *const amdgpu_nbio_funcs,
    pub ras: *mut amdgpu_nbio_ras,
}

extern "C" {
    pub fn amdgpu_nbio_ras_sw_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_nbio_ras_late_init(adev: *mut amdgpu_device, ras_block: *mut ras_common_if) -> c_int;
}
extern "C" {
    pub fn amdgpu_nbio_get_pcie_replay_count(adev: *mut amdgpu_device) -> u64;
}
extern "C" {
    pub fn amdgpu_nbio_is_replay_cnt_supported(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_nbio_program_aspm(adev: *mut amdgpu_device);
}
