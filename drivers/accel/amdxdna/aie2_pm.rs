//! Automatically rewritten from C to Rust
//! Source: drivers/accel/amdxdna/aie2_pm.c
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
// Copyright (C) 2024, Advanced Micro Devices, Inc.
//

pub const AIE2_CLK_GATING_ENABLE: c_int = 1;
pub const AIE2_CLK_GATING_DISABLE: c_int = 0;
#[no_mangle]
unsafe extern "C" fn aie2_pm_set_clk_gating(ndev: *mut amdxdna_dev_hdl, val: u32) -> c_int {
    static int aie2_pm_set_clk_gating(struct amdxdna_dev_hdl *ndev, u32 val)
    {
    int ret;
    ret = aie2_runtime_cfg(ndev, AIE2_RT_CFG_CLK_GATING, &val);
    if (ret)
    return ret;
    ndev.clk_gating = val;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn aie2_pm_set_dpm(ndev: *mut amdxdna_dev_hdl, dpm_level: u32) -> c_int {
    int aie2_pm_set_dpm(struct amdxdna_dev_hdl *ndev, u32 dpm_level)
    {
    int ret;
    ret = amdxdna_pm_resume_get_locked(ndev.aie.xdna);
    if (ret)
    return ret;
    ret = ndev.priv.hw_ops.set_dpm(ndev, dpm_level);
    if (!ret)
    ndev.dpm_level = dpm_level;
    amdxdna_pm_suspend_put(ndev.aie.xdna);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn aie2_pm_init(ndev: *mut amdxdna_dev_hdl) -> c_int {
    int aie2_pm_init(struct amdxdna_dev_hdl *ndev)
    {
    int ret;
    if (ndev.dev_status != AIE2_DEV_UNINIT) {
// Resume device
    ret = ndev.priv.hw_ops.set_dpm(ndev, ndev.dpm_level);
    if (ret)
    return ret;
    ret = aie2_pm_set_clk_gating(ndev, ndev.clk_gating);
    if (ret)
    return ret;
    return 0;
    }
    while (ndev.priv.dpm_clk_tbl[ndev.max_dpm_level].hclk)
    ndev.max_dpm_level++;
    ndev.max_dpm_level--;
    ret = ndev.priv.hw_ops.set_dpm(ndev, ndev.max_dpm_level);
    if (ret)
    return ret;
    ndev.dpm_level = ndev.max_dpm_level;
    ret = aie2_pm_set_clk_gating(ndev, AIE2_CLK_GATING_ENABLE);
    if (ret)
    return ret;
    ndev.pw_mode = POWER_MODE_DEFAULT;
    ndev.dft_dpm_level = 0;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn aie2_pm_set_mode(ndev: *mut amdxdna_dev_hdl, target: enum amdxdna_power_mode_type) -> c_int {
    int aie2_pm_set_mode(struct amdxdna_dev_hdl *ndev, enum amdxdna_power_mode_type target)
    {
    struct amdxdna_dev *xdna = ndev.aie.xdna;
    u32 clk_gating, dpm_level;
    int ret;
    drm_WARN_ON(&xdna.ddev, !mutex_is_locked(&xdna.dev_lock));
    if (ndev.pw_mode == target)
    return 0;
    switch (target) {
    case POWER_MODE_TURBO:
    if (ndev.hwctx_num) {
    XDNA_ERR(xdna, "Can not set turbo when there is active hwctx");
    return -EINVAL;
    }
    clk_gating = AIE2_CLK_GATING_DISABLE;
    dpm_level = ndev.max_dpm_level;
    break;
    case POWER_MODE_HIGH:
    clk_gating = AIE2_CLK_GATING_ENABLE;
    dpm_level = ndev.max_dpm_level;
    break;
    case POWER_MODE_DEFAULT:
    clk_gating = AIE2_CLK_GATING_ENABLE;
    dpm_level = ndev.dft_dpm_level;
    break;
    case POWER_MODE_LOW:
    clk_gating = AIE2_CLK_GATING_ENABLE;
    dpm_level = 0;
    break;
    case POWER_MODE_MEDIUM:
    clk_gating = AIE2_CLK_GATING_ENABLE;
    dpm_level = ndev.max_dpm_level / 2;
    break;
    default:
    return -EOPNOTSUPP;
    }
    ret = aie2_pm_set_dpm(ndev, dpm_level);
    if (ret)
    return ret;
    ret = aie2_pm_set_clk_gating(ndev, clk_gating);
    if (ret)
    return ret;
    ndev.pw_mode = target;
    return 0;
    }
