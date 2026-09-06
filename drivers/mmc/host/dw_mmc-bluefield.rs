//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/dw_mmc-bluefield.c
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
// Copyright (C) 2018 Mellanox Technologies.
//

pub const BLUEFIELD_UHS_REG_EXT_SAMPLE: c_int = 2;
pub const BLUEFIELD_UHS_REG_EXT_DRIVE: c_int = 4;
// SMC call for RST_N
pub const BLUEFIELD_SMC_SET_EMMC_RST_N: c_uint = 0x82000007;
#[no_mangle]
unsafe extern "C" fn dw_mci_bluefield_set_ios(host: *mut dw_mci, ios: *mut mmc_ios) {
    static void dw_mci_bluefield_set_ios(struct dw_mci *host, struct mmc_ios *ios)
    {
    u32 reg;
// Update the Drive and Sample fields in register UHS_REG_EXT.
    reg = mci_readl(host, UHS_REG_EXT);
    reg &= ~UHS_REG_EXT_SAMPLE_MASK;
    reg |= FIELD_PREP(UHS_REG_EXT_SAMPLE_MASK,
    BLUEFIELD_UHS_REG_EXT_SAMPLE);
    reg &= ~UHS_REG_EXT_DRIVE_MASK;
    reg |= FIELD_PREP(UHS_REG_EXT_DRIVE_MASK, BLUEFIELD_UHS_REG_EXT_DRIVE);
    mci_writel(host, UHS_REG_EXT, reg);
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_bluefield_hw_reset(host: *mut dw_mci) {
    static void dw_mci_bluefield_hw_reset(struct dw_mci *host)
    {
    let mut res: arm_smccc_res = { 0 };
    arm_smccc_smc(BLUEFIELD_SMC_SET_EMMC_RST_N, 0, 0, 0, 0, 0, 0, 0,
    &res);
    if (res.a0)
    pr_err("RST_N failed.\n");
    }
    static const struct dw_mci_drv_data bluefield_drv_data = {
    .set_ios		= dw_mci_bluefield_set_ios,
    .hw_reset		= dw_mci_bluefield_hw_reset
    };
    static const struct of_device_id dw_mci_bluefield_match[] = {
    { .compatible = "mellanox,bluefield-dw-mshc",
    .data = &bluefield_drv_data },
    {},
    };
    MODULE_DEVICE_TABLE(of, dw_mci_bluefield_match);
#[no_mangle]
unsafe extern "C" fn dw_mci_bluefield_probe(pdev: *mut platform_device) -> c_int {
    static int dw_mci_bluefield_probe(struct platform_device *pdev)
    {
    return dw_mci_pltfm_register(pdev, &bluefield_drv_data);
    }
    static struct platform_driver dw_mci_bluefield_pltfm_driver = {
    .probe		= dw_mci_bluefield_probe,
    .remove		= dw_mci_pltfm_remove,
    .driver		= {
    .name		= "dwmmc_bluefield",
    .probe_type	= PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table	= dw_mci_bluefield_match,
    .pm		= pm_ptr(&dw_mci_pmops),
    },
    };
    module_platform_driver(dw_mci_bluefield_pltfm_driver);
    MODULE_DESCRIPTION("BlueField DW Multimedia Card driver");
    MODULE_AUTHOR("Mellanox Technologies");
    MODULE_LICENSE("GPL v2");
