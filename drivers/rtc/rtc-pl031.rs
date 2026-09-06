//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-pl031.c
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
// drivers/rtc/rtc-pl031.c
//
// Real Time Clock interface for ARM AMBA PrimeCell 031 RTC
//
// Author: Deepak Saxena <dsaxena@plexity.net>
//
// Copyright 2006 (c) MontaVista Software, Inc.
//
// Author: Mian Yousaf Kaukab <mian.yousaf.kaukab@stericsson.com>
// Copyright 2010 (c) ST-Ericsson AB
//

//
// Register definitions
//
pub const RTC_DR: c_uint = 0x00	/* Data read register */;
pub const RTC_MR: c_uint = 0x04	/* Match register */;
pub const RTC_LR: c_uint = 0x08	/* Data load register */;
pub const RTC_CR: c_uint = 0x0c	/* Control register */;
pub const RTC_IMSC: c_uint = 0x10	/* Interrupt mask and set register */;
pub const RTC_RIS: c_uint = 0x14	/* Raw interrupt status register */;
pub const RTC_MIS: c_uint = 0x18	/* Masked interrupt status register */;
pub const RTC_ICR: c_uint = 0x1c	/* Interrupt clear register */;
// ST variants have additional timer functionality
pub const RTC_TDR: c_uint = 0x20	/* Timer data read register */;
pub const RTC_TLR: c_uint = 0x24	/* Timer data load register */;
pub const RTC_TCR: c_uint = 0x28	/* Timer control register */;
pub const RTC_YDR: c_uint = 0x30	/* Year data read register */;
pub const RTC_YMR: c_uint = 0x34	/* Year match register */;
pub const RTC_YLR: c_uint = 0x38	/* Year data load register */;

// Common bit definitions for Interrupt status and control registers

// Common bit definations for ST v2 for reading/writing time
pub const RTC_SEC_SHIFT: c_int = 0;

pub const RTC_MIN_SHIFT: c_int = 6;

pub const RTC_HOUR_SHIFT: c_int = 12;

pub const RTC_WDAY_SHIFT: c_int = 17;

pub const RTC_MDAY_SHIFT: c_int = 20;

pub const RTC_MON_SHIFT: c_int = 25;

pub const RTC_TIMER_FREQ: c_int = 32768;
//
// struct pl031_vendor_data - per-vendor variations
// @ops: the vendor-specific operations used on this silicon version
// @clockwatch: if this is an ST Microelectronics silicon version with a
// clockwatch function
// @st_weekday: if this is an ST Microelectronics silicon version that need
// the weekday fix
// @irqflags: special IRQ flags per variant
// @range_min: minimum date/time supported by the RTC
// @range_max: maximum date/time supported by the RTC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pl031_vendor_data {
    pub ops: rtc_class_ops,
    pub clockwatch: bool,
    pub st_weekday: bool,
    pub irqflags: c_ulong,
    pub range_min: time64_t,
    pub range_max: timeu64_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pl031_local {
    pub vendor: *mut pl031_vendor_data,
    pub rtc: *mut rtc_device,
    pub base: *mut void __iomem,
}

    static int pl031_alarm_irq_enable(struct device *dev,
    unsigned int enabled)
    {
    struct pl031_local *ldata = dev_get_drvdata(dev);
    unsigned long imsc;
// Clear any pending alarm interrupts.
    writel(RTC_BIT_AI, ldata.base + RTC_ICR);
    imsc = readl(ldata.base + RTC_IMSC);
    if (enabled == 1)
    writel(imsc | RTC_BIT_AI, ldata.base + RTC_IMSC);
    else
    writel(imsc & ~RTC_BIT_AI, ldata.base + RTC_IMSC);
    return 0;
    }
