//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-pci-dwc-mshc.c
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
// SDHCI driver for Synopsys DWC_MSHC controller
//
// Copyright (C) 2018 Synopsys, Inc. (www.synopsys.com)
//
// Authors:
// Prabu Thangamuthu <prabu.t@synopsys.com>
// Manjunath M B <manjumb@synopsys.com>
//

pub const SDHCI_VENDOR_PTR_R: c_uint = 0xE8;
// Synopsys vendor specific registers
pub const SDHC_GPIO_OUT: c_uint = 0x34;
pub const SDHC_AT_CTRL_R: c_uint = 0x40;
pub const SDHC_SW_TUNE_EN: c_uint = 0x00000010;
// MMCM DRP
pub const SDHC_MMCM_DIV_REG: c_uint = 0x1020;
pub const DIV_REG_100_MHZ: c_uint = 0x1145;
pub const DIV_REG_200_MHZ: c_uint = 0x1083;
pub const SDHC_MMCM_CLKFBOUT: c_uint = 0x1024;
pub const CLKFBOUT_100_MHZ: c_uint = 0x0000;
pub const CLKFBOUT_200_MHZ: c_uint = 0x0080;
pub const SDHC_CCLK_MMCM_RST: c_uint = 0x00000001;
#[no_mangle]
unsafe extern "C" fn sdhci_snps_set_clock(host: *mut sdhci_host, clock: c_uint) {
    static void sdhci_snps_set_clock(struct sdhci_host *host, unsigned int clock)
    {
    u16 clk;
    u32 reg, vendor_ptr;
    vendor_ptr = sdhci_readw(host, SDHCI_VENDOR_PTR_R);
// Disable software managed rx tuning
    reg = sdhci_readl(host, (SDHC_AT_CTRL_R + vendor_ptr));
    reg &= ~SDHC_SW_TUNE_EN;
    sdhci_writel(host, reg, (SDHC_AT_CTRL_R + vendor_ptr));
    if (clock <= 52000000) {
    sdhci_set_clock(host, clock);
    } else {
// Assert reset to MMCM
    reg = sdhci_readl(host, (SDHC_GPIO_OUT + vendor_ptr));
    reg |= SDHC_CCLK_MMCM_RST;
    sdhci_writel(host, reg, (SDHC_GPIO_OUT + vendor_ptr));
// Configure MMCM
    if (clock == 100000000) {
    sdhci_writel(host, DIV_REG_100_MHZ, SDHC_MMCM_DIV_REG);
    sdhci_writel(host, CLKFBOUT_100_MHZ,
    SDHC_MMCM_CLKFBOUT);
    } else {
    sdhci_writel(host, DIV_REG_200_MHZ, SDHC_MMCM_DIV_REG);
    sdhci_writel(host, CLKFBOUT_200_MHZ,
    SDHC_MMCM_CLKFBOUT);
    }
// De-assert reset to MMCM
    reg = sdhci_readl(host, (SDHC_GPIO_OUT + vendor_ptr));
    reg &= ~SDHC_CCLK_MMCM_RST;
    sdhci_writel(host, reg, (SDHC_GPIO_OUT + vendor_ptr));
// Enable clock
    clk = SDHCI_PROG_CLOCK_MODE | SDHCI_CLOCK_INT_EN |
    SDHCI_CLOCK_CARD_EN;
    sdhci_writew(host, clk, SDHCI_CLOCK_CONTROL);
    }
    }
    static const struct sdhci_ops sdhci_snps_ops = {
    .set_clock	= sdhci_snps_set_clock,
    .enable_dma	= sdhci_pci_enable_dma,
    .set_bus_width	= sdhci_set_bus_width,
    .reset		= sdhci_reset,
    .set_uhs_signaling = sdhci_set_uhs_signaling,
    };
    const struct sdhci_pci_fixes sdhci_snps = {
    .ops		= &sdhci_snps_ops,
    };
