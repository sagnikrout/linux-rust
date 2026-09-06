//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/amdgpu/vega20_reg_init.c
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
// Copyright 2018 Advanced Micro Devices, Inc.
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
pub unsafe extern "C" fn vega20_reg_base_init(adev: *mut amdgpu_device) -> c_int {
    int vega20_reg_base_init(struct amdgpu_device *adev)
    {
// HW has more IP blocks,  only initialized the blocke beend by our driver
    uint32_t i;
    for (i = 0 ; i < MAX_INSTANCE ; ++i) {
    adev.reg_offset[GC_HWIP][i] = (uint32_t *)(&(GC_BASE.instance[i]));
    adev.reg_offset[HDP_HWIP][i] = (uint32_t *)(&(HDP_BASE.instance[i]));
    adev.reg_offset[MMHUB_HWIP][i] = (uint32_t *)(&(MMHUB_BASE.instance[i]));
    adev.reg_offset[ATHUB_HWIP][i] = (uint32_t *)(&(ATHUB_BASE.instance[i]));
    adev.reg_offset[NBIO_HWIP][i] = (uint32_t *)(&(NBIO_BASE.instance[i]));
    adev.reg_offset[MP0_HWIP][i] = (uint32_t *)(&(MP0_BASE.instance[i]));
    adev.reg_offset[MP1_HWIP][i] = (uint32_t *)(&(MP1_BASE.instance[i]));
    adev.reg_offset[UVD_HWIP][i] = (uint32_t *)(&(UVD_BASE.instance[i]));
    adev.reg_offset[VCE_HWIP][i] = (uint32_t *)(&(VCE_BASE.instance[i]));
    adev.reg_offset[DF_HWIP][i] = (uint32_t *)(&(DF_BASE.instance[i]));
    adev.reg_offset[DCE_HWIP][i] = (uint32_t *)(&(DCE_BASE.instance[i]));
    adev.reg_offset[OSSSYS_HWIP][i] = (uint32_t *)(&(OSSSYS_BASE.instance[i]));
    adev.reg_offset[SDMA0_HWIP][i] = (uint32_t *)(&(SDMA0_BASE.instance[i]));
    adev.reg_offset[SDMA1_HWIP][i] = (uint32_t *)(&(SDMA1_BASE.instance[i]));
    adev.reg_offset[SMUIO_HWIP][i] = (uint32_t *)(&(SMUIO_BASE.instance[i]));
    adev.reg_offset[NBIF_HWIP][i] = (uint32_t *)(&(NBIO_BASE.instance[i]));
    adev.reg_offset[THM_HWIP][i] = (uint32_t *)(&(THM_BASE.instance[i]));
    adev.reg_offset[CLK_HWIP][i] = (uint32_t *)(&(CLK_BASE.instance[i]));
    adev.reg_offset[UMC_HWIP][i] = (uint32_t *)(&(UMC_BASE.instance[i]));
    adev.reg_offset[RSMU_HWIP][i] = (uint32_t *)(&(RSMU_BASE.instance[i]));
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vega20_doorbell_index_init(adev: *mut amdgpu_device) {
    void vega20_doorbell_index_init(struct amdgpu_device *adev)
    {
    adev.doorbell_index.kiq = AMDGPU_VEGA20_DOORBELL_KIQ;
    adev.doorbell_index.mec_ring0 = AMDGPU_VEGA20_DOORBELL_MEC_RING0;
    adev.doorbell_index.mec_ring1 = AMDGPU_VEGA20_DOORBELL_MEC_RING1;
    adev.doorbell_index.mec_ring2 = AMDGPU_VEGA20_DOORBELL_MEC_RING2;
    adev.doorbell_index.mec_ring3 = AMDGPU_VEGA20_DOORBELL_MEC_RING3;
    adev.doorbell_index.mec_ring4 = AMDGPU_VEGA20_DOORBELL_MEC_RING4;
    adev.doorbell_index.mec_ring5 = AMDGPU_VEGA20_DOORBELL_MEC_RING5;
    adev.doorbell_index.mec_ring6 = AMDGPU_VEGA20_DOORBELL_MEC_RING6;
    adev.doorbell_index.mec_ring7 = AMDGPU_VEGA20_DOORBELL_MEC_RING7;
    adev.doorbell_index.userqueue_start = AMDGPU_VEGA20_DOORBELL_USERQUEUE_START;
    adev.doorbell_index.userqueue_end = AMDGPU_VEGA20_DOORBELL_USERQUEUE_END;
    adev.doorbell_index.gfx_ring0 = AMDGPU_VEGA20_DOORBELL_GFX_RING0;
    adev.doorbell_index.sdma_engine[0] = AMDGPU_VEGA20_DOORBELL_sDMA_ENGINE0;
    adev.doorbell_index.sdma_engine[1] = AMDGPU_VEGA20_DOORBELL_sDMA_ENGINE1;
    adev.doorbell_index.sdma_engine[2] = AMDGPU_VEGA20_DOORBELL_sDMA_ENGINE2;
    adev.doorbell_index.sdma_engine[3] = AMDGPU_VEGA20_DOORBELL_sDMA_ENGINE3;
    adev.doorbell_index.sdma_engine[4] = AMDGPU_VEGA20_DOORBELL_sDMA_ENGINE4;
    adev.doorbell_index.sdma_engine[5] = AMDGPU_VEGA20_DOORBELL_sDMA_ENGINE5;
    adev.doorbell_index.sdma_engine[6] = AMDGPU_VEGA20_DOORBELL_sDMA_ENGINE6;
    adev.doorbell_index.sdma_engine[7] = AMDGPU_VEGA20_DOORBELL_sDMA_ENGINE7;
    adev.doorbell_index.ih = AMDGPU_VEGA20_DOORBELL_IH;
    adev.doorbell_index.uvd_vce.uvd_ring0_1 = AMDGPU_VEGA20_DOORBELL64_UVD_RING0_1;
    adev.doorbell_index.uvd_vce.uvd_ring2_3 = AMDGPU_VEGA20_DOORBELL64_UVD_RING2_3;
    adev.doorbell_index.uvd_vce.uvd_ring4_5 = AMDGPU_VEGA20_DOORBELL64_UVD_RING4_5;
    adev.doorbell_index.uvd_vce.uvd_ring6_7 = AMDGPU_VEGA20_DOORBELL64_UVD_RING6_7;
    adev.doorbell_index.uvd_vce.vce_ring0_1 = AMDGPU_VEGA20_DOORBELL64_VCE_RING0_1;
    adev.doorbell_index.uvd_vce.vce_ring2_3 = AMDGPU_VEGA20_DOORBELL64_VCE_RING2_3;
    adev.doorbell_index.uvd_vce.vce_ring4_5 = AMDGPU_VEGA20_DOORBELL64_VCE_RING4_5;
    adev.doorbell_index.uvd_vce.vce_ring6_7 = AMDGPU_VEGA20_DOORBELL64_VCE_RING6_7;
    adev.doorbell_index.vcn.vcn_ring0_1 = AMDGPU_VEGA20_DOORBELL64_VCN0_1;
    adev.doorbell_index.vcn.vcn_ring2_3 = AMDGPU_VEGA20_DOORBELL64_VCN2_3;
    adev.doorbell_index.vcn.vcn_ring4_5 = AMDGPU_VEGA20_DOORBELL64_VCN4_5;
    adev.doorbell_index.vcn.vcn_ring6_7 = AMDGPU_VEGA20_DOORBELL64_VCN6_7;
    adev.doorbell_index.first_non_cp = AMDGPU_VEGA20_DOORBELL64_FIRST_NON_CP;
    adev.doorbell_index.last_non_cp = AMDGPU_VEGA20_DOORBELL64_LAST_NON_CP;
    adev.doorbell_index.max_assignment = AMDGPU_VEGA20_DOORBELL_MAX_ASSIGNMENT << 1;
    adev.doorbell_index.sdma_doorbell_range = 20;
    }
