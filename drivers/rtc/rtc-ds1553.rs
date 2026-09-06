//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ds1553.c
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
// An rtc driver for the Dallas DS1553
//
// Copyright (C) 2006 Atsushi Nemoto <anemo@mba.ocn.ne.jp>
//

pub const RTC_REG_SIZE: c_uint = 0x2000;
pub const RTC_OFFSET: c_uint = 0x1ff0;

pub const RTC_CENTURY_MASK: c_uint = 0x3f;
pub const RTC_SECONDS_MASK: c_uint = 0x7f;
pub const RTC_DAY_MASK: c_uint = 0x07;
// Bits in the Control/Century register
pub const RTC_WRITE: c_uint = 0x80;
pub const RTC_READ: c_uint = 0x40;
// Bits in the Seconds register
pub const RTC_STOP: c_uint = 0x80;
// Bits in the Flags register
pub const RTC_FLAGS_AF: c_uint = 0x40;
pub const RTC_FLAGS_BLF: c_uint = 0x10;
// Bits in the Interrupts register
pub const RTC_INTS_AE: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtc_plat_data {
    pub rtc: *mut rtc_device,
    pub ioaddr: *mut void __iomem,
    pub last_jiffies: c_ulong,
    pub irq: c_int,
    pub irqen: c_uint,
    pub alrm_sec: c_int,
    pub alrm_min: c_int,
    pub alrm_hour: c_int,
    pub alrm_mday: c_int,
    pub lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn ds1553_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ds1553_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    void __iomem *ioaddr = pdata.ioaddr;
    u8 century;
    century = bin2bcd((tm.tm_year + 1900) / 100);
    writeb(RTC_WRITE, pdata.ioaddr + RTC_CONTROL);
    writeb(bin2bcd(tm.tm_year % 100), ioaddr + RTC_YEAR);
    writeb(bin2bcd(tm.tm_mon + 1), ioaddr + RTC_MONTH);
    writeb(bin2bcd(tm.tm_wday) & RTC_DAY_MASK, ioaddr + RTC_DAY);
    writeb(bin2bcd(tm.tm_mday), ioaddr + RTC_DATE);
    writeb(bin2bcd(tm.tm_hour), ioaddr + RTC_HOURS);
    writeb(bin2bcd(tm.tm_min), ioaddr + RTC_MINUTES);
    writeb(bin2bcd(tm.tm_sec) & RTC_SECONDS_MASK, ioaddr + RTC_SECONDS);
// RTC_CENTURY and RTC_CONTROL share same register
    writeb(RTC_WRITE | (century & RTC_CENTURY_MASK), ioaddr + RTC_CENTURY);
    writeb(century & RTC_CENTURY_MASK, ioaddr + RTC_CONTROL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds1553_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ds1553_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    void __iomem *ioaddr = pdata.ioaddr;
    unsigned int year, month, day, hour, minute, second, week;
    unsigned int century;
// give enough time to update RTC in case of continuous read
    if (pdata.last_jiffies == jiffies)
    msleep(1);
    pdata.last_jiffies = jiffies;
    writeb(RTC_READ, ioaddr + RTC_CONTROL);
    second = readb(ioaddr + RTC_SECONDS) & RTC_SECONDS_MASK;
    minute = readb(ioaddr + RTC_MINUTES);
    hour = readb(ioaddr + RTC_HOURS);
    day = readb(ioaddr + RTC_DATE);
    week = readb(ioaddr + RTC_DAY) & RTC_DAY_MASK;
    month = readb(ioaddr + RTC_MONTH);
    year = readb(ioaddr + RTC_YEAR);
    century = readb(ioaddr + RTC_CENTURY) & RTC_CENTURY_MASK;
    writeb(0, ioaddr + RTC_CONTROL);
    tm.tm_sec = bcd2bin(second);
    tm.tm_min = bcd2bin(minute);
    tm.tm_hour = bcd2bin(hour);
    tm.tm_mday = bcd2bin(day);
    tm.tm_wday = bcd2bin(week);
    tm.tm_mon = bcd2bin(month) - 1;
// year is 1900 + tm->tm_year
    tm.tm_year = bcd2bin(year) + bcd2bin(century) * 100 - 1900;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds1553_rtc_update_alarm(pdata: *mut rtc_plat_data) {
    static void ds1553_rtc_update_alarm(struct rtc_plat_data *pdata)
    {
    void __iomem *ioaddr = pdata.ioaddr;
    unsigned long flags;
    spin_lock_irqsave(&pdata.lock, flags);
    writeb(pdata.alrm_mday < 0 || (pdata.irqen & RTC_UF) ?
    0x80 : bin2bcd(pdata.alrm_mday),
    ioaddr + RTC_DATE_ALARM);
    writeb(pdata.alrm_hour < 0 || (pdata.irqen & RTC_UF) ?
    0x80 : bin2bcd(pdata.alrm_hour),
    ioaddr + RTC_HOURS_ALARM);
    writeb(pdata.alrm_min < 0 || (pdata.irqen & RTC_UF) ?
    0x80 : bin2bcd(pdata.alrm_min),
    ioaddr + RTC_MINUTES_ALARM);
    writeb(pdata.alrm_sec < 0 || (pdata.irqen & RTC_UF) ?
    0x80 : bin2bcd(pdata.alrm_sec),
    ioaddr + RTC_SECONDS_ALARM);
    writeb(pdata.irqen ? RTC_INTS_AE : 0, ioaddr + RTC_INTERRUPTS);
    readb(ioaddr + RTC_FLAGS);	/* clear interrupts */
    spin_unlock_irqrestore(&pdata.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ds1553_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int ds1553_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    if (pdata.irq <= 0)
    return -EINVAL;
    pdata.alrm_mday = alrm.time.tm_mday;
    pdata.alrm_hour = alrm.time.tm_hour;
    pdata.alrm_min = alrm.time.tm_min;
    pdata.alrm_sec = alrm.time.tm_sec;
    if (alrm.enabled)
    pdata.irqen |= RTC_AF;
    ds1553_rtc_update_alarm(pdata);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds1553_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int ds1553_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    if (pdata.irq <= 0)
    return -EINVAL;
    alrm.time.tm_mday = pdata.alrm_mday < 0 ? 0 : pdata.alrm_mday;
    alrm.time.tm_hour = pdata.alrm_hour < 0 ? 0 : pdata.alrm_hour;
    alrm.time.tm_min = pdata.alrm_min < 0 ? 0 : pdata.alrm_min;
    alrm.time.tm_sec = pdata.alrm_sec < 0 ? 0 : pdata.alrm_sec;
    alrm.enabled = (pdata.irqen & RTC_AF) ? 1 : 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds1553_rtc_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ds1553_rtc_interrupt(int irq, void *dev_id)
    {
    struct platform_device *pdev = dev_id;
    struct rtc_plat_data *pdata = platform_get_drvdata(pdev);
    void __iomem *ioaddr = pdata.ioaddr;
    let mut events: c_ulong = 0;
    spin_lock(&pdata.lock);
// read and clear interrupt
    if (readb(ioaddr + RTC_FLAGS) & RTC_FLAGS_AF) {
    events = RTC_IRQF;
    if (readb(ioaddr + RTC_SECONDS_ALARM) & 0x80)
    events |= RTC_UF;
    else
    events |= RTC_AF;
    rtc_update_irq(pdata.rtc, 1, events);
    }
    spin_unlock(&pdata.lock);
    return events ? IRQ_HANDLED : IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn ds1553_rtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int ds1553_rtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    if (pdata.irq <= 0)
    return -EINVAL;
    if (enabled)
    pdata.irqen |= RTC_AF;
    else
    pdata.irqen &= ~RTC_AF;
    ds1553_rtc_update_alarm(pdata);
    return 0;
    }
    static const struct rtc_class_ops ds1553_rtc_ops = {
    .read_time		= ds1553_rtc_read_time,
    .set_time		= ds1553_rtc_set_time,
    .read_alarm		= ds1553_rtc_read_alarm,
    .set_alarm		= ds1553_rtc_set_alarm,
    .alarm_irq_enable	= ds1553_rtc_alarm_irq_enable,
    };
    static int ds1553_nvram_read(void *priv, unsigned int pos, void *val,
    size_t bytes)
    {
    struct platform_device *pdev = priv;
    struct rtc_plat_data *pdata = platform_get_drvdata(pdev);
    void __iomem *ioaddr = pdata.ioaddr;
    u8 *buf = val;
    for (; bytes; bytes--)
// buf++ = readb(ioaddr + pos++);
    return 0;
    }
    static int ds1553_nvram_write(void *priv, unsigned int pos, void *val,
    size_t bytes)
    {
    struct platform_device *pdev = priv;
    struct rtc_plat_data *pdata = platform_get_drvdata(pdev);
    void __iomem *ioaddr = pdata.ioaddr;
    u8 *buf = val;
    for (; bytes; bytes--)
    writeb(*buf++, ioaddr + pos++);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds1553_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int ds1553_rtc_probe(struct platform_device *pdev)
    {
    unsigned int cen, sec;
    struct rtc_plat_data *pdata;
    void __iomem *ioaddr;
    let mut ret: c_int = 0;
    struct nvmem_config nvmem_cfg = {
    .name = "ds1553_nvram",
    .word_size = 1,
    .stride = 1,
    .size = RTC_OFFSET,
    .reg_read = ds1553_nvram_read,
    .reg_write = ds1553_nvram_write,
    .priv = pdev,
    };
    pdata = devm_kzalloc(&pdev.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    ioaddr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ioaddr))
    return PTR_ERR(ioaddr);
    pdata.ioaddr = ioaddr;
    pdata.irq = platform_get_irq(pdev, 0);
// turn RTC on if it was not on
    sec = readb(ioaddr + RTC_SECONDS);
    if (sec & RTC_STOP) {
    sec &= RTC_SECONDS_MASK;
    cen = readb(ioaddr + RTC_CENTURY) & RTC_CENTURY_MASK;
    writeb(RTC_WRITE, ioaddr + RTC_CONTROL);
    writeb(sec, ioaddr + RTC_SECONDS);
    writeb(cen & RTC_CENTURY_MASK, ioaddr + RTC_CONTROL);
    }
    if (readb(ioaddr + RTC_FLAGS) & RTC_FLAGS_BLF)
    dev_warn(&pdev.dev, "voltage-low detected.\n");
    spin_lock_init(&pdata.lock);
    pdata.last_jiffies = jiffies;
    platform_set_drvdata(pdev, pdata);
    pdata.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(pdata.rtc))
    return PTR_ERR(pdata.rtc);
    pdata.rtc.ops = &ds1553_rtc_ops;
    ret = devm_rtc_register_device(pdata.rtc);
    if (ret)
    return ret;
    if (pdata.irq > 0) {
    writeb(0, ioaddr + RTC_INTERRUPTS);
    if (devm_request_irq(&pdev.dev, pdata.irq,
    ds1553_rtc_interrupt,
    0, pdev.name, pdev) < 0) {
    dev_warn(&pdev.dev, "interrupt not available.\n");
    pdata.irq = 0;
    }
    }
    devm_rtc_nvmem_register(pdata.rtc, &nvmem_cfg);
    return 0;
    }
// work with hotplug and coldplug
    MODULE_ALIAS("platform:rtc-ds1553");
    static struct platform_driver ds1553_rtc_driver = {
    .probe		= ds1553_rtc_probe,
    .driver		= {
    .name	= "rtc-ds1553",
    },
    };
    module_platform_driver(ds1553_rtc_driver);
    MODULE_AUTHOR("Atsushi Nemoto <anemo@mba.ocn.ne.jp>");
    MODULE_DESCRIPTION("Dallas DS1553 RTC driver");
    MODULE_LICENSE("GPL");
