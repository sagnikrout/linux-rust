//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/dw_mmc-hi3798mv200.c
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
// Modified from dw_mmc-hi3798cv200.c
//
// Copyright (c) 2024 Yang Xiwen <forbidden405@outlook.com>
// Copyright (c) 2018 HiSilicon Technologies Co., Ltd.
//

pub const SDMMC_TUNING_CTRL: c_uint = 0x118;

pub const ALL_INT_CLR: c_uint = 0x1ffff;
// DLL ctrl reg

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mci_hi3798mv200_priv {
    pub sample_clk: *mut clk,
    pub drive_clk: *mut clk,
    pub crg_reg: *mut regmap,
    pub sap_dll_offset: u32,
}

#[no_mangle]
unsafe extern "C" fn dw_mci_hi3798mv200_set_ios(host: *mut dw_mci, ios: *mut mmc_ios) {
    static void dw_mci_hi3798mv200_set_ios(struct dw_mci *host, struct mmc_ios *ios)
    {
    struct dw_mci_hi3798mv200_priv *priv = host.priv;
    let mut phase: mmc_clk_phase = host.phase_map.phase[ios.timing];
    u32 val;
    val = mci_readl(host, ENABLE_SHIFT);
    if (ios.timing == MMC_TIMING_MMC_DDR52
    || ios.timing == MMC_TIMING_UHS_DDR50)
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
    if (clk_set_rate(host.ciu_clk, ios.clock))
    dev_warn(host.dev, "Failed to set rate to %u\n", ios.clock);
    else
//
// CLK_MUX_ROUND_NEAREST is enabled for this clock
// The actual clock rate is not what we set, but a rounded value
// so we should get the rate once again
//
    host.bus_hz = clk_get_rate(host.ciu_clk);
    if (phase.valid) {
    clk_set_phase(priv.drive_clk, phase.out_deg);
    clk_set_phase(priv.sample_clk, phase.in_deg);
    } else {
    dev_warn(host.dev,
    "The phase entry for timing mode %d is missing in device tree.\n",
    ios.timing);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dw_mci_hi3798mv200_enable_tuning(host: *mut dw_mci) -> c_int {
    static inline int dw_mci_hi3798mv200_enable_tuning(struct dw_mci *host)
    {
    struct dw_mci_hi3798mv200_priv *priv = host.priv;
    return regmap_clear_bits(priv.crg_reg, priv.sap_dll_offset, SAP_DLL_CTRL_DLLMODE);
    }
#[no_mangle]
pub unsafe extern "C" fn dw_mci_hi3798mv200_disable_tuning(host: *mut dw_mci) -> c_int {
    static inline int dw_mci_hi3798mv200_disable_tuning(struct dw_mci *host)
    {
    struct dw_mci_hi3798mv200_priv *priv = host.priv;
    return regmap_set_bits(priv.crg_reg, priv.sap_dll_offset, SAP_DLL_CTRL_DLLMODE);
    }
    static int dw_mci_hi3798mv200_execute_tuning_mix_mode(struct dw_mci *host,
    u32 opcode)
    {
    static const int degrees[] = { 0, 45, 90, 135, 180, 225, 270, 315 };
    struct dw_mci_hi3798mv200_priv *priv = host.priv;
    let mut raise_point: c_int = -1, fall_point = -1, mid;
    int err, prev_err = -1;
    let mut found: c_int = 0;
    int regval;
    int i;
    int ret;
    ret = dw_mci_hi3798mv200_enable_tuning(host);
    if (ret < 0)
    return ret;
    for (i = 0; i < ARRAY_SIZE(degrees); i++) {
    clk_set_phase(priv.sample_clk, degrees[i]);
    mci_writel(host, RINTSTS, ALL_INT_CLR);
//
// HiSilicon implemented a tuning mechanism.
// It needs special interaction with the DLL.
//
// Treat edge(flip) found as an error too.
//
    err = mmc_send_tuning(host.mmc, opcode, core::ptr::null_mut());
    regval = mci_readl(host, TUNING_CTRL);
    if (err || (regval & SDMMC_TUNING_FIND_EDGE))
    err = 1;
    else
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
    ret = dw_mci_hi3798mv200_disable_tuning(host);
    if (ret < 0)
    return ret;
    if (found) {
    if (raise_point == -1)
    raise_point = 0;
    if (fall_point == -1)
    fall_point = ARRAY_SIZE(degrees) - 1;
    if (fall_point < raise_point) {
    if ((raise_point + fall_point) >
    (ARRAY_SIZE(degrees) - 1))
    mid = fall_point / 2;
    else
    mid = (raise_point + ARRAY_SIZE(degrees) - 1) / 2;
    } else {
    mid = (raise_point + fall_point) / 2;
    }
//
// We don't care what timing we are tuning for,
// simply use the same phase for all timing needs tuning.
//
    host.phase_map.phase[MMC_TIMING_MMC_HS200].in_deg = degrees[mid];
    host.phase_map.phase[MMC_TIMING_MMC_HS400].in_deg = degrees[mid];
    host.phase_map.phase[MMC_TIMING_UHS_SDR104].in_deg = degrees[mid];
    clk_set_phase(priv.sample_clk, degrees[mid]);
    dev_dbg(host.dev, "Tuning clk_sample[%d, %d], set[%d]\n",
    raise_point, fall_point, degrees[mid]);
    ret = 0;
    } else {
    dev_err(host.dev, "No valid clk_sample shift!\n");
    ret = -EINVAL;
    }
    mci_writel(host, RINTSTS, ALL_INT_CLR);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_hi3798mv200_init(host: *mut dw_mci) -> c_int {
    static int dw_mci_hi3798mv200_init(struct dw_mci *host)
    {
    struct dw_mci_hi3798mv200_priv *priv;
    struct device_node *np = host.dev.of_node;
    priv = devm_kzalloc(host.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.sample_clk = devm_clk_get_enabled(host.dev, "ciu-sample");
    if (IS_ERR(priv.sample_clk))
    return dev_err_probe(host.dev, PTR_ERR(priv.sample_clk),
    "failed to get enabled ciu-sample clock\n");
    priv.drive_clk = devm_clk_get_enabled(host.dev, "ciu-drive");
    if (IS_ERR(priv.drive_clk))
    return dev_err_probe(host.dev, PTR_ERR(priv.drive_clk),
    "failed to get enabled ciu-drive clock\n");
    priv.crg_reg = syscon_regmap_lookup_by_phandle_args(np, "hisilicon,sap-dll-reg",
    1, &priv.sap_dll_offset);
    if (IS_ERR(priv.crg_reg))
    return dev_err_probe(host.dev, PTR_ERR(priv.crg_reg),
    "failed to get CRG reg\n");
    host.priv = priv;
    return 0;
    }
    static const struct dw_mci_drv_data hi3798mv200_data = {
    .common_caps = MMC_CAP_CMD23,
    .init = dw_mci_hi3798mv200_init,
    .set_ios = dw_mci_hi3798mv200_set_ios,
    .execute_tuning = dw_mci_hi3798mv200_execute_tuning_mix_mode,
    };
    static const struct of_device_id dw_mci_hi3798mv200_match[] = {
    { .compatible = "hisilicon,hi3798mv200-dw-mshc" },
    {},
    };
#[no_mangle]
unsafe extern "C" fn dw_mci_hi3798mv200_probe(pdev: *mut platform_device) -> c_int {
    static int dw_mci_hi3798mv200_probe(struct platform_device *pdev)
    {
    return dw_mci_pltfm_register(pdev, &hi3798mv200_data);
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_hi3798mv200_remove(pdev: *mut platform_device) {
    static void dw_mci_hi3798mv200_remove(struct platform_device *pdev)
    {
    dw_mci_pltfm_remove(pdev);
    }
    MODULE_DEVICE_TABLE(of, dw_mci_hi3798mv200_match);
    static struct platform_driver dw_mci_hi3798mv200_driver = {
    .probe = dw_mci_hi3798mv200_probe,
    .remove = dw_mci_hi3798mv200_remove,
    .driver = {
    .name = "dwmmc_hi3798mv200",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = dw_mci_hi3798mv200_match,
    },
    };
    module_platform_driver(dw_mci_hi3798mv200_driver);
    MODULE_DESCRIPTION("HiSilicon Hi3798MV200 Specific DW-MSHC Driver Extension");
    MODULE_LICENSE("GPL");
