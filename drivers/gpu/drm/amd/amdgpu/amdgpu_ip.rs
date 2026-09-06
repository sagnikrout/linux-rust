//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_ip.h
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

// Define the HW IP blocks will be used in driver , add more if necessary
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_hw_ip_block_type {
    GC_HWIP = 1,
    HDP_HWIP,
    SDMA0_HWIP,
    SDMA1_HWIP,
    SDMA2_HWIP,
    SDMA3_HWIP,
    SDMA4_HWIP,
    SDMA5_HWIP,
    SDMA6_HWIP,
    SDMA7_HWIP,
    LSDMA_HWIP,
    MMHUB_HWIP,
    ATHUB_HWIP,
    NBIO_HWIP,
    MP0_HWIP,
    MP1_HWIP,
    UVD_HWIP,
    VCN_HWIP = UVD_HWIP,
    JPEG_HWIP = VCN_HWIP,
    VCN1_HWIP,
    VCE_HWIP,
    VPE_HWIP,
    DF_HWIP,
    DCE_HWIP,
    OSSSYS_HWIP,
    SMUIO_HWIP,
    PWR_HWIP,
    NBIF_HWIP,
    THM_HWIP,
    CLK_HWIP,
    UMC_HWIP,
    RSMU_HWIP,
    XGMI_HWIP,
    DCI_HWIP,
    PCIE_HWIP,
    ISP_HWIP,
    ATU_HWIP,
    AIGC_HWIP,
    UMSCH_HWIP,
    MAX_HWIP
}

pub const HWIP_MAX_INSTANCE: c_int = 48;
pub const HW_ID_MAX: c_int = 300;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ip_map_info {
// Map of logical to actual dev instances/mask
    pub dev_inst: [u32; MAX_HWIP][HWIP_MAX_INSTANCE],
    pub inst): i8,
    pub mask): u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ip_block_status {
    pub valid: bool,
    pub sw: bool,
    pub hw: bool,
    pub late_initialized: bool,
    pub hang: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ip_block_version {
    pub type: amd_ip_block_type,
    pub major: u32,
    pub minor: u32,
    pub rev: u32,
    pub funcs: *const amd_ip_funcs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ip_block {
    pub status: amdgpu_ip_block_status,
    pub version: *const amdgpu_ip_block_version,
    pub adev: *mut amdgpu_device,
}

extern "C" {
    pub fn amdgpu_ip_map_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_ip_block_suspend(ip_block: *mut amdgpu_ip_block) -> c_int;
}
extern "C" {
    pub fn amdgpu_ip_block_resume(ip_block: *mut amdgpu_ip_block) -> c_int;
}
