//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-dove.c
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
// sdhci-dove.c Support for SDHCI on Marvell's Dove SoC
//
// Author: Saeed Bishara <saeed@marvell.com>
// Mike Rapoport <mike@compulab.co.il>
// Based on sdhci-cns3xxx.c
//

#[no_mangle]
unsafe extern "C" fn sdhci_dove_readw(host: *mut sdhci_host, reg: c_int) -> u16 {
    static u16 sdhci_dove_readw(struct sdhci_host *host, int reg)
    {
    u16 ret;
    switch (reg) {
    case SDHCI_HOST_VERSION:
    case SDHCI_SLOT_INT_STATUS:
// those registers don't exist
    return 0;
    default:
    ret = readw(host.ioaddr + reg);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_dove_readl(host: *mut sdhci_host, reg: c_int) -> u32 {
    static u32 sdhci_dove_readl(struct sdhci_host *host, int reg)
    {
    u32 ret;
    ret = readl(host.ioaddr + reg);
    switch (reg) {
    case SDHCI_CAPABILITIES:
// Mask the support for 3.0V
    ret &= ~SDHCI_CAN_VDD_300;
    break;
    }
    return ret;
    }
    static const struct sdhci_ops sdhci_dove_ops = {
    .read_w	= sdhci_dove_readw,
    .read_l	= sdhci_dove_readl,
    .set_clock = sdhci_set_clock,
    .set_bus_width = sdhci_set_bus_width,
    .reset = sdhci_reset,
    .set_uhs_signaling = sdhci_set_uhs_signaling,
    };
    static const struct sdhci_pltfm_data sdhci_dove_pdata = {
    .ops	= &sdhci_dove_ops,
    .quirks	= SDHCI_QUIRK_NO_SIMULT_VDD_AND_POWER |
    SDHCI_QUIRK_NO_BUSY_IRQ |
    SDHCI_QUIRK_BROKEN_TIMEOUT_VAL |
    SDHCI_QUIRK_FORCE_DMA |
    SDHCI_QUIRK_NO_HISPD_BIT,
    };
#[no_mangle]
unsafe extern "C" fn sdhci_dove_probe(pdev: *mut platform_device) -> c_int {
    static int sdhci_dove_probe(struct platform_device *pdev)
    {
    struct sdhci_host *host;
    struct sdhci_pltfm_host *pltfm_host;
    int ret;
    host = sdhci_pltfm_init(pdev, &sdhci_dove_pdata, 0);
    if (IS_ERR(host))
    return PTR_ERR(host);
    pltfm_host = sdhci_priv(host);
    pltfm_host.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    ret = mmc_of_parse(host.mmc);
    if (ret)
    return ret;
    return sdhci_add_host(host);
    }
    static const struct of_device_id sdhci_dove_of_match_table[] = {
    { .compatible = "marvell,dove-sdhci", },
    {}
    };
    MODULE_DEVICE_TABLE(of, sdhci_dove_of_match_table);
    static struct platform_driver sdhci_dove_driver = {
    .driver		= {
    .name	= "sdhci-dove",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .pm	= &sdhci_pltfm_pmops,
    .of_match_table = sdhci_dove_of_match_table,
    },
    .probe		= sdhci_dove_probe,
    .remove		= sdhci_pltfm_remove,
    };
    module_platform_driver(sdhci_dove_driver);
    MODULE_DESCRIPTION("SDHCI driver for Dove");
    MODULE_AUTHOR("Saeed Bishara <saeed@marvell.com>, "
    "Mike Rapoport <mike@compulab.co.il>");
    MODULE_LICENSE("GPL v2");
