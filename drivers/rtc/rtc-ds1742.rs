//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ds1742.c
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
// An rtc driver for the Dallas DS1742
//
// Copyright (C) 2006 Atsushi Nemoto <anemo@mba.ocn.ne.jp>
//
// Copyright (C) 2006 Torsten Ertbjerg Rasmussen <tr@newtec.dk>
// - nvram size determined from resource
// - this ds1742 driver now supports ds1743.
//

pub const RTC_SIZE: c_int = 8;
pub const RTC_CONTROL: c_int = 0;
pub const RTC_CENTURY: c_int = 0;
pub const RTC_SECONDS: c_int = 1;
pub const RTC_MINUTES: c_int = 2;
pub const RTC_HOURS: c_int = 3;
pub const RTC_DAY: c_int = 4;
pub const RTC_DATE: c_int = 5;
pub const RTC_MONTH: c_int = 6;
pub const RTC_YEAR: c_int = 7;
pub const RTC_CENTURY_MASK: c_uint = 0x3f;
pub const RTC_SECONDS_MASK: c_uint = 0x7f;
pub const RTC_DAY_MASK: c_uint = 0x07;
// Bits in the Control/Century register
pub const RTC_WRITE: c_uint = 0x80;
pub const RTC_READ: c_uint = 0x40;
// Bits in the Seconds register
pub const RTC_STOP: c_uint = 0x80;
// Bits in the Day register
pub const RTC_BATT_FLAG: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtc_plat_data {
    pub ioaddr_nvram: *mut void __iomem,
    pub ioaddr_rtc: *mut void __iomem,
    pub last_jiffies: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn ds1742_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ds1742_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    void __iomem *ioaddr = pdata.ioaddr_rtc;
    u8 century;
    century = bin2bcd((tm.tm_year + 1900) / 100);
    writeb(RTC_WRITE, ioaddr + RTC_CONTROL);
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
unsafe extern "C" fn ds1742_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ds1742_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct rtc_plat_data *pdata = dev_get_drvdata(dev);
    void __iomem *ioaddr = pdata.ioaddr_rtc;
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
    static const struct rtc_class_ops ds1742_rtc_ops = {
    .read_time	= ds1742_rtc_read_time,
    .set_time	= ds1742_rtc_set_time,
    };
    static int ds1742_nvram_read(void *priv, unsigned int pos, void *val,
    size_t bytes)
    {
    struct rtc_plat_data *pdata = priv;
    void __iomem *ioaddr = pdata.ioaddr_nvram;
    u8 *buf = val;
    for (; bytes; bytes--)
// buf++ = readb(ioaddr + pos++);
    return 0;
    }
    static int ds1742_nvram_write(void *priv, unsigned int pos, void *val,
    size_t bytes)
    {
    struct rtc_plat_data *pdata = priv;
    void __iomem *ioaddr = pdata.ioaddr_nvram;
    u8 *buf = val;
    for (; bytes; bytes--)
    writeb(*buf++, ioaddr + pos++);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds1742_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int ds1742_rtc_probe(struct platform_device *pdev)
    {
    struct rtc_device *rtc;
    struct resource *res;
    unsigned int cen, sec;
    struct rtc_plat_data *pdata;
    void __iomem *ioaddr;
    let mut ret: c_int = 0;
    struct nvmem_config nvmem_cfg = {
    .name = "ds1742_nvram",
    .reg_read = ds1742_nvram_read,
    .reg_write = ds1742_nvram_write,
    };
    pdata = devm_kzalloc(&pdev.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    ioaddr = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(ioaddr))
    return PTR_ERR(ioaddr);
    pdata.ioaddr_nvram = ioaddr;
    pdata.ioaddr_rtc = ioaddr + resource_size(res) - RTC_SIZE;
    nvmem_cfg.size = resource_size(res) - RTC_SIZE;
    nvmem_cfg.priv = pdata;
// turn RTC on if it was not on
    ioaddr = pdata.ioaddr_rtc;
    sec = readb(ioaddr + RTC_SECONDS);
    if (sec & RTC_STOP) {
    sec &= RTC_SECONDS_MASK;
    cen = readb(ioaddr + RTC_CENTURY) & RTC_CENTURY_MASK;
    writeb(RTC_WRITE, ioaddr + RTC_CONTROL);
    writeb(sec, ioaddr + RTC_SECONDS);
    writeb(cen & RTC_CENTURY_MASK, ioaddr + RTC_CONTROL);
    }
    if (!(readb(ioaddr + RTC_DAY) & RTC_BATT_FLAG))
    dev_warn(&pdev.dev, "voltage-low detected.\n");
    pdata.last_jiffies = jiffies;
    platform_set_drvdata(pdev, pdata);
    rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    rtc.ops = &ds1742_rtc_ops;
    ret = devm_rtc_register_device(rtc);
    if (ret)
    return ret;
    devm_rtc_nvmem_register(rtc, &nvmem_cfg);
    return 0;
    }
    static const struct of_device_id __maybe_unused ds1742_rtc_of_match[] = {
    { .compatible = "maxim,ds1742", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ds1742_rtc_of_match);
    static struct platform_driver ds1742_rtc_driver = {
    .probe		= ds1742_rtc_probe,
    .driver		= {
    .name	= "rtc-ds1742",
    .of_match_table = of_match_ptr(ds1742_rtc_of_match),
    },
    };
    module_platform_driver(ds1742_rtc_driver);
    MODULE_AUTHOR("Atsushi Nemoto <anemo@mba.ocn.ne.jp>");
    MODULE_DESCRIPTION("Dallas DS1742 RTC driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:rtc-ds1742");
