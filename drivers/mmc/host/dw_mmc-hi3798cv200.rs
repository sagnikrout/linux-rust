//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/dw_mmc-hi3798cv200.c
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
// Copyright (c) 2018 HiSilicon Technologies Co., Ltd.
//

pub const ALL_INT_CLR: c_uint = 0x1ffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi3798cv200_priv {
    pub sample_clk: *mut clk,
    pub drive_clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn dw_mci_hi3798cv200_set_ios(host: *mut dw_mci, ios: *mut mmc_ios) {
    static void dw_mci_hi3798cv200_set_ios(struct dw_mci *host, struct mmc_ios *ios)
    {
    struct hi3798cv200_priv *priv = host.priv;
    u32 val;
    val = mci_readl(host, UHS_REG);
    if (ios.timing == MMC_TIMING_MMC_DDR52 ||
    ios.timing == MMC_TIMING_UHS_DDR50)
    val |= SDMMC_UHS_DDR;
    else
    val &= ~SDMMC_UHS_DDR;
    mci_writel(host, UHS_REG, val);
    val = mci_readl(host, ENABLE_SHIFT);
    if (ios.timing == MMC_TIMING_MMC_DDR52)
    val |= SDMMC_ENABLE_PHASE;
    else
    val &= ~SDMMC_ENABLE_PHASE;
    mci_writel(host, ENABLE_SHIFT, val);
    val = mci_readl(host, DDR_REG);
    if (ios.timing == MMC_TIMING_MMC_HS400)
    val |= SDMMC_DDR_HS400;
    else
    val &= ~SDMMC_DDR_HS400;
    mci_writel(host, DDR_REG, val);
    if (ios.timing == MMC_TIMING_MMC_HS ||
    ios.timing == MMC_TIMING_LEGACY)
    clk_set_phase(priv.drive_clk, 180);
#[no_mangle]
pub unsafe extern "C" fn if(MMC_TIMING_MMC_HS200: ios->timing ==) -> else {
    else if (ios.timing == MMC_TIMING_MMC_HS200)
    clk_set_phase(priv.drive_clk, 135);
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_hi3798cv200_execute_tuning(host: *mut dw_mci, opcode: u32) -> c_int {
    static int dw_mci_hi3798cv200_execute_tuning(struct dw_mci *host, u32 opcode)
    {
    static const int degrees[] = { 0, 45, 90, 135, 180, 225, 270, 315 };
    struct hi3798cv200_priv *priv = host.priv;
    let mut raise_point: c_int = -1, fall_point = -1;
    int err, prev_err = -1;
    let mut found: c_int = 0;
    int i;
    for (i = 0; i < ARRAY_SIZE(degrees); i++) {
    clk_set_phase(priv.sample_clk, degrees[i]);
    mci_writel(host, RINTSTS, ALL_INT_CLR);
    err = mmc_send_tuning(host.mmc, opcode, core::ptr::null_mut());
    if (!err)
    found = 1;
    if (i > 0) {
    if (err && !prev_err)
    fall_point = i - 1;
    if (!err && prev_err)
    raise_point = i;
    }
    if (raise_point != -1 && fall_point != -1)
    goto tuning_out;
    prev_err = err;
    }
    tuning_out:
    if (found) {
    if (raise_point == -1)
    raise_point = 0;
    if (fall_point == -1)
    fall_point = ARRAY_SIZE(degrees) - 1;
    if (fall_point < raise_point) {
    if ((raise_point + fall_point) >
    (ARRAY_SIZE(degrees) - 1))
    i = fall_point / 2;
    else
    i = (raise_point + ARRAY_SIZE(degrees) - 1) / 2;
    } else {
    i = (raise_point + fall_point) / 2;
    }
    clk_set_phase(priv.sample_clk, degrees[i]);
    dev_dbg(host.dev, "Tuning clk_sample[%d, %d], set[%d]\n",
    raise_point, fall_point, degrees[i]);
    } else {
    dev_err(host.dev, "No valid clk_sample shift! use default\n");
    err = -EINVAL;
    }
    mci_writel(host, RINTSTS, ALL_INT_CLR);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_hi3798cv200_init(host: *mut dw_mci) -> c_int {
    static int dw_mci_hi3798cv200_init(struct dw_mci *host)
    {
    struct hi3798cv200_priv *priv;
    int ret;
    priv = devm_kzalloc(host.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.sample_clk = devm_clk_get(host.dev, "ciu-sample");
    if (IS_ERR(priv.sample_clk)) {
    dev_err(host.dev, "failed to get ciu-sample clock\n");
    return PTR_ERR(priv.sample_clk);
    }
    priv.drive_clk = devm_clk_get(host.dev, "ciu-drive");
    if (IS_ERR(priv.drive_clk)) {
    dev_err(host.dev, "failed to get ciu-drive clock\n");
    return PTR_ERR(priv.drive_clk);
    }
    ret = clk_prepare_enable(priv.sample_clk);
    if (ret) {
    dev_err(host.dev, "failed to enable ciu-sample clock\n");
    return ret;
    }
    ret = clk_prepare_enable(priv.drive_clk);
    if (ret) {
    dev_err(host.dev, "failed to enable ciu-drive clock\n");
    goto disable_sample_clk;
    }
    host.priv = priv;
    return 0;
    disable_sample_clk:
    clk_disable_unprepare(priv.sample_clk);
    return ret;
    }
    static const struct dw_mci_drv_data hi3798cv200_data = {
    .common_caps = MMC_CAP_CMD23,
    .init = dw_mci_hi3798cv200_init,
    .set_ios = dw_mci_hi3798cv200_set_ios,
    .execute_tuning = dw_mci_hi3798cv200_execute_tuning,
    };
#[no_mangle]
unsafe extern "C" fn dw_mci_hi3798cv200_probe(pdev: *mut platform_device) -> c_int {
    static int dw_mci_hi3798cv200_probe(struct platform_device *pdev)
    {
    return dw_mci_pltfm_register(pdev, &hi3798cv200_data);
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_hi3798cv200_remove(pdev: *mut platform_device) {
    static void dw_mci_hi3798cv200_remove(struct platform_device *pdev)
    {
    struct dw_mci *host = platform_get_drvdata(pdev);
    struct hi3798cv200_priv *priv = host.priv;
    clk_disable_unprepare(priv.drive_clk);
    clk_disable_unprepare(priv.sample_clk);
    dw_mci_pltfm_remove(pdev);
    }
    static const struct of_device_id dw_mci_hi3798cv200_match[] = {
    { .compatible = "hisilicon,hi3798cv200-dw-mshc", },
    {},
    };
    MODULE_DEVICE_TABLE(of, dw_mci_hi3798cv200_match);
    static struct platform_driver dw_mci_hi3798cv200_driver = {
    .probe = dw_mci_hi3798cv200_probe,
    .remove = dw_mci_hi3798cv200_remove,
    .driver = {
    .name = "dwmmc_hi3798cv200",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = dw_mci_hi3798cv200_match,
    },
    };
    module_platform_driver(dw_mci_hi3798cv200_driver);
    MODULE_DESCRIPTION("HiSilicon Hi3798CV200 Specific DW-MSHC Driver Extension");
    MODULE_LICENSE("GPL v2");
