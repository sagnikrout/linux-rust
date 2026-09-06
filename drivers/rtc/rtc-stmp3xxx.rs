//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-stmp3xxx.c
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
// Freescale STMP37XX/STMP378X Real Time Clock driver
//
// Copyright (c) 2007 Sigmatel, Inc.
// Peter Hartley, <peter.hartley@sigmatel.com>
//
// Copyright 2008 Freescale Semiconductor, Inc. All Rights Reserved.
// Copyright 2008 Embedded Alley Solutions, Inc All Rights Reserved.
// Copyright 2011 Wolfram Sang, Pengutronix e.K.
//

pub const STMP3XXX_RTC_CTRL: c_uint = 0x0;
pub const STMP3XXX_RTC_CTRL_ALARM_IRQ_EN: c_uint = 0x00000001;
pub const STMP3XXX_RTC_CTRL_ONEMSEC_IRQ_EN: c_uint = 0x00000002;
pub const STMP3XXX_RTC_CTRL_ALARM_IRQ: c_uint = 0x00000004;
pub const STMP3XXX_RTC_CTRL_WATCHDOGEN: c_uint = 0x00000010;
pub const STMP3XXX_RTC_STAT: c_uint = 0x10;
pub const STMP3XXX_RTC_STAT_STALE_SHIFT: c_int = 16;
pub const STMP3XXX_RTC_STAT_RTC_PRESENT: c_uint = 0x80000000;
pub const STMP3XXX_RTC_STAT_XTAL32000_PRESENT: c_uint = 0x10000000;
pub const STMP3XXX_RTC_STAT_XTAL32768_PRESENT: c_uint = 0x08000000;
pub const STMP3XXX_RTC_SECONDS: c_uint = 0x30;
pub const STMP3XXX_RTC_ALARM: c_uint = 0x40;
pub const STMP3XXX_RTC_WATCHDOG: c_uint = 0x50;
pub const STMP3XXX_RTC_PERSISTENT0: c_uint = 0x60;

pub const STMP3XXX_RTC_PERSISTENT1: c_uint = 0x70;
// missing bitmask in headers
pub const STMP3XXX_RTC_PERSISTENT1_FORCE_UPDATER: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmp3xxx_rtc_data {
    pub rtc: *mut rtc_device,
    pub io: *mut void __iomem,
    pub irq_alarm: c_int,
}

