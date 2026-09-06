//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/sprd_wdt.c
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
// Spreadtrum watchdog driver
// Copyright (C) 2017 Spreadtrum - http://www.spreadtrum.com
//

pub const SPRD_WDT_LOAD_LOW: c_uint = 0x0;
pub const SPRD_WDT_LOAD_HIGH: c_uint = 0x4;
pub const SPRD_WDT_CTRL: c_uint = 0x8;
pub const SPRD_WDT_INT_CLR: c_uint = 0xc;
pub const SPRD_WDT_INT_RAW: c_uint = 0x10;
pub const SPRD_WDT_INT_MSK: c_uint = 0x14;
pub const SPRD_WDT_CNT_LOW: c_uint = 0x18;
pub const SPRD_WDT_CNT_HIGH: c_uint = 0x1c;
pub const SPRD_WDT_LOCK: c_uint = 0x20;
pub const SPRD_WDT_IRQ_LOAD_LOW: c_uint = 0x2c;
pub const SPRD_WDT_IRQ_LOAD_HIGH: c_uint = 0x30;
// WDT_CTRL

// WDT_INT_CLR

// WDT_INT_RAW

// 1s equal to 32768 counter steps
pub const SPRD_WDT_CNT_STEP: c_int = 32768;
pub const SPRD_WDT_UNLOCK_KEY: c_uint = 0xe551;
pub const SPRD_WDT_MIN_TIMEOUT: c_int = 3;
pub const SPRD_WDT_MAX_TIMEOUT: c_int = 60;
pub const SPRD_WDT_CNT_HIGH_SHIFT: c_int = 16;

pub const SPRD_WDT_LOAD_TIMEOUT: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_wdt {
    pub base: *mut void __iomem,
    pub wdd: watchdog_device,
    pub enable: *mut clk,
    pub rtc_enable: *mut clk,
    pub irq: c_int,
}

    static inline struct sprd_wdt *to_sprd_wdt(struct watchdog_device *wdd)
    {
    return container_of(wdd, struct sprd_wdt, wdd);
    }
