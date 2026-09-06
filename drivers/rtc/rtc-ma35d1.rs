//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ma35d1.c
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
// RTC driver for Nuvoton MA35D1
//
// Copyright (C) 2023 Nuvoton Technology Corp.
//

// MA35D1 RTC Control Registers
pub const MA35_REG_RTC_INIT: c_uint = 0x00;
pub const MA35_REG_RTC_SINFASTS: c_uint = 0x04;
pub const MA35_REG_RTC_FREQADJ: c_uint = 0x08;
pub const MA35_REG_RTC_TIME: c_uint = 0x0c;
pub const MA35_REG_RTC_CAL: c_uint = 0x10;
pub const MA35_REG_RTC_CLKFMT: c_uint = 0x14;
pub const MA35_REG_RTC_WEEKDAY: c_uint = 0x18;
pub const MA35_REG_RTC_TALM: c_uint = 0x1c;
pub const MA35_REG_RTC_CALM: c_uint = 0x20;
pub const MA35_REG_RTC_LEAPYEAR: c_uint = 0x24;
pub const MA35_REG_RTC_INTEN: c_uint = 0x28;
pub const MA35_REG_RTC_INTSTS: c_uint = 0x2c;
// register MA35_REG_RTC_INIT

pub const RTC_INIT_MAGIC_CODE: c_uint = 0xa5eb1357;
// register MA35_REG_RTC_CLKFMT

// register MA35_REG_RTC_INTEN

// register MA35_REG_RTC_INTSTS

pub const RTC_INIT_TIMEOUT: c_int = 250;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ma35_rtc {
    pub irq_num: c_int,
    pub rtc_reg: *mut void __iomem,
    pub rtcdev: *mut rtc_device,
}

