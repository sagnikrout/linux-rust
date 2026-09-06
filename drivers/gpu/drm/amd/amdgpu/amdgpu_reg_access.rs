//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_reg_access.h
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

extern "C" {
    pub fn uint32_t(: *mut *mut amdgpu_rreg_t)(struct amdgpu_device, _arg: u32) -> typedef;
}
extern "C" {
    pub fn void(: *mut *mut amdgpu_wreg_t)(struct amdgpu_device, _arg: u32, _arg: u32) -> typedef;
}
extern "C" {
    pub fn uint32_t(: *mut *mut amdgpu_rreg_ext_t)(struct amdgpu_device, _arg: u64) -> typedef;
}
extern "C" {
    pub fn void(: *mut *mut amdgpu_wreg_ext_t)(struct amdgpu_device, _arg: u64, _arg: u32) -> typedef;
}
extern "C" {
    pub fn uint64_t(: *mut *mut amdgpu_rreg64_t)(struct amdgpu_device, _arg: u32) -> typedef;
}
extern "C" {
    pub fn void(: *mut *mut amdgpu_wreg64_t)(struct amdgpu_device, _arg: u32, _arg: u64) -> typedef;
}
extern "C" {
    pub fn uint64_t(: *mut *mut amdgpu_rreg64_ext_t)(struct amdgpu_device, _arg: u64) -> typedef;
}
extern "C" {
    pub fn void(: *mut *mut amdgpu_wreg64_ext_t)(struct amdgpu_device, _arg: u64, _arg: u64) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_reg_ind {
    pub lock: spinlock_t,
    pub rreg: amdgpu_rreg_t,
    pub wreg: amdgpu_wreg_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_reg_ind_blk {
    pub lock: spinlock_t,
    pub rreg: amdgpu_block_rreg_t,
    pub wreg: amdgpu_block_wreg_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_reg_pcie_ind {
    pub lock: spinlock_t,
    pub rreg: amdgpu_rreg_t,
    pub wreg: amdgpu_wreg_t,
    pub rreg_ext: amdgpu_rreg_ext_t,
    pub wreg_ext: amdgpu_wreg_ext_t,
    pub rreg64: amdgpu_rreg64_t,
    pub wreg64: amdgpu_wreg64_t,
    pub rreg64_ext: amdgpu_rreg64_ext_t,
    pub wreg64_ext: amdgpu_wreg64_ext_t,
    pub port_rreg: amdgpu_rreg_t,
    pub port_wreg: amdgpu_wreg_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_reg_smn_ext {
    pub get_smn_base: amdgpu_reg_get_smn_base64_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_reg_access {
    pub smc: amdgpu_reg_ind,
    pub uvd_ctx: amdgpu_reg_ind,
    pub didt: amdgpu_reg_ind,
    pub gc_cac: amdgpu_reg_ind,
    pub se_cac: amdgpu_reg_ind,
    pub audio_endpt: amdgpu_reg_ind_blk,
    pub pcie: amdgpu_reg_pcie_ind,
    pub smn: amdgpu_reg_smn_ext,
}

//
// ASIC specific register table accessible by UMD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_allowed_register_entry {
    pub reg_offset: u32,
    pub grbm_indexed: bool,
}

extern "C" {
    pub fn amdgpu_reg_access_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_reg_smc_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
}
extern "C" {
    pub fn amdgpu_reg_smc_wr32(adev: *mut amdgpu_device, reg: u32, v: u32);
}
extern "C" {
    pub fn amdgpu_reg_uvd_ctx_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
}
extern "C" {
    pub fn amdgpu_reg_uvd_ctx_wr32(adev: *mut amdgpu_device, reg: u32, v: u32);
}
extern "C" {
    pub fn amdgpu_reg_didt_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
}
extern "C" {
    pub fn amdgpu_reg_didt_wr32(adev: *mut amdgpu_device, reg: u32, v: u32);
}
extern "C" {
    pub fn amdgpu_reg_gc_cac_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
}
extern "C" {
    pub fn amdgpu_reg_se_cac_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
}
extern "C" {
    pub fn amdgpu_reg_pcie_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
}
extern "C" {
    pub fn amdgpu_reg_pcie_wr32(adev: *mut amdgpu_device, reg: u32, v: u32);
}
extern "C" {
    pub fn amdgpu_reg_pcie_ext_rd32(adev: *mut amdgpu_device, reg: u64) -> u32;
}
extern "C" {
    pub fn amdgpu_reg_pcie_rd64(adev: *mut amdgpu_device, reg: u32) -> u64;
}
extern "C" {
    pub fn amdgpu_reg_pcie_wr64(adev: *mut amdgpu_device, reg: u32, v: u64);
}
extern "C" {
    pub fn amdgpu_reg_pcie_ext_rd64(adev: *mut amdgpu_device, reg: u64) -> u64;
}
extern "C" {
    pub fn amdgpu_reg_pciep_rd32(adev: *mut amdgpu_device, reg: u32) -> u32;
}
extern "C" {
    pub fn amdgpu_mm_rreg8(adev: *mut amdgpu_device, offset: u32) -> u8;
}
extern "C" {
    pub fn amdgpu_device_indirect_rreg(adev: *mut amdgpu_device, reg_addr: u32) -> u32;
}
extern "C" {
    pub fn amdgpu_device_indirect_rreg_ext(adev: *mut amdgpu_device, reg_addr: u64) -> u32;
}
extern "C" {
    pub fn amdgpu_device_indirect_rreg64(adev: *mut amdgpu_device, reg_addr: u32) -> u64;
}
extern "C" {
    pub fn amdgpu_device_indirect_rreg64_ext(adev: *mut amdgpu_device, reg_addr: u64) -> u64;
}
extern "C" {
    pub fn amdgpu_device_pcie_port_rreg(adev: *mut amdgpu_device, reg: u32) -> u32;
}
extern "C" {
    pub fn amdgpu_device_pcie_port_wreg(adev: *mut amdgpu_device, reg: u32, v: u32);
}
