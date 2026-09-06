//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-pic32.c
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
// Support of SDHCI platform devices for Microchip PIC32.
//
// Copyright (C) 2015 Microchip
// Andrei Pistirica, Paul Thacker
//
// Inspired by sdhci-pltfm.c
//

pub const SDH_SHARED_BUS_CTRL: c_uint = 0x000000E0;
pub const SDH_SHARED_BUS_NR_CLK_PINS_MASK: c_uint = 0x7;
pub const SDH_SHARED_BUS_NR_IRQ_PINS_MASK: c_uint = 0x30;
pub const SDH_SHARED_BUS_CLK_PINS: c_uint = 0x10;
pub const SDH_SHARED_BUS_IRQ_PINS: c_uint = 0x14;
pub const SDH_CAPS_SDH_SLOT_TYPE_MASK: c_uint = 0xC0000000;
pub const SDH_SLOT_TYPE_REMOVABLE: c_uint = 0x0;
pub const SDH_SLOT_TYPE_EMBEDDED: c_uint = 0x1;
pub const SDH_SLOT_TYPE_SHARED_BUS: c_uint = 0x2;
pub const SDHCI_CTRL_CDSSEL: c_uint = 0x80;
pub const SDHCI_CTRL_CDTLVL: c_uint = 0x40;
pub const ADMA_FIFO_RD_THSHLD: c_int = 512;
pub const ADMA_FIFO_WR_THSHLD: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_sdhci_priv {
    pub pdev: *mut platform_device,
    pub sys_clk: *mut clk,
    pub base_clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn pic32_sdhci_get_max_clock(host: *mut sdhci_host) -> c_uint {
    static unsigned int pic32_sdhci_get_max_clock(struct sdhci_host *host)
    {
    struct pic32_sdhci_priv *sdhci_pdata = sdhci_priv(host);
    return clk_get_rate(sdhci_pdata.base_clk);
    }
#[no_mangle]
unsafe extern "C" fn pic32_sdhci_set_bus_width(host: *mut sdhci_host, width: c_int) {
    static void pic32_sdhci_set_bus_width(struct sdhci_host *host, int width)
    {
    u8 ctrl;
    ctrl = sdhci_readb(host, SDHCI_HOST_CONTROL);
    if (width == MMC_BUS_WIDTH_8) {
    ctrl &= ~SDHCI_CTRL_4BITBUS;
    if (host.version >= SDHCI_SPEC_300)
    ctrl |= SDHCI_CTRL_8BITBUS;
    } else {
    if (host.version >= SDHCI_SPEC_300)
    ctrl &= ~SDHCI_CTRL_8BITBUS;
    if (width == MMC_BUS_WIDTH_4)
    ctrl |= SDHCI_CTRL_4BITBUS;
    else
    ctrl &= ~SDHCI_CTRL_4BITBUS;
    }
// CD select and test bits must be set for errata workaround.
    ctrl &= ~SDHCI_CTRL_CDTLVL;
    ctrl |= SDHCI_CTRL_CDSSEL;
    sdhci_writeb(host, ctrl, SDHCI_HOST_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn pic32_sdhci_get_ro(host: *mut sdhci_host) -> c_uint {
    static unsigned int pic32_sdhci_get_ro(struct sdhci_host *host)
    {
//
// The SDHCI_WRITE_PROTECT bit is unstable on current hardware so we
// can't depend on its value in any way.
//
    return 0;
    }
    static const struct sdhci_ops pic32_sdhci_ops = {
    .get_max_clock = pic32_sdhci_get_max_clock,
    .set_clock = sdhci_set_clock,
    .set_bus_width = pic32_sdhci_set_bus_width,
    .reset = sdhci_reset,
    .set_uhs_signaling = sdhci_set_uhs_signaling,
    .get_ro = pic32_sdhci_get_ro,
    };
    static const struct sdhci_pltfm_data sdhci_pic32_pdata = {
    .ops = &pic32_sdhci_ops,
    .quirks = SDHCI_QUIRK_NO_HISPD_BIT,
    .quirks2 = SDHCI_QUIRK2_NO_1_8_V,
    };
#[no_mangle]
unsafe extern "C" fn pic32_sdhci_shared_bus(pdev: *mut platform_device) {
    static void pic32_sdhci_shared_bus(struct platform_device *pdev)
    {
    struct sdhci_host *host = platform_get_drvdata(pdev);
    let mut bus: u32 = readl(host.ioaddr + SDH_SHARED_BUS_CTRL);
    let mut clk_pins: u32 = (bus & SDH_SHARED_BUS_NR_CLK_PINS_MASK) >> 0;
    let mut irq_pins: u32 = (bus & SDH_SHARED_BUS_NR_IRQ_PINS_MASK) >> 4;
// select first clock
    if (clk_pins & 1)
    bus |= (1 << SDH_SHARED_BUS_CLK_PINS);
// select first interrupt
    if (irq_pins & 1)
    bus |= (1 << SDH_SHARED_BUS_IRQ_PINS);
    writel(bus, host.ioaddr + SDH_SHARED_BUS_CTRL);
    }
    static void pic32_sdhci_probe_platform(struct platform_device *pdev,
    struct pic32_sdhci_priv *pdata)
    {
    u32 caps_slot_type;
    struct sdhci_host *host = platform_get_drvdata(pdev);
// Check card slot connected on shared bus.
    host.caps = readl(host.ioaddr + SDHCI_CAPABILITIES);
    caps_slot_type = (host.caps & SDH_CAPS_SDH_SLOT_TYPE_MASK) >> 30;
    if (caps_slot_type == SDH_SLOT_TYPE_SHARED_BUS)
    pic32_sdhci_shared_bus(pdev);
    }
#[no_mangle]
unsafe extern "C" fn pic32_sdhci_probe(pdev: *mut platform_device) -> c_int {
    static int pic32_sdhci_probe(struct platform_device *pdev)
    {
    struct sdhci_host *host;
    struct sdhci_pltfm_host *pltfm_host;
    struct pic32_sdhci_priv *sdhci_pdata;
    struct pic32_sdhci_platform_data *plat_data;
    int ret;
    host = sdhci_pltfm_init(pdev, &sdhci_pic32_pdata,
    sizeof(struct pic32_sdhci_priv));
    if (IS_ERR(host)) {
    ret = PTR_ERR(host);
    goto err;
    }
    pltfm_host = sdhci_priv(host);
    sdhci_pdata = sdhci_pltfm_priv(pltfm_host);
    plat_data = pdev.dev.platform_data;
    if (plat_data && plat_data.setup_dma) {
    ret = plat_data.setup_dma(ADMA_FIFO_RD_THSHLD,
    ADMA_FIFO_WR_THSHLD);
    if (ret)
    goto err;
    }
    sdhci_pdata.sys_clk = devm_clk_get(&pdev.dev, "sys_clk");
    if (IS_ERR(sdhci_pdata.sys_clk)) {
    ret = PTR_ERR(sdhci_pdata.sys_clk);
    dev_err(&pdev.dev, "Error getting clock\n");
    goto err;
    }
    ret = clk_prepare_enable(sdhci_pdata.sys_clk);
    if (ret) {
    dev_err(&pdev.dev, "Error enabling clock\n");
    goto err;
    }
    sdhci_pdata.base_clk = devm_clk_get(&pdev.dev, "base_clk");
    if (IS_ERR(sdhci_pdata.base_clk)) {
    ret = PTR_ERR(sdhci_pdata.base_clk);
    dev_err(&pdev.dev, "Error getting clock\n");
    goto err_sys_clk;
    }
    ret = clk_prepare_enable(sdhci_pdata.base_clk);
    if (ret) {
    dev_err(&pdev.dev, "Error enabling clock\n");
    goto err_base_clk;
    }
    ret = mmc_of_parse(host.mmc);
    if (ret)
    goto err_base_clk;
    pic32_sdhci_probe_platform(pdev, sdhci_pdata);
    ret = sdhci_add_host(host);
    if (ret)
    goto err_base_clk;
    dev_info(&pdev.dev, "Successfully added sdhci host\n");
    return 0;
    err_base_clk:
    clk_disable_unprepare(sdhci_pdata.base_clk);
    err_sys_clk:
    clk_disable_unprepare(sdhci_pdata.sys_clk);
    err:
    dev_err(&pdev.dev, "pic32-sdhci probe failed: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pic32_sdhci_remove(pdev: *mut platform_device) {
    static void pic32_sdhci_remove(struct platform_device *pdev)
    {
    struct sdhci_host *host = platform_get_drvdata(pdev);
    struct pic32_sdhci_priv *sdhci_pdata = sdhci_priv(host);
    u32 scratch;
    scratch = readl(host.ioaddr + SDHCI_INT_STATUS);
    sdhci_remove_host(host, scratch == (u32)~0);
    clk_disable_unprepare(sdhci_pdata.base_clk);
    clk_disable_unprepare(sdhci_pdata.sys_clk);
    }
    static const struct of_device_id pic32_sdhci_id_table[] = {
    { .compatible = "microchip,pic32mzda-sdhci" },
    {}
    };
    MODULE_DEVICE_TABLE(of, pic32_sdhci_id_table);
    static struct platform_driver pic32_sdhci_driver = {
    .driver = {
    .name	= "pic32-sdhci",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(pic32_sdhci_id_table),
    },
    .probe		= pic32_sdhci_probe,
    .remove		= pic32_sdhci_remove,
    };
    module_platform_driver(pic32_sdhci_driver);
    MODULE_DESCRIPTION("Microchip PIC32 SDHCI driver");
    MODULE_AUTHOR("Pistirica Sorin Andrei & Sandeep Sheriker");
    MODULE_LICENSE("GPL v2");
