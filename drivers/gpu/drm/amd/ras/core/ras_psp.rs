//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/ras/core/ras_psp.h
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
// Copyright 2025 Advanced Micro Devices, Inc.
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
pub struct ras_ta_image_header {
    pub reserved1: [u32; 24],
    pub /: *mut *mut uint32_t image_version; / [0x60] Off Chip Firmware Version,
    pub reserved2: [u32; 39],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_psp_sys_status {
    pub initialized: bool,
    pub session_id: u32,
    pub psp_cmd_mutex: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ta_init_param {
    pub poison_mode_en: u8,
    pub dgpu_mode: u8,
    pub xcc_mask: u16,
    pub channel_dis_num: u8,
    pub nps_mode: u8,
    pub active_umc_mask: u32,
    pub vram_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_mem_block {
    pub mem_type: u32,
    pub mem_bo: *mut c_void,
    pub mem_mc_addr: u64,
    pub mem_cpu_addr: *mut c_void,
    pub mem_size: u32,
    pub ref_count: c_int,
    pub private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_psp_ip_func {
    pub ras_core): *mut *mut uint32_t (psp_ras_ring_wptr_get)(struct ras_core_context,
    pub wptr): *mut *mut *mut int (psp_ras_ring_wptr_set)(struct ras_core_context ras_core, uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_psp_ring {
    pub ras_ring_gpu_mem: gpu_mem_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_cmd_resp {
    pub status: u32,
    pub session_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_psp_ctx {
    pub external_mutex: *mut c_void,
    pub internal_mutex: mutex,
    pub in_fence_value: u64,
    pub psp_cmd_gpu_mem: gpu_mem_block,
    pub out_fence_gpu_mem: gpu_mem_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ta_fw_bin {
    pub fw_version: u32,
    pub feature_version: u32,
    pub bin_size: u32,
    pub bin_addr: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ta_ctx {
    pub preload_ras_ta_enabled: bool,
    pub ras_ta_initialized: bool,
    pub session_id: u32,
    pub resp_status: u32,
    pub ta_version: u32,
    pub ta_mutex: mutex,
    pub fw_bin: ras_ta_fw_bin,
    pub init_param: ras_ta_init_param,
    pub fw_gpu_mem: gpu_mem_block,
    pub cmd_gpu_mem: gpu_mem_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_psp {
    pub psp_ip_version: u32,
    pub psp_ring: ras_psp_ring,
    pub psp_ctx: ras_psp_ctx,
    pub ta_ctx: ras_ta_ctx,
    pub ip_func: *const ras_psp_ip_func,
    pub sys_func: *const ras_psp_sys_func,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_psp_ta_load {
    pub fw_version: u32,
    pub feature_version: u32,
    pub bin_size: u32,
    pub bin_addr: *mut u8,
    pub out_session_id: u64,
    pub out_loaded_ta_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_psp_ta_unload {
    pub ras_session_id: u64,
}

extern "C" {
    pub fn ras_psp_sw_init(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_psp_sw_fini(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_psp_hw_init(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_psp_hw_fini(ras_core: *mut ras_core_context) -> c_int;
}
