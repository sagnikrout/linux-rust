//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_reset.h
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
// Copyright 2021 Advanced Micro Devices, Inc.
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

pub const AMDGPU_RESET_MAX_HANDLERS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AMDGPU_RESET_FLAGS {

    AMDGPU_NEED_FULL_RESET = 0,
    AMDGPU_SKIP_HW_RESET = 1,
    AMDGPU_SKIP_COREDUMP = 2,
    AMDGPU_HOST_FLR = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AMDGPU_RESET_SRCS {
    AMDGPU_RESET_SRC_UNKNOWN,
    AMDGPU_RESET_SRC_JOB,
    AMDGPU_RESET_SRC_RAS,
    AMDGPU_RESET_SRC_MES,
    AMDGPU_RESET_SRC_HWS,
    AMDGPU_RESET_SRC_USER,
    AMDGPU_RESET_SRC_USERQ,
}

//
// enum amd_reset_method - Methods for resetting AMD GPU devices
//
// @AMD_RESET_METHOD_NONE: The device will not be reset.
// @AMD_RESET_METHOD_LEGACY: Method reserved for SI, CIK and VI ASICs.
// @AMD_RESET_METHOD_MODE0: Reset the entire ASIC. Not currently available for
// the any device.
// @AMD_RESET_METHOD_MODE1: Resets all IP blocks on the ASIC (SDMA, GFX, VCN,
// etc.) individually. Suitable only for some discrete GPU,
// not available for all ASICs.
// @AMD_RESET_METHOD_MODE2: Resets a lesser level of IPs compared to MODE1.
// Which IPs are reset depends on the ASIC. Notably doesn't
// reset IPs shared with the CPU on APUs or the memory
// controllers (so VRAM is not lost). Not available on all
// ASICs.
// @AMD_RESET_METHOD_LINK: Triggers SW-UP link reset on other GPUs
// @AMD_RESET_METHOD_BACO: BACO (Bus Alive, Chip Off) method powers off and on
// the card but without powering off the PCI bus. Suitable
// only for discrete GPUs.
// @AMD_RESET_METHOD_PCI: Does a full bus reset using core Linux subsystem
// PCI reset and does a secondary bus reset or FLR,
// depending on what the underlying hardware supports.
// @AMD_RESET_METHOD_ON_INIT: Does a device reset during the driver init
// sequence.
//
// Methods available for AMD GPU driver for resetting the device. Not all
// methods are suitable for every device. User can override the method using
// module parameter `reset_method`.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_reset_method {
    AMD_RESET_METHOD_NONE = -1,
    AMD_RESET_METHOD_LEGACY = 0,
    AMD_RESET_METHOD_MODE0,
    AMD_RESET_METHOD_MODE1,
    AMD_RESET_METHOD_MODE2,
    AMD_RESET_METHOD_LINK,
    AMD_RESET_METHOD_BACO,
    AMD_RESET_METHOD_PCI,
    AMD_RESET_METHOD_ON_INIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_reset_context {
    pub method: amd_reset_method,
    pub reset_req_dev: *mut amdgpu_device,
    pub job: *mut amdgpu_job,
    pub hive: *mut amdgpu_hive_info,
    pub reset_device_list: *mut list_head,
    pub flags: c_ulong,
    pub src: AMDGPU_RESET_SRCS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_reset_control {
    pub handle: *mut c_void,
    pub reset_work: work_struct,
    pub reset_lock: mutex,
// reset_handlers)[AMDGPU_RESET_MAX_HANDLERS];
    pub in_reset: core::sync::atomic::AtomicI32,
    pub active_reset: amd_reset_method,
    pub context): *mut amdgpu_reset_context,
    pub work): *mut *mut void (async_reset)(struct work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_reset_handler {
    pub reset_method: amd_reset_method,
    pub context): *mut amdgpu_reset_context,
    pub context): *mut amdgpu_reset_context,
    pub context): *mut amdgpu_reset_context,
    pub context): *mut amdgpu_reset_context,
    pub context): *mut amdgpu_reset_context,
    pub adev): *mut *mut int (do_reset)(struct amdgpu_device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_reset_domain_type {
    SINGLE_DEVICE,
    XGMI_HIVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_reset_domain {
    pub refcount: kref,
    pub wq: *mut workqueue_struct,
    pub type: amdgpu_reset_domain_type,
    pub sem: rw_semaphore,
    pub in_gpu_reset: core::sync::atomic::AtomicI32,
    pub reset_res: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn amdgpu_reset_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_reset_fini(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_reset_destroy_reset_domain(ref: *mut kref);
}
extern "C" {
    pub fn queue_work(_arg: domain->wq, _arg: work) -> return;
}
extern "C" {
    pub fn rwsem_is_contended(_arg: &domain->sem) -> return;
}
extern "C" {
    pub fn amdgpu_device_lock_reset_domain(reset_domain: *mut amdgpu_reset_domain);
}
extern "C" {
    pub fn amdgpu_device_unlock_reset_domain(reset_domain: *mut amdgpu_reset_domain);
}

extern "C" {
    pub fn amdgpu_reset_in_recovery(adev: *mut amdgpu_device) -> bool;
}
