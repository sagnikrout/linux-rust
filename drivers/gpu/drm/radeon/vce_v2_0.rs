//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/radeon/vce_v2_0.c
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
// Copyright 2013 Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// Authors: Christian König <christian.koenig@amd.com>
//

#[no_mangle]
unsafe extern "C" fn vce_v2_0_set_sw_cg(rdev: *mut radeon_device, gated: bool) {
    static void vce_v2_0_set_sw_cg(struct radeon_device *rdev, bool gated)
    {
    u32 tmp;
    if (gated) {
    tmp = RREG32(VCE_CLOCK_GATING_B);
    tmp |= 0xe70000;
    WREG32(VCE_CLOCK_GATING_B, tmp);
    tmp = RREG32(VCE_UENC_CLOCK_GATING);
    tmp |= 0xff000000;
    WREG32(VCE_UENC_CLOCK_GATING, tmp);
    tmp = RREG32(VCE_UENC_REG_CLOCK_GATING);
    tmp &= ~0x3fc;
    WREG32(VCE_UENC_REG_CLOCK_GATING, tmp);
    WREG32(VCE_CGTT_CLK_OVERRIDE, 0);
    } else {
    tmp = RREG32(VCE_CLOCK_GATING_B);
    tmp |= 0xe7;
    tmp &= ~0xe70000;
    WREG32(VCE_CLOCK_GATING_B, tmp);
    tmp = RREG32(VCE_UENC_CLOCK_GATING);
    tmp |= 0x1fe000;
    tmp &= ~0xff000000;
    WREG32(VCE_UENC_CLOCK_GATING, tmp);
    tmp = RREG32(VCE_UENC_REG_CLOCK_GATING);
    tmp |= 0x3fc;
    WREG32(VCE_UENC_REG_CLOCK_GATING, tmp);
    }
    }
#[no_mangle]
unsafe extern "C" fn vce_v2_0_set_dyn_cg(rdev: *mut radeon_device, gated: bool) {
    static void vce_v2_0_set_dyn_cg(struct radeon_device *rdev, bool gated)
    {
    u32 orig, tmp;
    tmp = RREG32(VCE_CLOCK_GATING_B);
    tmp &= ~0x00060006;
    if (gated) {
    tmp |= 0xe10000;
    } else {
    tmp |= 0xe1;
    tmp &= ~0xe10000;
    }
    WREG32(VCE_CLOCK_GATING_B, tmp);
    orig = tmp = RREG32(VCE_UENC_CLOCK_GATING);
    tmp &= ~0x1fe000;
    tmp &= ~0xff000000;
    if (tmp != orig)
    WREG32(VCE_UENC_CLOCK_GATING, tmp);
    orig = tmp = RREG32(VCE_UENC_REG_CLOCK_GATING);
    tmp &= ~0x3fc;
    if (tmp != orig)
    WREG32(VCE_UENC_REG_CLOCK_GATING, tmp);
    if (gated)
    WREG32(VCE_CGTT_CLK_OVERRIDE, 0);
    }
#[no_mangle]
unsafe extern "C" fn vce_v2_0_disable_cg(rdev: *mut radeon_device) {
    static void vce_v2_0_disable_cg(struct radeon_device *rdev)
    {
    WREG32(VCE_CGTT_CLK_OVERRIDE, 7);
    }
//
// Local variable sw_cg is used for debugging purposes, in case we
// ran into problems with dynamic clock gating. Don't remove it.
//
#[no_mangle]
pub unsafe extern "C" fn vce_v2_0_enable_mgcg(rdev: *mut radeon_device, enable: bool) {
    void vce_v2_0_enable_mgcg(struct radeon_device *rdev, bool enable)
    {
    let mut sw_cg: bool = false;
    if (enable && (rdev.cg_flags & RADEON_CG_SUPPORT_VCE_MGCG)) {
    if (sw_cg)
    vce_v2_0_set_sw_cg(rdev, true);
    else
    vce_v2_0_set_dyn_cg(rdev, true);
    } else {
    vce_v2_0_disable_cg(rdev);
    if (sw_cg)
    vce_v2_0_set_sw_cg(rdev, false);
    else
    vce_v2_0_set_dyn_cg(rdev, false);
    }
    }
#[no_mangle]
unsafe extern "C" fn vce_v2_0_init_cg(rdev: *mut radeon_device) {
    static void vce_v2_0_init_cg(struct radeon_device *rdev)
    {
    u32 tmp;
    tmp = RREG32(VCE_CLOCK_GATING_A);
    tmp &= ~(CGC_CLK_GATE_DLY_TIMER_MASK | CGC_CLK_GATER_OFF_DLY_TIMER_MASK);
    tmp |= (CGC_CLK_GATE_DLY_TIMER(0) | CGC_CLK_GATER_OFF_DLY_TIMER(4));
    tmp |= CGC_UENC_WAIT_AWAKE;
    WREG32(VCE_CLOCK_GATING_A, tmp);
    tmp = RREG32(VCE_UENC_CLOCK_GATING);
    tmp &= ~(CLOCK_ON_DELAY_MASK | CLOCK_OFF_DELAY_MASK);
    tmp |= (CLOCK_ON_DELAY(0) | CLOCK_OFF_DELAY(4));
    WREG32(VCE_UENC_CLOCK_GATING, tmp);
    tmp = RREG32(VCE_CLOCK_GATING_B);
    tmp |= 0x10;
    tmp &= ~0x100000;
    WREG32(VCE_CLOCK_GATING_B, tmp);
    }
#[no_mangle]
pub unsafe extern "C" fn vce_v2_0_bo_size(rdev: *mut radeon_device) -> unsigned {
    unsigned vce_v2_0_bo_size(struct radeon_device *rdev)
    {
    WARN_ON(rdev.vce_fw.size > VCE_V2_0_FW_SIZE);
    return VCE_V2_0_FW_SIZE + VCE_V2_0_STACK_SIZE + VCE_V2_0_DATA_SIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn vce_v2_0_resume(rdev: *mut radeon_device) -> c_int {
    int vce_v2_0_resume(struct radeon_device *rdev)
    {
    let mut addr: u64 = rdev.vce.gpu_addr;
    uint32_t size;
    WREG32_P(VCE_CLOCK_GATING_A, 0, ~(1 << 16));
    WREG32_P(VCE_UENC_CLOCK_GATING, 0x1FF000, ~0xFF9FF000);
    WREG32_P(VCE_UENC_REG_CLOCK_GATING, 0x3F, ~0x3F);
    WREG32(VCE_CLOCK_GATING_B, 0xf7);
    WREG32(VCE_LMI_CTRL, 0x00398000);
    WREG32_P(VCE_LMI_CACHE_CTRL, 0x0, ~0x1);
    WREG32(VCE_LMI_SWAP_CNTL, 0);
    WREG32(VCE_LMI_SWAP_CNTL1, 0);
    WREG32(VCE_LMI_VM_CTRL, 0);
    WREG32(VCE_LMI_VCPU_CACHE_40BIT_BAR, addr >> 8);
    addr &= 0xff;
    size = VCE_V2_0_FW_SIZE;
    WREG32(VCE_VCPU_CACHE_OFFSET0, addr & 0x7fffffff);
    WREG32(VCE_VCPU_CACHE_SIZE0, size);
    addr += size;
    size = VCE_V2_0_STACK_SIZE;
    WREG32(VCE_VCPU_CACHE_OFFSET1, addr & 0x7fffffff);
    WREG32(VCE_VCPU_CACHE_SIZE1, size);
    addr += size;
    size = VCE_V2_0_DATA_SIZE;
    WREG32(VCE_VCPU_CACHE_OFFSET2, addr & 0x7fffffff);
    WREG32(VCE_VCPU_CACHE_SIZE2, size);
    WREG32_P(VCE_LMI_CTRL2, 0x0, ~0x100);
    WREG32_P(VCE_SYS_INT_EN, VCE_SYS_INT_TRAP_INTERRUPT_EN,
    ~VCE_SYS_INT_TRAP_INTERRUPT_EN);
    vce_v2_0_init_cg(rdev);
    return 0;
    }
