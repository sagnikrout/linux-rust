//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-xgene.c
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
// APM X-Gene SoC Real Time Clock Driver
//
// Copyright (c) 2014, Applied Micro Circuits Corporation
// Author: Rameshwar Prasad Sahu <rsahu@apm.com>
// Loc Ho <lho@apm.com>
//

// RTC CSR Registers
pub const RTC_CCVR: c_uint = 0x00;
pub const RTC_CMR: c_uint = 0x04;
pub const RTC_CLR: c_uint = 0x08;
pub const RTC_CCR: c_uint = 0x0C;

pub const RTC_STAT: c_uint = 0x10;

pub const RTC_RSTAT: c_uint = 0x14;
pub const RTC_EOI: c_uint = 0x18;
pub const RTC_VER: c_uint = 0x1C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_rtc_dev {
    pub rtc: *mut rtc_device,
    pub csr_base: *mut void __iomem,
    pub clk: *mut clk,
    pub irq_wake: c_uint,
    pub irq_enabled: c_uint,
}

#[no_mangle]
unsafe extern "C" fn xgene_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int xgene_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct xgene_rtc_dev *pdata = dev_get_drvdata(dev);
    rtc_time64_to_tm(readl(pdata.csr_base + RTC_CCVR), tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgene_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int xgene_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct xgene_rtc_dev *pdata = dev_get_drvdata(dev);
//
// NOTE: After the following write, the RTC_CCVR is only reflected
// after the update cycle of 1 seconds.
//
    writel((u32)rtc_tm_to_time64(tm), pdata.csr_base + RTC_CLR);
    readl(pdata.csr_base + RTC_CLR); /* Force a barrier */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgene_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int xgene_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct xgene_rtc_dev *pdata = dev_get_drvdata(dev);
// If possible, CMR should be read here
    rtc_time64_to_tm(0, &alrm.time);
    alrm.enabled = readl(pdata.csr_base + RTC_CCR) & RTC_CCR_IE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgene_rtc_alarm_irq_enable(dev: *mut device, enabled: u32) -> c_int {
    static int xgene_rtc_alarm_irq_enable(struct device *dev, u32 enabled)
    {
    struct xgene_rtc_dev *pdata = dev_get_drvdata(dev);
    u32 ccr;
    ccr = readl(pdata.csr_base + RTC_CCR);
    if (enabled) {
    ccr &= ~RTC_CCR_MASK;
    ccr |= RTC_CCR_IE;
    } else {
    ccr &= ~RTC_CCR_IE;
    ccr |= RTC_CCR_MASK;
    }
    writel(ccr, pdata.csr_base + RTC_CCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgene_rtc_alarm_irq_enabled(dev: *mut device) -> c_int {
    static int xgene_rtc_alarm_irq_enabled(struct device *dev)
    {
    struct xgene_rtc_dev *pdata = dev_get_drvdata(dev);
    return readl(pdata.csr_base + RTC_CCR) & RTC_CCR_IE ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn xgene_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int xgene_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct xgene_rtc_dev *pdata = dev_get_drvdata(dev);
    writel((u32)rtc_tm_to_time64(&alrm.time), pdata.csr_base + RTC_CMR);
    xgene_rtc_alarm_irq_enable(dev, alrm.enabled);
    return 0;
    }
    static const struct rtc_class_ops xgene_rtc_ops = {
    .read_time	= xgene_rtc_read_time,
    .set_time	= xgene_rtc_set_time,
    .read_alarm	= xgene_rtc_read_alarm,
    .set_alarm	= xgene_rtc_set_alarm,
    .alarm_irq_enable = xgene_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn xgene_rtc_interrupt(irq: c_int, id: *mut c_void) -> irqreturn_t {
    static irqreturn_t xgene_rtc_interrupt(int irq, void *id)
    {
    struct xgene_rtc_dev *pdata = id;
// Check if interrupt asserted
    if (!(readl(pdata.csr_base + RTC_STAT) & RTC_STAT_BIT))
    return IRQ_NONE;
// Clear interrupt
    readl(pdata.csr_base + RTC_EOI);
    rtc_update_irq(pdata.rtc, 1, RTC_IRQF | RTC_AF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn xgene_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int xgene_rtc_probe(struct platform_device *pdev)
    {
    struct xgene_rtc_dev *pdata;
    int ret;
    int irq;
    pdata = devm_kzalloc(&pdev.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    platform_set_drvdata(pdev, pdata);
    pdata.csr_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pdata.csr_base))
    return PTR_ERR(pdata.csr_base);
    pdata.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(pdata.rtc))
    return PTR_ERR(pdata.rtc);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(&pdev.dev, irq, xgene_rtc_interrupt, 0,
    dev_name(&pdev.dev), pdata);
    if (ret) {
    dev_err(&pdev.dev, "Could not request IRQ\n");
    return ret;
    }
    pdata.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(pdata.clk)) {
    dev_err(&pdev.dev, "Couldn't get the clock for RTC\n");
    return -ENODEV;
    }
    ret = clk_prepare_enable(pdata.clk);
    if (ret)
    return ret;
// Turn on the clock and the crystal
    writel(RTC_CCR_EN, pdata.csr_base + RTC_CCR);
    ret = device_init_wakeup(&pdev.dev, true);
    if (ret) {
    clk_disable_unprepare(pdata.clk);
    return ret;
    }
    pdata.rtc.ops = &xgene_rtc_ops;
    pdata.rtc.range_max = U32_MAX;
    ret = devm_rtc_register_device(pdata.rtc);
    if (ret) {
    clk_disable_unprepare(pdata.clk);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgene_rtc_remove(pdev: *mut platform_device) {
    static void xgene_rtc_remove(struct platform_device *pdev)
    {
    struct xgene_rtc_dev *pdata = platform_get_drvdata(pdev);
    xgene_rtc_alarm_irq_enable(&pdev.dev, 0);
    device_init_wakeup(&pdev.dev, false);
    clk_disable_unprepare(pdata.clk);
    }
#[no_mangle]
unsafe extern "C" fn xgene_rtc_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused xgene_rtc_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct xgene_rtc_dev *pdata = platform_get_drvdata(pdev);
    int irq;
    irq = platform_get_irq(pdev, 0);
//
// If this RTC alarm will be used for waking the system up,
// don't disable it of course. Else we just disable the alarm
// and await suspension.
//
    if (device_may_wakeup(&pdev.dev)) {
    if (!enable_irq_wake(irq))
    pdata.irq_wake = 1;
    } else {
    pdata.irq_enabled = xgene_rtc_alarm_irq_enabled(dev);
    xgene_rtc_alarm_irq_enable(dev, 0);
    clk_disable_unprepare(pdata.clk);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgene_rtc_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused xgene_rtc_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct xgene_rtc_dev *pdata = platform_get_drvdata(pdev);
    int irq;
    int rc;
    irq = platform_get_irq(pdev, 0);
    if (device_may_wakeup(&pdev.dev)) {
    if (pdata.irq_wake) {
    disable_irq_wake(irq);
    pdata.irq_wake = 0;
    }
    } else {
    rc = clk_prepare_enable(pdata.clk);
    if (rc) {
    dev_err(dev, "Unable to enable clock error %d\n", rc);
    return rc;
    }
    xgene_rtc_alarm_irq_enable(dev, pdata.irq_enabled);
    }
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(xgene_rtc_pm_ops, xgene_rtc_suspend, xgene_rtc_resume);

    static const struct of_device_id xgene_rtc_of_match[] = {
    {.compatible = "apm,xgene-rtc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, xgene_rtc_of_match);

    static struct platform_driver xgene_rtc_driver = {
    .probe		= xgene_rtc_probe,
    .remove		= xgene_rtc_remove,
    .driver		= {
    .name	= "xgene-rtc",
    .pm = &xgene_rtc_pm_ops,
    .of_match_table	= of_match_ptr(xgene_rtc_of_match),
    },
    };
    module_platform_driver(xgene_rtc_driver);
    MODULE_DESCRIPTION("APM X-Gene SoC RTC driver");
    MODULE_AUTHOR("Rameshwar Sahu <rsahu@apm.com>");
    MODULE_LICENSE("GPL");
