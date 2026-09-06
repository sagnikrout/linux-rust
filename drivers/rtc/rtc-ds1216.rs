//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ds1216.c
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
// Dallas DS1216 RTC driver
//
// Copyright (c) 2007 Thomas Bogendoerfer
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds1216_regs {
    pub tsec: u8,
    pub sec: u8,
    pub min: u8,
    pub hour: u8,
    pub wday: u8,
    pub mday: u8,
    pub month: u8,
    pub year: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds1216_priv {
    pub rtc: *mut rtc_device,
    pub ioaddr: *mut void __iomem,
}

    static const u8 magic[] = {
    0xc5, 0x3a, 0xa3, 0x5c, 0xc5, 0x3a, 0xa3, 0x5c
    };
//
// Read the 64 bit we'd like to have - It a series
// of 64 bits showing up in the LSB of the base register.
//
#[no_mangle]
unsafe extern "C" fn ds1216_read(ioaddr: *mut u8 __iomem, buf: *mut u8) {
    static void ds1216_read(u8 __iomem *ioaddr, u8 *buf)
    {
    unsigned char c;
    int i, j;
    for (i = 0; i < 8; i++) {
    c = 0;
    for (j = 0; j < 8; j++)
    c |= (readb(ioaddr) & 0x1) << j;
    buf[i] = c;
    }
    }
#[no_mangle]
unsafe extern "C" fn ds1216_write(ioaddr: *mut u8 __iomem, buf: *const u8) {
    static void ds1216_write(u8 __iomem *ioaddr, const u8 *buf)
    {
    unsigned char c;
    int i, j;
    for (i = 0; i < 8; i++) {
    c = buf[i];
    for (j = 0; j < 8; j++) {
    writeb(c, ioaddr);
    c = c >> 1;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ds1216_switch_ds_to_clock(ioaddr: *mut u8 __iomem) {
    static void ds1216_switch_ds_to_clock(u8 __iomem *ioaddr)
    {
// Reset magic pointer
    readb(ioaddr);
// Write 64 bit magic to DS1216
    ds1216_write(ioaddr, magic);
    }
#[no_mangle]
unsafe extern "C" fn ds1216_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ds1216_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct ds1216_priv *priv = dev_get_drvdata(dev);
    struct ds1216_regs regs;
    ds1216_switch_ds_to_clock(priv.ioaddr);
    ds1216_read(priv.ioaddr, (u8 *)&regs);
    tm.tm_sec = bcd2bin(regs.sec);
    tm.tm_min = bcd2bin(regs.min);
    if (regs.hour & DS1216_HOUR_1224) {
// AM/PM mode
    tm.tm_hour = bcd2bin(regs.hour & 0x1f);
    if (regs.hour & DS1216_HOUR_AMPM)
    tm.tm_hour += 12;
    } else
    tm.tm_hour = bcd2bin(regs.hour & 0x3f);
    tm.tm_wday = (regs.wday & 7) - 1;
    tm.tm_mday = bcd2bin(regs.mday & 0x3f);
    tm.tm_mon = bcd2bin(regs.month & 0x1f);
    tm.tm_year = bcd2bin(regs.year);
    if (tm.tm_year < 70)
    tm.tm_year += 100;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds1216_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ds1216_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct ds1216_priv *priv = dev_get_drvdata(dev);
    struct ds1216_regs regs;
    ds1216_switch_ds_to_clock(priv.ioaddr);
    ds1216_read(priv.ioaddr, (u8 *)&regs);
    regs.tsec = 0; /* clear 0.1 and 0.01 seconds */
    regs.sec = bin2bcd(tm.tm_sec);
    regs.min = bin2bcd(tm.tm_min);
    regs.hour &= DS1216_HOUR_1224;
    if (regs.hour && tm.tm_hour > 12) {
    regs.hour |= DS1216_HOUR_AMPM;
    tm.tm_hour -= 12;
    }
    regs.hour |= bin2bcd(tm.tm_hour);
    regs.wday &= ~7;
    regs.wday |= tm.tm_wday;
    regs.mday = bin2bcd(tm.tm_mday);
    regs.month = bin2bcd(tm.tm_mon);
    regs.year = bin2bcd(tm.tm_year % 100);
    ds1216_switch_ds_to_clock(priv.ioaddr);
    ds1216_write(priv.ioaddr, (u8 *)&regs);
    return 0;
    }
    static const struct rtc_class_ops ds1216_rtc_ops = {
    .read_time	= ds1216_rtc_read_time,
    .set_time	= ds1216_rtc_set_time,
    };
#[no_mangle]
unsafe extern "C" fn ds1216_rtc_probe(pdev: *mut platform_device) -> int __init {
    static int __init ds1216_rtc_probe(struct platform_device *pdev)
    {
    struct ds1216_priv *priv;
    u8 dummy[8];
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    priv.ioaddr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.ioaddr))
    return PTR_ERR(priv.ioaddr);
    priv.rtc = devm_rtc_device_register(&pdev.dev, "ds1216",
    &ds1216_rtc_ops, THIS_MODULE);
    if (IS_ERR(priv.rtc))
    return PTR_ERR(priv.rtc);
// dummy read to get clock into a known state
    ds1216_read(priv.ioaddr, dummy);
    return 0;
    }
    static struct platform_driver ds1216_rtc_platform_driver = {
    .driver		= {
    .name	= "rtc-ds1216",
    },
    };
    module_platform_driver_probe(ds1216_rtc_platform_driver, ds1216_rtc_probe);
    MODULE_AUTHOR("Thomas Bogendoerfer <tsbogend@alpha.franken.de>");
    MODULE_DESCRIPTION("DS1216 RTC driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:rtc-ds1216");
