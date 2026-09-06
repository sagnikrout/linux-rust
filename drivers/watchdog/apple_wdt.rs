//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/apple_wdt.c
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple SoC Watchdog driver
//
// Copyright (C) The Asahi Linux Contributors
//

//
// Apple Watchdog MMIO registers
//
// This HW block has three separate watchdogs. WD0 resets the machine
// to recovery mode and is not very useful for us. WD1 and WD2 trigger a normal
// machine reset. WD0 additionally supports a configurable interrupt.
// This information can be used to implement pretimeout support at a later time.
//
// APPLE_WDT_WDx_CUR_TIME is a simple counter incremented for each tick of the
// reference clock. It can also be overwritten to any value.
// Whenever APPLE_WDT_CTRL_RESET_EN is set in APPLE_WDT_WDx_CTRL and
// APPLE_WDT_WDx_CUR_TIME >= APPLE_WDT_WDx_BITE_TIME the entire machine is
// reset.
// Whenever APPLE_WDT_CTRL_IRQ_EN is set and APPLE_WDTx_WD1_CUR_TIME >=
// APPLE_WDTx_WD1_BARK_TIME an interrupt is triggered and
// APPLE_WDT_CTRL_IRQ_STATUS is set. The interrupt can be cleared by writing
// 1 to APPLE_WDT_CTRL_IRQ_STATUS.
//
pub const APPLE_WDT_WD0_CUR_TIME: c_uint = 0x00;
pub const APPLE_WDT_WD0_BITE_TIME: c_uint = 0x04;
pub const APPLE_WDT_WD0_BARK_TIME: c_uint = 0x08;
pub const APPLE_WDT_WD0_CTRL: c_uint = 0x0c;
pub const APPLE_WDT_WD1_CUR_TIME: c_uint = 0x10;
pub const APPLE_WDT_WD1_BITE_TIME: c_uint = 0x14;
pub const APPLE_WDT_WD1_CTRL: c_uint = 0x1c;
pub const APPLE_WDT_WD2_CUR_TIME: c_uint = 0x20;
pub const APPLE_WDT_WD2_BITE_TIME: c_uint = 0x24;
pub const APPLE_WDT_WD2_CTRL: c_uint = 0x2c;

pub const APPLE_WDT_TIMEOUT_DEFAULT: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_wdt {
    pub wdd: watchdog_device,
    pub regs: *mut void __iomem,
    pub clk_rate: c_ulong,
}

    static struct apple_wdt *to_apple_wdt(struct watchdog_device *wdd)
    {
    return container_of(wdd, struct apple_wdt, wdd);
    }
