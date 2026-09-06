//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-pci-arasan.c
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
// sdhci-pci-arasan.c - Driver for Arasan PCI Controller with
// integrated phy.
//
// Copyright (C) 2017 Arasan Chip Systems Inc.
//
// Author: Atul Garg <agarg@arasan.com>
//

// Extra registers for Arasan SD/SDIO/MMC Host Controller with PHY
pub const PHY_ADDR_REG: c_uint = 0x300;
pub const PHY_DAT_REG: c_uint = 0x304;

pub const DATA_MASK: c_uint = 0xFF;
// PHY Specific Registers
pub const DLL_STATUS: c_uint = 0x00;
pub const IPAD_CTRL1: c_uint = 0x01;
pub const IPAD_CTRL2: c_uint = 0x02;
pub const IPAD_STS: c_uint = 0x03;
pub const IOREN_CTRL1: c_uint = 0x06;
pub const IOREN_CTRL2: c_uint = 0x07;
pub const IOPU_CTRL1: c_uint = 0x08;
pub const IOPU_CTRL2: c_uint = 0x09;
pub const ITAP_DELAY: c_uint = 0x0C;
pub const OTAP_DELAY: c_uint = 0x0D;
pub const STRB_SEL: c_uint = 0x0E;
pub const CLKBUF_SEL: c_uint = 0x0F;
pub const MODE_CTRL: c_uint = 0x11;
pub const DLL_TRIM: c_uint = 0x12;
pub const CMD_CTRL: c_uint = 0x20;
pub const DATA_CTRL: c_uint = 0x21;
pub const STRB_CTRL: c_uint = 0x22;
pub const CLK_CTRL: c_uint = 0x23;
pub const PHY_CTRL: c_uint = 0x24;

pub const ODEN_DAT: c_uint = 0xFF;

pub const REN_DATA: c_uint = 0xFF;

pub const PU_DAT: c_uint = 0xFF;

pub const OD_REL_DAT: c_uint = 0xFF;
pub const DLLTRM_ICP: c_uint = 0x8;

pub const PDB_DATA: c_uint = 0xFF;

pub const CALDONE_MASK: c_uint = 0x10;
pub const DLL_RDY_MASK: c_uint = 0x10;
pub const MAX_CLK_BUF: c_uint = 0x7;
// Mode Controls

//
// Controller has no specific bits for HS200/HS.
// Used BIT(4), BIT(5) for software programming.
//

// Arasan private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arasan_host {
    pub chg_clk: u32,
}

