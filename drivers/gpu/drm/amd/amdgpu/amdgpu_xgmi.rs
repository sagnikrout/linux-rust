//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_xgmi.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_hive_info {
    pub kobj: kobject,
    pub hive_id: u64,
    pub device_list: list_head,
    pub node: list_head,
    pub number_devices: core::sync::atomic::AtomicI32,
    pub hive_lock: mutex,
    pub hi_req_count: c_int,
    pub hi_req_gpu: *mut amdgpu_device,
    pub tb: task_barrier,
    pub pstate: },
    pub reset_domain: *mut amdgpu_reset_domain,
    pub ras_recovery: core::sync::atomic::AtomicI32,
    pub event_mgr: ras_event_manager,
    pub reset_on_init_work: work_struct,
    pub requested_nps_mode: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_pcs_ras_field {
    pub err_name: *const c_char,
    pub pcs_err_mask: u32,
    pub pcs_err_shift: u32,
}

//
// Bandwidth range reporting comes in two modes.
//
// PER_LINK - range for any xgmi link
// PER_PEER - range of max of single xgmi link to max of multiple links based on source peer
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_xgmi_bw_mode {
    AMDGPU_XGMI_BW_MODE_PER_LINK = 0,
    AMDGPU_XGMI_BW_MODE_PER_PEER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_xgmi_bw_unit {
    AMDGPU_XGMI_BW_UNIT_GBYTES = 0,
    AMDGPU_XGMI_BW_UNIT_MBYTES
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_xgmi_ras {
    pub ras_block: amdgpu_ras_block_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_xgmi {
// from psp
    pub node_id: u64,
    pub hive_id: u64,
// fixed per family
    pub node_segment_size: u64,
// physical node (0-3)
    pub physical_node_id: unsigned,
// number of nodes (0-4)
    pub num_physical_nodes: unsigned,
// gpu list in the same hive
    pub head: list_head,
    pub supported: bool,
    pub ras_if: *mut ras_common_if,
    pub connected_to_cpu: bool,
    pub ras: *mut amdgpu_xgmi_ras,
    pub max_speed: u16,
    pub max_width: u8,
}

extern "C" {
    pub fn amdgpu_put_xgmi_hive(hive: *mut amdgpu_hive_info);
}
extern "C" {
    pub fn amdgpu_xgmi_update_topology(hive: *mut amdgpu_hive_info, adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_xgmi_add_device(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_xgmi_remove_device(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_xgmi_set_pstate(adev: *mut amdgpu_device, pstate: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_xgmi_get_hops_count(adev: *mut amdgpu_device, peer_adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_xgmi_ras_sw_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_xgmi_reset_on_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_xgmi_get_ext_link(adev: *mut amdgpu_device, link_num: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_xgmi_early_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_xgmi_get_max_bandwidth(adev: *mut amdgpu_device) -> u32;
}
// Cleanup macro for use with __free(xgmi_put_hive)