#[no_mangle]
unsafe extern "C" fn apple_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int apple_wdt_start(struct watchdog_device *wdd)
    {
    struct apple_wdt *wdt = to_apple_wdt(wdd);
    writel_relaxed(0, wdt.regs + APPLE_WDT_WD1_CUR_TIME);
    writel_relaxed(APPLE_WDT_CTRL_RESET_EN, wdt.regs + APPLE_WDT_WD1_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int apple_wdt_stop(struct watchdog_device *wdd)
    {
    struct apple_wdt *wdt = to_apple_wdt(wdd);
    writel_relaxed(0, wdt.regs + APPLE_WDT_WD1_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int apple_wdt_ping(struct watchdog_device *wdd)
    {
    struct apple_wdt *wdt = to_apple_wdt(wdd);
    writel_relaxed(0, wdt.regs + APPLE_WDT_WD1_CUR_TIME);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_wdt_set_timeout(wdd: *mut watchdog_device, s: c_uint) -> c_int {
    static int apple_wdt_set_timeout(struct watchdog_device *wdd, unsigned int s)
    {
    struct apple_wdt *wdt = to_apple_wdt(wdd);
    u32 actual;
    writel_relaxed(0, wdt.regs + APPLE_WDT_WD1_CUR_TIME);
    actual = min(s, wdd.max_hw_heartbeat_ms / 1000);
    writel_relaxed(wdt.clk_rate * actual, wdt.regs + APPLE_WDT_WD1_BITE_TIME);
    wdd.timeout = s;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_wdt_get_timeleft(wdd: *mut watchdog_device) -> c_uint {
    static unsigned int apple_wdt_get_timeleft(struct watchdog_device *wdd)
    {
    struct apple_wdt *wdt = to_apple_wdt(wdd);
    u32 cur_time, reset_time;
    cur_time = readl_relaxed(wdt.regs + APPLE_WDT_WD1_CUR_TIME);
    reset_time = readl_relaxed(wdt.regs + APPLE_WDT_WD1_BITE_TIME);
    return (reset_time - cur_time) / wdt.clk_rate;
    }
    static int apple_wdt_restart(struct watchdog_device *wdd, unsigned long mode,
    void *cmd)
    {
    struct apple_wdt *wdt = to_apple_wdt(wdd);
    writel_relaxed(APPLE_WDT_CTRL_RESET_EN, wdt.regs + APPLE_WDT_WD1_CTRL);
    writel_relaxed(0, wdt.regs + APPLE_WDT_WD1_BITE_TIME);
    writel_relaxed(0, wdt.regs + APPLE_WDT_WD1_CUR_TIME);
//
// Flush writes and then wait for the SoC to reset. Even though the
// reset is queued almost immediately experiments have shown that it
// can take up to ~120-125ms until the SoC is actually reset. Just
// wait 150ms here to be safe.
//
    (void)readl(wdt.regs + APPLE_WDT_WD1_CUR_TIME);
    mdelay(150);
    return 0;
    }
    static const struct watchdog_ops apple_wdt_ops = {
    .owner = THIS_MODULE,
    .start = apple_wdt_start,
    .stop = apple_wdt_stop,
    .ping = apple_wdt_ping,
    .set_timeout = apple_wdt_set_timeout,
    .get_timeleft = apple_wdt_get_timeleft,
    .restart = apple_wdt_restart,
    };
    static const struct watchdog_info apple_wdt_info = {
    .identity = "Apple SoC Watchdog",
    .options = WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING | WDIOF_SETTIMEOUT,
    };
#[no_mangle]
unsafe extern "C" fn apple_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int apple_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct apple_wdt *wdt;
    struct clk *clk;
    u32 wdt_ctrl;
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    wdt.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(wdt.regs))
    return PTR_ERR(wdt.regs);
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    wdt.clk_rate = clk_get_rate(clk);
    if (!wdt.clk_rate)
    return -EINVAL;
    platform_set_drvdata(pdev, wdt);
    wdt.wdd.ops = &apple_wdt_ops;
    wdt.wdd.info = &apple_wdt_info;
    wdt.wdd.max_hw_heartbeat_ms = U32_MAX / wdt.clk_rate * 1000;
    wdt.wdd.timeout = APPLE_WDT_TIMEOUT_DEFAULT;
    wdt_ctrl = readl_relaxed(wdt.regs + APPLE_WDT_WD1_CTRL);
    if (wdt_ctrl & APPLE_WDT_CTRL_RESET_EN)
    set_bit(WDOG_HW_RUNNING, &wdt.wdd.status);
    watchdog_init_timeout(&wdt.wdd, 0, dev);
    apple_wdt_set_timeout(&wdt.wdd, wdt.wdd.timeout);
    watchdog_stop_on_unregister(&wdt.wdd);
    watchdog_set_restart_priority(&wdt.wdd, 128);
    return devm_watchdog_register_device(dev, &wdt.wdd);
    }
#[no_mangle]
unsafe extern "C" fn apple_wdt_resume(dev: *mut device) -> c_int {
    static int apple_wdt_resume(struct device *dev)
    {
    struct apple_wdt *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.wdd) || watchdog_hw_running(&wdt.wdd))
    apple_wdt_start(&wdt.wdd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_wdt_suspend(dev: *mut device) -> c_int {
    static int apple_wdt_suspend(struct device *dev)
    {
    struct apple_wdt *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.wdd) || watchdog_hw_running(&wdt.wdd))
    apple_wdt_stop(&wdt.wdd);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(apple_wdt_pm_ops, apple_wdt_suspend, apple_wdt_resume);
    static const struct of_device_id apple_wdt_of_match[] = {
    { .compatible = "apple,t8103-wdt" },
    { .compatible = "apple,wdt" },
    {},
    };
    MODULE_DEVICE_TABLE(of, apple_wdt_of_match);
    static struct platform_driver apple_wdt_driver = {
    .driver = {
    .name = "apple-watchdog",
    .of_match_table = apple_wdt_of_match,
    .pm = pm_sleep_ptr(&apple_wdt_pm_ops),
    },
    .probe = apple_wdt_probe,
    };
    module_platform_driver(apple_wdt_driver);
    MODULE_DESCRIPTION("Apple SoC watchdog driver");
    MODULE_AUTHOR("Sven Peter <sven@svenpeter.dev>");
    MODULE_LICENSE("Dual MIT/GPL");
