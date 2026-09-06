//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/at91-poweroff.c
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
// Atmel AT91 SAM9 SoCs reset code
//
// Copyright (C) 2007 Atmel Corporation.
// Copyright (C) 2011 Jean-Christophe PLAGNIOL-VILLARD <plagnioj@jcrosoft.com>
// Copyright (C) 2014 Free Electrons
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const AT91_SHDW_CR: c_uint = 0x00		/* Shut Down Control Register */;

pub const AT91_SHDW_MR: c_uint = 0x04		/* Shut Down Mode Register */;

pub const AT91_SHDW_CPTWK0_MAX: c_uint = 0xf			/* Maximum Counter On Wake Up 0 */;

pub const AT91_SHDW_SR: c_uint = 0x08		/* Shut Down Status Register */;

    enum wakeup_type {
    AT91_SHDW_WKMODE0_NONE		= 0,
    AT91_SHDW_WKMODE0_HIGH		= 1,
    AT91_SHDW_WKMODE0_LOW		= 2,
    AT91_SHDW_WKMODE0_ANYLEVEL	= 3,
    };
    static const char *shdwc_wakeup_modes[] = {
    [AT91_SHDW_WKMODE0_NONE]	= "none",
    [AT91_SHDW_WKMODE0_HIGH]	= "high",
    [AT91_SHDW_WKMODE0_LOW]		= "low",
    [AT91_SHDW_WKMODE0_ANYLEVEL]	= "any",
    };
    static struct shdwc {
    struct clk *sclk;
    void __iomem *shdwc_base;
    void __iomem *mpddrc_base;
    } at91_shdwc;
