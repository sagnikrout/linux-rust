//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-mv.c
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
// Driver for the RTC in Marvell SoCs.
//

pub const RTC_TIME_REG_OFFS: c_int = 0;
pub const RTC_SECONDS_OFFS: c_int = 0;
pub const RTC_MINUTES_OFFS: c_int = 8;
pub const RTC_HOURS_OFFS: c_int = 16;
pub const RTC_WDAY_OFFS: c_int = 24;

pub const RTC_DATE_REG_OFFS: c_int = 4;
pub const RTC_MDAY_OFFS: c_int = 0;
pub const RTC_MONTH_OFFS: c_int = 8;
pub const RTC_YEAR_OFFS: c_int = 16;
pub const RTC_ALARM_TIME_REG_OFFS: c_int = 8;
pub const RTC_ALARM_DATE_REG_OFFS: c_uint = 0xc;

pub const RTC_ALARM_INTERRUPT_MASK_REG_OFFS: c_uint = 0x10;
pub const RTC_ALARM_INTERRUPT_CASUE_REG_OFFS: c_uint = 0x14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtc_plat_data {
    pub rtc: *mut rtc_device,
    pub ioaddr: *mut void __iomem,
    pub irq: c_int,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn mv_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int mv_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    void __iomem *ioaddr = pdata.ioaddr;
    u32	rtc_reg;
    rtc_reg = (bin2bcd(tm.tm_sec) << RTC_SECONDS_OFFS) |
    (bin2bcd(tm.tm_min) << RTC_MINUTES_OFFS) |
    (bin2bcd(tm.tm_hour) << RTC_HOURS_OFFS) |
    (bin2bcd(tm.tm_wday) << RTC_WDAY_OFFS);
    writel(rtc_reg, ioaddr + RTC_TIME_REG_OFFS);
    rtc_reg = (bin2bcd(tm.tm_mday) << RTC_MDAY_OFFS) |
    (bin2bcd(tm.tm_mon + 1) << RTC_MONTH_OFFS) |
    (bin2bcd(tm.tm_year - 100) << RTC_YEAR_OFFS);
    writel(rtc_reg, ioaddr + RTC_DATE_REG_OFFS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int mv_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    void __iomem *ioaddr = pdata.ioaddr;
    u32	rtc_time, rtc_date;
    unsigned int year, month, day, hour, minute, second, wday;
    rtc_time = readl(ioaddr + RTC_TIME_REG_OFFS);
    rtc_date = readl(ioaddr + RTC_DATE_REG_OFFS);
    second = rtc_time & 0x7f;
    minute = (rtc_time >> RTC_MINUTES_OFFS) & 0x7f;
    hour = (rtc_time >> RTC_HOURS_OFFS) & 0x3f; /* assume 24 hour mode */
    wday = (rtc_time >> RTC_WDAY_OFFS) & 0x7;
    day = rtc_date & 0x3f;
    month = (rtc_date >> RTC_MONTH_OFFS) & 0x3f;
    year = (rtc_date >> RTC_YEAR_OFFS) & 0xff;
    tm.tm_sec = bcd2bin(second);
    tm.tm_min = bcd2bin(minute);
    tm.tm_hour = bcd2bin(hour);
    tm.tm_mday = bcd2bin(day);
    tm.tm_wday = bcd2bin(wday);
    tm.tm_mon = bcd2bin(month) - 1;
// hw counts from year 2000, but tm_year is relative to 1900
    tm.tm_year = bcd2bin(year) + 100;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv_rtc_read_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int mv_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    void __iomem *ioaddr = pdata.ioaddr;
    u32	rtc_time, rtc_date;
    unsigned int year, month, day, hour, minute, second, wday;
    rtc_time = readl(ioaddr + RTC_ALARM_TIME_REG_OFFS);
    rtc_date = readl(ioaddr + RTC_ALARM_DATE_REG_OFFS);
    second = rtc_time & 0x7f;
    minute = (rtc_time >> RTC_MINUTES_OFFS) & 0x7f;
    hour = (rtc_time >> RTC_HOURS_OFFS) & 0x3f; /* assume 24 hour mode */
    wday = (rtc_time >> RTC_WDAY_OFFS) & 0x7;
    day = rtc_date & 0x3f;
    month = (rtc_date >> RTC_MONTH_OFFS) & 0x3f;
    year = (rtc_date >> RTC_YEAR_OFFS) & 0xff;
    alm.time.tm_sec = bcd2bin(second);
    alm.time.tm_min = bcd2bin(minute);
    alm.time.tm_hour = bcd2bin(hour);
    alm.time.tm_mday = bcd2bin(day);
    alm.time.tm_wday = bcd2bin(wday);
    alm.time.tm_mon = bcd2bin(month) - 1;
// hw counts from year 2000, but tm_year is relative to 1900
    alm.time.tm_year = bcd2bin(year) + 100;
    alm.enabled = !!readl(ioaddr + RTC_ALARM_INTERRUPT_MASK_REG_OFFS);
    return rtc_valid_tm(&alm.time);
    }
#[no_mangle]
unsafe extern "C" fn mv_rtc_set_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int mv_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    void __iomem *ioaddr = pdata.ioaddr;
    let mut rtc_reg: u32 = 0;
    if (alm.time.tm_sec >= 0)
    rtc_reg |= (RTC_ALARM_VALID | bin2bcd(alm.time.tm_sec))
    << RTC_SECONDS_OFFS;
    if (alm.time.tm_min >= 0)
    rtc_reg |= (RTC_ALARM_VALID | bin2bcd(alm.time.tm_min))
    << RTC_MINUTES_OFFS;
    if (alm.time.tm_hour >= 0)
    rtc_reg |= (RTC_ALARM_VALID | bin2bcd(alm.time.tm_hour))
    << RTC_HOURS_OFFS;
    writel(rtc_reg, ioaddr + RTC_ALARM_TIME_REG_OFFS);
    if (alm.time.tm_mday >= 0)
    rtc_reg = (RTC_ALARM_VALID | bin2bcd(alm.time.tm_mday))
    << RTC_MDAY_OFFS;
    else
    rtc_reg = 0;
    if (alm.time.tm_mon >= 0)
    rtc_reg |= (RTC_ALARM_VALID | bin2bcd(alm.time.tm_mon + 1))
    << RTC_MONTH_OFFS;
    if (alm.time.tm_year >= 0)
    rtc_reg |= (RTC_ALARM_VALID | bin2bcd(alm.time.tm_year - 100))
    << RTC_YEAR_OFFS;
    writel(rtc_reg, ioaddr + RTC_ALARM_DATE_REG_OFFS);
    writel(0, ioaddr + RTC_ALARM_INTERRUPT_CASUE_REG_OFFS);
    writel(alm.enabled ? 1 : 0,
    ioaddr + RTC_ALARM_INTERRUPT_MASK_REG_OFFS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv_rtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int mv_rtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    void __iomem *ioaddr = pdata.ioaddr;
    if (pdata.irq < 0)
    return -EINVAL; /* fall back into rtc-dev's emulation */
    if (enabled)
    writel(1, ioaddr + RTC_ALARM_INTERRUPT_MASK_REG_OFFS);
    else
    writel(0, ioaddr + RTC_ALARM_INTERRUPT_MASK_REG_OFFS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv_rtc_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t mv_rtc_interrupt(int irq, void *data)
    {
    struct rtc_plat_data *pdata = data;
    void __iomem *ioaddr = pdata.ioaddr;
// alarm irq?
    if (!readl(ioaddr + RTC_ALARM_INTERRUPT_CASUE_REG_OFFS))
    return IRQ_NONE;
// clear interrupt
    writel(0, ioaddr + RTC_ALARM_INTERRUPT_CASUE_REG_OFFS);
    rtc_update_irq(pdata.rtc, 1, RTC_IRQF | RTC_AF);
    return IRQ_HANDLED;
    }
    static const struct rtc_class_ops mv_rtc_ops = {
    .read_time	= mv_rtc_read_time,
    .set_time	= mv_rtc_set_time,
    .read_alarm	= mv_rtc_read_alarm,
    .set_alarm	= mv_rtc_set_alarm,
    .alarm_irq_enable = mv_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn mv_rtc_probe(pdev: *mut platform_device) -> int __init {
    static int __init mv_rtc_probe(struct platform_device *pdev)
    {
    struct rtc_plat_data *pdata;
    u32 rtc_time;
    let mut ret: c_int = 0;
    pdata = devm_kzalloc(&pdev.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    pdata.ioaddr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pdata.ioaddr))
    return PTR_ERR(pdata.ioaddr);
    pdata.clk = devm_clk_get_optional_prepared(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(pdata.clk))
    return PTR_ERR(pdata.clk);
// make sure the 24 hour mode is enabled
    rtc_time = readl(pdata.ioaddr + RTC_TIME_REG_OFFS);
    if (rtc_time & RTC_HOURS_12H_MODE) {
    dev_err(&pdev.dev, "12 Hour mode is enabled but not supported.\n");
    return  -EINVAL;
    }
// make sure it is actually functional
    if (rtc_time == 0x01000000) {
    ssleep(1);
    rtc_time = readl(pdata.ioaddr + RTC_TIME_REG_OFFS);
    if (rtc_time == 0x01000000) {
    dev_err(&pdev.dev, "internal RTC not ticking\n");
    return -ENODEV;
    }
    }
    pdata.irq = platform_get_irq(pdev, 0);
    platform_set_drvdata(pdev, pdata);
    pdata.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(pdata.rtc)) {
    return PTR_ERR(pdata.rtc);
    }
    if (pdata.irq >= 0) {
    writel(0, pdata.ioaddr + RTC_ALARM_INTERRUPT_MASK_REG_OFFS);
    if (devm_request_irq(&pdev.dev, pdata.irq, mv_rtc_interrupt,
    IRQF_SHARED,
    pdev.name, pdata) < 0) {
    dev_warn(&pdev.dev, "interrupt not available.\n");
    pdata.irq = -1;
    }
    }
    if (pdata.irq >= 0)
    devm_device_init_wakeup(&pdev.dev);
    else
    clear_bit(RTC_FEATURE_ALARM, pdata.rtc.features);
    pdata.rtc.ops = &mv_rtc_ops;
    pdata.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    pdata.rtc.range_max = RTC_TIMESTAMP_END_2099;
    ret = devm_rtc_register_device(pdata.rtc);
    if (!ret)
    return 0;
    return ret;
    }

    static const struct of_device_id rtc_mv_of_match_table[] = {
    { .compatible = "marvell,orion-rtc", },
    {}
    };
    MODULE_DEVICE_TABLE(of, rtc_mv_of_match_table);

#[no_mangle]
unsafe extern "C" fn mv_rtc_suspend(dev: *mut device) -> c_int {
    static int mv_rtc_suspend(struct device *dev)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    if (device_may_wakeup(dev) && pdata.irq >= 0)
    enable_irq_wake(pdata.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv_rtc_resume(dev: *mut device) -> c_int {
    static int mv_rtc_resume(struct device *dev)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    if (device_may_wakeup(dev) && pdata.irq >= 0)
    disable_irq_wake(pdata.irq);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(mv_rtc_pm_ops, mv_rtc_suspend, mv_rtc_resume);
    static struct platform_driver mv_rtc_driver = {
    .driver		= {
    .name	= "rtc-mv",
    .of_match_table = of_match_ptr(rtc_mv_of_match_table),
    .pm	= &mv_rtc_pm_ops,
    },
    };
    module_platform_driver_probe(mv_rtc_driver, mv_rtc_probe);
    MODULE_AUTHOR("Saeed Bishara <saeed@marvell.com>");
    MODULE_DESCRIPTION("Marvell RTC driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:rtc-mv");
