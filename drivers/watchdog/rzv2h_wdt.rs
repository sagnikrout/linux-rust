//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/rzv2h_wdt.c
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
// Renesas RZ/V2H(P) WDT Watchdog Driver
//
// Copyright (C) 2024 Renesas Electronics Corporation.
//

pub const WDTRR: c_uint = 0x00	/* WDT Refresh Register RW, 8  */;
pub const WDTCR: c_uint = 0x02	/* WDT Control Register RW, 16 */;
pub const WDTSR: c_uint = 0x04	/* WDT Status Register RW, 16 */;
pub const WDTRCR: c_uint = 0x06	/* WDT Reset Control Register RW, 8  */;
// This register is only available on RZ/T2H and RZ/N2H SoCs
pub const WDTDCR: c_uint = 0x00	/* WDT Debug Control Register RW, 32  */;
pub const WDTCR_TOPS_1024: c_uint = 0x00;
pub const WDTCR_TOPS_4096: c_uint = 0x01;
pub const WDTCR_TOPS_16384: c_uint = 0x03;
pub const WDTCR_CKS_CLK_1: c_uint = 0x00;
pub const WDTCR_CKS_CLK_4: c_uint = 0x10;
pub const WDTCR_CKS_CLK_256: c_uint = 0x50;
pub const WDTCR_CKS_CLK_8192: c_uint = 0x80;
pub const WDTCR_RPES_0: c_uint = 0x300;
pub const WDTCR_RPES_75: c_uint = 0x000;
pub const WDTCR_RPSS_25: c_uint = 0x00;
pub const WDTCR_RPSS_100: c_uint = 0x3000;

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    enum rzv2h_wdt_count_source {
    COUNT_SOURCE_LOCO,
    COUNT_SOURCE_PCLK,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_of_data {
    pub cks_min: u8,
    pub cks_max: u8,
    pub cks_div: u16,
    pub tops: u8,
    pub timeout_cycles: u16,
    pub count_source: enum rzv2h_wdt_count_source,
    pub wdtdcr: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_wdt_priv {
    pub base: *mut void __iomem,
    pub wdtdcr: *mut void __iomem,
    pub pclk: *mut clk,
    pub oscclk: *mut clk,
    pub rstc: *mut reset_control,
    pub wdev: watchdog_device,
    pub of_data: *const rzv2h_of_data,
}

#[no_mangle]
unsafe extern "C" fn rzv2h_wdt_ping(wdev: *mut watchdog_device) -> c_int {
    static int rzv2h_wdt_ping(struct watchdog_device *wdev)
    {
    struct rzv2h_wdt_priv *priv = watchdog_get_drvdata(wdev);
//
// The down-counter is refreshed and starts counting operation on
// a write of the values 00h and FFh to the WDTRR register.
//
    writeb(0x0, priv.base + WDTRR);
    writeb(0xFF, priv.base + WDTRR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzt2h_wdt_wdtdcr_count_stop(priv: *mut rzv2h_wdt_priv) {
    static void rzt2h_wdt_wdtdcr_count_stop(struct rzv2h_wdt_priv *priv)
    {
    let mut reg: u32 = readl(priv.wdtdcr + WDTDCR);
    writel(reg | WDTDCR_WDTSTOPCTRL, priv.wdtdcr + WDTDCR);
    }
#[no_mangle]
unsafe extern "C" fn rzt2h_wdt_wdtdcr_count_start(priv: *mut rzv2h_wdt_priv) {
    static void rzt2h_wdt_wdtdcr_count_start(struct rzv2h_wdt_priv *priv)
    {
    let mut reg: u32 = readl(priv.wdtdcr + WDTDCR);
    writel(reg & ~WDTDCR_WDTSTOPCTRL, priv.wdtdcr + WDTDCR);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_wdt_setup(wdev: *mut watchdog_device, wdtcr: u16) {
    static void rzv2h_wdt_setup(struct watchdog_device *wdev, u16 wdtcr)
    {
    struct rzv2h_wdt_priv *priv = watchdog_get_drvdata(wdev);
// Configure the timeout, clock division ratio, and window start and end positions.
    writew(wdtcr, priv.base + WDTCR);
// Enable interrupt output to the ICU.
    writeb(0, priv.base + WDTRCR);
// Clear underflow flag and refresh error flag.
    writew(0, priv.base + WDTSR);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_wdt_start(wdev: *mut watchdog_device) -> c_int {
    static int rzv2h_wdt_start(struct watchdog_device *wdev)
    {
    struct rzv2h_wdt_priv *priv = watchdog_get_drvdata(wdev);
    const struct rzv2h_of_data *of_data = priv.of_data;
    int ret;
    ret = pm_runtime_resume_and_get(wdev.parent);
    if (ret)
    return ret;
    ret = reset_control_deassert(priv.rstc);
    if (ret) {
    pm_runtime_put(wdev.parent);
    return ret;
    }
// delay to handle clock halt after de-assert operation
    udelay(3);
//
// WDTCR
// - CKS[7:4] - Clock Division Ratio Select
// - 0101b: oscclk/256 for RZ/V2H(P)
// - 1000b: pclkl/8192 for RZ/T2H
// - RPSS[13:12] - Window Start Position Select - 11b: 100%
// - RPES[9:8] - Window End Position Select - 11b: 0%
// - TOPS[1:0] - Timeout Period Select
// - 11b: 16384 cycles (3FFFh) for RZ/V2H(P)
// - 01b: 4096 cycles (0FFFh) for RZ/T2H
//
    rzv2h_wdt_setup(wdev, of_data.cks_max | WDTCR_RPSS_100 |
    WDTCR_RPES_0 | of_data.tops);
    if (priv.of_data.wdtdcr)
    rzt2h_wdt_wdtdcr_count_start(priv);
//
// Down counting starts after writing the sequence 00h -> FFh to the
// WDTRR register. Hence, call the ping operation after loading the counter.
//
    rzv2h_wdt_ping(wdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_wdt_stop(wdev: *mut watchdog_device) -> c_int {
    static int rzv2h_wdt_stop(struct watchdog_device *wdev)
    {
    struct rzv2h_wdt_priv *priv = watchdog_get_drvdata(wdev);
    int ret;
    ret = reset_control_assert(priv.rstc);
    if (ret)
    return ret;
    if (priv.of_data.wdtdcr)
    rzt2h_wdt_wdtdcr_count_stop(priv);
    pm_runtime_put(wdev.parent);
    return 0;
    }
    static const struct watchdog_info rzv2h_wdt_ident = {
    .options = WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING | WDIOF_SETTIMEOUT,
    .identity = "Renesas RZ/V2H WDT Watchdog",
    };
    static int rzv2h_wdt_restart(struct watchdog_device *wdev,
    unsigned long action, void *data)
    {
    struct rzv2h_wdt_priv *priv = watchdog_get_drvdata(wdev);
    int ret;
    if (!watchdog_active(wdev)) {
    ret = clk_enable(priv.pclk);
    if (ret)
    return ret;
    ret = clk_enable(priv.oscclk);
    if (ret) {
    clk_disable(priv.pclk);
    return ret;
    }
    ret = reset_control_deassert(priv.rstc);
    if (ret) {
    clk_disable(priv.oscclk);
    clk_disable(priv.pclk);
    return ret;
    }
    } else {
//
// Writing to the WDT Control Register (WDTCR) or WDT Reset
// Control Register (WDTRCR) is possible once between the
// release from the reset state and the first refresh operation.
// Therefore, issue a reset if the watchdog is active.
//
    ret = reset_control_reset(priv.rstc);
    if (ret)
    return ret;
    }
// delay to handle clock halt after de-assert operation
    udelay(3);
//
// WDTCR
// - CKS[7:4] - Clock Division Ratio Select
// - 0000b: oscclk/1 for RZ/V2H(P)
// - 0100b: pclkl/4 for RZ/T2H
// - RPSS[13:12] - Window Start Position Select - 00b: 25%
// - RPES[9:8] - Window End Position Select - 00b: 75%
// - TOPS[1:0] - Timeout Period Select - 00b: 1024 cycles (03FFh)
//
    rzv2h_wdt_setup(wdev, priv.of_data.cks_min | WDTCR_RPSS_25 |
    WDTCR_RPES_75 | WDTCR_TOPS_1024);
    if (priv.of_data.wdtdcr)
    rzt2h_wdt_wdtdcr_count_start(priv);
    rzv2h_wdt_ping(wdev);
// wait for underflow to trigger...
    udelay(5);
    return 0;
    }
    static const struct watchdog_ops rzv2h_wdt_ops = {
    .owner = THIS_MODULE,
    .start = rzv2h_wdt_start,
    .stop = rzv2h_wdt_stop,
    .ping = rzv2h_wdt_ping,
    .restart = rzv2h_wdt_restart,
    };
    static int rzt2h_wdt_wdtdcr_init(struct platform_device *pdev,
    struct rzv2h_wdt_priv *priv)
    {
    int ret;
    priv.wdtdcr = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(priv.wdtdcr))
    return PTR_ERR(priv.wdtdcr);
    ret = pm_runtime_resume_and_get(&pdev.dev);
    if (ret)
    return ret;
    rzt2h_wdt_wdtdcr_count_stop(priv);
    pm_runtime_put(&pdev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int rzv2h_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rzv2h_wdt_priv *priv;
    struct clk *count_clk;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.of_data = of_device_get_match_data(dev);
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    priv.pclk = devm_clk_get_prepared(dev, "pclk");
    if (IS_ERR(priv.pclk))
    return dev_err_probe(dev, PTR_ERR(priv.pclk), "Failed to get pclk\n");
    priv.oscclk = devm_clk_get_optional_prepared(dev, "oscclk");
    if (IS_ERR(priv.oscclk))
    return dev_err_probe(dev, PTR_ERR(priv.oscclk), "Failed to get oscclk\n");
    priv.rstc = devm_reset_control_get_optional_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(priv.rstc))
    return dev_err_probe(dev, PTR_ERR(priv.rstc),
    "Failed to get cpg reset\n");
    switch (priv.of_data.count_source) {
    case COUNT_SOURCE_LOCO:
    count_clk = priv.oscclk;
    break;
    case COUNT_SOURCE_PCLK:
    count_clk = priv.pclk;
    break;
    default:
    return dev_err_probe(dev, -EINVAL, "Invalid count source\n");
    }
    priv.wdev.max_hw_heartbeat_ms = (MILLI * priv.of_data.timeout_cycles *
    priv.of_data.cks_div) / clk_get_rate(count_clk);
    dev_dbg(dev, "max hw timeout of %dms\n", priv.wdev.max_hw_heartbeat_ms);
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return ret;
    if (priv.of_data.wdtdcr) {
    ret = rzt2h_wdt_wdtdcr_init(pdev, priv);
    if (ret)
    return dev_err_probe(dev, ret, "WDTDCR init failed\n");
    }
    priv.wdev.min_timeout = 1;
    priv.wdev.timeout = WDT_DEFAULT_TIMEOUT;
    priv.wdev.info = &rzv2h_wdt_ident;
    priv.wdev.ops = &rzv2h_wdt_ops;
    priv.wdev.parent = dev;
    watchdog_set_drvdata(&priv.wdev, priv);
    watchdog_set_nowayout(&priv.wdev, nowayout);
    watchdog_stop_on_unregister(&priv.wdev);
    watchdog_init_timeout(&priv.wdev, 0, dev);
    return devm_watchdog_register_device(dev, &priv.wdev);
    }
    static const struct rzv2h_of_data rzt2h_wdt_of_data = {
    .cks_min = WDTCR_CKS_CLK_4,
    .cks_max = WDTCR_CKS_CLK_8192,
    .cks_div = 8192,
    .tops = WDTCR_TOPS_4096,
    .timeout_cycles = 4096,
    .count_source = COUNT_SOURCE_PCLK,
    .wdtdcr = true,
    };
    static const struct rzv2h_of_data rzv2h_wdt_of_data = {
    .cks_min = WDTCR_CKS_CLK_1,
    .cks_max = WDTCR_CKS_CLK_256,
    .cks_div = 256,
    .tops = WDTCR_TOPS_16384,
    .timeout_cycles = 16384,
    .count_source = COUNT_SOURCE_LOCO,
    };
    static const struct of_device_id rzv2h_wdt_ids[] = {
    { .compatible = "renesas,r9a09g057-wdt", .data = &rzv2h_wdt_of_data },
    { .compatible = "renesas,r9a09g077-wdt", .data = &rzt2h_wdt_of_data },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rzv2h_wdt_ids);
    static struct platform_driver rzv2h_wdt_driver = {
    .driver = {
    .name = "rzv2h_wdt",
    .of_match_table = rzv2h_wdt_ids,
    },
    .probe = rzv2h_wdt_probe,
    };
    module_platform_driver(rzv2h_wdt_driver);
    MODULE_AUTHOR("Lad Prabhakar <prabhakar.mahadev-lad.rj@bp.renesas.com>");
    MODULE_DESCRIPTION("Renesas RZ/V2H(P) WDT Watchdog Driver");
    MODULE_LICENSE("GPL");
