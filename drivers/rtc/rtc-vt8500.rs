//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-vt8500.c
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
// drivers/rtc/rtc-vt8500.c
//
// Copyright (C) 2010 Alexey Charkov <alchark@gmail.com>
//
// Based on rtc-pxa.c
//

//
// Register definitions
//
pub const VT8500_RTC_TS: c_uint = 0x00	/* Time set */;
pub const VT8500_RTC_DS: c_uint = 0x04	/* Date set */;
pub const VT8500_RTC_AS: c_uint = 0x08	/* Alarm set */;
pub const VT8500_RTC_CR: c_uint = 0x0c	/* Control */;
pub const VT8500_RTC_TR: c_uint = 0x10	/* Time read */;
pub const VT8500_RTC_DR: c_uint = 0x14	/* Date read */;
pub const VT8500_RTC_WS: c_uint = 0x18	/* Write status */;
pub const VT8500_RTC_CL: c_uint = 0x20	/* Calibration */;
pub const VT8500_RTC_IS: c_uint = 0x24	/* Interrupt status */;
pub const VT8500_RTC_ST: c_uint = 0x28	/* Status */;

pub const DATE_CENTURY_S: c_int = 19;
pub const DATE_YEAR_S: c_int = 11;

pub const DATE_MONTH_S: c_int = 6;

pub const DATE_DAY_MASK: c_uint = 0x3f;
pub const TIME_DOW_S: c_int = 20;

pub const TIME_HOUR_S: c_int = 14;

pub const TIME_MIN_S: c_int = 7;

pub const TIME_SEC_MASK: c_uint = 0x7f;
pub const ALARM_DAY_S: c_int = 20;

    | ALARM_HOUR_BIT \
    | ALARM_MIN_BIT \
    | ALARM_SEC_BIT)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt8500_rtc {
    pub regbase: *mut void __iomem,
    pub irq_alarm: c_int,
    pub rtc: *mut rtc_device,
    pub /: *mut *mut spinlock_t lock; / Protects this structure,
}

