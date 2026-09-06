//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-of-at91.c
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
// Atmel SDMMC controller driver.
//
// Copyright (C) 2015 Atmel,
// 2015 Ludovic Desroches <ludovic.desroches@atmel.com>
//

pub const SDMMC_MC1R: c_uint = 0x204;

pub const SDMMC_CACR: c_uint = 0x230;

pub const SDMMC_CALCR: c_uint = 0x240;

pub const SDHCI_AT91_PRESET_COMMON_CONF: c_uint = 0x400 /* drv type B, programmable clock mode */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_at91_soc_data {
    pub pdata: *const sdhci_pltfm_data,
    pub baseclk_is_generated_internally: bool,
    pub divider_for_baseclk: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_at91_priv {
    pub soc_data: *const sdhci_at91_soc_data,
    pub hclock: *mut clk,
    pub gck: *mut clk,
    pub mainck: *mut clk,
    pub restore_needed: bool,
    pub cal_always_on: bool,
}

#[no_mangle]
unsafe extern "C" fn sdhci_at91_set_force_card_detect(host: *mut sdhci_host) {
    static void sdhci_at91_set_force_card_detect(struct sdhci_host *host)
    {
    u8 mc1r;
    mc1r = readb(host.ioaddr + SDMMC_MC1R);
    mc1r |= SDMMC_MC1R_FCD;
    writeb(mc1r, host.ioaddr + SDMMC_MC1R);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_at91_set_clock(host: *mut sdhci_host, clock: c_uint) {
    static void sdhci_at91_set_clock(struct sdhci_host *host, unsigned int clock)
    {
    u16 clk;
    host.mmc.actual_clock = 0;
//
// There is no requirement to disable the internal clock before
// changing the SD clock configuration. Moreover, disabling the
// internal clock, changing the configuration and re-enabling the
// internal clock causes some bugs. It can prevent to get the internal
// clock stable flag ready and an unexpected switch to the base clock
// when using presets.
//
    clk = sdhci_readw(host, SDHCI_CLOCK_CONTROL);
    clk &= SDHCI_CLOCK_INT_EN;
    sdhci_writew(host, clk, SDHCI_CLOCK_CONTROL);
    if (clock == 0)
    return;
    clk = sdhci_calc_clk(host, clock, &host.mmc.actual_clock);
    clk |= SDHCI_CLOCK_INT_EN;
    sdhci_writew(host, clk, SDHCI_CLOCK_CONTROL);
// Wait max 20 ms
    if (read_poll_timeout(sdhci_readw, clk, (clk & SDHCI_CLOCK_INT_STABLE),
    1000, 20000, false, host, SDHCI_CLOCK_CONTROL)) {
    pr_err("%s: Internal clock never stabilised.\n",
    mmc_hostname(host.mmc));
    return;
    }
    clk |= SDHCI_CLOCK_CARD_EN;
    sdhci_writew(host, clk, SDHCI_CLOCK_CONTROL);
    }
    static void sdhci_at91_set_uhs_signaling(struct sdhci_host *host,
    unsigned int timing)
    {
    u8 mc1r;
    if (timing == MMC_TIMING_MMC_DDR52) {
    mc1r = sdhci_readb(host, SDMMC_MC1R);
    mc1r |= SDMMC_MC1R_DDR;
    sdhci_writeb(host, mc1r, SDMMC_MC1R);
    }
    sdhci_set_uhs_signaling(host, timing);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_at91_reset(host: *mut sdhci_host, mask: u8) {
    static void sdhci_at91_reset(struct sdhci_host *host, u8 mask)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_at91_priv *priv = sdhci_pltfm_priv(pltfm_host);
    unsigned int tmp;
    sdhci_reset(host, mask);
    if ((host.mmc.caps & MMC_CAP_NONREMOVABLE)
    || mmc_gpio_get_cd(host.mmc) >= 0)
    sdhci_at91_set_force_card_detect(host);
    if (priv.cal_always_on && (mask & SDHCI_RESET_ALL)) {
    let mut calcr: u32 = sdhci_readl(host, SDMMC_CALCR);
    sdhci_writel(host, calcr | SDMMC_CALCR_ALWYSON | SDMMC_CALCR_EN,
    SDMMC_CALCR);
    if (read_poll_timeout(sdhci_readl, tmp, !(tmp & SDMMC_CALCR_EN),
    10, 20000, false, host, SDMMC_CALCR))
    dev_err(mmc_dev(host.mmc), "Failed to calibrate\n");
    }
    }
    static const struct sdhci_ops sdhci_at91_sama5d2_ops = {
    .set_clock		= sdhci_at91_set_clock,
    .set_bus_width		= sdhci_set_bus_width,
    .reset			= sdhci_at91_reset,
    .set_uhs_signaling	= sdhci_at91_set_uhs_signaling,
    .set_power		= sdhci_set_power_and_bus_voltage,
    };
    static const struct sdhci_pltfm_data sdhci_sama5d2_pdata = {
    .ops = &sdhci_at91_sama5d2_ops,
    };
    static const struct sdhci_at91_soc_data soc_data_sama5d2 = {
    .pdata = &sdhci_sama5d2_pdata,
    .baseclk_is_generated_internally = false,
    };
    static const struct sdhci_at91_soc_data soc_data_sam9x60 = {
    .pdata = &sdhci_sama5d2_pdata,
    .baseclk_is_generated_internally = true,
    .divider_for_baseclk = 2,
    };
    static const struct of_device_id sdhci_at91_dt_match[] = {
    { .compatible = "atmel,sama5d2-sdhci", .data = &soc_data_sama5d2 },
    { .compatible = "microchip,sam9x60-sdhci", .data = &soc_data_sam9x60 },
    {}
    };
    MODULE_DEVICE_TABLE(of, sdhci_at91_dt_match);
#[no_mangle]
unsafe extern "C" fn sdhci_at91_set_clks_presets(dev: *mut device) -> c_int {
    static int sdhci_at91_set_clks_presets(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_at91_priv *priv = sdhci_pltfm_priv(pltfm_host);
    unsigned int			caps0, caps1;
    unsigned int			clk_base, clk_mul;
    unsigned int			gck_rate, clk_base_rate;
    unsigned int			preset_div;
    clk_prepare_enable(priv.hclock);
    caps0 = readl(host.ioaddr + SDHCI_CAPABILITIES);
    caps1 = readl(host.ioaddr + SDHCI_CAPABILITIES_1);
    gck_rate = clk_get_rate(priv.gck);
    if (priv.soc_data.baseclk_is_generated_internally)
    clk_base_rate = gck_rate / priv.soc_data.divider_for_baseclk;
    else
    clk_base_rate = clk_get_rate(priv.mainck);
    clk_base = clk_base_rate / 1000000;
    clk_mul = gck_rate / clk_base_rate - 1;
    caps0 &= ~SDHCI_CLOCK_V3_BASE_MASK;
    caps0 |= FIELD_PREP(SDHCI_CLOCK_V3_BASE_MASK, clk_base);
    caps1 &= ~SDHCI_CLOCK_MUL_MASK;
    caps1 |= FIELD_PREP(SDHCI_CLOCK_MUL_MASK, clk_mul);
// Set capabilities in r/w mode.
    writel(SDMMC_CACR_KEY | SDMMC_CACR_CAPWREN, host.ioaddr + SDMMC_CACR);
    writel(caps0, host.ioaddr + SDHCI_CAPABILITIES);
    writel(caps1, host.ioaddr + SDHCI_CAPABILITIES_1);
// Set capabilities in ro mode.
    writel(0, host.ioaddr + SDMMC_CACR);
    dev_dbg(dev, "update clk mul to %u as gck rate is %u Hz and clk base is %u Hz\n",
    clk_mul, gck_rate, clk_base_rate);
//
// We have to set preset values because it depends on the clk_mul
// value. Moreover, SDR104 is supported in a degraded mode since the
// maximum sd clock value is 120 MHz instead of 208 MHz. For that
// reason, we need to use presets to support SDR104.
//
    preset_div = DIV_ROUND_UP(gck_rate, 24000000) - 1;
    writew(SDHCI_AT91_PRESET_COMMON_CONF | preset_div,
    host.ioaddr + SDHCI_PRESET_FOR_SDR12);
    preset_div = DIV_ROUND_UP(gck_rate, 50000000) - 1;
    writew(SDHCI_AT91_PRESET_COMMON_CONF | preset_div,
    host.ioaddr + SDHCI_PRESET_FOR_SDR25);
    preset_div = DIV_ROUND_UP(gck_rate, 100000000) - 1;
    writew(SDHCI_AT91_PRESET_COMMON_CONF | preset_div,
    host.ioaddr + SDHCI_PRESET_FOR_SDR50);
    preset_div = DIV_ROUND_UP(gck_rate, 120000000) - 1;
    writew(SDHCI_AT91_PRESET_COMMON_CONF | preset_div,
    host.ioaddr + SDHCI_PRESET_FOR_SDR104);
    preset_div = DIV_ROUND_UP(gck_rate, 50000000) - 1;
    writew(SDHCI_AT91_PRESET_COMMON_CONF | preset_div,
    host.ioaddr + SDHCI_PRESET_FOR_DDR50);
    clk_prepare_enable(priv.mainck);
    clk_prepare_enable(priv.gck);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_at91_suspend(dev: *mut device) -> c_int {
    static int sdhci_at91_suspend(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_at91_priv *priv = sdhci_pltfm_priv(pltfm_host);
    int ret;
    ret = pm_runtime_force_suspend(dev);
    priv.restore_needed = true;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_at91_runtime_suspend(dev: *mut device) -> c_int {
    static int sdhci_at91_runtime_suspend(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_at91_priv *priv = sdhci_pltfm_priv(pltfm_host);
    sdhci_runtime_suspend_host(host);
    if (host.tuning_mode != SDHCI_TUNING_MODE_3)
    mmc_retune_needed(host.mmc);
    clk_disable_unprepare(priv.gck);
    clk_disable_unprepare(priv.hclock);
    clk_disable_unprepare(priv.mainck);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_at91_runtime_resume(dev: *mut device) -> c_int {
    static int sdhci_at91_runtime_resume(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_at91_priv *priv = sdhci_pltfm_priv(pltfm_host);
    int ret;
    if (priv.restore_needed) {
    ret = sdhci_at91_set_clks_presets(dev);
    if (ret)
    return ret;
    priv.restore_needed = false;
    goto out;
    }
    ret = clk_prepare_enable(priv.mainck);
    if (ret) {
    dev_err(dev, "can't enable mainck\n");
    return ret;
    }
    ret = clk_prepare_enable(priv.hclock);
    if (ret) {
    dev_err(dev, "can't enable hclock\n");
    return ret;
    }
    ret = clk_prepare_enable(priv.gck);
    if (ret) {
    dev_err(dev, "can't enable gck\n");
    return ret;
    }
    out:
    sdhci_runtime_resume_host(host, 0);
    return 0;
    }
    static const struct dev_pm_ops sdhci_at91_dev_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(sdhci_at91_suspend, pm_runtime_force_resume)
    RUNTIME_PM_OPS(sdhci_at91_runtime_suspend, sdhci_at91_runtime_resume, core::ptr::null_mut())
    };
#[no_mangle]
unsafe extern "C" fn sdhci_at91_probe(pdev: *mut platform_device) -> c_int {
    static int sdhci_at91_probe(struct platform_device *pdev)
    {
    const struct sdhci_at91_soc_data	*soc_data;
    struct sdhci_host		*host;
    struct sdhci_pltfm_host		*pltfm_host;
    struct sdhci_at91_priv		*priv;
    int				ret;
    soc_data = of_device_get_match_data(&pdev.dev);
    if (!soc_data)
    return -EINVAL;
    host = sdhci_pltfm_init(pdev, soc_data.pdata, sizeof(*priv));
    if (IS_ERR(host))
    return PTR_ERR(host);
    pltfm_host = sdhci_priv(host);
    priv = sdhci_pltfm_priv(pltfm_host);
    priv.soc_data = soc_data;
    priv.mainck = devm_clk_get(&pdev.dev, "baseclk");
    if (IS_ERR(priv.mainck)) {
    if (soc_data.baseclk_is_generated_internally)
    priv.mainck = core::ptr::null_mut();
    else
    return dev_err_probe(&pdev.dev, PTR_ERR(priv.mainck),
    "failed to get baseclk\n");
    }
    priv.hclock = devm_clk_get(&pdev.dev, "hclock");
    if (IS_ERR(priv.hclock))
    return dev_err_probe(&pdev.dev, PTR_ERR(priv.hclock),
    "failed to get hclock\n");
    priv.gck = devm_clk_get(&pdev.dev, "multclk");
    if (IS_ERR(priv.gck))
    return dev_err_probe(&pdev.dev, PTR_ERR(priv.gck),
    "failed to get multclk\n");
    ret = sdhci_at91_set_clks_presets(&pdev.dev);
    if (ret)
    return ret;
    priv.restore_needed = false;
//
// if SDCAL pin is wrongly connected, we must enable
// the analog calibration cell permanently.
//
    priv.cal_always_on =
    device_property_read_bool(&pdev.dev,
    "microchip,sdcal-inverted");
    ret = mmc_of_parse(host.mmc);
    if (ret)
    goto clocks_disable_unprepare;
    sdhci_get_of_property(pdev);
    pm_runtime_get_noresume(&pdev.dev);
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    pm_runtime_set_autosuspend_delay(&pdev.dev, 50);
    pm_runtime_use_autosuspend(&pdev.dev);
// HS200 is broken at this moment
    host.quirks2 |= SDHCI_QUIRK2_BROKEN_HS200;
    ret = sdhci_add_host(host);
    if (ret)
    goto pm_runtime_disable;
//
// When calling sdhci_runtime_suspend_host(), the sdhci layer makes
// the assumption that all the clocks of the controller are disabled.
// It means we can't get irq from it when it is runtime suspended.
// For that reason, it is not planned to wake-up on a card detect irq
// from the controller.
// If we want to use runtime PM and to be able to wake-up on card
// insertion, we have to use a GPIO for the card detection or we can
// use polling. Be aware that using polling will resume/suspend the
// controller between each attempt.
// Disable SDHCI_QUIRK_BROKEN_CARD_DETECTION to be sure nobody tries
// to enable polling via device tree with broken-cd property.
//
    if (mmc_card_is_removable(host.mmc) &&
    mmc_gpio_get_cd(host.mmc) < 0) {
    host.mmc.caps |= MMC_CAP_NEEDS_POLL;
    host.quirks &= ~SDHCI_QUIRK_BROKEN_CARD_DETECTION;
    }
//
// If the device attached to the MMC bus is not removable, it is safer
// to set the Force Card Detect bit. People often don't connect the
// card detect signal and use this pin for another purpose. If the card
// detect pin is not muxed to SDHCI controller, a default value is
// used. This value can be different from a SoC revision to another
// one. Problems come when this default value is not card present. To
// avoid this case, if the device is non removable then the card
// detection procedure using the SDMCC_CD signal is bypassed.
// This bit is reset when a software reset for all command is performed
// so we need to implement our own reset function to set back this bit.
//
// WA: SAMA5D2 doesn't drive CMD if using CD GPIO line.
//
    if ((host.mmc.caps & MMC_CAP_NONREMOVABLE)
    || mmc_gpio_get_cd(host.mmc) >= 0)
    sdhci_at91_set_force_card_detect(host);
    pm_runtime_put_autosuspend(&pdev.dev);
    return 0;
    pm_runtime_disable:
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    pm_runtime_put_noidle(&pdev.dev);
    clocks_disable_unprepare:
    clk_disable_unprepare(priv.gck);
    clk_disable_unprepare(priv.mainck);
    clk_disable_unprepare(priv.hclock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_at91_remove(pdev: *mut platform_device) {
    static void sdhci_at91_remove(struct platform_device *pdev)
    {
    struct sdhci_host	*host = platform_get_drvdata(pdev);
    struct sdhci_pltfm_host	*pltfm_host = sdhci_priv(host);
    struct sdhci_at91_priv	*priv = sdhci_pltfm_priv(pltfm_host);
    struct clk *gck = priv.gck;
    struct clk *hclock = priv.hclock;
    struct clk *mainck = priv.mainck;
    pm_runtime_get_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    pm_runtime_put_noidle(&pdev.dev);
    sdhci_pltfm_remove(pdev);
    clk_disable_unprepare(gck);
    clk_disable_unprepare(hclock);
    clk_disable_unprepare(mainck);
    }
    static struct platform_driver sdhci_at91_driver = {
    .driver		= {
    .name	= "sdhci-at91",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = sdhci_at91_dt_match,
    .pm	= pm_ptr(&sdhci_at91_dev_pm_ops),
    },
    .probe		= sdhci_at91_probe,
    .remove		= sdhci_at91_remove,
    };
    module_platform_driver(sdhci_at91_driver);
    MODULE_DESCRIPTION("SDHCI driver for at91");
    MODULE_AUTHOR("Ludovic Desroches <ludovic.desroches@atmel.com>");
    MODULE_LICENSE("GPL v2");
