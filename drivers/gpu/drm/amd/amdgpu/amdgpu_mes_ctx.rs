//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_mes_ctx.h
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

pub const AMDGPU_MES_CTX_MAX_GFX_RINGS: c_int = 1;
pub const AMDGPU_MES_CTX_MAX_COMPUTE_RINGS: c_int = 4;
pub const AMDGPU_MES_CTX_MAX_SDMA_RINGS: c_int = 2;

pub const AMDGPU_CSA_SDMA_SIZE: c_int = 64;
pub const GFX10_MEC_HPD_SIZE: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_wb_slot {
    pub data: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mes_ctx_meta_data {
    pub 4]: *mut *mut uint8_t ring[PAGE_SIZE,
// gfx csa
    pub gfx_meta_data: v10_gfx_meta_data,
    pub 1024]: *mut *mut uint8_t gds_backup[64,
    pub slots: [amdgpu_wb_slot; AMDGPU_MES_CTX_MAX_OFFS],
// only for ib test
    pub __aligned(256): uint32_t ib[256],
    pub padding: [u32; 64],
    pub gfx: [} __aligned(PAGE_SIZE); AMDGPU_MES_CTX_MAX_GFX_RINGS],
    pub 4]: *mut *mut uint8_t ring[PAGE_SIZE,
    pub mec_hpd: [u8; GFX10_MEC_HPD_SIZE],
    pub slots: [amdgpu_wb_slot; AMDGPU_MES_CTX_MAX_OFFS],
// only for ib test
    pub __aligned(256): uint32_t ib[256],
    pub padding: [u32; 64],
    pub compute: [} __aligned(PAGE_SIZE); AMDGPU_MES_CTX_MAX_COMPUTE_RINGS],
    pub 4]: *mut *mut uint8_t ring[PAGE_SIZE,
// sdma csa for mcbp
    pub sdma_meta_data: [u8; AMDGPU_CSA_SDMA_SIZE],
    pub slots: [amdgpu_wb_slot; AMDGPU_MES_CTX_MAX_OFFS],
// only for ib test
    pub __aligned(256): uint32_t ib[256],
    pub padding: [u32; 64],
    pub sdma: [} __aligned(PAGE_SIZE); AMDGPU_MES_CTX_MAX_SDMA_RINGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mes_ctx_data {
    pub meta_data_obj: *mut amdgpu_bo,
    pub meta_data_gpu_addr: u64,
    pub meta_data_mc_addr: u64,
    pub meta_data_va: *mut amdgpu_bo_va,
    pub meta_data_ptr: *mut c_void,
    pub gang_ids: [u32; AMDGPU_HW_IP_DMA+1],
}

pub const AMDGPU_FENCE_MES_QUEUE_FLAG: c_uint = 0x1000000u;

pub const AMDGPU_FENCE_MES_QUEUE_FLAG: c_uint = 0x1000000u;

