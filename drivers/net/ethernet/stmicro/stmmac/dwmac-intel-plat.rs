//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-intel-plat.c
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
// Intel DWMAC platform driver
//
// Copyright(C) 2020 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dwmac {
    pub dev: *mut device,
    pub tx_clk: *mut clk,
    pub data: *const intel_dwmac_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dwmac_data {
    pub ptp_ref_clk_rate: c_ulong,
    pub tx_clk_rate: c_ulong,
    pub tx_clk_en: bool,
}

    static const struct intel_dwmac_data kmb_data = {
    .ptp_ref_clk_rate = 200000000,
    .tx_clk_rate = 125000000,
    .tx_clk_en = true,
    };
    static const struct of_device_id intel_eth_plat_match[] = {
    { .compatible = "intel,keembay-dwmac", .data = &kmb_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, intel_eth_plat_match);
#[no_mangle]
unsafe extern "C" fn intel_eth_plat_probe(pdev: *mut platform_device) -> c_int {
    static int intel_eth_plat_probe(struct platform_device *pdev)
    {
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    struct intel_dwmac *dwmac;
    unsigned long rate;
    int ret;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return ret;
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat)) {
    dev_err(&pdev.dev, "dt configuration failed\n");
    return PTR_ERR(plat_dat);
    }
    dwmac = devm_kzalloc(&pdev.dev, sizeof(*dwmac), GFP_KERNEL);
    if (!dwmac)
    return -ENOMEM;
    dwmac.dev = &pdev.dev;
    dwmac.tx_clk = core::ptr::null_mut();
//
// This cannot return NULL at this point because the driver’s
// compatibility with the device has already been validated in
// platform_match().
//
    dwmac.data = device_get_match_data(&pdev.dev);
// Enable TX clock
    if (dwmac.data.tx_clk_en) {
    dwmac.tx_clk = devm_clk_get(&pdev.dev, "tx_clk");
    if (IS_ERR(dwmac.tx_clk))
    return PTR_ERR(dwmac.tx_clk);
    ret = clk_prepare_enable(dwmac.tx_clk);
    if (ret) {
    dev_err(&pdev.dev,
    "Failed to enable tx_clk\n");
    return ret;
    }
// Check and configure TX clock rate
    rate = clk_get_rate(dwmac.tx_clk);
    if (dwmac.data.tx_clk_rate &&
    rate != dwmac.data.tx_clk_rate) {
    rate = dwmac.data.tx_clk_rate;
    ret = clk_set_rate(dwmac.tx_clk, rate);
    if (ret) {
    dev_err(&pdev.dev,
    "Failed to set tx_clk\n");
    goto err_tx_clk_disable;
    }
    }
// Check and configure PTP ref clock rate
    rate = clk_get_rate(plat_dat.clk_ptp_ref);
    if (dwmac.data.ptp_ref_clk_rate &&
    rate != dwmac.data.ptp_ref_clk_rate) {
    rate = dwmac.data.ptp_ref_clk_rate;
    ret = clk_set_rate(plat_dat.clk_ptp_ref, rate);
    if (ret) {
    dev_err(&pdev.dev,
    "Failed to set clk_ptp_ref\n");
    goto err_tx_clk_disable;
    }
    }
    }
    plat_dat.clk_tx_i = dwmac.tx_clk;
    plat_dat.set_clk_tx_rate = stmmac_set_clk_tx_rate;
    plat_dat.bsp_priv = dwmac;
    ret = stmmac_dvr_probe(&pdev.dev, plat_dat, &stmmac_res);
    if (ret)
    goto err_tx_clk_disable;
    return 0;
    err_tx_clk_disable:
    if (dwmac.data.tx_clk_en)
    clk_disable_unprepare(dwmac.tx_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn intel_eth_plat_remove(pdev: *mut platform_device) {
    static void intel_eth_plat_remove(struct platform_device *pdev)
    {
    struct intel_dwmac *dwmac = get_stmmac_bsp_priv(&pdev.dev);
    stmmac_pltfr_remove(pdev);
    if (dwmac.data.tx_clk_en)
    clk_disable_unprepare(dwmac.tx_clk);
    }
    static struct platform_driver intel_eth_plat_driver = {
    .probe  = intel_eth_plat_probe,
    .remove = intel_eth_plat_remove,
    .driver = {
    .name		= "intel-eth-plat",
    .pm		= &stmmac_pltfr_pm_ops,
    .of_match_table = intel_eth_plat_match,
    },
    };
    module_platform_driver(intel_eth_plat_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Intel DWMAC platform driver");