#[no_mangle]
unsafe extern "C" fn rtc_reg_read(p: *mut ma35_rtc, offset: u32) -> u32 {
    static u32 rtc_reg_read(struct ma35_rtc *p, u32 offset)
    {
    return __raw_readl(p.rtc_reg + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn rtc_reg_write(p: *mut ma35_rtc, offset: u32, value: u32) {
    static inline void rtc_reg_write(struct ma35_rtc *p, u32 offset, u32 value)
    {
    __raw_writel(value, p.rtc_reg + offset);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_rtc_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ma35d1_rtc_interrupt(int irq, void *data)
    {
    struct ma35_rtc *rtc = (struct ma35_rtc *)data;
    let mut events: c_ulong = 0, rtc_irq;
    rtc_irq = rtc_reg_read(rtc, MA35_REG_RTC_INTSTS);
    if (rtc_irq & RTC_INTSTS_ALMIF) {
    rtc_reg_write(rtc, MA35_REG_RTC_INTSTS, RTC_INTSTS_ALMIF);
    events |= RTC_AF | RTC_IRQF;
    }
    rtc_update_irq(rtc.rtcdev, 1, events);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_rtc_init(rtc: *mut ma35_rtc, ms_timeout: u32) -> c_int {
    static int ma35d1_rtc_init(struct ma35_rtc *rtc, u32 ms_timeout)
    {
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(ms_timeout);
    do {
    if (rtc_reg_read(rtc, MA35_REG_RTC_INIT) & RTC_INIT_ACTIVE)
    return 0;
    rtc_reg_write(rtc, MA35_REG_RTC_INIT, RTC_INIT_MAGIC_CODE);
    mdelay(1);
    } while (time_before(jiffies, timeout));
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_alarm_irq_enable(dev: *mut device, enabled: u32) -> c_int {
    static int ma35d1_alarm_irq_enable(struct device *dev, u32 enabled)
    {
    struct ma35_rtc *rtc = dev_get_drvdata(dev);
    u32 reg_ien;
    reg_ien = rtc_reg_read(rtc, MA35_REG_RTC_INTEN);
    if (enabled)
    rtc_reg_write(rtc, MA35_REG_RTC_INTEN, reg_ien | RTC_INTEN_ALMIEN);
    else
    rtc_reg_write(rtc, MA35_REG_RTC_INTEN, reg_ien & ~RTC_INTEN_ALMIEN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ma35d1_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct ma35_rtc *rtc = dev_get_drvdata(dev);
    u32 time, cal, wday;
    do {
    time = rtc_reg_read(rtc, MA35_REG_RTC_TIME);
    cal  = rtc_reg_read(rtc, MA35_REG_RTC_CAL);
    wday = rtc_reg_read(rtc, MA35_REG_RTC_WEEKDAY);
    } while (time != rtc_reg_read(rtc, MA35_REG_RTC_TIME) ||
    cal != rtc_reg_read(rtc, MA35_REG_RTC_CAL));
    tm.tm_mday = bcd2bin(cal >> 0);
    tm.tm_wday = wday;
    tm.tm_mon = bcd2bin(cal >> 8);
    tm.tm_mon = tm.tm_mon - 1;
    tm.tm_year = bcd2bin(cal >> 16) + 100;
    tm.tm_sec = bcd2bin(time >> 0);
    tm.tm_min = bcd2bin(time >> 8);
    tm.tm_hour = bcd2bin(time >> 16);
    return rtc_valid_tm(tm);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ma35d1_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct ma35_rtc *rtc = dev_get_drvdata(dev);
    u32 val;
    val = bin2bcd(tm.tm_mday) << 0 | bin2bcd(tm.tm_mon + 1) << 8 |
    bin2bcd(tm.tm_year - 100) << 16;
    rtc_reg_write(rtc, MA35_REG_RTC_CAL, val);
    val = bin2bcd(tm.tm_sec) << 0 | bin2bcd(tm.tm_min) << 8 |
    bin2bcd(tm.tm_hour) << 16;
    rtc_reg_write(rtc, MA35_REG_RTC_TIME, val);
    val = tm.tm_wday;
    rtc_reg_write(rtc, MA35_REG_RTC_WEEKDAY, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int ma35d1_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct ma35_rtc *rtc = dev_get_drvdata(dev);
    u32 talm, calm;
    talm = rtc_reg_read(rtc, MA35_REG_RTC_TALM);
    calm = rtc_reg_read(rtc, MA35_REG_RTC_CALM);
    alrm.time.tm_mday = bcd2bin(calm >> 0);
    alrm.time.tm_mon = bcd2bin(calm >> 8);
    alrm.time.tm_mon = alrm.time.tm_mon - 1;
    alrm.time.tm_year = bcd2bin(calm >> 16) + 100;
    alrm.time.tm_sec = bcd2bin(talm >> 0);
    alrm.time.tm_min = bcd2bin(talm >> 8);
    alrm.time.tm_hour = bcd2bin(talm >> 16);
    return rtc_valid_tm(&alrm.time);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int ma35d1_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct ma35_rtc *rtc = dev_get_drvdata(dev);
    unsigned long val;
    val = bin2bcd(alrm.time.tm_mday) << 0 | bin2bcd(alrm.time.tm_mon + 1) << 8 |
    bin2bcd(alrm.time.tm_year - 100) << 16;
    rtc_reg_write(rtc, MA35_REG_RTC_CALM, val);
    val = bin2bcd(alrm.time.tm_sec) << 0 | bin2bcd(alrm.time.tm_min) << 8 |
    bin2bcd(alrm.time.tm_hour) << 16;
    rtc_reg_write(rtc, MA35_REG_RTC_TALM, val);
    ma35d1_alarm_irq_enable(dev, alrm.enabled);
    return 0;
    }
    static const struct rtc_class_ops ma35d1_rtc_ops = {
    .read_time = ma35d1_rtc_read_time,
    .set_time = ma35d1_rtc_set_time,
    .read_alarm = ma35d1_rtc_read_alarm,
    .set_alarm = ma35d1_rtc_set_alarm,
    .alarm_irq_enable = ma35d1_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn ma35d1_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int ma35d1_rtc_probe(struct platform_device *pdev)
    {
    struct ma35_rtc *rtc;
    struct clk *clk;
    int ret;
    rtc = devm_kzalloc(&pdev.dev, sizeof(*rtc), GFP_KERNEL);
    if (!rtc)
    return -ENOMEM;
    rtc.rtc_reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rtc.rtc_reg))
    return PTR_ERR(rtc.rtc_reg);
    clk = of_clk_get(pdev.dev.of_node, 0);
    if (IS_ERR(clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(clk), "failed to find rtc clock\n");
    ret = clk_prepare_enable(clk);
    if (ret)
    return ret;
    if (!(rtc_reg_read(rtc, MA35_REG_RTC_INIT) & RTC_INIT_ACTIVE)) {
    ret = ma35d1_rtc_init(rtc, RTC_INIT_TIMEOUT);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "rtc init failed\n");
    }
    rtc.irq_num = platform_get_irq(pdev, 0);
    ret = devm_request_irq(&pdev.dev, rtc.irq_num, ma35d1_rtc_interrupt,
    IRQF_NO_SUSPEND, "ma35d1rtc", rtc);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Failed to request rtc irq\n");
    platform_set_drvdata(pdev, rtc);
    device_init_wakeup(&pdev.dev, true);
    rtc.rtcdev = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc.rtcdev))
    return PTR_ERR(rtc.rtcdev);
    rtc.rtcdev.ops = &ma35d1_rtc_ops;
    rtc.rtcdev.range_min = RTC_TIMESTAMP_BEGIN_2000;
    rtc.rtcdev.range_max = RTC_TIMESTAMP_END_2099;
    ret = devm_rtc_register_device(rtc.rtcdev);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Failed to register rtc device\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_rtc_suspend(pdev: *mut platform_device, state: pm_message_t) -> c_int {
    static int ma35d1_rtc_suspend(struct platform_device *pdev, pm_message_t state)
    {
    struct ma35_rtc *rtc = platform_get_drvdata(pdev);
    if (device_may_wakeup(&pdev.dev))
    enable_irq_wake(rtc.irq_num);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_rtc_resume(pdev: *mut platform_device) -> c_int {
    static int ma35d1_rtc_resume(struct platform_device *pdev)
    {
    struct ma35_rtc *rtc = platform_get_drvdata(pdev);
    if (device_may_wakeup(&pdev.dev))
    disable_irq_wake(rtc.irq_num);
    return 0;
    }
    static const struct of_device_id ma35d1_rtc_of_match[] = {
    { .compatible = "nuvoton,ma35d1-rtc", },
    {},
    };
    MODULE_DEVICE_TABLE(of, ma35d1_rtc_of_match);
    static struct platform_driver ma35d1_rtc_driver = {
    .suspend    = ma35d1_rtc_suspend,
    .resume     = ma35d1_rtc_resume,
    .probe      = ma35d1_rtc_probe,
    .driver		= {
    .name	= "rtc-ma35d1",
    .of_match_table = ma35d1_rtc_of_match,
    },
    };
    module_platform_driver(ma35d1_rtc_driver);
    MODULE_AUTHOR("Ming-Jen Chen <mjchen@nuvoton.com>");
    MODULE_DESCRIPTION("MA35D1 RTC driver");
    MODULE_LICENSE("GPL");