//
// stmp3xxx_wdt_set_timeout - configure the watchdog inside the STMP3xxx RTC
// @dev: the parent device of the watchdog (= the RTC)
// @timeout: the desired value for the timeout register of the watchdog.
// 0 disables the watchdog
//
// The watchdog needs one register and two bits which are in the RTC domain.
// To handle the resource conflict, the RTC driver will create another
// platform_device for the watchdog driver as a child of the RTC device.
// The watchdog driver is passed the below accessor function via platform_data
// to configure the watchdog. Locking is not needed because accessing SET/CLR
// registers is atomic.
//
#[no_mangle]
unsafe extern "C" fn stmp3xxx_wdt_set_timeout(dev: *mut device, timeout: u32) {
    static void stmp3xxx_wdt_set_timeout(struct device *dev, u32 timeout)
    {
    struct stmp3xxx_rtc_data *rtc_data = dev_get_drvdata(dev);
    if (timeout) {
    writel(timeout, rtc_data.io + STMP3XXX_RTC_WATCHDOG);
    writel(STMP3XXX_RTC_CTRL_WATCHDOGEN,
    rtc_data.io + STMP3XXX_RTC_CTRL + STMP_OFFSET_REG_SET);
    writel(STMP3XXX_RTC_PERSISTENT1_FORCE_UPDATER,
    rtc_data.io + STMP3XXX_RTC_PERSISTENT1 + STMP_OFFSET_REG_SET);
    } else {
    writel(STMP3XXX_RTC_CTRL_WATCHDOGEN,
    rtc_data.io + STMP3XXX_RTC_CTRL + STMP_OFFSET_REG_CLR);
    writel(STMP3XXX_RTC_PERSISTENT1_FORCE_UPDATER,
    rtc_data.io + STMP3XXX_RTC_PERSISTENT1 + STMP_OFFSET_REG_CLR);
    }
    }
    static struct stmp3xxx_wdt_pdata wdt_pdata = {
    .wdt_set_timeout = stmp3xxx_wdt_set_timeout,
    };
#[no_mangle]
unsafe extern "C" fn stmp3xxx_wdt_register(rtc_pdev: *mut platform_device) {
    static void stmp3xxx_wdt_register(struct platform_device *rtc_pdev)
    {
    let mut rc: c_int = -1;
    struct platform_device *wdt_pdev =
    platform_device_alloc("stmp3xxx_rtc_wdt", rtc_pdev.id);
    if (wdt_pdev) {
    wdt_pdev.dev.parent = &rtc_pdev.dev;
    wdt_pdev.dev.platform_data = &wdt_pdata;
    rc = platform_device_add(wdt_pdev);
    if (rc)
    platform_device_put(wdt_pdev);
    }
    if (rc)
    dev_err(&rtc_pdev.dev,
    "failed to register stmp3xxx_rtc_wdt\n");
    }

#[no_mangle]
unsafe extern "C" fn stmp3xxx_wdt_register(rtc_pdev: *mut platform_device) {
    static void stmp3xxx_wdt_register(struct platform_device *rtc_pdev)
    {
    }

#[no_mangle]
unsafe extern "C" fn stmp3xxx_wait_time(rtc_data: *mut stmp3xxx_rtc_data) -> c_int {
    static int stmp3xxx_wait_time(struct stmp3xxx_rtc_data *rtc_data)
    {
    int timeout = 5000; /* 3ms according to i.MX28 Ref Manual */
//
// The i.MX28 Applications Processor Reference Manual, Rev. 1, 2010
// states:
// | The order in which registers are updated is
// | Persistent 0, 1, 2, 3, 4, 5, Alarm, Seconds.
// | (This list is in bitfield order, from LSB to MSB, as they would
// | appear in the STALE_REGS and NEW_REGS bitfields of the HW_RTC_STAT
// | register. For example, the Seconds register corresponds to
// | STALE_REGS or NEW_REGS containing 0x80.)
//
    do {
    if (!(readl(rtc_data.io + STMP3XXX_RTC_STAT) &
    (0x80 << STMP3XXX_RTC_STAT_STALE_SHIFT)))
    return 0;
    udelay(1);
    } while (--timeout > 0);
    return (readl(rtc_data.io + STMP3XXX_RTC_STAT) &
    (0x80 << STMP3XXX_RTC_STAT_STALE_SHIFT)) ? -ETIME : 0;
    }
// Time read/write
#[no_mangle]
unsafe extern "C" fn stmp3xxx_rtc_gettime(dev: *mut device, rtc_tm: *mut rtc_time) -> c_int {
    static int stmp3xxx_rtc_gettime(struct device *dev, struct rtc_time *rtc_tm)
    {
    int ret;
    struct stmp3xxx_rtc_data *rtc_data = dev_get_drvdata(dev);
    ret = stmp3xxx_wait_time(rtc_data);
    if (ret)
    return ret;
    rtc_time64_to_tm(readl(rtc_data.io + STMP3XXX_RTC_SECONDS), rtc_tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmp3xxx_rtc_settime(dev: *mut device, rtc_tm: *mut rtc_time) -> c_int {
    static int stmp3xxx_rtc_settime(struct device *dev, struct rtc_time *rtc_tm)
    {
    struct stmp3xxx_rtc_data *rtc_data = dev_get_drvdata(dev);
    writel(rtc_tm_to_time64(rtc_tm), rtc_data.io + STMP3XXX_RTC_SECONDS);
    return stmp3xxx_wait_time(rtc_data);
    }
// interrupt(s) handler
#[no_mangle]
unsafe extern "C" fn stmp3xxx_rtc_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t stmp3xxx_rtc_interrupt(int irq, void *dev_id)
    {
    struct stmp3xxx_rtc_data *rtc_data = dev_get_drvdata(dev_id);
    let mut status: u32 = readl(rtc_data.io + STMP3XXX_RTC_CTRL);
    if (status & STMP3XXX_RTC_CTRL_ALARM_IRQ) {
    writel(STMP3XXX_RTC_CTRL_ALARM_IRQ,
    rtc_data.io + STMP3XXX_RTC_CTRL + STMP_OFFSET_REG_CLR);
    rtc_update_irq(rtc_data.rtc, 1, RTC_AF | RTC_IRQF);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn stmp3xxx_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int stmp3xxx_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct stmp3xxx_rtc_data *rtc_data = dev_get_drvdata(dev);
    if (enabled) {
    writel(STMP3XXX_RTC_PERSISTENT0_ALARM_EN |
    STMP3XXX_RTC_PERSISTENT0_ALARM_WAKE_EN,
    rtc_data.io + STMP3XXX_RTC_PERSISTENT0 +
    STMP_OFFSET_REG_SET);
    writel(STMP3XXX_RTC_CTRL_ALARM_IRQ_EN,
    rtc_data.io + STMP3XXX_RTC_CTRL + STMP_OFFSET_REG_SET);
    } else {
    writel(STMP3XXX_RTC_PERSISTENT0_ALARM_EN |
    STMP3XXX_RTC_PERSISTENT0_ALARM_WAKE_EN,
    rtc_data.io + STMP3XXX_RTC_PERSISTENT0 +
    STMP_OFFSET_REG_CLR);
    writel(STMP3XXX_RTC_CTRL_ALARM_IRQ_EN,
    rtc_data.io + STMP3XXX_RTC_CTRL + STMP_OFFSET_REG_CLR);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmp3xxx_rtc_read_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int stmp3xxx_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct stmp3xxx_rtc_data *rtc_data = dev_get_drvdata(dev);
    rtc_time64_to_tm(readl(rtc_data.io + STMP3XXX_RTC_ALARM), &alm.time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmp3xxx_rtc_set_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int stmp3xxx_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct stmp3xxx_rtc_data *rtc_data = dev_get_drvdata(dev);
    writel(rtc_tm_to_time64(&alm.time), rtc_data.io + STMP3XXX_RTC_ALARM);
    stmp3xxx_alarm_irq_enable(dev, alm.enabled);
    return 0;
    }
    static const struct rtc_class_ops stmp3xxx_rtc_ops = {
    .alarm_irq_enable =
    stmp3xxx_alarm_irq_enable,
    .read_time	= stmp3xxx_rtc_gettime,
    .set_time	= stmp3xxx_rtc_settime,
    .read_alarm	= stmp3xxx_rtc_read_alarm,
    .set_alarm	= stmp3xxx_rtc_set_alarm,
    };
#[no_mangle]
unsafe extern "C" fn stmp3xxx_rtc_remove(pdev: *mut platform_device) {
    static void stmp3xxx_rtc_remove(struct platform_device *pdev)
    {
    struct stmp3xxx_rtc_data *rtc_data = platform_get_drvdata(pdev);
    if (!rtc_data)
    return;
    writel(STMP3XXX_RTC_CTRL_ALARM_IRQ_EN,
    rtc_data.io + STMP3XXX_RTC_CTRL + STMP_OFFSET_REG_CLR);
    }
#[no_mangle]
unsafe extern "C" fn stmp3xxx_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int stmp3xxx_rtc_probe(struct platform_device *pdev)
    {
    struct stmp3xxx_rtc_data *rtc_data;
    struct resource *r;
    u32 rtc_stat;
    u32 pers0_set, pers0_clr;
    let mut crystalfreq: u32 = 0;
    int err;
    rtc_data = devm_kzalloc(&pdev.dev, sizeof(*rtc_data), GFP_KERNEL);
    if (!rtc_data)
    return -ENOMEM;
    r = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!r) {
    dev_err(&pdev.dev, "failed to get resource\n");
    return -ENXIO;
    }
    rtc_data.io = devm_ioremap(&pdev.dev, r.start, resource_size(r));
    if (!rtc_data.io) {
    dev_err(&pdev.dev, "ioremap failed\n");
    return -EIO;
    }
    rtc_data.irq_alarm = platform_get_irq(pdev, 0);
    rtc_stat = readl(rtc_data.io + STMP3XXX_RTC_STAT);
    if (!(rtc_stat & STMP3XXX_RTC_STAT_RTC_PRESENT)) {
    dev_err(&pdev.dev, "no device onboard\n");
    return -ENODEV;
    }
    platform_set_drvdata(pdev, rtc_data);
//
// Resetting the rtc stops the watchdog timer that is potentially
// running. So (assuming it is running on purpose) don't reset if the
// watchdog is enabled.
//
    if (readl(rtc_data.io + STMP3XXX_RTC_CTRL) &
    STMP3XXX_RTC_CTRL_WATCHDOGEN) {
    dev_info(&pdev.dev,
    "Watchdog is running, skip resetting rtc\n");
    } else {
    err = stmp_reset_block(rtc_data.io);
    if (err) {
    dev_err(&pdev.dev, "stmp_reset_block failed: %d\n",
    err);
    return err;
    }
    }
//
// Obviously the rtc needs a clock input to be able to run.
// This clock can be provided by an external 32k crystal. If that one is
// missing XTAL must not be disabled in suspend which consumes a
// lot of power. Normally the presence and exact frequency (supported
// are 32000 Hz and 32768 Hz) is detectable from fuses, but as reality
// proves these fuses are not blown correctly on all machines, so the
// frequency can be overridden in the device tree.
//
    if (rtc_stat & STMP3XXX_RTC_STAT_XTAL32000_PRESENT)
    crystalfreq = 32000;
#[no_mangle]
pub unsafe extern "C" fn if(STMP3XXX_RTC_STAT_XTAL32768_PRESENT: rtc_stat &) -> else {
    else if (rtc_stat & STMP3XXX_RTC_STAT_XTAL32768_PRESENT)
    crystalfreq = 32768;
    of_property_read_u32(pdev.dev.of_node, "stmp,crystal-freq",
    &crystalfreq);
    switch (crystalfreq) {
    case 32000:
// keep 32kHz crystal running in low-power mode
    pers0_set = STMP3XXX_RTC_PERSISTENT0_XTAL32_FREQ |
    STMP3XXX_RTC_PERSISTENT0_XTAL32KHZ_PWRUP |
    STMP3XXX_RTC_PERSISTENT0_CLOCKSOURCE;
    pers0_clr = STMP3XXX_RTC_PERSISTENT0_XTAL24MHZ_PWRUP;
    break;
    case 32768:
// keep 32.768kHz crystal running in low-power mode
    pers0_set = STMP3XXX_RTC_PERSISTENT0_XTAL32KHZ_PWRUP |
    STMP3XXX_RTC_PERSISTENT0_CLOCKSOURCE;
    pers0_clr = STMP3XXX_RTC_PERSISTENT0_XTAL24MHZ_PWRUP |
    STMP3XXX_RTC_PERSISTENT0_XTAL32_FREQ;
    break;
    default:
    dev_warn(&pdev.dev,
    "invalid crystal-freq specified in device-tree. Assuming no crystal\n");
    fallthrough;
    case 0:
// keep XTAL on in low-power mode
    pers0_set = STMP3XXX_RTC_PERSISTENT0_XTAL24MHZ_PWRUP;
    pers0_clr = STMP3XXX_RTC_PERSISTENT0_XTAL32KHZ_PWRUP |
    STMP3XXX_RTC_PERSISTENT0_CLOCKSOURCE;
    }
    writel(pers0_set, rtc_data.io + STMP3XXX_RTC_PERSISTENT0 +
    STMP_OFFSET_REG_SET);
    writel(STMP3XXX_RTC_PERSISTENT0_ALARM_EN |
    STMP3XXX_RTC_PERSISTENT0_ALARM_WAKE_EN |
    STMP3XXX_RTC_PERSISTENT0_ALARM_WAKE | pers0_clr,
    rtc_data.io + STMP3XXX_RTC_PERSISTENT0 + STMP_OFFSET_REG_CLR);
    writel(STMP3XXX_RTC_CTRL_ONEMSEC_IRQ_EN |
    STMP3XXX_RTC_CTRL_ALARM_IRQ_EN,
    rtc_data.io + STMP3XXX_RTC_CTRL + STMP_OFFSET_REG_CLR);
    rtc_data.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc_data.rtc))
    return PTR_ERR(rtc_data.rtc);
    err = devm_request_irq(&pdev.dev, rtc_data.irq_alarm,
    stmp3xxx_rtc_interrupt, 0, "RTC alarm", &pdev.dev);
    if (err) {
    dev_err(&pdev.dev, "Cannot claim IRQ%d\n",
    rtc_data.irq_alarm);
    return err;
    }
    rtc_data.rtc.ops = &stmp3xxx_rtc_ops;
    rtc_data.rtc.range_max = U32_MAX;
    err = devm_rtc_register_device(rtc_data.rtc);
    if (err)
    return err;
    stmp3xxx_wdt_register(pdev);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn stmp3xxx_rtc_suspend(dev: *mut device) -> c_int {
    static int stmp3xxx_rtc_suspend(struct device *dev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmp3xxx_rtc_resume(dev: *mut device) -> c_int {
    static int stmp3xxx_rtc_resume(struct device *dev)
    {
    struct stmp3xxx_rtc_data *rtc_data = dev_get_drvdata(dev);
    stmp_reset_block(rtc_data.io);
    writel(STMP3XXX_RTC_PERSISTENT0_ALARM_EN |
    STMP3XXX_RTC_PERSISTENT0_ALARM_WAKE_EN |
    STMP3XXX_RTC_PERSISTENT0_ALARM_WAKE,
    rtc_data.io + STMP3XXX_RTC_PERSISTENT0 + STMP_OFFSET_REG_CLR);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(stmp3xxx_rtc_pm_ops, stmp3xxx_rtc_suspend,
    stmp3xxx_rtc_resume);
    static const struct of_device_id rtc_dt_ids[] = {
    { .compatible = "fsl,stmp3xxx-rtc", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rtc_dt_ids);
    static struct platform_driver stmp3xxx_rtcdrv = {
    .probe		= stmp3xxx_rtc_probe,
    .remove		= stmp3xxx_rtc_remove,
    .driver		= {
    .name	= "stmp3xxx-rtc",
    .pm	= &stmp3xxx_rtc_pm_ops,
    .of_match_table = rtc_dt_ids,
    },
    };
    module_platform_driver(stmp3xxx_rtcdrv);
    MODULE_DESCRIPTION("STMP3xxx RTC Driver");
    MODULE_AUTHOR("dmitry pervushin <dpervushin@embeddedalley.com> and "
    "Wolfram Sang <kernel@pengutronix.de>");
    MODULE_LICENSE("GPL");
