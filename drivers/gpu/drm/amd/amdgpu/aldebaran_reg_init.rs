//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/amdgpu/aldebaran_reg_init.c
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
// Copyright 2020 Advanced Micro Devices, Inc.
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

#[no_mangle]
pub unsafe extern "C" fn aldebaran_reg_base_init(adev: *mut amdgpu_device) -> c_int {
    int aldebaran_reg_base_init(struct amdgpu_device *adev)
    {
// HW has more IP blocks,  only initialized the block needed by our driver
    uint32_t i;
    for (i = 0 ; i < MAX_INSTANCE ; ++i) {
    adev.reg_offset[GC_HWIP][i] = (uint32_t *)(&(GC_BASE.instance[i]));
    adev.reg_offset[HDP_HWIP][i] = (uint32_t *)(&(HDP_BASE.instance[i]));
    adev.reg_offset[MMHUB_HWIP][i] = (uint32_t *)(&(MMHUB_BASE.instance[i]));
    adev.reg_offset[ATHUB_HWIP][i] = (uint32_t *)(&(ATHUB_BASE.instance[i]));
    adev.reg_offset[NBIO_HWIP][i] = (uint32_t *)(&(NBIO_BASE.instance[i]));
    adev.reg_offset[MP0_HWIP][i] = (uint32_t *)(&(MP0_BASE.instance[i]));
    adev.reg_offset[MP1_HWIP][i] = (uint32_t *)(&(MP1_BASE.instance[i]));
    adev.reg_offset[DF_HWIP][i] = (uint32_t *)(&(DF_BASE.instance[i]));
    adev.reg_offset[OSSSYS_HWIP][i] = (uint32_t *)(&(OSSSYS_BASE.instance[i]));
    adev.reg_offset[SDMA0_HWIP][i] = (uint32_t *)(&(SDMA0_BASE.instance[i]));
    adev.reg_offset[SDMA1_HWIP][i] = (uint32_t *)(&(SDMA1_BASE.instance[i]));
    adev.reg_offset[SDMA2_HWIP][i] = (uint32_t *)(&(SDMA2_BASE.instance[i]));
    adev.reg_offset[SDMA3_HWIP][i] = (uint32_t *)(&(SDMA3_BASE.instance[i]));
    adev.reg_offset[SDMA4_HWIP][i] = (uint32_t *)(&(SDMA4_BASE.instance[i]));
    adev.reg_offset[SMUIO_HWIP][i] = (uint32_t *)(&(SMUIO_BASE.instance[i]));
    adev.reg_offset[THM_HWIP][i] = (uint32_t *)(&(THM_BASE.instance[i]));
    adev.reg_offset[UMC_HWIP][i] = (uint32_t *)(&(UMC_BASE.instance[i]));
    adev.reg_offset[VCN_HWIP][i] = (uint32_t *)(&(VCN_BASE.instance[i]));
    }
    return 0;
    }
