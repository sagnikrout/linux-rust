//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-max6902.c
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
// drivers/rtc/rtc-max6902.c
//
// Copyright (C) 2006 8D Technologies inc.
// Copyright (C) 2004 Compulab Ltd.
//
// Driver for MAX6902 spi RTC
//

pub const MAX6902_REG_SECONDS: c_uint = 0x01;
pub const MAX6902_REG_MINUTES: c_uint = 0x03;
pub const MAX6902_REG_HOURS: c_uint = 0x05;
pub const MAX6902_REG_DATE: c_uint = 0x07;
pub const MAX6902_REG_MONTH: c_uint = 0x09;
pub const MAX6902_REG_DAY: c_uint = 0x0B;
pub const MAX6902_REG_YEAR: c_uint = 0x0D;
pub const MAX6902_REG_CONTROL: c_uint = 0x0F;
pub const MAX6902_REG_CENTURY: c_uint = 0x13;
    static int max6902_set_reg(struct device *dev, unsigned char address,
    unsigned char data)
    {
    struct spi_device *spi = to_spi_device(dev);
    unsigned char buf[2];
// MSB must be '0' to write
    buf[0] = address & 0x7f;
    buf[1] = data;
    return spi_write_then_read(spi, buf, 2, core::ptr::null_mut(), 0);
    }
    static int max6902_get_reg(struct device *dev, unsigned char address,
    unsigned char *data)
    {
    struct spi_device *spi = to_spi_device(dev);
// Set MSB to indicate read
// data = address | 0x80;
    return spi_write_then_read(spi, data, 1, data, 1);
    }
#[no_mangle]
unsafe extern "C" fn max6902_read_time(dev: *mut device, dt: *mut rtc_time) -> c_int {
    static int max6902_read_time(struct device *dev, struct rtc_time *dt)
    {
    int err, century;
    struct spi_device *spi = to_spi_device(dev);
    unsigned char buf[8];
    buf[0] = 0xbf;	/* Burst read */
    err = spi_write_then_read(spi, buf, 1, buf, 8);
    if (err != 0)
    return err;
// The chip sends data in this order:
// Seconds, Minutes, Hours, Date, Month, Day, Year
    dt.tm_sec	= bcd2bin(buf[0]);
    dt.tm_min	= bcd2bin(buf[1]);
    dt.tm_hour	= bcd2bin(buf[2]);
    dt.tm_mday	= bcd2bin(buf[3]);
    dt.tm_mon	= bcd2bin(buf[4]) - 1;
    dt.tm_wday	= bcd2bin(buf[5]);
    dt.tm_year	= bcd2bin(buf[6]);
// Read century
    err = max6902_get_reg(dev, MAX6902_REG_CENTURY, &buf[0]);
    if (err != 0)
    return err;
    century = bcd2bin(buf[0]) * 100;
    dt.tm_year += century;
    dt.tm_year -= 1900;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max6902_set_time(dev: *mut device, dt: *mut rtc_time) -> c_int {
    static int max6902_set_time(struct device *dev, struct rtc_time *dt)
    {
    dt.tm_year = dt.tm_year + 1900;
// Remove write protection
    max6902_set_reg(dev, MAX6902_REG_CONTROL, 0);
    max6902_set_reg(dev, MAX6902_REG_SECONDS, bin2bcd(dt.tm_sec));
    max6902_set_reg(dev, MAX6902_REG_MINUTES, bin2bcd(dt.tm_min));
    max6902_set_reg(dev, MAX6902_REG_HOURS, bin2bcd(dt.tm_hour));
    max6902_set_reg(dev, MAX6902_REG_DATE, bin2bcd(dt.tm_mday));
    max6902_set_reg(dev, MAX6902_REG_MONTH, bin2bcd(dt.tm_mon + 1));
    max6902_set_reg(dev, MAX6902_REG_DAY, bin2bcd(dt.tm_wday));
    max6902_set_reg(dev, MAX6902_REG_YEAR, bin2bcd(dt.tm_year % 100));
    max6902_set_reg(dev, MAX6902_REG_CENTURY, bin2bcd(dt.tm_year / 100));
// Compulab used a delay here. However, the datasheet
// does not mention a delay being required anywhere...
// delay(2000);
// Write protect
    max6902_set_reg(dev, MAX6902_REG_CONTROL, 0x80);
    return 0;
    }
    static const struct rtc_class_ops max6902_rtc_ops = {
    .read_time	= max6902_read_time,
    .set_time	= max6902_set_time,
    };
#[no_mangle]
unsafe extern "C" fn max6902_probe(spi: *mut spi_device) -> c_int {
    static int max6902_probe(struct spi_device *spi)
    {
    struct rtc_device *rtc;
    unsigned char tmp;
    int res;
    spi.mode = SPI_MODE_3;
    spi.bits_per_word = 8;
    spi_setup(spi);
    res = max6902_get_reg(&spi.dev, MAX6902_REG_SECONDS, &tmp);
    if (res != 0)
    return res;
    rtc = devm_rtc_device_register(&spi.dev, "max6902",
    &max6902_rtc_ops, THIS_MODULE);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    spi_set_drvdata(spi, rtc);
    return 0;
    }
    static struct spi_driver max6902_driver = {
    .driver = {
    .name	= "rtc-max6902",
    },
    .probe	= max6902_probe,
    };
    module_spi_driver(max6902_driver);
    MODULE_DESCRIPTION("max6902 spi RTC driver");
    MODULE_AUTHOR("Raphael Assenat");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("spi:rtc-max6902");