#[no_mangle]
unsafe extern "C" fn vt8500_rtc_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t vt8500_rtc_irq(int irq, void *dev_id)
    {
    struct vt8500_rtc *vt8500_rtc = dev_id;
    u32 isr;
    let mut events: c_ulong = 0;
    spin_lock(&vt8500_rtc.lock);
// clear interrupt sources
    isr = readl(vt8500_rtc.regbase + VT8500_RTC_IS);
    writel(isr, vt8500_rtc.regbase + VT8500_RTC_IS);
    spin_unlock(&vt8500_rtc.lock);
    if (isr & VT8500_RTC_IS_ALARM)
    events |= RTC_AF | RTC_IRQF;
    rtc_update_irq(vt8500_rtc.rtc, 1, events);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn vt8500_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int vt8500_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct vt8500_rtc *vt8500_rtc = dev_get_drvdata(dev);
    u32 date, time;
    date = readl(vt8500_rtc.regbase + VT8500_RTC_DR);
    time = readl(vt8500_rtc.regbase + VT8500_RTC_TR);
    tm.tm_sec = bcd2bin(time & TIME_SEC_MASK);
    tm.tm_min = bcd2bin((time & TIME_MIN_MASK) >> TIME_MIN_S);
    tm.tm_hour = bcd2bin((time & TIME_HOUR_MASK) >> TIME_HOUR_S);
    tm.tm_mday = bcd2bin(date & DATE_DAY_MASK);
    tm.tm_mon = bcd2bin((date & DATE_MONTH_MASK) >> DATE_MONTH_S) - 1;
    tm.tm_year = bcd2bin((date & DATE_YEAR_MASK) >> DATE_YEAR_S)
    + ((date >> DATE_CENTURY_S) & 1 ? 200 : 100);
    tm.tm_wday = (time & TIME_DOW_MASK) >> TIME_DOW_S;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vt8500_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int vt8500_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct vt8500_rtc *vt8500_rtc = dev_get_drvdata(dev);
    writel((bin2bcd(tm.tm_year % 100) << DATE_YEAR_S)
    | (bin2bcd(tm.tm_mon + 1) << DATE_MONTH_S)
    | (bin2bcd(tm.tm_mday))
    | ((tm.tm_year >= 200) << DATE_CENTURY_S),
    vt8500_rtc.regbase + VT8500_RTC_DS);
    writel((bin2bcd(tm.tm_wday) << TIME_DOW_S)
    | (bin2bcd(tm.tm_hour) << TIME_HOUR_S)
    | (bin2bcd(tm.tm_min) << TIME_MIN_S)
    | (bin2bcd(tm.tm_sec)),
    vt8500_rtc.regbase + VT8500_RTC_TS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vt8500_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int vt8500_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct vt8500_rtc *vt8500_rtc = dev_get_drvdata(dev);
    u32 isr, alarm;
    alarm = readl(vt8500_rtc.regbase + VT8500_RTC_AS);
    isr = readl(vt8500_rtc.regbase + VT8500_RTC_IS);
    alrm.time.tm_mday = bcd2bin((alarm & ALARM_DAY_MASK) >> ALARM_DAY_S);
    alrm.time.tm_hour = bcd2bin((alarm & TIME_HOUR_MASK) >> TIME_HOUR_S);
    alrm.time.tm_min = bcd2bin((alarm & TIME_MIN_MASK) >> TIME_MIN_S);
    alrm.time.tm_sec = bcd2bin((alarm & TIME_SEC_MASK));
    alrm.enabled = (alarm & ALARM_ENABLE_MASK) ? 1 : 0;
    alrm.pending = (isr & VT8500_RTC_IS_ALARM) ? 1 : 0;
    return rtc_valid_tm(&alrm.time);
    }
#[no_mangle]
unsafe extern "C" fn vt8500_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int vt8500_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct vt8500_rtc *vt8500_rtc = dev_get_drvdata(dev);
    writel((alrm.enabled ? ALARM_ENABLE_MASK : 0)
    | (bin2bcd(alrm.time.tm_mday) << ALARM_DAY_S)
    | (bin2bcd(alrm.time.tm_hour) << TIME_HOUR_S)
    | (bin2bcd(alrm.time.tm_min) << TIME_MIN_S)
    | (bin2bcd(alrm.time.tm_sec)),
    vt8500_rtc.regbase + VT8500_RTC_AS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vt8500_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int vt8500_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct vt8500_rtc *vt8500_rtc = dev_get_drvdata(dev);
    let mut tmp: c_ulong = readl(vt8500_rtc.regbase + VT8500_RTC_AS);
    if (enabled)
    tmp |= ALARM_ENABLE_MASK;
    else
    tmp &= ~ALARM_ENABLE_MASK;
    writel(tmp, vt8500_rtc.regbase + VT8500_RTC_AS);
    return 0;
    }
    static const struct rtc_class_ops vt8500_rtc_ops = {
    .read_time = vt8500_rtc_read_time,
    .set_time = vt8500_rtc_set_time,
    .read_alarm = vt8500_rtc_read_alarm,
    .set_alarm = vt8500_rtc_set_alarm,
    .alarm_irq_enable = vt8500_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn vt8500_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int vt8500_rtc_probe(struct platform_device *pdev)
    {
    struct vt8500_rtc *vt8500_rtc;
    int ret;
    vt8500_rtc = devm_kzalloc(&pdev.dev,
    sizeof(struct vt8500_rtc), GFP_KERNEL);
    if (!vt8500_rtc)
    return -ENOMEM;
    spin_lock_init(&vt8500_rtc.lock);
    platform_set_drvdata(pdev, vt8500_rtc);
    vt8500_rtc.irq_alarm = platform_get_irq(pdev, 0);
    if (vt8500_rtc.irq_alarm < 0)
    return vt8500_rtc.irq_alarm;
    vt8500_rtc.regbase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(vt8500_rtc.regbase))
    return PTR_ERR(vt8500_rtc.regbase);
// Enable RTC and set it to 24-hour mode
    writel(VT8500_RTC_CR_ENABLE,
    vt8500_rtc.regbase + VT8500_RTC_CR);
    vt8500_rtc.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(vt8500_rtc.rtc))
    return PTR_ERR(vt8500_rtc.rtc);
    vt8500_rtc.rtc.ops = &vt8500_rtc_ops;
    vt8500_rtc.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    vt8500_rtc.rtc.range_max = RTC_TIMESTAMP_END_2199;
    ret = devm_request_irq(&pdev.dev, vt8500_rtc.irq_alarm,
    vt8500_rtc_irq, 0, "rtc alarm", vt8500_rtc);
    if (ret < 0) {
    dev_err(&pdev.dev, "can't get irq %i, err %d\n",
    vt8500_rtc.irq_alarm, ret);
    return ret;
    }
    return devm_rtc_register_device(vt8500_rtc.rtc);
    }
#[no_mangle]
unsafe extern "C" fn vt8500_rtc_remove(pdev: *mut platform_device) {
    static void vt8500_rtc_remove(struct platform_device *pdev)
    {
    struct vt8500_rtc *vt8500_rtc = platform_get_drvdata(pdev);
// Disable alarm matching
    writel(0, vt8500_rtc.regbase + VT8500_RTC_IS);
    }
    static const struct of_device_id wmt_dt_ids[] = {
    { .compatible = "via,vt8500-rtc", },
    {}
    };
    MODULE_DEVICE_TABLE(of, wmt_dt_ids);
    static struct platform_driver vt8500_rtc_driver = {
    .probe		= vt8500_rtc_probe,
    .remove		= vt8500_rtc_remove,
    .driver		= {
    .name	= "vt8500-rtc",
    .of_match_table = wmt_dt_ids,
    },
    };
    module_platform_driver(vt8500_rtc_driver);
    MODULE_AUTHOR("Alexey Charkov <alchark@gmail.com>");
    MODULE_DESCRIPTION("VIA VT8500 SoC Realtime Clock Driver (RTC)");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:vt8500-rtc");
