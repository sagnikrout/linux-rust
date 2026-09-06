//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-of-ma35d1.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2024 Nuvoton Technology Corp.
//
// Author: Shan-Chun Hung <shanchun1218@gmail.com>
//

pub const MA35_SYS_MISCFCR0: c_uint = 0x070;
pub const MA35_SDHCI_MSHCCTL: c_uint = 0x508;
pub const MA35_SDHCI_MBIUCTL: c_uint = 0x510;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ma35_priv {
    pub rst: *mut reset_control,
    pub pinctrl: *mut pinctrl,
    pub pins_uhs: *mut pinctrl_state,
    pub pins_default: *mut pinctrl_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ma35_restore_data {
    pub reg: u32,
    pub width: u32,
}

    static const struct ma35_restore_data restore_data[] = {
    { SDHCI_CLOCK_CONTROL,		sizeof(u32)},
    { SDHCI_BLOCK_SIZE,		sizeof(u32)},
    { SDHCI_INT_ENABLE,		sizeof(u32)},
    { SDHCI_SIGNAL_ENABLE,		sizeof(u32)},
    { SDHCI_AUTO_CMD_STATUS,	sizeof(u32)},
    { SDHCI_HOST_CONTROL,		sizeof(u32)},
    { SDHCI_TIMEOUT_CONTROL,	sizeof(u8) },
    { MA35_SDHCI_MSHCCTL,		sizeof(u16)},
    { MA35_SDHCI_MBIUCTL,		sizeof(u16)},
    };
//
// If DMA addr spans 128MB boundary, we split the DMA transfer into two
// so that each DMA transfer doesn't exceed the boundary.
//
    static void ma35_adma_write_desc(struct sdhci_host *host, void **desc, dma_addr_t addr, int len,
    unsigned int cmd)
    {
    int tmplen, offset;
    if (likely(!len || (ALIGN(addr, SZ_128M) == ALIGN(addr + len - 1, SZ_128M)))) {
    sdhci_adma_write_desc(host, desc, addr, len, cmd);
    return;
    }
    offset = addr & (SZ_128M - 1);
    tmplen = SZ_128M - offset;
    sdhci_adma_write_desc(host, desc, addr, tmplen, cmd);
    addr += tmplen;
    len -= tmplen;
    sdhci_adma_write_desc(host, desc, addr, len, cmd);
    }
#[no_mangle]
unsafe extern "C" fn ma35_set_clock(host: *mut sdhci_host, clock: c_uint) {
    static void ma35_set_clock(struct sdhci_host *host, unsigned int clock)
    {
    u32 ctl;
//
// If the clock frequency exceeds MMC_HIGH_52_MAX_DTR,
// disable command conflict check.
//
    ctl = sdhci_readw(host, MA35_SDHCI_MSHCCTL);
    if (clock > MMC_HIGH_52_MAX_DTR)
    ctl &= ~MA35_SDHCI_CMD_CONFLICT_CHK;
    else
    ctl |= MA35_SDHCI_CMD_CONFLICT_CHK;
    sdhci_writew(host, ctl, MA35_SDHCI_MSHCCTL);
    sdhci_set_clock(host, clock);
    }
#[no_mangle]
unsafe extern "C" fn ma35_start_signal_voltage_switch(mmc: *mut mmc_host, ios: *mut mmc_ios) -> c_int {
    static int ma35_start_signal_voltage_switch(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct ma35_priv *priv = sdhci_pltfm_priv(pltfm_host);
    switch (ios.signal_voltage) {
    case MMC_SIGNAL_VOLTAGE_180:
    if (!IS_ERR(priv.pinctrl) && !IS_ERR(priv.pins_uhs))
    pinctrl_select_state(priv.pinctrl, priv.pins_uhs);
    break;
    case MMC_SIGNAL_VOLTAGE_330:
    if (!IS_ERR(priv.pinctrl) && !IS_ERR(priv.pins_default))
    pinctrl_select_state(priv.pinctrl, priv.pins_default);
    break;
    default:
    dev_err(mmc_dev(host.mmc), "Unsupported signal voltage!\n");
    return -EINVAL;
    }
    return sdhci_start_signal_voltage_switch(mmc, ios);
    }
#[no_mangle]
unsafe extern "C" fn ma35_voltage_switch(host: *mut sdhci_host) {
    static void ma35_voltage_switch(struct sdhci_host *host)
    {
// Wait for 5ms after set 1.8V signal enable bit
    fsleep(5000);
    }
#[no_mangle]
unsafe extern "C" fn ma35_execute_tuning(mmc: *mut mmc_host, opcode: u32) -> c_int {
    static int ma35_execute_tuning(struct mmc_host *mmc, u32 opcode)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct ma35_priv *priv = sdhci_pltfm_priv(pltfm_host);
    int idx;
    u32 regs[ARRAY_SIZE(restore_data)] = {};
//
// Limitations require a reset of SD/eMMC before tuning and
// saving the registers before resetting, then restoring
// after the reset.
//
    for (idx = 0; idx < ARRAY_SIZE(restore_data); idx++) {
    if (restore_data[idx].width == sizeof(u32))
    regs[idx] = sdhci_readl(host, restore_data[idx].reg);
#[no_mangle]
pub unsafe extern "C" fn if(sizeof(u16): restore_data[idx].width ==) -> else {
    else if (restore_data[idx].width == sizeof(u16))
    regs[idx] = sdhci_readw(host, restore_data[idx].reg);
#[no_mangle]
pub unsafe extern "C" fn if(sizeof(u8): restore_data[idx].width ==) -> else {
    else if (restore_data[idx].width == sizeof(u8))
    regs[idx] = sdhci_readb(host, restore_data[idx].reg);
    }
    reset_control_assert(priv.rst);
    reset_control_deassert(priv.rst);
    for (idx = 0; idx < ARRAY_SIZE(restore_data); idx++) {
    if (restore_data[idx].width == sizeof(u32))
    sdhci_writel(host, regs[idx], restore_data[idx].reg);
#[no_mangle]
pub unsafe extern "C" fn if(sizeof(u16): restore_data[idx].width ==) -> else {
    else if (restore_data[idx].width == sizeof(u16))
    sdhci_writew(host, regs[idx], restore_data[idx].reg);
#[no_mangle]
pub unsafe extern "C" fn if(sizeof(u8): restore_data[idx].width ==) -> else {
    else if (restore_data[idx].width == sizeof(u8))
    sdhci_writeb(host, regs[idx], restore_data[idx].reg);
    }
    return sdhci_execute_tuning(mmc, opcode);
    }
    static const struct sdhci_ops sdhci_ma35_ops = {
    .set_clock		= ma35_set_clock,
    .set_bus_width		= sdhci_set_bus_width,
    .set_uhs_signaling	= sdhci_set_uhs_signaling,
    .get_max_clock		= sdhci_pltfm_clk_get_max_clock,
    .reset			= sdhci_reset,
    .adma_write_desc	= ma35_adma_write_desc,
    .voltage_switch		= ma35_voltage_switch,
    };
    static const struct sdhci_pltfm_data sdhci_ma35_pdata = {
    .ops = &sdhci_ma35_ops,
    .quirks = SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN,
    .quirks2 = SDHCI_QUIRK2_PRESET_VALUE_BROKEN | SDHCI_QUIRK2_BROKEN_DDR50 |
    SDHCI_QUIRK2_ACMD23_BROKEN,
    };
#[no_mangle]
unsafe extern "C" fn ma35_probe(pdev: *mut platform_device) -> c_int {
    static int ma35_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct sdhci_pltfm_host *pltfm_host;
    struct sdhci_host *host;
    struct ma35_priv *priv;
    int err;
    u32 extra, ctl;
    host = sdhci_pltfm_init(pdev, &sdhci_ma35_pdata, sizeof(struct ma35_priv));
    if (IS_ERR(host))
    return PTR_ERR(host);
// Extra adma table cnt for cross 128M boundary handling.
    extra = DIV_ROUND_UP_ULL(dma_get_required_mask(dev), SZ_128M);
    extra = min(extra, SDHCI_MAX_SEGS);
    host.adma_table_cnt += extra;
    pltfm_host = sdhci_priv(host);
    priv = sdhci_pltfm_priv(pltfm_host);
    pltfm_host.clk = devm_clk_get_optional_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(pltfm_host.clk))
    return dev_err_probe(dev, PTR_ERR(pltfm_host.clk),
    "failed to get clk\n");
    err = mmc_of_parse(host.mmc);
    if (err)
    return err;
    priv.rst = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(priv.rst))
    return dev_err_probe(dev, PTR_ERR(priv.rst),
    "failed to get reset control\n");
    sdhci_get_of_property(pdev);
    priv.pinctrl = devm_pinctrl_get(dev);
    if (!IS_ERR(priv.pinctrl)) {
    priv.pins_default = pinctrl_lookup_state(priv.pinctrl, "default");
    priv.pins_uhs = pinctrl_lookup_state(priv.pinctrl, "state_uhs");
    pinctrl_select_state(priv.pinctrl, priv.pins_default);
    }
    if (!(host.quirks2 & SDHCI_QUIRK2_NO_1_8_V)) {
    struct regmap	*regmap;
    u32		reg;
    regmap = syscon_regmap_lookup_by_phandle(dev_of_node(dev), "nuvoton,sys");
    if (!IS_ERR(regmap)) {
// Enable SDHCI voltage stable for 1.8V
    regmap_read(regmap, MA35_SYS_MISCFCR0, &reg);
    reg |= BIT(17);
    regmap_write(regmap, MA35_SYS_MISCFCR0, reg);
    }
    host.mmc_host_ops.start_signal_voltage_switch =
    ma35_start_signal_voltage_switch;
    }
    host.mmc_host_ops.execute_tuning = ma35_execute_tuning;
    err = sdhci_add_host(host);
    if (err)
    return err;
//
// Split data into chunks of 16 or 8 bytes for transmission.
// Each chunk transfer is guaranteed to be uninterrupted on the bus.
// This likely corresponds to the AHB bus DMA burst size.
//
    ctl = sdhci_readw(host, MA35_SDHCI_MBIUCTL);
    ctl &= ~MA35_SDHCI_INCR_MSK;
    ctl |= MA35_SDHCI_INCR16 | MA35_SDHCI_INCR8;
    sdhci_writew(host, ctl, MA35_SDHCI_MBIUCTL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35_disable_card_clk(host: *mut sdhci_host) {
    static void ma35_disable_card_clk(struct sdhci_host *host)
    {
    u16 ctrl;
    ctrl = sdhci_readw(host, SDHCI_CLOCK_CONTROL);
    if (ctrl & SDHCI_CLOCK_CARD_EN) {
    ctrl &= ~SDHCI_CLOCK_CARD_EN;
    sdhci_writew(host, ctrl, SDHCI_CLOCK_CONTROL);
    }
    }
#[no_mangle]
unsafe extern "C" fn ma35_remove(pdev: *mut platform_device) {
    static void ma35_remove(struct platform_device *pdev)
    {
    struct sdhci_host *host = platform_get_drvdata(pdev);
    sdhci_remove_host(host, 0);
    ma35_disable_card_clk(host);
    }
    static const struct of_device_id sdhci_ma35_dt_ids[] = {
    { .compatible = "nuvoton,ma35d1-sdhci" },
    {}
    };
    MODULE_DEVICE_TABLE(of, sdhci_ma35_dt_ids);
    static struct platform_driver sdhci_ma35_driver = {
    .driver	= {
    .name	= "sdhci-ma35",
    .of_match_table = sdhci_ma35_dt_ids,
    },
    .probe	= ma35_probe,
    .remove = ma35_remove,
    };
    module_platform_driver(sdhci_ma35_driver);
    MODULE_DESCRIPTION("SDHCI platform driver for Nuvoton MA35");
    MODULE_AUTHOR("Shan-Chun Hung <shanchun1218@gmail.com>");
    MODULE_LICENSE("GPL");
