//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/dw_mmc-starfive.c
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
// StarFive Designware Mobile Storage Host Controller Driver
//
// Copyright (c) 2022 StarFive Technology Co., Ltd.
//

pub const ALL_INT_CLR: c_uint = 0x1ffff;
pub const MAX_DELAY_CHAIN: c_int = 32;

#[no_mangle]
unsafe extern "C" fn dw_mci_starfive_set_ios(host: *mut dw_mci, ios: *mut mmc_ios) {
    static void dw_mci_starfive_set_ios(struct dw_mci *host, struct mmc_ios *ios)
    {
    int ret;
    unsigned int clock;
    if (ios.timing == MMC_TIMING_MMC_DDR52 || ios.timing == MMC_TIMING_UHS_DDR50) {
    clock = (ios.clock > 50000000 && ios.clock <= 52000000) ? 100000000 : ios.clock;
    ret = clk_set_rate(host.ciu_clk, clock);
    if (ret)
    dev_dbg(host.dev, "Use an external frequency divider %uHz\n", ios.clock);
    host.bus_hz = clk_get_rate(host.ciu_clk);
    } else {
    dev_dbg(host.dev, "Using the internal divider\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_starfive_set_sample_phase(host: *mut dw_mci, smpl_phase: u32) {
    static void dw_mci_starfive_set_sample_phase(struct dw_mci *host, u32 smpl_phase)
    {
// change driver phase and sample phase
    let mut reg_value: u32 = mci_readl(host, UHS_REG_EXT);
// In UHS_REG_EXT, only 5 bits valid in DRV_PHASE and SMPL_PHASE
    reg_value &= ~STARFIVE_SMPL_PHASE;
    reg_value |= FIELD_PREP(STARFIVE_SMPL_PHASE, smpl_phase);
    mci_writel(host, UHS_REG_EXT, reg_value);
// We should delay 1ms wait for timing setting finished.
    mdelay(1);
    }
    static int dw_mci_starfive_execute_tuning(struct dw_mci *host,
    u32 opcode)
    {
    let mut grade: static int = MAX_DELAY_CHAIN;
    int smpl_phase, smpl_raise = -1, smpl_fall = -1;
    int ret;
    for (smpl_phase = 0; smpl_phase < grade; smpl_phase++) {
    dw_mci_starfive_set_sample_phase(host, smpl_phase);
    mci_writel(host, RINTSTS, ALL_INT_CLR);
    ret = mmc_send_tuning(host.mmc, opcode, core::ptr::null_mut());
    if (!ret && smpl_raise < 0) {
    smpl_raise = smpl_phase;
    } else if (ret && smpl_raise >= 0) {
    smpl_fall = smpl_phase - 1;
    break;
    }
    }
    if (smpl_phase >= grade)
    smpl_fall = grade - 1;
    if (smpl_raise < 0) {
    smpl_phase = 0;
    dev_err(host.dev, "No valid delay chain! use default\n");
    ret = -EINVAL;
    goto out;
    }
    smpl_phase = (smpl_raise + smpl_fall) / 2;
    dev_dbg(host.dev, "Found valid delay chain! use it [delay=%d]\n", smpl_phase);
    ret = 0;
    out:
    dw_mci_starfive_set_sample_phase(host, smpl_phase);
    mci_writel(host, RINTSTS, ALL_INT_CLR);
    return ret;
    }
    static const struct dw_mci_drv_data starfive_data = {
    .common_caps		= MMC_CAP_CMD23,
    .set_ios		= dw_mci_starfive_set_ios,
    .execute_tuning		= dw_mci_starfive_execute_tuning,
    };
    static const struct of_device_id dw_mci_starfive_match[] = {
    { .compatible = "starfive,jh7110-mmc",
    .data = &starfive_data },
    {},
    };
    MODULE_DEVICE_TABLE(of, dw_mci_starfive_match);
#[no_mangle]
unsafe extern "C" fn dw_mci_starfive_probe(pdev: *mut platform_device) -> c_int {
    static int dw_mci_starfive_probe(struct platform_device *pdev)
    {
    return dw_mci_pltfm_register(pdev, &starfive_data);
    }
    static struct platform_driver dw_mci_starfive_driver = {
    .probe = dw_mci_starfive_probe,
    .remove = dw_mci_pltfm_remove,
    .driver = {
    .name = "dwmmc_starfive",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = dw_mci_starfive_match,
    },
    };
    module_platform_driver(dw_mci_starfive_driver);
    MODULE_DESCRIPTION("StarFive JH7110 Specific DW-MSHC Driver Extension");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:dwmmc_starfive");
