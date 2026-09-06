//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/at91-sama5d2_shdwc.c
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


//
// Atmel SAMA5D2-Compatible Shutdown Controller (SHDWC) driver.
// Found on some SoCs as the sama5d2 (obviously).
//
// Copyright (C) 2015 Atmel Corporation,
// Nicolas Ferre <nicolas.ferre@atmel.com>
//
// Evolved from driver at91-poweroff.c.
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//
// TODO:
// - addition to status of other wake-up inputs [1 - 15]
// - Analog Comparator wake-up alarm
// - Serial RX wake-up alarm
// - low power debouncer
//

pub const SLOW_CLOCK_FREQ: c_int = 32768;
pub const AT91_SHDW_CR: c_uint = 0x00		/* Shut Down Control Register */;

pub const AT91_SHDW_MR: c_uint = 0x04		/* Shut Down Mode Register */;
pub const AT91_SHDW_WKUPDBC_SHIFT: c_int = 24;

    & AT91_SHDW_WKUPDBC_MASK)
pub const AT91_SHDW_SR: c_uint = 0x08		/* Shut Down Status Register */;
pub const AT91_SHDW_WKUPIS_SHIFT: c_int = 16;

    & AT91_SHDW_WKUPIS_MASK)
pub const AT91_SHDW_WUIR: c_uint = 0x0c		/* Shutdown Wake-up Inputs Register */;

