//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-of-hlwd.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// drivers/mmc/host/sdhci-of-hlwd.c
//
// Nintendo Wii Secure Digital Host Controller Interface.
// Copyright (C) 2009 The GameCube Linux Team
// Copyright (C) 2009 Albert Herranz
//
// Based on sdhci-of-esdhc.c
//
// Copyright (c) 2007 Freescale Semiconductor, Inc.
// Copyright (c) 2009 MontaVista Software, Inc.
//
// Authors: Xiaobo Xie <X.Xie@freescale.com>
// Anton Vorontsov <avorontsov@ru.mvista.com>
//

//
// Ops and quirks for the Nintendo Wii SDHCI controllers.
//
// We need a small delay after each write, or things go horribly wrong.
//

#[no_mangle]
unsafe extern "C" fn sdhci_hlwd_writel(host: *mut sdhci_host, val: u32, reg: c_int) {
    static void sdhci_hlwd_writel(struct sdhci_host *host, u32 val, int reg)
    {
    sdhci_be32bs_writel(host, val, reg);
    udelay(SDHCI_HLWD_WRITE_DELAY);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_hlwd_writew(host: *mut sdhci_host, val: u16, reg: c_int) {
    static void sdhci_hlwd_writew(struct sdhci_host *host, u16 val, int reg)
    {
    sdhci_be32bs_writew(host, val, reg);
    udelay(SDHCI_HLWD_WRITE_DELAY);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_hlwd_writeb(host: *mut sdhci_host, val: u8, reg: c_int) {
    static void sdhci_hlwd_writeb(struct sdhci_host *host, u8 val, int reg)
    {
    sdhci_be32bs_writeb(host, val, reg);
    udelay(SDHCI_HLWD_WRITE_DELAY);
    }
    static const struct sdhci_ops sdhci_hlwd_ops = {
    .read_l = sdhci_be32bs_readl,
    .read_w = sdhci_be32bs_readw,
    .read_b = sdhci_be32bs_readb,
    .write_l = sdhci_hlwd_writel,
    .write_w = sdhci_hlwd_writew,
    .write_b = sdhci_hlwd_writeb,
    .set_clock = sdhci_set_clock,
    .set_bus_width = sdhci_set_bus_width,
    .reset = sdhci_reset,
    .set_uhs_signaling = sdhci_set_uhs_signaling,
    };
    static const struct sdhci_pltfm_data sdhci_hlwd_pdata = {
    .quirks = SDHCI_QUIRK_32BIT_DMA_ADDR |
    SDHCI_QUIRK_32BIT_DMA_SIZE,
    .ops = &sdhci_hlwd_ops,
    };
#[no_mangle]
unsafe extern "C" fn sdhci_hlwd_probe(pdev: *mut platform_device) -> c_int {
    static int sdhci_hlwd_probe(struct platform_device *pdev)
    {
    return sdhci_pltfm_init_and_add_host(pdev, &sdhci_hlwd_pdata, 0);
    }
    static const struct of_device_id sdhci_hlwd_of_match[] = {
    { .compatible = "nintendo,hollywood-sdhci" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sdhci_hlwd_of_match);
    static struct platform_driver sdhci_hlwd_driver = {
    .driver = {
    .name = "sdhci-hlwd",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = sdhci_hlwd_of_match,
    .pm = &sdhci_pltfm_pmops,
    },
    .probe = sdhci_hlwd_probe,
    .remove = sdhci_pltfm_remove,
    };
    module_platform_driver(sdhci_hlwd_driver);
    MODULE_DESCRIPTION("Nintendo Wii SDHCI OF driver");
    MODULE_AUTHOR("The GameCube Linux Team, Albert Herranz");
    MODULE_LICENSE("GPL v2");
