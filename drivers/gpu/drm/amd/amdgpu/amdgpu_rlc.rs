//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_rlc.h
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

pub const AMDGPU_MAX_RLC_INSTANCES: c_int = 8;
// firmware ID used in rlc toc
pub const RLC_TOC_MAX_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_rlc_funcs {
    pub adev): *mut *mut bool (is_rlc_enabled)(struct amdgpu_device,
    pub xcc_id): *mut *mut *mut void (set_safe_mode)(struct amdgpu_device adev, int,
    pub xcc_id): *mut *mut *mut void (unset_safe_mode)(struct amdgpu_device adev, int,
    pub adev): *mut *mut int (init)(struct amdgpu_device,
    pub adev): *mut *mut u32 (get_csb_size)(struct amdgpu_device,
//
// @get_csb_buffer: Get the clear state to be put into the hardware.
//
// The parameter adev is used to get the CS data and other gfx info,
// and buffer is the RLC CS pointer
//
// Sometimes, the user space puts a request to clear the state in the
// command buffer; this function provides the clear state that gets put
// into the hardware. Note that the driver programs Clear State
// Indirect Buffer (CSB) explicitly when it sets up the kernel rings,
// and it also provides a pointer to it which is used by the firmware
// to load the clear state in some cases.
//
    pub buffer): *mut *mut *mut void (get_csb_buffer)(struct amdgpu_device adev, u32,
    pub adev): *mut *mut int (get_cp_table_num)(struct amdgpu_device,
    pub adev): *mut *mut int (resume)(struct amdgpu_device,
    pub adev): *mut *mut void (stop)(struct amdgpu_device,
    pub adev): *mut *mut void (reset)(struct amdgpu_device,
    pub adev): *mut *mut void (start)(struct amdgpu_device,
    pub vmid): *mut *mut amdgpu_ring ring, unsigned,
    pub reg): *mut *mut *mut bool (is_rlcg_access_range)(struct amdgpu_device adev, uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_rlc_reg_funcs {
    pub xcc_id): *mut *mut *mut u32 (rreg32)(struct amdgpu_device adev, u32 reg, u32 acc_flags, u32 hwip, u32,
    pub xcc_id): *mut *mut *mut void (wreg32)(struct amdgpu_device adev, u32 reg, u32 val, u32 acc_flags, u32 hwip, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_rlcg_reg_access_ctrl {
    pub scratch_reg0: u32,
    pub scratch_reg1: u32,
    pub scratch_reg2: u32,
    pub scratch_reg3: u32,
    pub grbm_cntl: u32,
    pub grbm_idx: u32,
    pub spare_int: u32,
    pub vfi_cmd: u32,
    pub vfi_stat: u32,
    pub vfi_addr: u32,
    pub vfi_data: u32,
    pub vfi_grbm_cntl: u32,
    pub vfi_grbm_idx: u32,
    pub vfi_grbm_cntl_data: u32,
    pub vfi_grbm_idx_data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_rlc {
// for power gating
    pub save_restore_obj: *mut amdgpu_bo,
    pub save_restore_gpu_addr: u64,
    pub sr_ptr: *mut u32,
    pub reg_list: *const u32,
    pub reg_list_size: u32,
// for clear state
    pub clear_state_obj: *mut amdgpu_bo,
    pub clear_state_gpu_addr: u64,
    pub cs_ptr: *mut u32,
    pub cs_data: *const cs_section_def,
    pub clear_state_size: u32,
// for cp tables
    pub cp_table_obj: *mut amdgpu_bo,
    pub cp_table_gpu_addr: u64,
    pub cp_table_ptr: *mut u32,
    pub cp_table_size: u32,
// safe mode for updating CG/PG state
    pub in_safe_mode: [bool; AMDGPU_MAX_RLC_INSTANCES],
    pub funcs: *const amdgpu_rlc_funcs,
    pub reg_funcs: *const amdgpu_rlc_reg_funcs,
// for firmware data
    pub save_and_restore_offset: u32,
    pub clear_state_descriptor_offset: u32,
    pub avail_scratch_ram_locations: u32,
    pub reg_restore_list_size: u32,
    pub reg_list_format_start: u32,
    pub reg_list_format_separate_start: u32,
    pub starting_offsets_start: u32,
    pub reg_list_format_size_bytes: u32,
    pub reg_list_size_bytes: u32,
    pub reg_list_format_direct_reg_list_length: u32,
    pub save_restore_list_cntl_size_bytes: u32,
    pub save_restore_list_gpm_size_bytes: u32,
    pub save_restore_list_srm_size_bytes: u32,
    pub rlc_iram_ucode_size_bytes: u32,
    pub rlc_dram_ucode_size_bytes: u32,
    pub rlc_1_iram_ucode_size_bytes: u32,
    pub rlc_1_dram_ucode_size_bytes: u32,
    pub rlcp_ucode_size_bytes: u32,
    pub rlcv_ucode_size_bytes: u32,
    pub global_tap_delays_ucode_size_bytes: u32,
    pub se0_tap_delays_ucode_size_bytes: u32,
    pub se1_tap_delays_ucode_size_bytes: u32,
    pub se2_tap_delays_ucode_size_bytes: u32,
    pub se3_tap_delays_ucode_size_bytes: u32,
    pub register_list_format: *mut u32,
    pub register_restore: *mut u32,
    pub save_restore_list_cntl: *mut u8,
    pub save_restore_list_gpm: *mut u8,
    pub save_restore_list_srm: *mut u8,
    pub rlc_iram_ucode: *mut u8,
    pub rlc_dram_ucode: *mut u8,
    pub rlc_1_iram_ucode: *mut u8,
    pub rlc_1_dram_ucode: *mut u8,
    pub rlcp_ucode: *mut u8,
    pub rlcv_ucode: *mut u8,
    pub global_tap_delays_ucode: *mut u8,
    pub se0_tap_delays_ucode: *mut u8,
    pub se1_tap_delays_ucode: *mut u8,
    pub se2_tap_delays_ucode: *mut u8,
    pub se3_tap_delays_ucode: *mut u8,
    pub is_rlc_v2_1: bool,
// for rlc autoload
    pub rlc_autoload_bo: *mut amdgpu_bo,
    pub rlc_autoload_gpu_addr: u64,
    pub rlc_autoload_ptr: *mut c_void,
// rlc toc buffer
    pub rlc_toc_bo: *mut amdgpu_bo,
    pub rlc_toc_gpu_addr: u64,
    pub rlc_toc_buf: *mut c_void,
    pub rlcg_reg_access_supported: bool,
// registers for rlcg indirect reg access
    pub reg_access_ctrl: [amdgpu_rlcg_reg_access_ctrl; AMDGPU_MAX_RLC_INSTANCES],
}

extern "C" {
    pub fn amdgpu_gfx_rlc_enter_safe_mode(adev: *mut amdgpu_device, xcc_id: c_int);
}
extern "C" {
    pub fn amdgpu_gfx_rlc_exit_safe_mode(adev: *mut amdgpu_device, xcc_id: c_int);
}
extern "C" {
    pub fn amdgpu_gfx_rlc_init_sr(adev: *mut amdgpu_device, dws: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_rlc_init_csb(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_rlc_init_cpt(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_rlc_setup_cp_table(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gfx_rlc_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_early_init_rlc_reg_funcs(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_init_rlc_reg_funcs(adev: *mut amdgpu_device);
}
