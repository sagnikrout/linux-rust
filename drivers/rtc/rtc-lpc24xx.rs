//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-lpc24xx.c
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
// RTC driver for NXP LPC178x/18xx/43xx Real-Time Clock (RTC)
//
// Copyright (C) 2011 NXP Semiconductors
// Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
//

// LPC24xx RTC register offsets and bits
pub const LPC24XX_ILR: c_uint = 0x00;

pub const LPC24XX_CTC: c_uint = 0x04;
pub const LPC24XX_CCR: c_uint = 0x08;

pub const LPC24XX_CIIR: c_uint = 0x0c;
pub const LPC24XX_AMR: c_uint = 0x10;
pub const LPC24XX_ALARM_DISABLE: c_uint = 0xff;
pub const LPC24XX_CTIME0: c_uint = 0x14;
pub const LPC24XX_CTIME1: c_uint = 0x18;
pub const LPC24XX_CTIME2: c_uint = 0x1c;
pub const LPC24XX_SEC: c_uint = 0x20;
pub const LPC24XX_MIN: c_uint = 0x24;
pub const LPC24XX_HOUR: c_uint = 0x28;
pub const LPC24XX_DOM: c_uint = 0x2c;
pub const LPC24XX_DOW: c_uint = 0x30;
pub const LPC24XX_DOY: c_uint = 0x34;
pub const LPC24XX_MONTH: c_uint = 0x38;
pub const LPC24XX_YEAR: c_uint = 0x3c;
pub const LPC24XX_ALSEC: c_uint = 0x60;
pub const LPC24XX_ALMIN: c_uint = 0x64;
pub const LPC24XX_ALHOUR: c_uint = 0x68;
pub const LPC24XX_ALDOM: c_uint = 0x6c;
pub const LPC24XX_ALDOW: c_uint = 0x70;
pub const LPC24XX_ALDOY: c_uint = 0x74;
pub const LPC24XX_ALMON: c_uint = 0x78;
pub const LPC24XX_ALYEAR: c_uint = 0x7c;
// Macros to read fields in consolidated time (CT) registers

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc24xx_rtc {
    pub rtc_base: *mut void __iomem,
    pub rtc: *mut rtc_device,
    pub clk_rtc: *mut clk,
    pub clk_reg: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn lpc24xx_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int lpc24xx_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct lpc24xx_rtc *rtc = dev_get_drvdata(dev);
// Disable RTC during update
    rtc_writel(rtc, LPC24XX_CCR, LPC178X_CCALEN);
    rtc_writel(rtc, LPC24XX_SEC,	tm.tm_sec);
    rtc_writel(rtc, LPC24XX_MIN,	tm.tm_min);
    rtc_writel(rtc, LPC24XX_HOUR,	tm.tm_hour);
    rtc_writel(rtc, LPC24XX_DOW,	tm.tm_wday);
    rtc_writel(rtc, LPC24XX_DOM,	tm.tm_mday);
    rtc_writel(rtc, LPC24XX_DOY,	tm.tm_yday);
    rtc_writel(rtc, LPC24XX_MONTH,	tm.tm_mon);
    rtc_writel(rtc, LPC24XX_YEAR,	tm.tm_year);
    rtc_writel(rtc, LPC24XX_CCR, LPC24XX_CLKEN | LPC178X_CCALEN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc24xx_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int lpc24xx_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct lpc24xx_rtc *rtc = dev_get_drvdata(dev);
    u32 ct0, ct1, ct2;
    ct0 = rtc_readl(rtc, LPC24XX_CTIME0);
    ct1 = rtc_readl(rtc, LPC24XX_CTIME1);
    ct2 = rtc_readl(rtc, LPC24XX_CTIME2);
    tm.tm_sec  = CT0_SECS(ct0);
    tm.tm_min  = CT0_MINS(ct0);
    tm.tm_hour = CT0_HOURS(ct0);
    tm.tm_wday = CT0_DOW(ct0);
    tm.tm_mon  = CT1_MONTH(ct1);
    tm.tm_mday = CT1_DOM(ct1);
    tm.tm_year = CT1_YEAR(ct1);
    tm.tm_yday = CT2_DOY(ct2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc24xx_rtc_read_alarm(dev: *mut device, wkalrm: *mut rtc_wkalrm) -> c_int {
    static int lpc24xx_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *wkalrm)
    {
    struct lpc24xx_rtc *rtc = dev_get_drvdata(dev);
    struct rtc_time *tm = &wkalrm.time;
    tm.tm_sec  = rtc_readl(rtc, LPC24XX_ALSEC);
    tm.tm_min  = rtc_readl(rtc, LPC24XX_ALMIN);
    tm.tm_hour = rtc_readl(rtc, LPC24XX_ALHOUR);
    tm.tm_mday = rtc_readl(rtc, LPC24XX_ALDOM);
    tm.tm_wday = rtc_readl(rtc, LPC24XX_ALDOW);
    tm.tm_yday = rtc_readl(rtc, LPC24XX_ALDOY);
    tm.tm_mon  = rtc_readl(rtc, LPC24XX_ALMON);
    tm.tm_year = rtc_readl(rtc, LPC24XX_ALYEAR);
    wkalrm.enabled = rtc_readl(rtc, LPC24XX_AMR) == 0;
    wkalrm.pending = !!(rtc_readl(rtc, LPC24XX_ILR) & LPC24XX_RTCCIF);
    return rtc_valid_tm(&wkalrm.time);
    }
#[no_mangle]
unsafe extern "C" fn lpc24xx_rtc_set_alarm(dev: *mut device, wkalrm: *mut rtc_wkalrm) -> c_int {
    static int lpc24xx_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *wkalrm)
    {
    struct lpc24xx_rtc *rtc = dev_get_drvdata(dev);
    struct rtc_time *tm = &wkalrm.time;
// Disable alarm irq during update
    rtc_writel(rtc, LPC24XX_AMR, LPC24XX_ALARM_DISABLE);
    rtc_writel(rtc, LPC24XX_ALSEC,  tm.tm_sec);
    rtc_writel(rtc, LPC24XX_ALMIN,  tm.tm_min);
    rtc_writel(rtc, LPC24XX_ALHOUR, tm.tm_hour);
    rtc_writel(rtc, LPC24XX_ALDOM,  tm.tm_mday);
    rtc_writel(rtc, LPC24XX_ALDOW,  tm.tm_wday);
    rtc_writel(rtc, LPC24XX_ALDOY,  tm.tm_yday);
    rtc_writel(rtc, LPC24XX_ALMON,  tm.tm_mon);
    rtc_writel(rtc, LPC24XX_ALYEAR, tm.tm_year);
    if (wkalrm.enabled)
    rtc_writel(rtc, LPC24XX_AMR, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc24xx_rtc_alarm_irq_enable(dev: *mut device, enable: c_uint) -> c_int {
    static int lpc24xx_rtc_alarm_irq_enable(struct device *dev, unsigned int enable)
    {
    struct lpc24xx_rtc *rtc = dev_get_drvdata(dev);
    if (enable)
    rtc_writel(rtc, LPC24XX_AMR, 0);
    else
    rtc_writel(rtc, LPC24XX_AMR, LPC24XX_ALARM_DISABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc24xx_rtc_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t lpc24xx_rtc_interrupt(int irq, void *data)
    {
    let mut events: c_ulong = RTC_IRQF;
    struct lpc24xx_rtc *rtc = data;
    u32 rtc_iir;
// Check interrupt cause
    rtc_iir = rtc_readl(rtc, LPC24XX_ILR);
    if (rtc_iir & LPC24XX_RTCALF) {
    events |= RTC_AF;
    rtc_writel(rtc, LPC24XX_AMR, LPC24XX_ALARM_DISABLE);
    }
// Clear interrupt status and report event
    rtc_writel(rtc, LPC24XX_ILR, rtc_iir);
    rtc_update_irq(rtc.rtc, 1, events);
    return IRQ_HANDLED;
    }
    static const struct rtc_class_ops lpc24xx_rtc_ops = {
    .read_time		= lpc24xx_rtc_read_time,
    .set_time		= lpc24xx_rtc_set_time,
    .read_alarm		= lpc24xx_rtc_read_alarm,
    .set_alarm		= lpc24xx_rtc_set_alarm,
    .alarm_irq_enable	= lpc24xx_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn lpc24xx_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int lpc24xx_rtc_probe(struct platform_device *pdev)
    {
    struct lpc24xx_rtc *rtc;
    int irq, ret;
    rtc = devm_kzalloc(&pdev.dev, sizeof(*rtc), GFP_KERNEL);
    if (!rtc)
    return -ENOMEM;
    rtc.rtc_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rtc.rtc_base))
    return PTR_ERR(rtc.rtc_base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    rtc.clk_rtc = devm_clk_get(&pdev.dev, "rtc");
    if (IS_ERR(rtc.clk_rtc)) {
    dev_err(&pdev.dev, "error getting rtc clock\n");
    return PTR_ERR(rtc.clk_rtc);
    }
    rtc.clk_reg = devm_clk_get(&pdev.dev, "reg");
    if (IS_ERR(rtc.clk_reg)) {
    dev_err(&pdev.dev, "error getting reg clock\n");
    return PTR_ERR(rtc.clk_reg);
    }
    ret = clk_prepare_enable(rtc.clk_rtc);
    if (ret) {
    dev_err(&pdev.dev, "unable to enable rtc clock\n");
    return ret;
    }
    ret = clk_prepare_enable(rtc.clk_reg);
    if (ret) {
    dev_err(&pdev.dev, "unable to enable reg clock\n");
    goto disable_rtc_clk;
    }
    platform_set_drvdata(pdev, rtc);
// Clear any pending interrupts
    rtc_writel(rtc, LPC24XX_ILR, LPC24XX_RTCCIF | LPC24XX_RTCALF);
// Enable RTC count
    rtc_writel(rtc, LPC24XX_CCR, LPC24XX_CLKEN | LPC178X_CCALEN);
    ret = devm_request_irq(&pdev.dev, irq, lpc24xx_rtc_interrupt, 0,
    pdev.name, rtc);
    if (ret < 0) {
    dev_warn(&pdev.dev, "can't request interrupt\n");
    goto disable_clks;
    }
    rtc.rtc = devm_rtc_device_register(&pdev.dev, "lpc24xx-rtc",
    &lpc24xx_rtc_ops, THIS_MODULE);
    if (IS_ERR(rtc.rtc)) {
    dev_err(&pdev.dev, "can't register rtc device\n");
    ret = PTR_ERR(rtc.rtc);
    goto disable_clks;
    }
    return 0;
    disable_clks:
    clk_disable_unprepare(rtc.clk_reg);
    disable_rtc_clk:
    clk_disable_unprepare(rtc.clk_rtc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lpc24xx_rtc_remove(pdev: *mut platform_device) {
    static void lpc24xx_rtc_remove(struct platform_device *pdev)
    {
    struct lpc24xx_rtc *rtc = platform_get_drvdata(pdev);
// Ensure all interrupt sources are masked
    rtc_writel(rtc, LPC24XX_AMR, LPC24XX_ALARM_DISABLE);
    rtc_writel(rtc, LPC24XX_CIIR, 0);
    rtc_writel(rtc, LPC24XX_CCR, LPC178X_CCALEN);
    clk_disable_unprepare(rtc.clk_rtc);
    clk_disable_unprepare(rtc.clk_reg);
    }
    static const struct of_device_id lpc24xx_rtc_match[] = {
    { .compatible = "nxp,lpc1788-rtc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, lpc24xx_rtc_match);
    static struct platform_driver lpc24xx_rtc_driver = {
    .probe	= lpc24xx_rtc_probe,
    .remove = lpc24xx_rtc_remove,
    .driver	= {
    .name = "lpc24xx-rtc",
    .of_match_table	= lpc24xx_rtc_match,
    },
    };
    module_platform_driver(lpc24xx_rtc_driver);
    MODULE_AUTHOR("Kevin Wells <wellsk40@gmail.com>");
    MODULE_DESCRIPTION("RTC driver for the LPC178x/18xx/408x/43xx SoCs");
    MODULE_LICENSE("GPL");
