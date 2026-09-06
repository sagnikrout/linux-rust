//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_xcp.h
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

pub const MAX_XCP: c_int = 8;

pub const AMDGPU_XCP_FL_NONE: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AMDGPU_XCP_IP_BLOCK {
    AMDGPU_XCP_GFXHUB,
    AMDGPU_XCP_GFX,
    AMDGPU_XCP_SDMA,
    AMDGPU_XCP_VCN,
    AMDGPU_XCP_MAX_BLOCKS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AMDGPU_XCP_STATE {
    AMDGPU_XCP_PREPARE_SUSPEND,
    AMDGPU_XCP_SUSPEND,
    AMDGPU_XCP_PREPARE_RESUME,
    AMDGPU_XCP_RESUME,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_xcp_res_id {
    AMDGPU_XCP_RES_XCC,
    AMDGPU_XCP_RES_DMA,
    AMDGPU_XCP_RES_DEC,
    AMDGPU_XCP_RES_JPEG,
    AMDGPU_XCP_RES_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_xcp_res_details {
    pub id: amdgpu_xcp_res_id,
    pub num_inst: u8,
    pub num_shared: u8,
    pub kobj: kobject,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_xcp_cfg {
    pub mode: u8,
    pub xcp_res: [amdgpu_xcp_res_details; AMDGPU_XCP_RES_MAX],
    pub num_res: u8,
    pub xcp_mgr: *mut amdgpu_xcp_mgr,
    pub kobj: kobject,
    pub compatible_nps_modes: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_xcp_ip_funcs {
    pub inst_mask): *mut *mut *mut int (prepare_suspend)(void handle, uint32_t,
    pub inst_mask): *mut *mut *mut int (suspend)(void handle, uint32_t,
    pub inst_mask): *mut *mut *mut int (prepare_resume)(void handle, uint32_t,
    pub inst_mask): *mut *mut *mut int (resume)(void handle, uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_xcp_ip {
    pub ip_funcs: *mut amdgpu_xcp_ip_funcs,
    pub inst_mask: u32,
    pub ip_id: AMDGPU_XCP_IP_BLOCK,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_xcp {
    pub ip: [amdgpu_xcp_ip; AMDGPU_XCP_MAX_BLOCKS],
    pub id: u8,
    pub mem_id: u8,
    pub valid: bool,
    pub ref_cnt: core::sync::atomic::AtomicI32,
    pub ddev: *mut drm_device,
    pub rdev: *mut drm_device,
    pub pdev: *mut drm_device,
    pub driver: *mut drm_driver,
    pub vma_offset_manager: *mut drm_vma_offset_manager,
    pub gpu_sched: [amdgpu_sched; AMDGPU_HW_IP_NUM][AMDGPU_RING_PRIO_MAX],
    pub xcp_mgr: *mut amdgpu_xcp_mgr,
    pub kobj: kobject,
    pub unique_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_xcp_mgr {
    pub adev: *mut amdgpu_device,
    pub xcp_lock: mutex,
    pub funcs: *mut amdgpu_xcp_mgr_funcs,
    pub xcp: [amdgpu_xcp; MAX_XCP],
    pub num_xcps: u8,
    pub mode: i8,
// Used to determine KFD memory size limits per XCP
    pub num_xcp_per_mem_partition: c_uint,
    pub xcp_cfg: *mut amdgpu_xcp_cfg,
    pub supp_xcp_modes: u32,
    pub avail_xcp_modes: u32,
// used to determin KFD memory alloc mode for each partition
    pub mem_alloc_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_xcp_mgr_funcs {
    pub num_xcps): *mut c_int,
    pub xcp_mgr): *mut *mut int (query_partition_mode)(struct amdgpu_xcp_mgr,
    pub ip): *mut amdgpu_xcp_ip,
    pub mem_id): *mut *mut amdgpu_xcp xcp, uint8_t,
    pub xcp_cfg): *mut amdgpu_xcp_cfg,
    pub xcp_id): *mut *mut *mut int (prepare_suspend)(struct amdgpu_xcp_mgr xcp_mgr, int,
    pub xcp_id): *mut *mut *mut int (suspend)(struct amdgpu_xcp_mgr xcp_mgr, int,
    pub xcp_id): *mut *mut *mut int (prepare_resume)(struct amdgpu_xcp_mgr xcp_mgr, int,
    pub xcp_id): *mut *mut *mut int (resume)(struct amdgpu_xcp_mgr xcp_mgr, int,
}

extern "C" {
    pub fn amdgpu_xcp_prepare_suspend(xcp_mgr: *mut amdgpu_xcp_mgr, xcp_id: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_xcp_suspend(xcp_mgr: *mut amdgpu_xcp_mgr, xcp_id: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_xcp_prepare_resume(xcp_mgr: *mut amdgpu_xcp_mgr, xcp_id: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_xcp_resume(xcp_mgr: *mut amdgpu_xcp_mgr, xcp_id: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_xcp_init(xcp_mgr: *mut amdgpu_xcp_mgr, num_xcps: c_int, mode: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_xcp_query_partition_mode(xcp_mgr: *mut amdgpu_xcp_mgr, flags: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_xcp_switch_partition_mode(xcp_mgr: *mut amdgpu_xcp_mgr, mode: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_xcp_restore_partition_mode(xcp_mgr: *mut amdgpu_xcp_mgr) -> c_int;
}
extern "C" {
    pub fn amdgpu_xcp_dev_unplug(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_xcp_update_supported_modes(xcp_mgr: *mut amdgpu_xcp_mgr);
}
extern "C" {
    pub fn amdgpu_xcp_update_partition_sched_list(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_xcp_pre_partition_switch(xcp_mgr: *mut amdgpu_xcp_mgr, flags: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_xcp_post_partition_switch(xcp_mgr: *mut amdgpu_xcp_mgr, flags: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_xcp_sysfs_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_xcp_sysfs_fini(adev: *mut amdgpu_device);
}