#[no_mangle]
unsafe extern "C" fn at91_wakeup_status(pdev: *mut platform_device) {
    static void at91_wakeup_status(struct platform_device *pdev)
    {
    const char *reason;
    let mut reg: u32 = readl(at91_shdwc.shdwc_base + AT91_SHDW_SR);
// Simple power-on, just bail out
    if (!reg)
    return;
    if (reg & AT91_SHDW_RTTWK)
    reason = "RTT";
#[no_mangle]
pub unsafe extern "C" fn if(AT91_SHDW_RTCWK: reg &) -> else {
    else if (reg & AT91_SHDW_RTCWK)
    reason = "RTC";
    else
    reason = "unknown";
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
// Shutdown CPU
    "1:	str	%3, [%2, #" __stringify(AT91_SHDW_CR) "]\n\t"
    "	b	.\n\t"
    :
    : "r" (at91_shdwc.mpddrc_base),
    "r" cpu_to_le32(AT91_DDRSDRC_LPDDR2_PWOFF),
    "r" (at91_shdwc.shdwc_base),
    "r" cpu_to_le32(AT91_SHDW_KEY | AT91_SHDW_SHDW)
    : "r6");
    }
#[no_mangle]
unsafe extern "C" fn at91_poweroff_get_wakeup_mode(np: *mut device_node) -> c_int {
    static int at91_poweroff_get_wakeup_mode(struct device_node *np)
    {
    const char *pm;
    unsigned int i;
    int err;
    err = of_property_read_string(np, "atmel,wakeup-mode", &pm);
    if (err < 0)
    return AT91_SHDW_WKMODE0_ANYLEVEL;
    for (i = 0; i < ARRAY_SIZE(shdwc_wakeup_modes); i++)
    if (!strcasecmp(pm, shdwc_wakeup_modes[i]))
    return i;
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn at91_poweroff_dt_set_wakeup_mode(pdev: *mut platform_device) {
    static void at91_poweroff_dt_set_wakeup_mode(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    int wakeup_mode;
    let mut mode: u32 = 0, tmp;
    wakeup_mode = at91_poweroff_get_wakeup_mode(np);
    if (wakeup_mode < 0) {
    dev_warn(&pdev.dev, "shdwc unknown wakeup mode\n");
    return;
    }
    if (!of_property_read_u32(np, "atmel,wakeup-counter", &tmp)) {
    if (tmp > AT91_SHDW_CPTWK0_MAX) {
    dev_warn(&pdev.dev,
    "shdwc wakeup counter 0x%x > 0x%x reduce it to 0x%x\n",
    tmp, AT91_SHDW_CPTWK0_MAX, AT91_SHDW_CPTWK0_MAX);
    tmp = AT91_SHDW_CPTWK0_MAX;
    }
    mode |= AT91_SHDW_CPTWK0_(tmp);
    }
    if (of_property_read_bool(np, "atmel,wakeup-rtc-timer"))
    mode |= AT91_SHDW_RTCWKEN;
    if (of_property_read_bool(np, "atmel,wakeup-rtt-timer"))
    mode |= AT91_SHDW_RTTWKEN;
    writel(wakeup_mode | mode, at91_shdwc.shdwc_base + AT91_SHDW_MR);
    }
#[no_mangle]
unsafe extern "C" fn at91_poweroff_probe(pdev: *mut platform_device) -> c_int {
    static int at91_poweroff_probe(struct platform_device *pdev)
    {
    struct device_node *np;
    u32 ddr_type;
    int ret;
    at91_shdwc.shdwc_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(at91_shdwc.shdwc_base))
    return PTR_ERR(at91_shdwc.shdwc_base);
    at91_shdwc.sclk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(at91_shdwc.sclk))
    return PTR_ERR(at91_shdwc.sclk);
    ret = clk_prepare_enable(at91_shdwc.sclk);
    if (ret) {
    dev_err(&pdev.dev, "Could not enable slow clock\n");
    return ret;
    }
    at91_wakeup_status(pdev);
    if (pdev.dev.of_node)
    at91_poweroff_dt_set_wakeup_mode(pdev);
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "atmel,sama5d3-ddramc");
    if (np) {
    at91_shdwc.mpddrc_base = of_iomap(np, 0);
    of_node_put(np);
    if (!at91_shdwc.mpddrc_base) {
    ret = -ENOMEM;
    goto clk_disable;
    }
    ddr_type = readl(at91_shdwc.mpddrc_base + AT91_DDRSDRC_MDR) &
    AT91_DDRSDRC_MD;
    if (ddr_type != AT91_DDRSDRC_MD_LPDDR2 &&
    ddr_type != AT91_DDRSDRC_MD_LPDDR3) {
    iounmap(at91_shdwc.mpddrc_base);
    at91_shdwc.mpddrc_base = core::ptr::null_mut();
    }
    }
    pm_power_off = at91_poweroff;
    return 0;
    clk_disable:
    clk_disable_unprepare(at91_shdwc.sclk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn at91_poweroff_remove(pdev: *mut platform_device) {
    static void at91_poweroff_remove(struct platform_device *pdev)
    {
    if (pm_power_off == at91_poweroff)
    pm_power_off = core::ptr::null_mut();
    if (at91_shdwc.mpddrc_base)
    iounmap(at91_shdwc.mpddrc_base);
    clk_disable_unprepare(at91_shdwc.sclk);
    }
    static const struct of_device_id at91_poweroff_of_match[] = {
    { .compatible = "atmel,at91sam9260-shdwc", },
    { .compatible = "atmel,at91sam9rl-shdwc", },
    { .compatible = "atmel,at91sam9x5-shdwc", },
    { /*sentinel*/ }
    };
    MODULE_DEVICE_TABLE(of, at91_poweroff_of_match);
    static struct platform_driver at91_poweroff_driver = {
    .probe = at91_poweroff_probe,
    .remove = at91_poweroff_remove,
    .driver = {
    .name = "at91-poweroff",
    .of_match_table = at91_poweroff_of_match,
    },
    };
    module_platform_driver(at91_poweroff_driver);
    MODULE_AUTHOR("Atmel Corporation");
    MODULE_DESCRIPTION("Shutdown driver for Atmel SoCs");
    MODULE_LICENSE("GPL v2");