#[no_mangle]
pub unsafe extern "C" fn sprd_wdt_lock(addr: *mut void __iomem) {
    static inline void sprd_wdt_lock(void __iomem *addr)
    {
    writel_relaxed(0x0, addr + SPRD_WDT_LOCK);
    }
#[no_mangle]
pub unsafe extern "C" fn sprd_wdt_unlock(addr: *mut void __iomem) {
    static inline void sprd_wdt_unlock(void __iomem *addr)
    {
    writel_relaxed(SPRD_WDT_UNLOCK_KEY, addr + SPRD_WDT_LOCK);
    }
#[no_mangle]
unsafe extern "C" fn sprd_wdt_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sprd_wdt_isr(int irq, void *dev_id)
    {
    struct sprd_wdt *wdt = (struct sprd_wdt *)dev_id;
    sprd_wdt_unlock(wdt.base);
    writel_relaxed(SPRD_WDT_INT_CLEAR_BIT, wdt.base + SPRD_WDT_INT_CLR);
    sprd_wdt_lock(wdt.base);
    watchdog_notify_pretimeout(&wdt.wdd);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sprd_wdt_get_cnt_value(wdt: *mut sprd_wdt) -> u32 {
    static u32 sprd_wdt_get_cnt_value(struct sprd_wdt *wdt)
    {
    u32 val;
    val = readl_relaxed(wdt.base + SPRD_WDT_CNT_HIGH) <<
    SPRD_WDT_CNT_HIGH_SHIFT;
    val |= readl_relaxed(wdt.base + SPRD_WDT_CNT_LOW) &
    SPRD_WDT_LOW_VALUE_MASK;
    return val;
    }
    static int sprd_wdt_load_value(struct sprd_wdt *wdt, u32 timeout,
    u32 pretimeout)
    {
    u32 val, delay_cnt = 0;
    let mut tmr_step: u32 = timeout * SPRD_WDT_CNT_STEP;
    let mut prtmr_step: u32 = pretimeout * SPRD_WDT_CNT_STEP;
//
// Checking busy bit to make sure the previous loading operation is
// done. According to the specification, the busy bit would be set
// after a new loading operation and last 2 or 3 RTC clock
// cycles (about 60us~92us).
//
    do {
    val = readl_relaxed(wdt.base + SPRD_WDT_INT_RAW);
    if (!(val & SPRD_WDT_LD_BUSY_BIT))
    break;
    usleep_range(10, 100);
    } while (delay_cnt++ < SPRD_WDT_LOAD_TIMEOUT);
    if (delay_cnt >= SPRD_WDT_LOAD_TIMEOUT)
    return -EBUSY;
    sprd_wdt_unlock(wdt.base);
    writel_relaxed((tmr_step >> SPRD_WDT_CNT_HIGH_SHIFT) &
    SPRD_WDT_LOW_VALUE_MASK, wdt.base + SPRD_WDT_LOAD_HIGH);
    writel_relaxed((tmr_step & SPRD_WDT_LOW_VALUE_MASK),
    wdt.base + SPRD_WDT_LOAD_LOW);
    writel_relaxed((prtmr_step >> SPRD_WDT_CNT_HIGH_SHIFT) &
    SPRD_WDT_LOW_VALUE_MASK,
    wdt.base + SPRD_WDT_IRQ_LOAD_HIGH);
    writel_relaxed(prtmr_step & SPRD_WDT_LOW_VALUE_MASK,
    wdt.base + SPRD_WDT_IRQ_LOAD_LOW);
    sprd_wdt_lock(wdt.base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_wdt_enable(wdt: *mut sprd_wdt) -> c_int {
    static int sprd_wdt_enable(struct sprd_wdt *wdt)
    {
    u32 val;
    int ret;
    ret = clk_prepare_enable(wdt.enable);
    if (ret)
    return ret;
    ret = clk_prepare_enable(wdt.rtc_enable);
    if (ret) {
    clk_disable_unprepare(wdt.enable);
    return ret;
    }
    sprd_wdt_unlock(wdt.base);
    val = readl_relaxed(wdt.base + SPRD_WDT_CTRL);
    val |= SPRD_WDT_NEW_VER_EN;
    writel_relaxed(val, wdt.base + SPRD_WDT_CTRL);
    sprd_wdt_lock(wdt.base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_wdt_disable(_data: *mut c_void) {
    static void sprd_wdt_disable(void *_data)
    {
    struct sprd_wdt *wdt = _data;
    sprd_wdt_unlock(wdt.base);
    writel_relaxed(0x0, wdt.base + SPRD_WDT_CTRL);
    sprd_wdt_lock(wdt.base);
    clk_disable_unprepare(wdt.rtc_enable);
    clk_disable_unprepare(wdt.enable);
    }
#[no_mangle]
unsafe extern "C" fn sprd_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int sprd_wdt_start(struct watchdog_device *wdd)
    {
    struct sprd_wdt *wdt = to_sprd_wdt(wdd);
    u32 val;
    int ret;
    ret = sprd_wdt_load_value(wdt, wdd.timeout, wdd.pretimeout);
    if (ret)
    return ret;
    sprd_wdt_unlock(wdt.base);
    val = readl_relaxed(wdt.base + SPRD_WDT_CTRL);
    val |= SPRD_WDT_CNT_EN_BIT | SPRD_WDT_INT_EN_BIT | SPRD_WDT_RST_EN_BIT;
    writel_relaxed(val, wdt.base + SPRD_WDT_CTRL);
    sprd_wdt_lock(wdt.base);
    set_bit(WDOG_HW_RUNNING, &wdd.status);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int sprd_wdt_stop(struct watchdog_device *wdd)
    {
    struct sprd_wdt *wdt = to_sprd_wdt(wdd);
    u32 val;
    sprd_wdt_unlock(wdt.base);
    val = readl_relaxed(wdt.base + SPRD_WDT_CTRL);
    val &= ~(SPRD_WDT_CNT_EN_BIT | SPRD_WDT_RST_EN_BIT |
    SPRD_WDT_INT_EN_BIT);
    writel_relaxed(val, wdt.base + SPRD_WDT_CTRL);
    sprd_wdt_lock(wdt.base);
    return 0;
    }
    static int sprd_wdt_set_timeout(struct watchdog_device *wdd,
    u32 timeout)
    {
    struct sprd_wdt *wdt = to_sprd_wdt(wdd);
    if (timeout == wdd.timeout)
    return 0;
    wdd.timeout = timeout;
    return sprd_wdt_load_value(wdt, timeout, wdd.pretimeout);
    }
    static int sprd_wdt_set_pretimeout(struct watchdog_device *wdd,
    u32 new_pretimeout)
    {
    struct sprd_wdt *wdt = to_sprd_wdt(wdd);
    if (new_pretimeout < wdd.min_timeout)
    return -EINVAL;
    wdd.pretimeout = new_pretimeout;
    return sprd_wdt_load_value(wdt, wdd.timeout, new_pretimeout);
    }
#[no_mangle]
unsafe extern "C" fn sprd_wdt_get_timeleft(wdd: *mut watchdog_device) -> u32 {
    static u32 sprd_wdt_get_timeleft(struct watchdog_device *wdd)
    {
    struct sprd_wdt *wdt = to_sprd_wdt(wdd);
    u32 val;
    val = sprd_wdt_get_cnt_value(wdt);
    return val / SPRD_WDT_CNT_STEP;
    }
    static const struct watchdog_ops sprd_wdt_ops = {
    .owner = THIS_MODULE,
    .start = sprd_wdt_start,
    .stop = sprd_wdt_stop,
    .set_timeout = sprd_wdt_set_timeout,
    .set_pretimeout = sprd_wdt_set_pretimeout,
    .get_timeleft = sprd_wdt_get_timeleft,
    };
    static const struct watchdog_info sprd_wdt_info = {
    .options = WDIOF_SETTIMEOUT |
    WDIOF_PRETIMEOUT |
    WDIOF_MAGICCLOSE |
    WDIOF_KEEPALIVEPING,
    .identity = "Spreadtrum Watchdog Timer",
    };
#[no_mangle]
unsafe extern "C" fn sprd_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int sprd_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct sprd_wdt *wdt;
    int ret;
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    wdt.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(wdt.base))
    return PTR_ERR(wdt.base);
    wdt.enable = devm_clk_get(dev, "enable");
    if (IS_ERR(wdt.enable)) {
    dev_err(dev, "can't get the enable clock\n");
    return PTR_ERR(wdt.enable);
    }
    wdt.rtc_enable = devm_clk_get(dev, "rtc_enable");
    if (IS_ERR(wdt.rtc_enable)) {
    dev_err(dev, "can't get the rtc enable clock\n");
    return PTR_ERR(wdt.rtc_enable);
    }
    wdt.irq = platform_get_irq(pdev, 0);
    if (wdt.irq < 0)
    return wdt.irq;
    ret = devm_request_irq(dev, wdt.irq, sprd_wdt_isr, IRQF_NO_SUSPEND,
    "sprd-wdt", (void *)wdt);
    if (ret)
    return ret;
    wdt.wdd.info = &sprd_wdt_info;
    wdt.wdd.ops = &sprd_wdt_ops;
    wdt.wdd.parent = dev;
    wdt.wdd.min_timeout = SPRD_WDT_MIN_TIMEOUT;
    wdt.wdd.max_timeout = SPRD_WDT_MAX_TIMEOUT;
    wdt.wdd.timeout = SPRD_WDT_MAX_TIMEOUT;
    ret = sprd_wdt_enable(wdt);
    if (ret) {
    dev_err(dev, "failed to enable wdt\n");
    return ret;
    }
    ret = devm_add_action_or_reset(dev, sprd_wdt_disable, wdt);
    if (ret) {
    dev_err(dev, "Failed to add wdt disable action\n");
    return ret;
    }
    watchdog_set_nowayout(&wdt.wdd, WATCHDOG_NOWAYOUT);
    watchdog_init_timeout(&wdt.wdd, 0, dev);
    ret = devm_watchdog_register_device(dev, &wdt.wdd);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, wdt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_wdt_pm_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused sprd_wdt_pm_suspend(struct device *dev)
    {
    struct sprd_wdt *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.wdd))
    sprd_wdt_stop(&wdt.wdd);
    sprd_wdt_disable(wdt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_wdt_pm_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused sprd_wdt_pm_resume(struct device *dev)
    {
    struct sprd_wdt *wdt = dev_get_drvdata(dev);
    int ret;
    ret = sprd_wdt_enable(wdt);
    if (ret)
    return ret;
    if (watchdog_active(&wdt.wdd))
    ret = sprd_wdt_start(&wdt.wdd);
    return ret;
    }
    static const struct dev_pm_ops sprd_wdt_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(sprd_wdt_pm_suspend,
    sprd_wdt_pm_resume)
    };
    static const struct of_device_id sprd_wdt_match_table[] = {
    { .compatible = "sprd,sp9860-wdt", },
    {},
    };
    MODULE_DEVICE_TABLE(of, sprd_wdt_match_table);
    static struct platform_driver sprd_watchdog_driver = {
    .probe	= sprd_wdt_probe,
    .driver	= {
    .name = "sprd-wdt",
    .of_match_table = sprd_wdt_match_table,
    .pm = &sprd_wdt_pm_ops,
    },
    };
    module_platform_driver(sprd_watchdog_driver);
    MODULE_AUTHOR("Eric Long <eric.long@spreadtrum.com>");
    MODULE_DESCRIPTION("Spreadtrum Watchdog Timer Controller Driver");
    MODULE_LICENSE("GPL v2");
