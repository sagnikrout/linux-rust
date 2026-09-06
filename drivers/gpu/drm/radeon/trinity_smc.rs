//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/radeon/trinity_smc.c
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
// Copyright 2012 Advanced Micro Devices, Inc.
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
unsafe extern "C" fn trinity_notify_message_to_smu(rdev: *mut radeon_device, id: u32) -> c_int {
    static int trinity_notify_message_to_smu(struct radeon_device *rdev, u32 id)
    {
    int i;
    let mut v: u32 = 0;
    WREG32(SMC_MESSAGE_0, id);
    for (i = 0; i < rdev.usec_timeout; i++) {
    if (RREG32(SMC_RESP_0) != 0)
    break;
    udelay(1);
    }
    v = RREG32(SMC_RESP_0);
    if (v != 1) {
    if (v == 0xFF) {
    DRM_ERROR("SMC failed to handle the message!\n");
    return -EINVAL;
    } else if (v == 0xFE) {
    DRM_ERROR("Unknown SMC message!\n");
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trinity_dpm_bapm_enable(rdev: *mut radeon_device, enable: bool) -> c_int {
    int trinity_dpm_bapm_enable(struct radeon_device *rdev, bool enable)
    {
    if (enable)
    return trinity_notify_message_to_smu(rdev, PPSMC_MSG_EnableBAPM);
    else
    return trinity_notify_message_to_smu(rdev, PPSMC_MSG_DisableBAPM);
    }
#[no_mangle]
pub unsafe extern "C" fn trinity_dpm_config(rdev: *mut radeon_device, enable: bool) -> c_int {
    int trinity_dpm_config(struct radeon_device *rdev, bool enable)
    {
    if (enable)
    WREG32_SMC(SMU_SCRATCH0, 1);
    else
    WREG32_SMC(SMU_SCRATCH0, 0);
    return trinity_notify_message_to_smu(rdev, PPSMC_MSG_DPM_Config);
    }
#[no_mangle]
pub unsafe extern "C" fn trinity_dpm_force_state(rdev: *mut radeon_device, n: u32) -> c_int {
    int trinity_dpm_force_state(struct radeon_device *rdev, u32 n)
    {
    WREG32_SMC(SMU_SCRATCH0, n);
    return trinity_notify_message_to_smu(rdev, PPSMC_MSG_DPM_ForceState);
    }
#[no_mangle]
pub unsafe extern "C" fn trinity_dpm_n_levels_disabled(rdev: *mut radeon_device, n: u32) -> c_int {
    int trinity_dpm_n_levels_disabled(struct radeon_device *rdev, u32 n)
    {
    WREG32_SMC(SMU_SCRATCH0, n);
    return trinity_notify_message_to_smu(rdev, PPSMC_MSG_DPM_N_LevelsDisabled);
    }
#[no_mangle]
pub unsafe extern "C" fn trinity_uvd_dpm_config(rdev: *mut radeon_device) -> c_int {
    int trinity_uvd_dpm_config(struct radeon_device *rdev)
    {
    return trinity_notify_message_to_smu(rdev, PPSMC_MSG_UVD_DPM_Config);
    }
#[no_mangle]
pub unsafe extern "C" fn trinity_dpm_no_forced_level(rdev: *mut radeon_device) -> c_int {
    int trinity_dpm_no_forced_level(struct radeon_device *rdev)
    {
    return trinity_notify_message_to_smu(rdev, PPSMC_MSG_NoForcedLevel);
    }
    int trinity_dce_enable_voltage_adjustment(struct radeon_device *rdev,
    bool enable)
    {
    if (enable)
    return trinity_notify_message_to_smu(rdev, PPSMC_MSG_DCE_AllowVoltageAdjustment);
    else
    return trinity_notify_message_to_smu(rdev, PPSMC_MSG_DCE_RemoveVoltageAdjustment);
    }
#[no_mangle]
pub unsafe extern "C" fn trinity_gfx_dynamic_mgpg_config(rdev: *mut radeon_device) -> c_int {
    int trinity_gfx_dynamic_mgpg_config(struct radeon_device *rdev)
    {
    return trinity_notify_message_to_smu(rdev, PPSMC_MSG_PG_SIMD_Config);
    }
#[no_mangle]
pub unsafe extern "C" fn trinity_acquire_mutex(rdev: *mut radeon_device) {
    void trinity_acquire_mutex(struct radeon_device *rdev)
    {
    int i;
    WREG32(SMC_INT_REQ, 1);
    for (i = 0; i < rdev.usec_timeout; i++) {
    if ((RREG32(SMC_INT_REQ) & 0xffff) == 1)
    break;
    udelay(1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn trinity_release_mutex(rdev: *mut radeon_device) {
    void trinity_release_mutex(struct radeon_device *rdev)
    {
    WREG32(SMC_INT_REQ, 0);
    }
