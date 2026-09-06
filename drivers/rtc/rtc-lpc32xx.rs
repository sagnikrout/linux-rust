//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-lpc32xx.c
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
// Copyright (C) 2010 NXP Semiconductors
//

//
// Clock and Power control register offsets
//
pub const LPC32XX_RTC_UCOUNT: c_uint = 0x00;
pub const LPC32XX_RTC_DCOUNT: c_uint = 0x04;
pub const LPC32XX_RTC_MATCH0: c_uint = 0x08;
pub const LPC32XX_RTC_MATCH1: c_uint = 0x0C;
pub const LPC32XX_RTC_CTRL: c_uint = 0x10;
pub const LPC32XX_RTC_INTSTAT: c_uint = 0x14;
pub const LPC32XX_RTC_KEY: c_uint = 0x18;
pub const LPC32XX_RTC_SRAM: c_uint = 0x80;

pub const LPC32XX_RTC_KEY_ONSW_LOADVAL: c_uint = 0xB5C13F27;

    __raw_readl((dev).rtc_base + (reg))

    __raw_writel((val), (dev).rtc_base + (reg))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_rtc {
    pub rtc_base: *mut void __iomem,
    pub irq: c_int,
    pub alarm_enabled: c_uchar,
    pub rtc: *mut rtc_device,
    pub lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn lpc32xx_rtc_read_time(dev: *mut device, time: *mut rtc_time) -> c_int {
    static int lpc32xx_rtc_read_time(struct device *dev, struct rtc_time *time)
    {
    unsigned long elapsed_sec;
    struct lpc32xx_rtc *rtc = dev_get_drvdata(dev);
    elapsed_sec = rtc_readl(rtc, LPC32XX_RTC_UCOUNT);
    rtc_time64_to_tm(elapsed_sec, time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_rtc_set_time(dev: *mut device, time: *mut rtc_time) -> c_int {
    static int lpc32xx_rtc_set_time(struct device *dev, struct rtc_time *time)
    {
    struct lpc32xx_rtc *rtc = dev_get_drvdata(dev);
    let mut secs: u32 = rtc_tm_to_time64(time);
    u32 tmp;
    spin_lock_irq(&rtc.lock);
// RTC must be disabled during count update
    tmp = rtc_readl(rtc, LPC32XX_RTC_CTRL);
    rtc_writel(rtc, LPC32XX_RTC_CTRL, tmp | LPC32XX_RTC_CTRL_CNTR_DIS);
    rtc_writel(rtc, LPC32XX_RTC_UCOUNT, secs);
    rtc_writel(rtc, LPC32XX_RTC_DCOUNT, 0xFFFFFFFF - secs);
    rtc_writel(rtc, LPC32XX_RTC_CTRL, tmp &= ~LPC32XX_RTC_CTRL_CNTR_DIS);
    spin_unlock_irq(&rtc.lock);
    return 0;
    }
    static int lpc32xx_rtc_read_alarm(struct device *dev,
    struct rtc_wkalrm *wkalrm)
    {
    struct lpc32xx_rtc *rtc = dev_get_drvdata(dev);
    rtc_time64_to_tm(rtc_readl(rtc, LPC32XX_RTC_MATCH0), &wkalrm.time);
    wkalrm.enabled = rtc.alarm_enabled;
    wkalrm.pending = !!(rtc_readl(rtc, LPC32XX_RTC_INTSTAT) &
    LPC32XX_RTC_INTSTAT_MATCH0);
    return rtc_valid_tm(&wkalrm.time);
    }
    static int lpc32xx_rtc_set_alarm(struct device *dev,
    struct rtc_wkalrm *wkalrm)
    {
    struct lpc32xx_rtc *rtc = dev_get_drvdata(dev);
    unsigned long alarmsecs;
    u32 tmp;
    alarmsecs = rtc_tm_to_time64(&wkalrm.time);
    spin_lock_irq(&rtc.lock);
// Disable alarm during update
    tmp = rtc_readl(rtc, LPC32XX_RTC_CTRL);
    rtc_writel(rtc, LPC32XX_RTC_CTRL, tmp & ~LPC32XX_RTC_CTRL_MATCH0);
    rtc_writel(rtc, LPC32XX_RTC_MATCH0, alarmsecs);
    rtc.alarm_enabled = wkalrm.enabled;
    if (wkalrm.enabled) {
    rtc_writel(rtc, LPC32XX_RTC_INTSTAT,
    LPC32XX_RTC_INTSTAT_MATCH0);
    rtc_writel(rtc, LPC32XX_RTC_CTRL, tmp |
    LPC32XX_RTC_CTRL_MATCH0);
    }
    spin_unlock_irq(&rtc.lock);
    return 0;
    }
    static int lpc32xx_rtc_alarm_irq_enable(struct device *dev,
    unsigned int enabled)
    {
    struct lpc32xx_rtc *rtc = dev_get_drvdata(dev);
    u32 tmp;
    spin_lock_irq(&rtc.lock);
    tmp = rtc_readl(rtc, LPC32XX_RTC_CTRL);
    if (enabled) {
    rtc.alarm_enabled = 1;
    tmp |= LPC32XX_RTC_CTRL_MATCH0;
    } else {
    rtc.alarm_enabled = 0;
    tmp &= ~LPC32XX_RTC_CTRL_MATCH0;
    }
    rtc_writel(rtc, LPC32XX_RTC_CTRL, tmp);
    spin_unlock_irq(&rtc.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_rtc_alarm_interrupt(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t lpc32xx_rtc_alarm_interrupt(int irq, void *dev)
    {
    struct lpc32xx_rtc *rtc = dev;
    spin_lock(&rtc.lock);
// Disable alarm interrupt
    rtc_writel(rtc, LPC32XX_RTC_CTRL,
    rtc_readl(rtc, LPC32XX_RTC_CTRL) &
    ~LPC32XX_RTC_CTRL_MATCH0);
    rtc.alarm_enabled = 0;
//
// Write a large value to the match value so the RTC won't
// keep firing the match status
//
    rtc_writel(rtc, LPC32XX_RTC_MATCH0, 0xFFFFFFFF);
    rtc_writel(rtc, LPC32XX_RTC_INTSTAT, LPC32XX_RTC_INTSTAT_MATCH0);
    spin_unlock(&rtc.lock);
    rtc_update_irq(rtc.rtc, 1, RTC_IRQF | RTC_AF);
    return IRQ_HANDLED;
    }
    static const struct rtc_class_ops lpc32xx_rtc_ops = {
    .read_time		= lpc32xx_rtc_read_time,
    .set_time		= lpc32xx_rtc_set_time,
    .read_alarm		= lpc32xx_rtc_read_alarm,
    .set_alarm		= lpc32xx_rtc_set_alarm,
    .alarm_irq_enable	= lpc32xx_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn lpc32xx_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int lpc32xx_rtc_probe(struct platform_device *pdev)
    {
    struct lpc32xx_rtc *rtc;
    int err;
    u32 tmp;
    rtc = devm_kzalloc(&pdev.dev, sizeof(*rtc), GFP_KERNEL);
    if (unlikely(!rtc))
    return -ENOMEM;
    rtc.rtc_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rtc.rtc_base))
    return PTR_ERR(rtc.rtc_base);
    spin_lock_init(&rtc.lock);
//
// The RTC is on a separate power domain and can keep it's state
// across a chip power cycle. If the RTC has never been previously
// setup, then set it up now for the first time.
//
    tmp = rtc_readl(rtc, LPC32XX_RTC_CTRL);
    if (rtc_readl(rtc, LPC32XX_RTC_KEY) != LPC32XX_RTC_KEY_ONSW_LOADVAL) {
    tmp &= ~(LPC32XX_RTC_CTRL_SW_RESET |
    LPC32XX_RTC_CTRL_CNTR_DIS |
    LPC32XX_RTC_CTRL_MATCH0 |
    LPC32XX_RTC_CTRL_MATCH1 |
    LPC32XX_RTC_CTRL_ONSW_MATCH0 |
    LPC32XX_RTC_CTRL_ONSW_MATCH1 |
    LPC32XX_RTC_CTRL_ONSW_FORCE_HI);
    rtc_writel(rtc, LPC32XX_RTC_CTRL, tmp);
// Clear latched interrupt states
    rtc_writel(rtc, LPC32XX_RTC_MATCH0, 0xFFFFFFFF);
    rtc_writel(rtc, LPC32XX_RTC_INTSTAT,
    LPC32XX_RTC_INTSTAT_MATCH0 |
    LPC32XX_RTC_INTSTAT_MATCH1 |
    LPC32XX_RTC_INTSTAT_ONSW);
// Write key value to RTC so it won't reload on reset
    rtc_writel(rtc, LPC32XX_RTC_KEY,
    LPC32XX_RTC_KEY_ONSW_LOADVAL);
    } else {
    rtc_writel(rtc, LPC32XX_RTC_CTRL,
    tmp & ~LPC32XX_RTC_CTRL_MATCH0);
    }
    platform_set_drvdata(pdev, rtc);
    rtc.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc.rtc))
    return PTR_ERR(rtc.rtc);
    rtc.rtc.ops = &lpc32xx_rtc_ops;
    rtc.rtc.range_max = U32_MAX;
    err = devm_rtc_register_device(rtc.rtc);
    if (err)
    return err;
//
// IRQ is enabled after device registration in case alarm IRQ
// is pending upon suspend exit.
//
    rtc.irq = platform_get_irq(pdev, 0);
    if (rtc.irq < 0) {
    dev_warn(&pdev.dev, "Can't get interrupt resource\n");
    } else {
    if (devm_request_irq(&pdev.dev, rtc.irq,
    lpc32xx_rtc_alarm_interrupt,
    0, pdev.name, rtc) < 0) {
    dev_warn(&pdev.dev, "Can't request interrupt.\n");
    rtc.irq = -1;
    } else {
    device_init_wakeup(&pdev.dev, true);
    }
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn lpc32xx_rtc_suspend(dev: *mut device) -> c_int {
    static int lpc32xx_rtc_suspend(struct device *dev)
    {
    struct lpc32xx_rtc *rtc = dev_get_drvdata(dev);
    if (rtc.irq >= 0) {
    if (device_may_wakeup(dev))
    enable_irq_wake(rtc.irq);
    else
    disable_irq_wake(rtc.irq);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_rtc_resume(dev: *mut device) -> c_int {
    static int lpc32xx_rtc_resume(struct device *dev)
    {
    struct lpc32xx_rtc *rtc = dev_get_drvdata(dev);
    if (rtc.irq >= 0 && device_may_wakeup(dev))
    disable_irq_wake(rtc.irq);
    return 0;
    }
// Unconditionally disable the alarm
#[no_mangle]
unsafe extern "C" fn lpc32xx_rtc_freeze(dev: *mut device) -> c_int {
    static int lpc32xx_rtc_freeze(struct device *dev)
    {
    struct lpc32xx_rtc *rtc = dev_get_drvdata(dev);
    spin_lock_irq(&rtc.lock);
    rtc_writel(rtc, LPC32XX_RTC_CTRL,
    rtc_readl(rtc, LPC32XX_RTC_CTRL) &
    ~LPC32XX_RTC_CTRL_MATCH0);
    spin_unlock_irq(&rtc.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_rtc_thaw(dev: *mut device) -> c_int {
    static int lpc32xx_rtc_thaw(struct device *dev)
    {
    struct lpc32xx_rtc *rtc = dev_get_drvdata(dev);
    if (rtc.alarm_enabled) {
    spin_lock_irq(&rtc.lock);
    rtc_writel(rtc, LPC32XX_RTC_CTRL,
    rtc_readl(rtc, LPC32XX_RTC_CTRL) |
    LPC32XX_RTC_CTRL_MATCH0);
    spin_unlock_irq(&rtc.lock);
    }
    return 0;
    }
    static const struct dev_pm_ops lpc32xx_rtc_pm_ops = {
    .suspend = lpc32xx_rtc_suspend,
    .resume = lpc32xx_rtc_resume,
    .freeze = lpc32xx_rtc_freeze,
    .thaw = lpc32xx_rtc_thaw,
    .restore = lpc32xx_rtc_resume
    };

    static const struct of_device_id lpc32xx_rtc_match[] = {
    { .compatible = "nxp,lpc3220-rtc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, lpc32xx_rtc_match);

    static struct platform_driver lpc32xx_rtc_driver = {
    .probe		= lpc32xx_rtc_probe,
    .driver = {
    .name	= "rtc-lpc32xx",
    .pm	= LPC32XX_RTC_PM_OPS,
    .of_match_table = of_match_ptr(lpc32xx_rtc_match),
    },
    };
    module_platform_driver(lpc32xx_rtc_driver);
    MODULE_AUTHOR("Kevin Wells <wellsk40@gmail.com");
    MODULE_DESCRIPTION("RTC driver for the LPC32xx SoC");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:rtc-lpc32xx");