//
// Convert Gregorian date to ST v2 RTC format.
//
    static int pl031_stv2_tm_to_time(struct device *dev,
    struct rtc_time *tm, unsigned long *st_time,
    unsigned long *bcd_year)
    {
    let mut year: c_int = tm.tm_year + 1900;
    let mut wday: c_int = tm.tm_wday;
// wday masking is not working in hardware so wday must be valid
    if (wday < -1 || wday > 6) {
    dev_err(dev, "invalid wday value %d\n", tm.tm_wday);
    return -EINVAL;
    } else if (wday == -1) {
// wday is not provided, calculate it here
    struct rtc_time calc_tm;
    rtc_time64_to_tm(rtc_tm_to_time64(tm), &calc_tm);
    wday = calc_tm.tm_wday;
    }
// bcd_year = (bin2bcd(year % 100) | bin2bcd(year / 100) << 8);
// st_time = ((tm->tm_mon + 1) << RTC_MON_SHIFT)
    |	(tm.tm_mday << RTC_MDAY_SHIFT)
    |	((wday + 1) << RTC_WDAY_SHIFT)
    |	(tm.tm_hour << RTC_HOUR_SHIFT)
    |	(tm.tm_min << RTC_MIN_SHIFT)
    |	(tm.tm_sec << RTC_SEC_SHIFT);
    return 0;
    }
