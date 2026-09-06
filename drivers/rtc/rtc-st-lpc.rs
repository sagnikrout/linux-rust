//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-st-lpc.c
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
// rtc-st-lpc.c - ST's LPC RTC, powered by the Low Power Timer
//
// Copyright (C) 2014 STMicroelectronics Limited
//
// Author: David Paris <david.paris@st.com> for STMicroelectronics
// Lee Jones <lee.jones@linaro.org> for STMicroelectronics
//
// Based on the original driver written by Stuart Menefy.
//

// Low Power Timer
pub const LPC_LPT_LSB_OFF: c_uint = 0x400;
pub const LPC_LPT_MSB_OFF: c_uint = 0x404;
pub const LPC_LPT_START_OFF: c_uint = 0x408;
// Low Power Alarm
pub const LPC_LPA_LSB_OFF: c_uint = 0x410;
pub const LPC_LPA_MSB_OFF: c_uint = 0x414;
pub const LPC_LPA_START_OFF: c_uint = 0x418;
// LPC as WDT
pub const LPC_WDT_OFF: c_uint = 0x510;
pub const LPC_WDT_FLAG_OFF: c_uint = 0x514;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_rtc {
    pub rtc_dev: *mut rtc_device,
    pub alarm: rtc_wkalrm,
    pub clk: *mut clk,
    pub clkrate: c_ulong,
    pub ioaddr: *mut void __iomem,
    pub irq_enabled:1: bool,
    pub lock: spinlock_t,
    pub irq: c_short,
}

    static void st_rtc_set_hw_alarm(struct st_rtc *rtc,
    unsigned long msb, unsigned long  lsb)
    {
    unsigned long flags;
    spin_lock_irqsave(&rtc.lock, flags);
    writel_relaxed(1, rtc.ioaddr + LPC_WDT_OFF);
    writel_relaxed(msb, rtc.ioaddr + LPC_LPA_MSB_OFF);
    writel_relaxed(lsb, rtc.ioaddr + LPC_LPA_LSB_OFF);
    writel_relaxed(1, rtc.ioaddr + LPC_LPA_START_OFF);
    writel_relaxed(0, rtc.ioaddr + LPC_WDT_OFF);
    spin_unlock_irqrestore(&rtc.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn st_rtc_handler(this_irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t st_rtc_handler(int this_irq, void *data)
    {
    struct st_rtc *rtc = (struct st_rtc *)data;
    rtc_update_irq(rtc.rtc_dev, 1, RTC_AF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn st_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int st_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct st_rtc *rtc = dev_get_drvdata(dev);
    unsigned long lpt_lsb, lpt_msb;
    unsigned long long lpt;
    unsigned long flags;
    spin_lock_irqsave(&rtc.lock, flags);
    do {
    lpt_msb = readl_relaxed(rtc.ioaddr + LPC_LPT_MSB_OFF);
    lpt_lsb = readl_relaxed(rtc.ioaddr + LPC_LPT_LSB_OFF);
    } while (readl_relaxed(rtc.ioaddr + LPC_LPT_MSB_OFF) != lpt_msb);
    spin_unlock_irqrestore(&rtc.lock, flags);
    lpt = ((unsigned long long)lpt_msb << 32) | lpt_lsb;
    do_div(lpt, rtc.clkrate);
    rtc_time64_to_tm(lpt, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int st_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct st_rtc *rtc = dev_get_drvdata(dev);
    unsigned long long lpt, secs;
    unsigned long flags;
    secs = rtc_tm_to_time64(tm);
    lpt = (unsigned long long)secs * rtc.clkrate;
    spin_lock_irqsave(&rtc.lock, flags);
    writel_relaxed(lpt >> 32, rtc.ioaddr + LPC_LPT_MSB_OFF);
    writel_relaxed(lpt, rtc.ioaddr + LPC_LPT_LSB_OFF);
    writel_relaxed(1, rtc.ioaddr + LPC_LPT_START_OFF);
    spin_unlock_irqrestore(&rtc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_rtc_read_alarm(dev: *mut device, wkalrm: *mut rtc_wkalrm) -> c_int {
    static int st_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *wkalrm)
    {
    struct st_rtc *rtc = dev_get_drvdata(dev);
    unsigned long flags;
    spin_lock_irqsave(&rtc.lock, flags);
    memcpy(wkalrm, &rtc.alarm, sizeof(struct rtc_wkalrm));
    spin_unlock_irqrestore(&rtc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_rtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int st_rtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct st_rtc *rtc = dev_get_drvdata(dev);
    if (enabled && !rtc.irq_enabled) {
    enable_irq(rtc.irq);
    rtc.irq_enabled = true;
    } else if (!enabled && rtc.irq_enabled) {
    disable_irq(rtc.irq);
    rtc.irq_enabled = false;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_rtc_set_alarm(dev: *mut device, t: *mut rtc_wkalrm) -> c_int {
    static int st_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *t)
    {
    struct st_rtc *rtc = dev_get_drvdata(dev);
    struct rtc_time now;
    unsigned long long now_secs;
    unsigned long long alarm_secs;
    unsigned long long lpa;
    st_rtc_read_time(dev, &now);
    now_secs = rtc_tm_to_time64(&now);
    alarm_secs = rtc_tm_to_time64(&t.time);
    memcpy(&rtc.alarm, t, sizeof(struct rtc_wkalrm));
// Now many secs to fire
    alarm_secs -= now_secs;
    lpa = (unsigned long long)alarm_secs * rtc.clkrate;
    st_rtc_set_hw_alarm(rtc, lpa >> 32, lpa);
    st_rtc_alarm_irq_enable(dev, t.enabled);
    return 0;
    }
    static const struct rtc_class_ops st_rtc_ops = {
    .read_time		= st_rtc_read_time,
    .set_time		= st_rtc_set_time,
    .read_alarm		= st_rtc_read_alarm,
    .set_alarm		= st_rtc_set_alarm,
    .alarm_irq_enable	= st_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn st_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int st_rtc_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct st_rtc *rtc;
    uint32_t mode;
    let mut ret: c_int = 0;
    ret = of_property_read_u32(np, "st,lpc-mode", &mode);
    if (ret) {
    dev_err(&pdev.dev, "An LPC mode must be provided\n");
    return -EINVAL;
    }
// LPC can either run as a Clocksource or in RTC or WDT mode
    if (mode != ST_LPC_MODE_RTC)
    return -ENODEV;
    rtc = devm_kzalloc(&pdev.dev, sizeof(struct st_rtc), GFP_KERNEL);
    if (!rtc)
    return -ENOMEM;
    rtc.rtc_dev = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc.rtc_dev))
    return PTR_ERR(rtc.rtc_dev);
    spin_lock_init(&rtc.lock);
    rtc.ioaddr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rtc.ioaddr))
    return PTR_ERR(rtc.ioaddr);
    rtc.irq = platform_get_irq(pdev, 0);
    if (rtc.irq < 0)
    return rtc.irq;
    ret = devm_request_irq(&pdev.dev, rtc.irq, st_rtc_handler,
    IRQF_NO_AUTOEN, pdev.name, rtc);
    if (ret) {
    dev_err(&pdev.dev, "Failed to request irq %i\n", rtc.irq);
    return ret;
    }
    enable_irq_wake(rtc.irq);
    rtc.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(rtc.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(rtc.clk),
    "Unable to request clock\n");
    rtc.clkrate = clk_get_rate(rtc.clk);
    if (!rtc.clkrate) {
    dev_err(&pdev.dev, "Unable to fetch clock rate\n");
    return -EINVAL;
    }
    device_set_wakeup_capable(&pdev.dev, 1);
    platform_set_drvdata(pdev, rtc);
    rtc.rtc_dev.ops = &st_rtc_ops;
    rtc.rtc_dev.range_max = U64_MAX;
    do_div(rtc.rtc_dev.range_max, rtc.clkrate);
    ret = devm_rtc_register_device(rtc.rtc_dev);
    if (ret)
    return ret;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn st_rtc_suspend(dev: *mut device) -> c_int {
    static int st_rtc_suspend(struct device *dev)
    {
    struct st_rtc *rtc = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    return 0;
    writel_relaxed(1, rtc.ioaddr + LPC_WDT_OFF);
    writel_relaxed(0, rtc.ioaddr + LPC_LPA_START_OFF);
    writel_relaxed(0, rtc.ioaddr + LPC_WDT_OFF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_rtc_resume(dev: *mut device) -> c_int {
    static int st_rtc_resume(struct device *dev)
    {
    struct st_rtc *rtc = dev_get_drvdata(dev);
    rtc_alarm_irq_enable(rtc.rtc_dev, 0);
//
// clean 'rtc->alarm' to allow a new
// .set_alarm to the upper RTC layer
//
    memset(&rtc.alarm, 0, sizeof(struct rtc_wkalrm));
    writel_relaxed(0, rtc.ioaddr + LPC_LPA_MSB_OFF);
    writel_relaxed(0, rtc.ioaddr + LPC_LPA_LSB_OFF);
    writel_relaxed(1, rtc.ioaddr + LPC_WDT_OFF);
    writel_relaxed(1, rtc.ioaddr + LPC_LPA_START_OFF);
    writel_relaxed(0, rtc.ioaddr + LPC_WDT_OFF);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(st_rtc_pm_ops, st_rtc_suspend, st_rtc_resume);
    static const struct of_device_id st_rtc_match[] = {
    { .compatible = "st,stih407-lpc" },
    {}
    };
    MODULE_DEVICE_TABLE(of, st_rtc_match);
    static struct platform_driver st_rtc_platform_driver = {
    .driver = {
    .name = "st-lpc-rtc",
    .pm = &st_rtc_pm_ops,
    .of_match_table = st_rtc_match,
    },
    .probe = st_rtc_probe,
    };
    module_platform_driver(st_rtc_platform_driver);
    MODULE_DESCRIPTION("STMicroelectronics LPC RTC driver");
    MODULE_AUTHOR("David Paris <david.paris@st.com>");
    MODULE_LICENSE("GPL");
