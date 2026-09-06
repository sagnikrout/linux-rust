//! Automatically rewritten from C to Rust
//! Source: sound/soc/amd/acp7x/acp7x-common.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// AMD ACP PCI driver callback routines for ACP7.x
// platforms.
//
// Copyright 2026 Advanced Micro Devices, Inc.
//

#[no_mangle]
unsafe extern "C" fn acp7x_power_on(acp_base: *mut void __iomem) -> c_int {
    static int acp7x_power_on(void __iomem *acp_base)
    {
    let mut val: u32 = 0;
    val = readl(acp_base + ACP_PGFSM_STATUS);
    if (!(val & ACP7X_PGFSM_STATUS_MASK))
    return 0;
    writel(ACP7X_PGFSM_CNTL_POWER_ON_MASK, acp_base + ACP_PGFSM_CONTROL);
    val = readl(acp_base + ACP_PGFSM_CONTROL);
    return readl_poll_timeout(acp_base + ACP_PGFSM_STATUS, val,
    ((val & ACP7X_PGFSM_STATUS_MASK) == 0), DELAY_US, ACP7X_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn acp7x_reset(acp_base: *mut void __iomem) -> c_int {
    static int acp7x_reset(void __iomem *acp_base)
    {
    u32 val;
    int ret;
    writel(1, acp_base + ACP_SOFT_RESET);
    ret = readl_poll_timeout(acp_base + ACP_SOFT_RESET, val,
    val & ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK,
    DELAY_US, ACP7X_TIMEOUT);
    if (ret)
    return ret;
    writel(0, acp_base + ACP_SOFT_RESET);
    return readl_poll_timeout(acp_base + ACP_SOFT_RESET, val, !val, DELAY_US, ACP7X_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn acp7x_init(acp_base: *mut void __iomem, dev: *mut device) -> c_int {
    static int acp7x_init(void __iomem *acp_base, struct device *dev)
    {
    int ret;
    ret = acp7x_power_on(acp_base);
    if (ret) {
    dev_err(dev, "ACP power on failed\n");
    return ret;
    }
    writel(0x01, acp_base + ACP_CONTROL);
    ret = acp7x_reset(acp_base);
    if (ret) {
    dev_err(dev, "ACP reset failed\n");
    return ret;
    }
    writel(0, acp_base + ACP_ZSC_DSP_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acp7x_deinit(acp_base: *mut void __iomem, dev: *mut device) -> c_int {
    static int acp7x_deinit(void __iomem *acp_base, struct device *dev)
    {
    int ret;
    ret = acp7x_reset(acp_base);
    if (ret) {
    dev_err(dev, "ACP reset failed\n");
    return ret;
    }
    writel(0x01, acp_base + ACP_ZSC_DSP_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_acp7x_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused snd_acp7x_suspend(struct device *dev)
    {
    struct acp7x_dev_data *adata;
    int ret;
    adata = dev_get_drvdata(dev);
    ret = acp_hw_deinit(adata, dev);
    if (ret)
    dev_err(dev, "ACP de-init failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn snd_acp7x_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused snd_acp7x_runtime_resume(struct device *dev)
    {
    struct acp7x_dev_data *adata;
    int ret;
    adata = dev_get_drvdata(dev);
    ret = acp_hw_init(adata, dev);
    if (ret) {
    dev_err(dev, "ACP init failed\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_acp7x_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused snd_acp7x_resume(struct device *dev)
    {
    struct acp7x_dev_data *adata;
    int ret;
    adata = dev_get_drvdata(dev);
    ret = acp_hw_init(adata, dev);
    if (ret)
    dev_err(dev, "ACP init failed\n");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn acp7x_hw_init_ops(hw_ops: *mut acp_hw_ops) {
    void acp7x_hw_init_ops(struct acp_hw_ops *hw_ops)
    {
    hw_ops.acp_init = acp7x_init;
    hw_ops.acp_deinit = acp7x_deinit;
    hw_ops.acp_suspend = snd_acp7x_suspend;
    hw_ops.acp_resume = snd_acp7x_resume;
    hw_ops.acp_suspend_runtime = snd_acp7x_suspend;
    hw_ops.acp_resume_runtime = snd_acp7x_runtime_resume;
    }