//
// Convert ST v2 RTC format to Gregorian date.
//
    static int pl031_stv2_time_to_tm(unsigned long st_time, unsigned long bcd_year,
    struct rtc_time *tm)
    {
    tm.tm_year = bcd2bin(bcd_year) + (bcd2bin(bcd_year >> 8) * 100);
    tm.tm_mon  = ((st_time & RTC_MON_MASK) >> RTC_MON_SHIFT) - 1;
    tm.tm_mday = ((st_time & RTC_MDAY_MASK) >> RTC_MDAY_SHIFT);
    tm.tm_wday = ((st_time & RTC_WDAY_MASK) >> RTC_WDAY_SHIFT) - 1;
    tm.tm_hour = ((st_time & RTC_HOUR_MASK) >> RTC_HOUR_SHIFT);
    tm.tm_min  = ((st_time & RTC_MIN_MASK) >> RTC_MIN_SHIFT);
    tm.tm_sec  = ((st_time & RTC_SEC_MASK) >> RTC_SEC_SHIFT);
    tm.tm_yday = rtc_year_days(tm.tm_mday, tm.tm_mon, tm.tm_year);
    tm.tm_year -= 1900;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl031_stv2_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pl031_stv2_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct pl031_local *ldata = dev_get_drvdata(dev);
    pl031_stv2_time_to_tm(readl(ldata.base + RTC_DR),
    readl(ldata.base + RTC_YDR), tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl031_stv2_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pl031_stv2_set_time(struct device *dev, struct rtc_time *tm)
    {
    unsigned long time;
    unsigned long bcd_year;
    struct pl031_local *ldata = dev_get_drvdata(dev);
    int ret;
    ret = pl031_stv2_tm_to_time(dev, tm, &time, &bcd_year);
    if (ret == 0) {
    writel(bcd_year, ldata.base + RTC_YLR);
    writel(time, ldata.base + RTC_LR);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pl031_stv2_read_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int pl031_stv2_read_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct pl031_local *ldata = dev_get_drvdata(dev);
    int ret;
    ret = pl031_stv2_time_to_tm(readl(ldata.base + RTC_MR),
    readl(ldata.base + RTC_YMR), &alarm.time);
    alarm.pending = readl(ldata.base + RTC_RIS) & RTC_BIT_AI;
    alarm.enabled = readl(ldata.base + RTC_IMSC) & RTC_BIT_AI;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pl031_stv2_set_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int pl031_stv2_set_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct pl031_local *ldata = dev_get_drvdata(dev);
    unsigned long time;
    unsigned long bcd_year;
    int ret;
    ret = pl031_stv2_tm_to_time(dev, &alarm.time,
    &time, &bcd_year);
    if (ret == 0) {
    writel(bcd_year, ldata.base + RTC_YMR);
    writel(time, ldata.base + RTC_MR);
    pl031_alarm_irq_enable(dev, alarm.enabled);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pl031_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pl031_interrupt(int irq, void *dev_id)
    {
    struct pl031_local *ldata = dev_id;
    unsigned long rtcmis;
    let mut events: c_ulong = 0;
    rtcmis = readl(ldata.base + RTC_MIS);
    if (rtcmis & RTC_BIT_AI) {
    writel(RTC_BIT_AI, ldata.base + RTC_ICR);
    events |= (RTC_AF | RTC_IRQF);
    rtc_update_irq(ldata.rtc, 1, events);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn pl031_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pl031_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct pl031_local *ldata = dev_get_drvdata(dev);
    rtc_time64_to_tm(readl(ldata.base + RTC_DR), tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl031_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pl031_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct pl031_local *ldata = dev_get_drvdata(dev);
    writel(rtc_tm_to_time64(tm), ldata.base + RTC_LR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl031_read_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int pl031_read_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct pl031_local *ldata = dev_get_drvdata(dev);
    rtc_time64_to_tm(readl(ldata.base + RTC_MR), &alarm.time);
    alarm.pending = readl(ldata.base + RTC_RIS) & RTC_BIT_AI;
    alarm.enabled = readl(ldata.base + RTC_IMSC) & RTC_BIT_AI;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl031_set_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int pl031_set_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct pl031_local *ldata = dev_get_drvdata(dev);
    writel(rtc_tm_to_time64(&alarm.time), ldata.base + RTC_MR);
    pl031_alarm_irq_enable(dev, alarm.enabled);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl031_remove(adev: *mut amba_device) {
    static void pl031_remove(struct amba_device *adev)
    {
    struct pl031_local *ldata = dev_get_drvdata(&adev.dev);
    if (adev.irq[0])
    free_irq(adev.irq[0], ldata);
    amba_release_regions(adev);
    }
#[no_mangle]
unsafe extern "C" fn pl031_probe(adev: *mut amba_device, id: *const amba_id) -> c_int {
    static int pl031_probe(struct amba_device *adev, const struct amba_id *id)
    {
    int ret;
    struct pl031_local *ldata;
    struct pl031_vendor_data *vendor = id.data;
    struct rtc_class_ops *ops;
    unsigned long time, data;
    ret = amba_request_regions(adev, core::ptr::null_mut());
    if (ret)
    goto err_req;
    ldata = devm_kzalloc(&adev.dev, sizeof(struct pl031_local),
    GFP_KERNEL);
    ops = devm_kmemdup(&adev.dev, &vendor.ops, sizeof(vendor.ops),
    GFP_KERNEL);
    if (!ldata || !ops) {
    ret = -ENOMEM;
    goto out;
    }
    ldata.vendor = vendor;
    ldata.base = devm_ioremap(&adev.dev, adev.res.start,
    resource_size(&adev.res));
    if (!ldata.base) {
    ret = -ENOMEM;
    goto out;
    }
    amba_set_drvdata(adev, ldata);
    dev_dbg(&adev.dev, "designer ID = 0x%02x\n", amba_manf(adev));
    dev_dbg(&adev.dev, "revision = 0x%01x\n", amba_rev(adev));
    data = readl(ldata.base + RTC_CR);
// Enable the clockwatch on ST Variants
    if (vendor.clockwatch)
    data |= RTC_CR_CWEN;
    else
    data |= RTC_CR_EN;
    writel(data, ldata.base + RTC_CR);
//
// On ST PL031 variants, the RTC reset value does not provide correct
// weekday for 2000-01-01. Correct the erroneous sunday to saturday.
//
    if (vendor.st_weekday) {
    if (readl(ldata.base + RTC_YDR) == 0x2000) {
    time = readl(ldata.base + RTC_DR);
    if ((time &
    (RTC_MON_MASK | RTC_MDAY_MASK | RTC_WDAY_MASK))
    == 0x02120000) {
    time = time | (0x7 << RTC_WDAY_SHIFT);
    writel(0x2000, ldata.base + RTC_YLR);
    writel(time, ldata.base + RTC_LR);
    }
    }
    }
    devm_device_init_wakeup(&adev.dev);
    ldata.rtc = devm_rtc_allocate_device(&adev.dev);
    if (IS_ERR(ldata.rtc)) {
    ret = PTR_ERR(ldata.rtc);
    goto out;
    }
    if (!adev.irq[0])
    clear_bit(RTC_FEATURE_ALARM, ldata.rtc.features);
    ldata.rtc.ops = ops;
    ldata.rtc.range_min = vendor.range_min;
    ldata.rtc.range_max = vendor.range_max;
    ret = devm_rtc_register_device(ldata.rtc);
    if (ret)
    goto out;
    if (adev.irq[0]) {
    ret = request_irq(adev.irq[0], pl031_interrupt,
    vendor.irqflags, "rtc-pl031", ldata);
    if (ret)
    goto out;
    devm_pm_set_wake_irq(&adev.dev, adev.irq[0]);
    }
    return 0;
    out:
    amba_release_regions(adev);
    err_req:
    return ret;
    }
// Operations for the original ARM version
    static struct pl031_vendor_data arm_pl031 = {
    .ops = {
    .read_time = pl031_read_time,
    .set_time = pl031_set_time,
    .read_alarm = pl031_read_alarm,
    .set_alarm = pl031_set_alarm,
    .alarm_irq_enable = pl031_alarm_irq_enable,
    },
    .range_max = U32_MAX,
    };
// The First ST derivative
    static struct pl031_vendor_data stv1_pl031 = {
    .ops = {
    .read_time = pl031_read_time,
    .set_time = pl031_set_time,
    .read_alarm = pl031_read_alarm,
    .set_alarm = pl031_set_alarm,
    .alarm_irq_enable = pl031_alarm_irq_enable,
    },
    .clockwatch = true,
    .st_weekday = true,
    .range_max = U32_MAX,
    };
// And the second ST derivative
    static struct pl031_vendor_data stv2_pl031 = {
    .ops = {
    .read_time = pl031_stv2_read_time,
    .set_time = pl031_stv2_set_time,
    .read_alarm = pl031_stv2_read_alarm,
    .set_alarm = pl031_stv2_set_alarm,
    .alarm_irq_enable = pl031_alarm_irq_enable,
    },
    .clockwatch = true,
    .st_weekday = true,
//
// This variant shares the IRQ with another block and must not
// suspend that IRQ line.
// TODO check if it shares with IRQF_NO_SUSPEND user, else we can
// remove IRQF_COND_SUSPEND
//
    .irqflags = IRQF_SHARED | IRQF_COND_SUSPEND,
    .range_min = RTC_TIMESTAMP_BEGIN_0000,
    .range_max = RTC_TIMESTAMP_END_9999,
    };
    static const struct amba_id pl031_ids[] = {
    {
    .id = 0x00041031,
    .mask = 0x000fffff,
    .data = &arm_pl031,
    },
// ST Micro variants
    {
    .id = 0x00180031,
    .mask = 0x00ffffff,
    .data = &stv1_pl031,
    },
    {
    .id = 0x00280031,
    .mask = 0x00ffffff,
    .data = &stv2_pl031,
    },
    {0, 0},
    };
    MODULE_DEVICE_TABLE(amba, pl031_ids);
    static struct amba_driver pl031_driver = {
    .drv = {
    .name = "rtc-pl031",
    },
    .id_table = pl031_ids,
    .probe = pl031_probe,
    .remove = pl031_remove,
    };
    module_amba_driver(pl031_driver);
    MODULE_AUTHOR("Deepak Saxena <dsaxena@plexity.net>");
    MODULE_DESCRIPTION("ARM AMBA PL031 RTC Driver");
    MODULE_LICENSE("GPL");
