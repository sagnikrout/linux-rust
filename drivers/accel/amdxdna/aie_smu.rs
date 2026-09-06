//! Automatically rewritten from C to Rust
//! Source: drivers/accel/amdxdna/aie_smu.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2026, Advanced Micro Devices, Inc.
//

pub const SMU_RESULT_OK: c_int = 1;
// SMU commands
pub const AIE_SMU_POWER_ON: c_uint = 0x3;
pub const AIE_SMU_POWER_OFF: c_uint = 0x4;
pub const AIE_SMU_SET_MPNPUCLK_FREQ: c_uint = 0x5;
pub const AIE_SMU_SET_HCLK_FREQ: c_uint = 0x6;
pub const AIE_SMU_SET_SOFT_DPMLEVEL: c_uint = 0x7;
pub const AIE_SMU_SET_HARD_DPMLEVEL: c_uint = 0x8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_device {
    pub ddev: *mut drm_device,
    pub conf: smu_config,
    pub smu_regs: [*mut void __iomem; SMU_MAX_REGS],
}

#[no_mangle]
unsafe extern "C" fn aie_smu_exec(smu: *mut smu_device, reg_cmd: u32, reg_arg: u32, out: *mut u32) -> c_int {
    static int aie_smu_exec(struct smu_device *smu, u32 reg_cmd, u32 reg_arg, u32 *out)
    {
    u32 resp;
    int ret;
    writel(0, SMU_REG(smu, SMU_RESP_REG));
    writel(reg_arg, SMU_REG(smu, SMU_ARG_REG));
    writel(reg_cmd, SMU_REG(smu, SMU_CMD_REG));
// Clear and set SMU_INTR_REG to kick off
    writel(0, SMU_REG(smu, SMU_INTR_REG));
    writel(1, SMU_REG(smu, SMU_INTR_REG));
    ret = readx_poll_timeout(readl, SMU_REG(smu, SMU_RESP_REG), resp,
    resp, AIE_INTERVAL, AIE_TIMEOUT);
    if (ret) {
    drm_err(smu.ddev, "smu cmd %d timed out", reg_cmd);
    return ret;
    }
    if (out)
// out = readl(SMU_REG(smu, SMU_OUT_REG));
    if (resp != SMU_RESULT_OK) {
    drm_err(smu.ddev, "smu cmd %d failed, 0x%x", reg_cmd, resp);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn aie_smu_init(smu: *mut smu_device) -> c_int {
    int aie_smu_init(struct smu_device *smu)
    {
    int ret;
//
// Failing to set power off indicates an unrecoverable hardware or
// firmware error.
//
    ret = aie_smu_exec(smu, AIE_SMU_POWER_OFF, 0, core::ptr::null_mut());
    if (ret) {
    drm_err(smu.ddev, "Access power failed, ret %d", ret);
    return ret;
    }
    ret = aie_smu_exec(smu, AIE_SMU_POWER_ON, 0, core::ptr::null_mut());
    if (ret) {
    drm_err(smu.ddev, "Power on failed, ret %d", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn aie_smu_fini(smu: *mut smu_device) {
    void aie_smu_fini(struct smu_device *smu)
    {
    int ret;
    ret = aie_smu_exec(smu, AIE_SMU_POWER_OFF, 0, core::ptr::null_mut());
    if (ret)
    drm_err(smu.ddev, "Power off failed, ret %d", ret);
    }
#[no_mangle]
pub unsafe extern "C" fn aie_smu_set_clocks(smu: *mut smu_device, npuclk: *mut u32, hclk: *mut u32) -> c_int {
    int aie_smu_set_clocks(struct smu_device *smu, u32 *npuclk, u32 *hclk)
    {
    int ret;
    if (npuclk) {
    ret = aie_smu_exec(smu, AIE_SMU_SET_MPNPUCLK_FREQ, *npuclk, npuclk);
    if (ret) {
    drm_err(smu.ddev, "Set mpnpu clock to %d failed, ret %d", *npuclk, ret);
    return ret;
    }
    }
    if (hclk) {
    ret = aie_smu_exec(smu, AIE_SMU_SET_HCLK_FREQ, *hclk, hclk);
    if (ret) {
    drm_err(smu.ddev, "Set hclock to %d failed, ret %d",
// hclk, ret);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn aie_smu_set_dpm(smu: *mut smu_device, dpm_level: u32) -> c_int {
    int aie_smu_set_dpm(struct smu_device *smu, u32 dpm_level)
    {
    int ret;
    ret = aie_smu_exec(smu, AIE_SMU_SET_HARD_DPMLEVEL, dpm_level, core::ptr::null_mut());
    if (ret) {
    drm_err(smu.ddev, "Set hard dpm level %d failed, ret %d",
    dpm_level, ret);
    return ret;
    }
    ret = aie_smu_exec(smu, AIE_SMU_SET_SOFT_DPMLEVEL, dpm_level, core::ptr::null_mut());
    if (ret) {
    drm_err(smu.ddev, "Set soft dpm level %d failed, ret %d",
    dpm_level, ret);
    return ret;
    }
    return 0;
    }
    struct smu_device *aiem_smu_create(struct drm_device *ddev, struct smu_config *conf)
    {
    struct smu_device *smu;
    smu = drmm_kzalloc(ddev, sizeof(*smu), GFP_KERNEL);
    if (!smu)
    return core::ptr::null_mut();
    smu.ddev = ddev;
    memcpy(smu.smu_regs, conf.smu_regs, sizeof(smu.smu_regs));
    return smu;
    }
