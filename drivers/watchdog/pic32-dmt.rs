//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/pic32-dmt.c
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
// PIC32 deadman timer driver
//
// Purna Chandra Mandal <purna.mandal@microchip.com>
// Copyright (c) 2016, Microchip Technology Inc.
//

// Deadman Timer Regs
pub const DMTCON_REG: c_uint = 0x00;
pub const DMTPRECLR_REG: c_uint = 0x10;
pub const DMTCLR_REG: c_uint = 0x20;
pub const DMTSTAT_REG: c_uint = 0x30;
pub const DMTCNT_REG: c_uint = 0x40;
pub const DMTPSCNT_REG: c_uint = 0x60;
pub const DMTPSINTV_REG: c_uint = 0x70;
// Deadman Timer Regs fields

// Reset Control Register fields for watchdog

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_dmt {
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
}

#[no_mangle]
pub unsafe extern "C" fn dmt_enable(dmt: *mut pic32_dmt) {
    static inline void dmt_enable(struct pic32_dmt *dmt)
    {
    writel(DMT_ON, PIC32_SET(dmt.regs + DMTCON_REG));
    }
#[no_mangle]
pub unsafe extern "C" fn dmt_disable(dmt: *mut pic32_dmt) {
    static inline void dmt_disable(struct pic32_dmt *dmt)
    {
    writel(DMT_ON, PIC32_CLR(dmt.regs + DMTCON_REG));
//
// Cannot touch registers in the CPU cycle following clearing the
// ON bit.
//
    nop();
    }
#[no_mangle]
pub unsafe extern "C" fn dmt_bad_status(dmt: *mut pic32_dmt) -> c_int {
    static inline int dmt_bad_status(struct pic32_dmt *dmt)
    {
    u32 val;
    val = readl(dmt.regs + DMTSTAT_REG);
    val &= (DMTSTAT_BAD1 | DMTSTAT_BAD2 | DMTSTAT_EVENT);
    if (val)
    return -EAGAIN;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dmt_keepalive(dmt: *mut pic32_dmt) -> c_int {
    static inline int dmt_keepalive(struct pic32_dmt *dmt)
    {
    u32 v;
    let mut timeout: u32 = 500;
// set pre-clear key
    writel(DMT_STEP1_KEY << 8, dmt.regs + DMTPRECLR_REG);
// wait for DMT window to open
    while (--timeout) {
    v = readl(dmt.regs + DMTSTAT_REG) & DMTSTAT_WINOPN;
    if (v == DMTSTAT_WINOPN)
    break;
    }
// apply key2
    writel(DMT_STEP2_KEY, dmt.regs + DMTCLR_REG);
// check whether keys are latched correctly
    return dmt_bad_status(dmt);
    }
#[no_mangle]
pub unsafe extern "C" fn pic32_dmt_get_timeout_secs(dmt: *mut pic32_dmt) -> u32 {
    static inline u32 pic32_dmt_get_timeout_secs(struct pic32_dmt *dmt)
    {
    unsigned long rate;
    rate = clk_get_rate(dmt.clk);
    if (rate)
    return readl(dmt.regs + DMTPSCNT_REG) / rate;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pic32_dmt_bootstatus(dmt: *mut pic32_dmt) -> u32 {
    static inline u32 pic32_dmt_bootstatus(struct pic32_dmt *dmt)
    {
    u32 v;
    void __iomem *rst_base;
    rst_base = ioremap(PIC32_BASE_RESET, 0x10);
    if (!rst_base)
    return 0;
    v = readl(rst_base);
    writel(RESETCON_DMT_TIMEOUT, PIC32_CLR(rst_base));
    iounmap(rst_base);
    return v & RESETCON_DMT_TIMEOUT;
    }
#[no_mangle]
unsafe extern "C" fn pic32_dmt_start(wdd: *mut watchdog_device) -> c_int {
    static int pic32_dmt_start(struct watchdog_device *wdd)
    {
    struct pic32_dmt *dmt = watchdog_get_drvdata(wdd);
    dmt_enable(dmt);
    return dmt_keepalive(dmt);
    }
#[no_mangle]
unsafe extern "C" fn pic32_dmt_stop(wdd: *mut watchdog_device) -> c_int {
    static int pic32_dmt_stop(struct watchdog_device *wdd)
    {
    struct pic32_dmt *dmt = watchdog_get_drvdata(wdd);
    dmt_disable(dmt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic32_dmt_ping(wdd: *mut watchdog_device) -> c_int {
    static int pic32_dmt_ping(struct watchdog_device *wdd)
    {
    struct pic32_dmt *dmt = watchdog_get_drvdata(wdd);
    return dmt_keepalive(dmt);
    }
    static const struct watchdog_ops pic32_dmt_fops = {
    .owner		= THIS_MODULE,
    .start		= pic32_dmt_start,
    .stop		= pic32_dmt_stop,
    .ping		= pic32_dmt_ping,
    };
    static const struct watchdog_info pic32_dmt_ident = {
    .options	= WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE,
    .identity	= "PIC32 Deadman Timer",
    };
    static struct watchdog_device pic32_dmt_wdd = {
    .info		= &pic32_dmt_ident,
    .ops		= &pic32_dmt_fops,
    };
#[no_mangle]
unsafe extern "C" fn pic32_dmt_probe(pdev: *mut platform_device) -> c_int {
    static int pic32_dmt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int ret;
    struct pic32_dmt *dmt;
    struct watchdog_device *wdd = &pic32_dmt_wdd;
    dmt = devm_kzalloc(dev, sizeof(*dmt), GFP_KERNEL);
    if (!dmt)
    return -ENOMEM;
    dmt.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(dmt.regs))
    return PTR_ERR(dmt.regs);
    dmt.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(dmt.clk)) {
    dev_err(dev, "clk not found\n");
    return PTR_ERR(dmt.clk);
    }
    wdd.timeout = pic32_dmt_get_timeout_secs(dmt);
    if (!wdd.timeout) {
    dev_err(dev, "failed to read watchdog register timeout\n");
    return -EINVAL;
    }
    dev_info(dev, "timeout %d\n", wdd.timeout);
    wdd.bootstatus = pic32_dmt_bootstatus(dmt) ? WDIOF_CARDRESET : 0;
    watchdog_set_nowayout(wdd, WATCHDOG_NOWAYOUT);
    watchdog_set_drvdata(wdd, dmt);
    ret = devm_watchdog_register_device(dev, wdd);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, wdd);
    return 0;
    }
    static const struct of_device_id pic32_dmt_of_ids[] = {
    { .compatible = "microchip,pic32mzda-dmt",},
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, pic32_dmt_of_ids);
    static struct platform_driver pic32_dmt_driver = {
    .probe		= pic32_dmt_probe,
    .driver		= {
    .name		= "pic32-dmt",
    .of_match_table = of_match_ptr(pic32_dmt_of_ids),
    }
    };
    module_platform_driver(pic32_dmt_driver);
    MODULE_AUTHOR("Purna Chandra Mandal <purna.mandal@microchip.com>");
    MODULE_DESCRIPTION("Microchip PIC32 DMT Driver");
    MODULE_LICENSE("GPL");
