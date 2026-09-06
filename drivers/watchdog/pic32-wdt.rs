//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/pic32-wdt.c
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
// PIC32 watchdog driver
//
// Joshua Henderson <joshua.henderson@microchip.com>
// Copyright (c) 2016, Microchip Technology Inc.
//

// Watchdog Timer Registers
pub const WDTCON_REG: c_uint = 0x00;
// Watchdog Timer Control Register fields

pub const WDTCON_RMCS_MASK: c_uint = 0x0003;
pub const WDTCON_RMCS_SHIFT: c_uint = 0x0006;
pub const WDTCON_RMPS_MASK: c_uint = 0x001F;
pub const WDTCON_RMPS_SHIFT: c_uint = 0x0008;

pub const WDTCON_CLR_KEY: c_uint = 0x5743;
// Reset Control Register fields for watchdog

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_wdt {
    pub regs: *mut void __iomem,
    pub rst_base: *mut void __iomem,
    pub clk: *mut clk,
}

#[no_mangle]
pub unsafe extern "C" fn pic32_wdt_is_win_enabled(wdt: *mut pic32_wdt) -> bool {
    static inline bool pic32_wdt_is_win_enabled(struct pic32_wdt *wdt)
    {
    return !!(readl(wdt.regs + WDTCON_REG) & WDTCON_WIN_EN);
    }
#[no_mangle]
pub unsafe extern "C" fn pic32_wdt_get_post_scaler(wdt: *mut pic32_wdt) -> u32 {
    static inline u32 pic32_wdt_get_post_scaler(struct pic32_wdt *wdt)
    {
    let mut v: u32 = readl(wdt.regs + WDTCON_REG);
    return (v >> WDTCON_RMPS_SHIFT) & WDTCON_RMPS_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn pic32_wdt_get_clk_id(wdt: *mut pic32_wdt) -> u32 {
    static inline u32 pic32_wdt_get_clk_id(struct pic32_wdt *wdt)
    {
    let mut v: u32 = readl(wdt.regs + WDTCON_REG);
    return (v >> WDTCON_RMCS_SHIFT) & WDTCON_RMCS_MASK;
    }
#[no_mangle]
unsafe extern "C" fn pic32_wdt_bootstatus(wdt: *mut pic32_wdt) -> c_int {
    static int pic32_wdt_bootstatus(struct pic32_wdt *wdt)
    {
    let mut v: u32 = readl(wdt.rst_base);
    writel(RESETCON_WDT_TIMEOUT, PIC32_CLR(wdt.rst_base));
    return v & RESETCON_WDT_TIMEOUT;
    }
#[no_mangle]
unsafe extern "C" fn pic32_wdt_get_timeout_secs(wdt: *mut pic32_wdt, dev: *mut device) -> u32 {
    static u32 pic32_wdt_get_timeout_secs(struct pic32_wdt *wdt, struct device *dev)
    {
    unsigned long rate;
    u32 period, ps, terminal;
    rate = clk_get_rate(wdt.clk);
    dev_dbg(dev, "wdt: clk_id %d, clk_rate %lu (prescale)\n",
    pic32_wdt_get_clk_id(wdt), rate);
// default, prescaler of 32 (i.e. div-by-32) is implicit.
    rate >>= 5;
    if (!rate)
    return 0;
// calculate terminal count from postscaler.
    ps = pic32_wdt_get_post_scaler(wdt);
    terminal = BIT(ps);
// find time taken (in secs) to reach terminal count
    period = terminal / rate;
    dev_dbg(dev,
    "wdt: clk_rate %lu (postscale) / terminal %d, timeout %dsec\n",
    rate, terminal, period);
    return period;
    }
#[no_mangle]
unsafe extern "C" fn pic32_wdt_keepalive(wdt: *mut pic32_wdt) {
    static void pic32_wdt_keepalive(struct pic32_wdt *wdt)
    {
// write key through single half-word
    writew(WDTCON_CLR_KEY, wdt.regs + WDTCON_REG + 2);
    }
#[no_mangle]
unsafe extern "C" fn pic32_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int pic32_wdt_start(struct watchdog_device *wdd)
    {
    struct pic32_wdt *wdt = watchdog_get_drvdata(wdd);
    writel(WDTCON_ON, PIC32_SET(wdt.regs + WDTCON_REG));
    pic32_wdt_keepalive(wdt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic32_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int pic32_wdt_stop(struct watchdog_device *wdd)
    {
    struct pic32_wdt *wdt = watchdog_get_drvdata(wdd);
    writel(WDTCON_ON, PIC32_CLR(wdt.regs + WDTCON_REG));
//
// Cannot touch registers in the CPU cycle following clearing the
// ON bit.
//
    nop();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic32_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int pic32_wdt_ping(struct watchdog_device *wdd)
    {
    struct pic32_wdt *wdt = watchdog_get_drvdata(wdd);
    pic32_wdt_keepalive(wdt);
    return 0;
    }
    static const struct watchdog_ops pic32_wdt_fops = {
    .owner		= THIS_MODULE,
    .start		= pic32_wdt_start,
    .stop		= pic32_wdt_stop,
    .ping		= pic32_wdt_ping,
    };
    static const struct watchdog_info pic32_wdt_ident = {
    .options = WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE | WDIOF_CARDRESET,
    .identity = "PIC32 Watchdog",
    };
    static struct watchdog_device pic32_wdd = {
    .info		= &pic32_wdt_ident,
    .ops		= &pic32_wdt_fops,
    };
    static const struct of_device_id pic32_wdt_dt_ids[] = {
    { .compatible = "microchip,pic32mzda-wdt", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, pic32_wdt_dt_ids);
#[no_mangle]
unsafe extern "C" fn pic32_wdt_drv_probe(pdev: *mut platform_device) -> c_int {
    static int pic32_wdt_drv_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int ret;
    struct watchdog_device *wdd = &pic32_wdd;
    struct pic32_wdt *wdt;
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    wdt.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(wdt.regs))
    return PTR_ERR(wdt.regs);
    wdt.rst_base = devm_ioremap(dev, PIC32_BASE_RESET, 0x10);
    if (!wdt.rst_base)
    return -ENOMEM;
    wdt.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(wdt.clk)) {
    dev_err(dev, "clk not found\n");
    return PTR_ERR(wdt.clk);
    }
    if (pic32_wdt_is_win_enabled(wdt)) {
    dev_err(dev, "windowed-clear mode is not supported.\n");
    return -ENODEV;
    }
    wdd.timeout = pic32_wdt_get_timeout_secs(wdt, dev);
    if (!wdd.timeout) {
    dev_err(dev, "failed to read watchdog register timeout\n");
    return -EINVAL;
    }
    dev_info(dev, "timeout %d\n", wdd.timeout);
    wdd.bootstatus = pic32_wdt_bootstatus(wdt) ? WDIOF_CARDRESET : 0;
    watchdog_set_nowayout(wdd, WATCHDOG_NOWAYOUT);
    watchdog_set_drvdata(wdd, wdt);
    ret = devm_watchdog_register_device(dev, wdd);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, wdd);
    return 0;
    }
    static struct platform_driver pic32_wdt_driver = {
    .probe		= pic32_wdt_drv_probe,
    .driver		= {
    .name		= "pic32-wdt",
    .of_match_table = of_match_ptr(pic32_wdt_dt_ids),
    }
    };
    module_platform_driver(pic32_wdt_driver);
    MODULE_AUTHOR("Joshua Henderson <joshua.henderson@microchip.com>");
    MODULE_DESCRIPTION("Microchip PIC32 Watchdog Timer");
    MODULE_LICENSE("GPL");