#[no_mangle]
unsafe extern "C" fn arasan_phy_addr_poll(host: *mut sdhci_host, offset: u32, mask: u32) -> c_int {
    static int arasan_phy_addr_poll(struct sdhci_host *host, u32 offset, u32 mask)
    {
    let mut timeout: ktime_t = ktime_add_us(ktime_get(), 100);
    bool failed;
    let mut val: u8 = 0;
    while (1) {
    failed = ktime_after(ktime_get(), timeout);
    val = sdhci_readw(host, PHY_ADDR_REG);
    if (!(val & mask))
    return 0;
    if (failed)
    return -EBUSY;
    }
    }
#[no_mangle]
unsafe extern "C" fn arasan_phy_write(host: *mut sdhci_host, data: u8, offset: u8) -> c_int {
    static int arasan_phy_write(struct sdhci_host *host, u8 data, u8 offset)
    {
    sdhci_writew(host, data, PHY_DAT_REG);
    sdhci_writew(host, (PHY_WRITE | offset), PHY_ADDR_REG);
    return arasan_phy_addr_poll(host, PHY_ADDR_REG, PHY_BUSY);
    }
#[no_mangle]
unsafe extern "C" fn arasan_phy_read(host: *mut sdhci_host, offset: u8, data: *mut u8) -> c_int {
    static int arasan_phy_read(struct sdhci_host *host, u8 offset, u8 *data)
    {
    int ret;
    sdhci_writew(host, 0, PHY_DAT_REG);
    sdhci_writew(host, offset, PHY_ADDR_REG);
    ret = arasan_phy_addr_poll(host, PHY_ADDR_REG, PHY_BUSY);
// Masking valid data bits
// data = sdhci_readw(host, PHY_DAT_REG) & DATA_MASK;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn arasan_phy_sts_poll(host: *mut sdhci_host, offset: u32, mask: u32) -> c_int {
    static int arasan_phy_sts_poll(struct sdhci_host *host, u32 offset, u32 mask)
    {
    int ret;
    let mut timeout: ktime_t = ktime_add_us(ktime_get(), 100);
    bool failed;
    let mut val: u8 = 0;
    while (1) {
    failed = ktime_after(ktime_get(), timeout);
    ret = arasan_phy_read(host, offset, &val);
    if (ret)
    return -EBUSY;
#[no_mangle]
pub unsafe extern "C" fn if(mask: val &) -> else {
    else if (val & mask)
    return 0;
    if (failed)
    return -EBUSY;
    }
    }
// Initialize the Arasan PHY
#[no_mangle]
unsafe extern "C" fn arasan_phy_init(host: *mut sdhci_host) -> c_int {
    static int arasan_phy_init(struct sdhci_host *host)
    {
    int ret;
    u8 val;
// Program IOPADs and wait for calibration to be done
    if (arasan_phy_read(host, IPAD_CTRL1, &val) ||
    arasan_phy_write(host, val | RETB_ENBL | PDB_ENBL, IPAD_CTRL1) ||
    arasan_phy_read(host, IPAD_CTRL2, &val) ||
    arasan_phy_write(host, val | RTRIM_EN, IPAD_CTRL2))
    return -EBUSY;
    ret = arasan_phy_sts_poll(host, IPAD_STS, CALDONE_MASK);
    if (ret)
    return -EBUSY;
// Program CMD/Data lines
    if (arasan_phy_read(host, IOREN_CTRL1, &val) ||
    arasan_phy_write(host, val | REN_CMND | REN_STRB, IOREN_CTRL1) ||
    arasan_phy_read(host, IOPU_CTRL1, &val) ||
    arasan_phy_write(host, val | PU_CMD, IOPU_CTRL1) ||
    arasan_phy_read(host, CMD_CTRL, &val) ||
    arasan_phy_write(host, val | PDB_CMND, CMD_CTRL) ||
    arasan_phy_read(host, IOREN_CTRL2, &val) ||
    arasan_phy_write(host, val | REN_DATA, IOREN_CTRL2) ||
    arasan_phy_read(host, IOPU_CTRL2, &val) ||
    arasan_phy_write(host, val | PU_DAT, IOPU_CTRL2) ||
    arasan_phy_read(host, DATA_CTRL, &val) ||
    arasan_phy_write(host, val | PDB_DATA, DATA_CTRL) ||
    arasan_phy_read(host, STRB_CTRL, &val) ||
    arasan_phy_write(host, val | PDB_STRB, STRB_CTRL) ||
    arasan_phy_read(host, CLK_CTRL, &val) ||
    arasan_phy_write(host, val | PDB_CLOCK, CLK_CTRL) ||
    arasan_phy_read(host, CLKBUF_SEL, &val) ||
    arasan_phy_write(host, val | MAX_CLK_BUF, CLKBUF_SEL) ||
    arasan_phy_write(host, LEGACY_MODE, MODE_CTRL))
    return -EBUSY;
    return 0;
    }
// Set Arasan PHY for different modes
    static int arasan_phy_set(struct sdhci_host *host, u8 mode, u8 otap,
    u8 drv_type, u8 itap, u8 trim, u8 clk)
    {
    u8 val;
    int ret;
    if (mode == HISPD_MODE || mode == HS200_MODE)
    ret = arasan_phy_write(host, 0x0, MODE_CTRL);
    else
    ret = arasan_phy_write(host, mode, MODE_CTRL);
    if (ret)
    return ret;
    if (mode == HS400_MODE || mode == HS200_MODE) {
    ret = arasan_phy_read(host, IPAD_CTRL1, &val);
    if (ret)
    return ret;
    ret = arasan_phy_write(host, IOPAD(val, drv_type), IPAD_CTRL1);
    if (ret)
    return ret;
    }
    if (mode == LEGACY_MODE) {
    ret = arasan_phy_write(host, 0x0, OTAP_DELAY);
    if (ret)
    return ret;
    ret = arasan_phy_write(host, 0x0, ITAP_DELAY);
    } else {
    ret = arasan_phy_write(host, OTAPDLY(otap), OTAP_DELAY);
    if (ret)
    return ret;
    if (mode != HS200_MODE)
    ret = arasan_phy_write(host, ITAPDLY(itap), ITAP_DELAY);
    else
    ret = arasan_phy_write(host, 0x0, ITAP_DELAY);
    }
    if (ret)
    return ret;
    if (mode != LEGACY_MODE) {
    ret = arasan_phy_write(host, trim, DLL_TRIM);
    if (ret)
    return ret;
    }
    ret = arasan_phy_write(host, 0, DLL_STATUS);
    if (ret)
    return ret;
    if (mode != LEGACY_MODE) {
    ret = arasan_phy_write(host, FREQSEL(clk), DLL_STATUS);
    if (ret)
    return ret;
    ret = arasan_phy_sts_poll(host, DLL_STATUS, DLL_RDY_MASK);
    if (ret)
    return -EBUSY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arasan_select_phy_clock(host: *mut sdhci_host) -> c_int {
    static int arasan_select_phy_clock(struct sdhci_host *host)
    {
    struct sdhci_pci_slot *slot = sdhci_priv(host);
    struct arasan_host *arasan_host = sdhci_pci_priv(slot);
    u8 clk;
    if (arasan_host.chg_clk == host.mmc.ios.clock)
    return 0;
    arasan_host.chg_clk = host.mmc.ios.clock;
    if (host.mmc.ios.clock == 200000000)
    clk = 0x0;
#[no_mangle]
pub unsafe extern "C" fn if(100000000: host->mmc->ios.clock ==) -> else {
    else if (host.mmc.ios.clock == 100000000)
    clk = 0x2;
#[no_mangle]
pub unsafe extern "C" fn if(50000000: host->mmc->ios.clock ==) -> else {
    else if (host.mmc.ios.clock == 50000000)
    clk = 0x1;
    else
    clk = 0x0;
    if (host.mmc_host_ops.hs400_enhanced_strobe) {
    arasan_phy_set(host, ENHSTRB_MODE, 1, 0x0, 0x0,
    DLLTRM_ICP, clk);
    } else {
    switch (host.mmc.ios.timing) {
    case MMC_TIMING_LEGACY:
    arasan_phy_set(host, LEGACY_MODE, 0x0, 0x0, 0x0,
    0x0, 0x0);
    break;
    case MMC_TIMING_MMC_HS:
    case MMC_TIMING_SD_HS:
    arasan_phy_set(host, HISPD_MODE, 0x3, 0x0, 0x2,
    DLLTRM_ICP, clk);
    break;
    case MMC_TIMING_MMC_HS200:
    case MMC_TIMING_UHS_SDR104:
    arasan_phy_set(host, HS200_MODE, 0x2,
    host.mmc.ios.drv_type, 0x0,
    DLLTRM_ICP, clk);
    break;
    case MMC_TIMING_MMC_DDR52:
    case MMC_TIMING_UHS_DDR50:
    arasan_phy_set(host, DDR50_MODE, 0x1, 0x0,
    0x0, DLLTRM_ICP, clk);
    break;
    case MMC_TIMING_MMC_HS400:
    arasan_phy_set(host, HS400_MODE, 0x1,
    host.mmc.ios.drv_type, 0xa,
    DLLTRM_ICP, clk);
    break;
    default:
    break;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arasan_pci_probe_slot(slot: *mut sdhci_pci_slot) -> c_int {
    static int arasan_pci_probe_slot(struct sdhci_pci_slot *slot)
    {
    int err;
    slot.host.mmc.caps |= MMC_CAP_NONREMOVABLE | MMC_CAP_8_BIT_DATA;
    err = arasan_phy_init(slot.host);
    if (err)
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arasan_sdhci_set_clock(host: *mut sdhci_host, clock: c_uint) {
    static void arasan_sdhci_set_clock(struct sdhci_host *host, unsigned int clock)
    {
    sdhci_set_clock(host, clock);
// Change phy settings for the new clock
    arasan_select_phy_clock(host);
    }
    static const struct sdhci_ops arasan_sdhci_pci_ops = {
    .set_clock	= arasan_sdhci_set_clock,
    .enable_dma	= sdhci_pci_enable_dma,
    .set_bus_width	= sdhci_set_bus_width,
    .reset		= sdhci_reset,
    .set_uhs_signaling	= sdhci_set_uhs_signaling,
    };
    const struct sdhci_pci_fixes sdhci_arasan = {
    .probe_slot = arasan_pci_probe_slot,
    .ops        = &arasan_sdhci_pci_ops,
    .priv_size  = sizeof(struct arasan_host),
    };
