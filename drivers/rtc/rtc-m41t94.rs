//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-m41t94.c
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
// Driver for ST M41T94 SPI RTC
//
// Copyright (C) 2008 Kim B. Heino
//

pub const M41T94_REG_SECONDS: c_uint = 0x01;
pub const M41T94_REG_MINUTES: c_uint = 0x02;
pub const M41T94_REG_HOURS: c_uint = 0x03;
pub const M41T94_REG_WDAY: c_uint = 0x04;
pub const M41T94_REG_DAY: c_uint = 0x05;
pub const M41T94_REG_MONTH: c_uint = 0x06;
pub const M41T94_REG_YEAR: c_uint = 0x07;
pub const M41T94_REG_HT: c_uint = 0x0c;
pub const M41T94_BIT_HALT: c_uint = 0x40;
pub const M41T94_BIT_STOP: c_uint = 0x80;
pub const M41T94_BIT_CB: c_uint = 0x40;
pub const M41T94_BIT_CEB: c_uint = 0x80;
#[no_mangle]
unsafe extern "C" fn m41t94_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int m41t94_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct spi_device *spi = to_spi_device(dev);
    u8 buf[8]; /* write cmd + 7 registers */
    dev_dbg(dev, "%s secs=%d, mins=%d, "
    "hours=%d, mday=%d, mon=%d, year=%d, wday=%d\n",
    "write", tm.tm_sec, tm.tm_min,
    tm.tm_hour, tm.tm_mday,
    tm.tm_mon, tm.tm_year, tm.tm_wday);
    buf[0] = 0x80 | M41T94_REG_SECONDS; /* write time + date */
    buf[M41T94_REG_SECONDS] = bin2bcd(tm.tm_sec);
    buf[M41T94_REG_MINUTES] = bin2bcd(tm.tm_min);
    buf[M41T94_REG_HOURS]   = bin2bcd(tm.tm_hour);
    buf[M41T94_REG_WDAY]    = bin2bcd(tm.tm_wday + 1);
    buf[M41T94_REG_DAY]     = bin2bcd(tm.tm_mday);
    buf[M41T94_REG_MONTH]   = bin2bcd(tm.tm_mon + 1);
    buf[M41T94_REG_HOURS] |= M41T94_BIT_CEB;
    if (tm.tm_year >= 100)
    buf[M41T94_REG_HOURS] |= M41T94_BIT_CB;
    buf[M41T94_REG_YEAR] = bin2bcd(tm.tm_year % 100);
    return spi_write(spi, buf, 8);
    }
#[no_mangle]
unsafe extern "C" fn m41t94_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int m41t94_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct spi_device *spi = to_spi_device(dev);
    u8 buf[2];
    int ret, hour;
// clear halt update bit
    ret = spi_w8r8(spi, M41T94_REG_HT);
    if (ret < 0)
    return ret;
    if (ret & M41T94_BIT_HALT) {
    buf[0] = 0x80 | M41T94_REG_HT;
    buf[1] = ret & ~M41T94_BIT_HALT;
    spi_write(spi, buf, 2);
    }
// clear stop bit
    ret = spi_w8r8(spi, M41T94_REG_SECONDS);
    if (ret < 0)
    return ret;
    if (ret & M41T94_BIT_STOP) {
    buf[0] = 0x80 | M41T94_REG_SECONDS;
    buf[1] = ret & ~M41T94_BIT_STOP;
    spi_write(spi, buf, 2);
    }
    tm.tm_sec  = bcd2bin(spi_w8r8(spi, M41T94_REG_SECONDS));
    tm.tm_min  = bcd2bin(spi_w8r8(spi, M41T94_REG_MINUTES));
    hour = spi_w8r8(spi, M41T94_REG_HOURS);
    tm.tm_hour = bcd2bin(hour & 0x3f);
    tm.tm_wday = bcd2bin(spi_w8r8(spi, M41T94_REG_WDAY)) - 1;
    tm.tm_mday = bcd2bin(spi_w8r8(spi, M41T94_REG_DAY));
    tm.tm_mon  = bcd2bin(spi_w8r8(spi, M41T94_REG_MONTH)) - 1;
    tm.tm_year = bcd2bin(spi_w8r8(spi, M41T94_REG_YEAR));
    if ((hour & M41T94_BIT_CB) || !(hour & M41T94_BIT_CEB))
    tm.tm_year += 100;
    dev_dbg(dev, "%s secs=%d, mins=%d, "
    "hours=%d, mday=%d, mon=%d, year=%d, wday=%d\n",
    "read", tm.tm_sec, tm.tm_min,
    tm.tm_hour, tm.tm_mday,
    tm.tm_mon, tm.tm_year, tm.tm_wday);
    return 0;
    }
    static const struct rtc_class_ops m41t94_rtc_ops = {
    .read_time	= m41t94_read_time,
    .set_time	= m41t94_set_time,
    };
    static struct spi_driver m41t94_driver;
#[no_mangle]
unsafe extern "C" fn m41t94_probe(spi: *mut spi_device) -> c_int {
    static int m41t94_probe(struct spi_device *spi)
    {
    struct rtc_device *rtc;
    int res;
    spi.bits_per_word = 8;
    spi_setup(spi);
    res = spi_w8r8(spi, M41T94_REG_SECONDS);
    if (res < 0) {
    dev_err(&spi.dev, "not found.\n");
    return res;
    }
    rtc = devm_rtc_device_register(&spi.dev, m41t94_driver.driver.name,
    &m41t94_rtc_ops, THIS_MODULE);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    spi_set_drvdata(spi, rtc);
    return 0;
    }
    static struct spi_driver m41t94_driver = {
    .driver = {
    .name	= "rtc-m41t94",
    },
    .probe	= m41t94_probe,
    };
    module_spi_driver(m41t94_driver);
    MODULE_AUTHOR("Kim B. Heino <Kim.Heino@bluegiga.com>");
    MODULE_DESCRIPTION("Driver for ST M41T94 SPI RTC");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("spi:rtc-m41t94");