pub const AT91_SHDW_WKUPT_SHIFT: c_int = 16;

    & AT91_SHDW_WKUPT_MASK)

    SLOW_CLOCK_FREQ)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shdwc_reg_config {
    pub wkup_pin_input: u8,
    pub mr_rtcwk_shift: u8,
    pub mr_rttwk_shift: u8,
    pub sr_rtcwk_shift: u8,
    pub sr_rttwk_shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_reg_config {
    pub mckr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddrc_reg_config {
    pub type_offset: u32,
    pub type_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_config {
    pub shdwc: shdwc_reg_config,
    pub pmc: pmc_reg_config,
    pub ddrc: ddrc_reg_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shdwc {
    pub rcfg: *const reg_config,
    pub sclk: *mut clk,
    pub shdwc_base: *mut void __iomem,
    pub mpddrc_base: *mut void __iomem,
    pub pmc_base: *mut void __iomem,
}

//
// Hold configuration here, cannot be more than one instance of the driver
// since pm_power_off itself is global.
//
    static struct shdwc *at91_shdwc;
    static const unsigned long long sdwc_dbc_period[] = {
    0, 3, 32, 512, 4096, 32768,
    };
#[no_mangle]
unsafe extern "C" fn at91_wakeup_status(pdev: *mut platform_device) {
    static void at91_wakeup_status(struct platform_device *pdev)
    {
    struct shdwc *shdw = platform_get_drvdata(pdev);
    const struct reg_config *rcfg = shdw.rcfg;
    u32 reg;
    char *reason = "unknown";
    reg = readl(shdw.shdwc_base + AT91_SHDW_SR);
    dev_dbg(&pdev.dev, "%s: status = %#x\n", __func__, reg);
// Simple power-on, just bail out
    if (!reg)
    return;
    if (SHDW_WK_PIN(reg, &rcfg.shdwc))
    reason = "WKUP pin";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: SHDW_RTCWK(reg, _arg: &rcfg->shdwc)) -> else {
    else if (SHDW_RTCWK(reg, &rcfg.shdwc))
    reason = "RTC";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: SHDW_RTTWK(reg, _arg: &rcfg->shdwc)) -> else {
    else if (SHDW_RTTWK(reg, &rcfg.shdwc))
    reason = "RTT";
    dev_info(&pdev.dev, "Wake-Up source: %s\n", reason);
    }
#[no_mangle]
unsafe extern "C" fn at91_poweroff() {
    static void at91_poweroff(void)
    {
    asm volatile(
// Align to cache lines
    ".balign 32\n\t"
// Ensure AT91_SHDW_CR is in the TLB by reading it
    "	ldr	r6, [%2, #" __stringify(AT91_SHDW_CR) "]\n\t"
// Power down SDRAM0
    "	tst	%0, #0\n\t"
    "	beq	1f\n\t"
    "	str	%1, [%0, #" __stringify(AT91_DDRSDRC_LPR) "]\n\t"
// Switch the master clock source to slow clock.
    "1:	ldr	r6, [%4, %5]\n\t"
    "	bic	r6, r6,  #" __stringify(AT91_PMC_CSS) "\n\t"
    "	str	r6, [%4, %5]\n\t"
// Wait for clock switch.
    "2:	ldr	r6, [%4, #" __stringify(AT91_PMC_SR) "]\n\t"
    "	tst	r6, #"	    __stringify(AT91_PMC_MCKRDY) "\n\t"
    "	beq	2b\n\t"
// Shutdown CPU
    "	str	%3, [%2, #" __stringify(AT91_SHDW_CR) "]\n\t"
    "	b	.\n\t"
    :
    : "r" (at91_shdwc.mpddrc_base),
    "r" cpu_to_le32(AT91_DDRSDRC_LPDDR2_PWOFF),
    "r" (at91_shdwc.shdwc_base),
    "r" cpu_to_le32(AT91_SHDW_KEY | AT91_SHDW_SHDW),
    "r" (at91_shdwc.pmc_base),
    "r" (at91_shdwc.rcfg.pmc.mckr)
    : "r6");
    }
    static u32 at91_shdwc_debouncer_value(struct platform_device *pdev,
    u32 in_period_us)
    {
    int i;
    let mut max_idx: c_int = ARRAY_SIZE(sdwc_dbc_period) - 1;
    unsigned long long period_us;
    let mut max_period_us: c_ulonglong = DBC_PERIOD_US(sdwc_dbc_period[max_idx]);
    if (in_period_us > max_period_us) {
    dev_warn(&pdev.dev,
    "debouncer period %u too big, reduced to %llu us\n",
    in_period_us, max_period_us);
    return max_idx;
    }
    for (i = max_idx - 1; i > 0; i--) {
    period_us = DBC_PERIOD_US(sdwc_dbc_period[i]);
    dev_dbg(&pdev.dev, "%s: ref[%d] = %llu\n",
    __func__, i, period_us);
    if (in_period_us > period_us)
    break;
    }
    return i + 1;
    }
    static u32 at91_shdwc_get_wakeup_input(struct platform_device *pdev,
    struct device_node *np)
    {
    struct device_node *cnp;
    u32 wk_input_mask;
    let mut wuir: u32 = 0;
    u32 wk_input;
    for_each_child_of_node(np, cnp) {
    if (of_property_read_u32(cnp, "reg", &wk_input)) {
    dev_warn(&pdev.dev, "reg property is missing for %pOF\n",
    cnp);
    continue;
    }
    wk_input_mask = 1 << wk_input;
    if (!(wk_input_mask & AT91_SHDW_WKUPEN_MASK)) {
    dev_warn(&pdev.dev,
    "wake-up input %d out of bounds ignore\n",
    wk_input);
    continue;
    }
    wuir |= wk_input_mask;
    if (of_property_read_bool(cnp, "atmel,wakeup-active-high"))
    wuir |= AT91_SHDW_WKUPT(wk_input);
    dev_dbg(&pdev.dev, "%s: (child %d) wuir = %#x\n",
    __func__, wk_input, wuir);
    }
    return wuir;
    }
#[no_mangle]
unsafe extern "C" fn at91_shdwc_dt_configure(pdev: *mut platform_device) {
    static void at91_shdwc_dt_configure(struct platform_device *pdev)
    {
    struct shdwc *shdw = platform_get_drvdata(pdev);
    const struct reg_config *rcfg = shdw.rcfg;
    struct device_node *np = pdev.dev.of_node;
    let mut mode: u32 = 0, tmp, input;
    if (!np) {
    dev_err(&pdev.dev, "device node not found\n");
    return;
    }
    if (!of_property_read_u32(np, "debounce-delay-us", &tmp))
    mode |= AT91_SHDW_WKUPDBC(at91_shdwc_debouncer_value(pdev, tmp));
    if (of_property_read_bool(np, "atmel,wakeup-rtc-timer"))
    mode |= SHDW_RTCWKEN(&rcfg.shdwc);
    if (of_property_read_bool(np, "atmel,wakeup-rtt-timer"))
    mode |= SHDW_RTTWKEN(&rcfg.shdwc);
    dev_dbg(&pdev.dev, "%s: mode = %#x\n", __func__, mode);
    writel(mode, shdw.shdwc_base + AT91_SHDW_MR);
    input = at91_shdwc_get_wakeup_input(pdev, np);
    writel(input, shdw.shdwc_base + AT91_SHDW_WUIR);
    }
    static const struct reg_config sama5d2_reg_config = {
    .shdwc = {
    .wkup_pin_input = 0,
    .mr_rtcwk_shift = 17,
    .mr_rttwk_shift	= SHDW_CFG_NOT_USED,
    .sr_rtcwk_shift = 5,
    .sr_rttwk_shift = SHDW_CFG_NOT_USED,
    },
    .pmc = {
    .mckr		= 0x30,
    },
    .ddrc = {
    .type_offset	= AT91_DDRSDRC_MDR,
    .type_mask	= AT91_DDRSDRC_MD
    },
    };
    static const struct reg_config sam9x60_reg_config = {
    .shdwc = {
    .wkup_pin_input = 0,
    .mr_rtcwk_shift = 17,
    .mr_rttwk_shift = 16,
    .sr_rtcwk_shift = 5,
    .sr_rttwk_shift = 4,
    },
    .pmc = {
    .mckr		= 0x28,
    },
    .ddrc = {
    .type_offset	= AT91_DDRSDRC_MDR,
    .type_mask	= AT91_DDRSDRC_MD
    },
    };
    static const struct reg_config sama7g5_reg_config = {
    .shdwc = {
    .wkup_pin_input = 0,
    .mr_rtcwk_shift = 17,
    .mr_rttwk_shift = 16,
    .sr_rtcwk_shift = 5,
    .sr_rttwk_shift = 4,
    },
    .pmc = {
    .mckr		= 0x28,
    },
    };
    static const struct of_device_id at91_shdwc_of_match[] = {
    {
    .compatible = "atmel,sama5d2-shdwc",
    .data = &sama5d2_reg_config,
    },
    {
    .compatible = "microchip,sam9x60-shdwc",
    .data = &sam9x60_reg_config,
    },
    {
    .compatible = "microchip,sama7g5-shdwc",
    .data = &sama7g5_reg_config,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, at91_shdwc_of_match);
    static const struct of_device_id at91_pmc_ids[] = {
    { .compatible = "atmel,sama5d2-pmc" },
    { .compatible = "microchip,sam9x60-pmc" },
    { .compatible = "microchip,sama7g5-pmc" },
    { .compatible = "microchip,sam9x7-pmc" },
    { .compatible = "microchip,sama7d65-pmc" },
    { /* Sentinel. */ }
    };
#[no_mangle]
unsafe extern "C" fn at91_shdwc_probe(pdev: *mut platform_device) -> c_int {
    static int at91_shdwc_probe(struct platform_device *pdev)
    {
    const struct of_device_id *match;
    struct device_node *np;
    u32 ddr_type;
    int ret;
    if (!pdev.dev.of_node)
    return -ENODEV;
    if (at91_shdwc)
    return -EBUSY;
    at91_shdwc = devm_kzalloc(&pdev.dev, sizeof(*at91_shdwc), GFP_KERNEL);
    if (!at91_shdwc)
    return -ENOMEM;
    platform_set_drvdata(pdev, at91_shdwc);
    at91_shdwc.shdwc_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(at91_shdwc.shdwc_base))
    return PTR_ERR(at91_shdwc.shdwc_base);
    match = of_match_node(at91_shdwc_of_match, pdev.dev.of_node);
    at91_shdwc.rcfg = match.data;
    at91_shdwc.sclk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(at91_shdwc.sclk))
    return PTR_ERR(at91_shdwc.sclk);
    ret = clk_prepare_enable(at91_shdwc.sclk);
    if (ret) {
    dev_err(&pdev.dev, "Could not enable slow clock\n");
    return ret;
    }
    at91_wakeup_status(pdev);
    at91_shdwc_dt_configure(pdev);
    np = of_find_matching_node(core::ptr::null_mut(), at91_pmc_ids);
    if (!np) {
    ret = -ENODEV;
    goto clk_disable;
    }
    at91_shdwc.pmc_base = of_iomap(np, 0);
    of_node_put(np);
    if (!at91_shdwc.pmc_base) {
    ret = -ENOMEM;
    goto clk_disable;
    }
    if (at91_shdwc.rcfg.ddrc.type_mask) {
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(),
    "atmel,sama5d3-ddramc");
    if (!np) {
    ret = -ENODEV;
    goto unmap;
    }
    at91_shdwc.mpddrc_base = of_iomap(np, 0);
    of_node_put(np);
    if (!at91_shdwc.mpddrc_base) {
    ret = -ENOMEM;
    goto unmap;
    }
    ddr_type = readl(at91_shdwc.mpddrc_base +
    at91_shdwc.rcfg.ddrc.type_offset) &
    at91_shdwc.rcfg.ddrc.type_mask;
    if (ddr_type != AT91_DDRSDRC_MD_LPDDR2 &&
    ddr_type != AT91_DDRSDRC_MD_LPDDR3) {
    iounmap(at91_shdwc.mpddrc_base);
    at91_shdwc.mpddrc_base = core::ptr::null_mut();
    }
    }
    pm_power_off = at91_poweroff;
    return 0;
    unmap:
    iounmap(at91_shdwc.pmc_base);
    clk_disable:
    clk_disable_unprepare(at91_shdwc.sclk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn at91_shdwc_remove(pdev: *mut platform_device) {
    static void at91_shdwc_remove(struct platform_device *pdev)
    {
    struct shdwc *shdw = platform_get_drvdata(pdev);
    if (pm_power_off == at91_poweroff)
    pm_power_off = core::ptr::null_mut();
// Reset values to disable wake-up features
    writel(0, shdw.shdwc_base + AT91_SHDW_MR);
    writel(0, shdw.shdwc_base + AT91_SHDW_WUIR);
    if (shdw.mpddrc_base)
    iounmap(shdw.mpddrc_base);
    iounmap(shdw.pmc_base);
    clk_disable_unprepare(shdw.sclk);
    }
    static struct platform_driver at91_shdwc_driver = {
    .probe = at91_shdwc_probe,
    .remove = at91_shdwc_remove,
    .driver = {
    .name = "at91-shdwc",
    .of_match_table = at91_shdwc_of_match,
    },
    };
    module_platform_driver(at91_shdwc_driver);
    MODULE_AUTHOR("Nicolas Ferre <nicolas.ferre@atmel.com>");
    MODULE_DESCRIPTION("Atmel shutdown controller driver");
    MODULE_LICENSE("GPL v2");
