//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-cadence.c
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
// Copyright 2019 Cadence
//
// Authors:
// Jan Kotas <jank@cadence.com>
//

// Registers
pub const CDNS_RTC_CTLR: c_uint = 0x00;
pub const CDNS_RTC_HMR: c_uint = 0x04;
pub const CDNS_RTC_TIMR: c_uint = 0x08;
pub const CDNS_RTC_CALR: c_uint = 0x0C;
pub const CDNS_RTC_TIMAR: c_uint = 0x10;
pub const CDNS_RTC_CALAR: c_uint = 0x14;
pub const CDNS_RTC_AENR: c_uint = 0x18;
pub const CDNS_RTC_EFLR: c_uint = 0x1C;
pub const CDNS_RTC_IENR: c_uint = 0x20;
pub const CDNS_RTC_IDISR: c_uint = 0x24;
pub const CDNS_RTC_IMSKR: c_uint = 0x28;
pub const CDNS_RTC_STSR: c_uint = 0x2C;
pub const CDNS_RTC_KRTCR: c_uint = 0x30;
// Control

// Status

// Keep RTC

// Alarm, Event, Interrupt

// Time

// Calendar

pub const CDNS_RTC_MAX_REGS_TRIES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_rtc {
    pub rtc_dev: *mut rtc_device,
    pub pclk: *mut clk,
    pub ref_clk: *mut clk,
    pub regs: *mut void __iomem,
    pub irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn cdns_rtc_set_enabled(crtc: *mut cdns_rtc, enabled: bool) {
    static void cdns_rtc_set_enabled(struct cdns_rtc *crtc, bool enabled)
    {
    let mut reg: u32 = enabled ? 0x0 : CDNS_RTC_CTLR_TIME_CAL;
    writel(reg, crtc.regs + CDNS_RTC_CTLR);
    }
#[no_mangle]
unsafe extern "C" fn cdns_rtc_get_enabled(crtc: *mut cdns_rtc) -> bool {
    static bool cdns_rtc_get_enabled(struct cdns_rtc *crtc)
    {
    return !(readl(crtc.regs + CDNS_RTC_CTLR) & CDNS_RTC_CTLR_TIME_CAL);
    }
#[no_mangle]
unsafe extern "C" fn cdns_rtc_irq_handler(irq: c_int, id: *mut c_void) -> irqreturn_t {
    static irqreturn_t cdns_rtc_irq_handler(int irq, void *id)
    {
    struct device *dev = id;
    struct cdns_rtc *crtc = dev_get_drvdata(dev);
// Reading the register clears it
    if (!(readl(crtc.regs + CDNS_RTC_EFLR) & CDNS_RTC_AEI_ALRM))
    return IRQ_NONE;
    rtc_update_irq(crtc.rtc_dev, 1, RTC_IRQF | RTC_AF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cdns_rtc_time2reg(tm: *mut rtc_time) -> u32 {
    static u32 cdns_rtc_time2reg(struct rtc_time *tm)
    {
#[no_mangle]
pub unsafe extern "C" fn FIELD_PREP(_arg: CDNS_RTC_TIME_S, _arg: bin2bcd(tm->tm_sec)) -> return {
    return FIELD_PREP(CDNS_RTC_TIME_S,  bin2bcd(tm.tm_sec))
    | FIELD_PREP(CDNS_RTC_TIME_M,  bin2bcd(tm.tm_min))
    | FIELD_PREP(CDNS_RTC_TIME_HR, bin2bcd(tm.tm_hour));
    }
#[no_mangle]
unsafe extern "C" fn cdns_rtc_reg2time(reg: u32, tm: *mut rtc_time) {
    static void cdns_rtc_reg2time(u32 reg, struct rtc_time *tm)
    {
    tm.tm_sec  = bcd2bin(FIELD_GET(CDNS_RTC_TIME_S, reg));
    tm.tm_min  = bcd2bin(FIELD_GET(CDNS_RTC_TIME_M, reg));
    tm.tm_hour = bcd2bin(FIELD_GET(CDNS_RTC_TIME_HR, reg));
    }
#[no_mangle]
unsafe extern "C" fn cdns_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int cdns_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct cdns_rtc *crtc = dev_get_drvdata(dev);
    u32 reg;
// If the RTC is disabled, assume the values are invalid
    if (!cdns_rtc_get_enabled(crtc))
    return -EINVAL;
    cdns_rtc_set_enabled(crtc, false);
    reg = readl(crtc.regs + CDNS_RTC_TIMR);
    cdns_rtc_reg2time(reg, tm);
    reg = readl(crtc.regs + CDNS_RTC_CALR);
    tm.tm_mday = bcd2bin(FIELD_GET(CDNS_RTC_CAL_D, reg));
    tm.tm_mon  = bcd2bin(FIELD_GET(CDNS_RTC_CAL_M, reg)) - 1;
    tm.tm_year = bcd2bin(FIELD_GET(CDNS_RTC_CAL_Y, reg))
    + bcd2bin(FIELD_GET(CDNS_RTC_CAL_C, reg)) * 100 - 1900;
    tm.tm_wday = bcd2bin(FIELD_GET(CDNS_RTC_CAL_DAY, reg)) - 1;
    cdns_rtc_set_enabled(crtc, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int cdns_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct cdns_rtc *crtc = dev_get_drvdata(dev);
    u32 timr, calr, stsr;
    let mut ret: c_int = -EIO;
    let mut year: c_int = tm.tm_year + 1900;
    int tries;
    cdns_rtc_set_enabled(crtc, false);
    timr = cdns_rtc_time2reg(tm);
    calr = FIELD_PREP(CDNS_RTC_CAL_D, bin2bcd(tm.tm_mday))
    | FIELD_PREP(CDNS_RTC_CAL_M, bin2bcd(tm.tm_mon + 1))
    | FIELD_PREP(CDNS_RTC_CAL_Y, bin2bcd(year % 100))
    | FIELD_PREP(CDNS_RTC_CAL_C, bin2bcd(year / 100))
    | FIELD_PREP(CDNS_RTC_CAL_DAY, tm.tm_wday + 1);
// Update registers, check valid flags
    for (tries = 0; tries < CDNS_RTC_MAX_REGS_TRIES; tries++) {
    writel(timr, crtc.regs + CDNS_RTC_TIMR);
    writel(calr, crtc.regs + CDNS_RTC_CALR);
    stsr = readl(crtc.regs + CDNS_RTC_STSR);
    if ((stsr & CDNS_RTC_STSR_VT_VC) == CDNS_RTC_STSR_VT_VC) {
    ret = 0;
    break;
    }
    }
    cdns_rtc_set_enabled(crtc, true);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns_rtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int cdns_rtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct cdns_rtc *crtc = dev_get_drvdata(dev);
    if (enabled) {
    writel((CDNS_RTC_AEI_SEC | CDNS_RTC_AEI_MIN | CDNS_RTC_AEI_HOUR
    | CDNS_RTC_AEI_DATE | CDNS_RTC_AEI_MNTH),
    crtc.regs + CDNS_RTC_AENR);
    writel(CDNS_RTC_AEI_ALRM, crtc.regs + CDNS_RTC_IENR);
    } else {
    writel(0, crtc.regs + CDNS_RTC_AENR);
    writel(CDNS_RTC_AEI_ALRM, crtc.regs + CDNS_RTC_IDISR);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_rtc_read_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int cdns_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct cdns_rtc *crtc = dev_get_drvdata(dev);
    u32 reg;
    reg = readl(crtc.regs + CDNS_RTC_TIMAR);
    cdns_rtc_reg2time(reg, &alarm.time);
    reg = readl(crtc.regs + CDNS_RTC_CALAR);
    alarm.time.tm_mday = bcd2bin(FIELD_GET(CDNS_RTC_CAL_D, reg));
    alarm.time.tm_mon  = bcd2bin(FIELD_GET(CDNS_RTC_CAL_M, reg)) - 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_rtc_set_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int cdns_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct cdns_rtc *crtc = dev_get_drvdata(dev);
    let mut ret: c_int = -EIO;
    int tries;
    u32 timar, calar, stsr;
    cdns_rtc_alarm_irq_enable(dev, 0);
    timar = cdns_rtc_time2reg(&alarm.time);
    calar = FIELD_PREP(CDNS_RTC_CAL_D, bin2bcd(alarm.time.tm_mday))
    | FIELD_PREP(CDNS_RTC_CAL_M, bin2bcd(alarm.time.tm_mon + 1));
// Update registers, check valid alarm flags
    for (tries = 0; tries < CDNS_RTC_MAX_REGS_TRIES; tries++) {
    writel(timar, crtc.regs + CDNS_RTC_TIMAR);
    writel(calar, crtc.regs + CDNS_RTC_CALAR);
    stsr = readl(crtc.regs + CDNS_RTC_STSR);
    if ((stsr & CDNS_RTC_STSR_VTA_VCA) == CDNS_RTC_STSR_VTA_VCA) {
    ret = 0;
    break;
    }
    }
    if (!ret)
    cdns_rtc_alarm_irq_enable(dev, alarm.enabled);
    return ret;
    }
    static const struct rtc_class_ops cdns_rtc_ops = {
    .read_time	= cdns_rtc_read_time,
    .set_time	= cdns_rtc_set_time,
    .read_alarm	= cdns_rtc_read_alarm,
    .set_alarm	= cdns_rtc_set_alarm,
    .alarm_irq_enable = cdns_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn cdns_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int cdns_rtc_probe(struct platform_device *pdev)
    {
    struct cdns_rtc *crtc;
    int ret;
    unsigned long ref_clk_freq;
    crtc = devm_kzalloc(&pdev.dev, sizeof(*crtc), GFP_KERNEL);
    if (!crtc)
    return -ENOMEM;
    crtc.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(crtc.regs))
    return PTR_ERR(crtc.regs);
    crtc.irq = platform_get_irq(pdev, 0);
    if (crtc.irq < 0)
    return -EINVAL;
    crtc.pclk = devm_clk_get(&pdev.dev, "pclk");
    if (IS_ERR(crtc.pclk)) {
    ret = PTR_ERR(crtc.pclk);
    dev_err(&pdev.dev,
    "Failed to retrieve the peripheral clock, %d\n", ret);
    return ret;
    }
    crtc.ref_clk = devm_clk_get(&pdev.dev, "ref_clk");
    if (IS_ERR(crtc.ref_clk)) {
    ret = PTR_ERR(crtc.ref_clk);
    dev_err(&pdev.dev,
    "Failed to retrieve the reference clock, %d\n", ret);
    return ret;
    }
    crtc.rtc_dev = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(crtc.rtc_dev))
    return PTR_ERR(crtc.rtc_dev);
    platform_set_drvdata(pdev, crtc);
    ret = clk_prepare_enable(crtc.pclk);
    if (ret) {
    dev_err(&pdev.dev,
    "Failed to enable the peripheral clock, %d\n", ret);
    return ret;
    }
    ret = clk_prepare_enable(crtc.ref_clk);
    if (ret) {
    dev_err(&pdev.dev,
    "Failed to enable the reference clock, %d\n", ret);
    goto err_disable_pclk;
    }
    ref_clk_freq = clk_get_rate(crtc.ref_clk);
    if ((ref_clk_freq != 1) && (ref_clk_freq != 100)) {
    dev_err(&pdev.dev,
    "Invalid reference clock frequency %lu Hz.\n",
    ref_clk_freq);
    ret = -EINVAL;
    goto err_disable_ref_clk;
    }
    ret = devm_request_irq(&pdev.dev, crtc.irq,
    cdns_rtc_irq_handler, 0,
    dev_name(&pdev.dev), &pdev.dev);
    if (ret) {
    dev_err(&pdev.dev,
    "Failed to request interrupt for the device, %d\n",
    ret);
    goto err_disable_ref_clk;
    }
// The RTC supports 01.01.1900 - 31.12.2999
    crtc.rtc_dev.range_min = mktime64(1900,  1,  1,  0,  0,  0);
    crtc.rtc_dev.range_max = mktime64(2999, 12, 31, 23, 59, 59);
    crtc.rtc_dev.ops = &cdns_rtc_ops;
    device_init_wakeup(&pdev.dev, true);
// Always use 24-hour mode and keep the RTC values
    writel(0, crtc.regs + CDNS_RTC_HMR);
    writel(CDNS_RTC_KRTCR_KRTC, crtc.regs + CDNS_RTC_KRTCR);
    ret = devm_rtc_register_device(crtc.rtc_dev);
    if (ret)
    goto err_disable_wakeup;
    return 0;
    err_disable_wakeup:
    device_init_wakeup(&pdev.dev, false);
    err_disable_ref_clk:
    clk_disable_unprepare(crtc.ref_clk);
    err_disable_pclk:
    clk_disable_unprepare(crtc.pclk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns_rtc_remove(pdev: *mut platform_device) {
    static void cdns_rtc_remove(struct platform_device *pdev)
    {
    struct cdns_rtc *crtc = platform_get_drvdata(pdev);
    cdns_rtc_alarm_irq_enable(&pdev.dev, 0);
    device_init_wakeup(&pdev.dev, false);
    clk_disable_unprepare(crtc.pclk);
    clk_disable_unprepare(crtc.ref_clk);
    }

#[no_mangle]
unsafe extern "C" fn cdns_rtc_suspend(dev: *mut device) -> c_int {
    static int cdns_rtc_suspend(struct device *dev)
    {
    struct cdns_rtc *crtc = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    enable_irq_wake(crtc.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_rtc_resume(dev: *mut device) -> c_int {
    static int cdns_rtc_resume(struct device *dev)
    {
    struct cdns_rtc *crtc = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    disable_irq_wake(crtc.irq);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(cdns_rtc_pm_ops, cdns_rtc_suspend, cdns_rtc_resume);
    static const struct of_device_id cdns_rtc_of_match[] = {
    { .compatible = "cdns,rtc-r109v3" },
    { },
    };
    MODULE_DEVICE_TABLE(of, cdns_rtc_of_match);
    static struct platform_driver cdns_rtc_driver = {
    .driver = {
    .name = "cdns-rtc",
    .of_match_table = cdns_rtc_of_match,
    .pm = &cdns_rtc_pm_ops,
    },
    .probe = cdns_rtc_probe,
    .remove = cdns_rtc_remove,
    };
    module_platform_driver(cdns_rtc_driver);
    MODULE_AUTHOR("Jan Kotas <jank@cadence.com>");
    MODULE_DESCRIPTION("Cadence RTC driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:cdns-rtc");
