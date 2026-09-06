//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ds1347.c
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
// rtc-ds1347.c
//
// Driver for Dallas Semiconductor DS1347 Low Current, SPI Compatible
// Real Time Clock
//
// Author : Raghavendra Chandra Ganiga <ravi23ganiga@gmail.com>
//

// Registers in ds1347 rtc
pub const DS1347_SECONDS_REG: c_uint = 0x01;
pub const DS1347_MINUTES_REG: c_uint = 0x03;
pub const DS1347_HOURS_REG: c_uint = 0x05;
pub const DS1347_DATE_REG: c_uint = 0x07;
pub const DS1347_MONTH_REG: c_uint = 0x09;
pub const DS1347_DAY_REG: c_uint = 0x0B;
pub const DS1347_YEAR_REG: c_uint = 0x0D;
pub const DS1347_CONTROL_REG: c_uint = 0x0F;
pub const DS1347_CENTURY_REG: c_uint = 0x13;
pub const DS1347_STATUS_REG: c_uint = 0x17;
pub const DS1347_CLOCK_BURST: c_uint = 0x3F;

    static const struct regmap_range ds1347_ranges[] = {
    {
    .range_min = DS1347_SECONDS_REG,
    .range_max = DS1347_STATUS_REG,
    },
    };
    static const struct regmap_access_table ds1347_access_table = {
    .yes_ranges = ds1347_ranges,
    .n_yes_ranges = ARRAY_SIZE(ds1347_ranges),
    };
#[no_mangle]
unsafe extern "C" fn ds1347_read_time(dev: *mut device, dt: *mut rtc_time) -> c_int {
    static int ds1347_read_time(struct device *dev, struct rtc_time *dt)
    {
    struct regmap *map = dev_get_drvdata(dev);
    unsigned int status, century, secs;
    unsigned char buf[8];
    int err;
    err = regmap_read(map, DS1347_STATUS_REG, &status);
    if (err)
    return err;
    if (status & DS1347_OSF_BIT)
    return -EINVAL;
    do {
    err = regmap_bulk_read(map, DS1347_CLOCK_BURST, buf, 8);
    if (err)
    return err;
    err = regmap_read(map, DS1347_CENTURY_REG, &century);
    if (err)
    return err;
    err = regmap_read(map, DS1347_SECONDS_REG, &secs);
    if (err)
    return err;
    } while (buf[0] != secs);
    dt.tm_sec = bcd2bin(buf[0]);
    dt.tm_min = bcd2bin(buf[1] & 0x7f);
    dt.tm_hour = bcd2bin(buf[2] & 0x3F);
    dt.tm_mday = bcd2bin(buf[3]);
    dt.tm_mon = bcd2bin(buf[4]) - 1;
    dt.tm_wday = bcd2bin(buf[5]) - 1;
    dt.tm_year = (bcd2bin(century) * 100) + bcd2bin(buf[6]) - 1900;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds1347_set_time(dev: *mut device, dt: *mut rtc_time) -> c_int {
    static int ds1347_set_time(struct device *dev, struct rtc_time *dt)
    {
    struct regmap *map = dev_get_drvdata(dev);
    unsigned int century;
    unsigned char buf[8];
    int err;
    err = regmap_update_bits(map, DS1347_STATUS_REG,
    DS1347_NEOSC_BIT, DS1347_NEOSC_BIT);
    if (err)
    return err;
    buf[0] = bin2bcd(dt.tm_sec);
    buf[1] = bin2bcd(dt.tm_min);
    buf[2] = (bin2bcd(dt.tm_hour) & 0x3F);
    buf[3] = bin2bcd(dt.tm_mday);
    buf[4] = bin2bcd(dt.tm_mon + 1);
    buf[5] = bin2bcd(dt.tm_wday + 1);
    buf[6] = bin2bcd(dt.tm_year % 100);
    buf[7] = bin2bcd(0x00);
    err = regmap_bulk_write(map, DS1347_CLOCK_BURST, buf, 8);
    if (err)
    return err;
    century = (dt.tm_year / 100) + 19;
    err = regmap_write(map, DS1347_CENTURY_REG, bin2bcd(century));
    if (err)
    return err;
    return regmap_update_bits(map, DS1347_STATUS_REG,
    DS1347_NEOSC_BIT | DS1347_OSF_BIT, 0);
    }
    static const struct rtc_class_ops ds1347_rtc_ops = {
    .read_time = ds1347_read_time,
    .set_time = ds1347_set_time,
    };
#[no_mangle]
unsafe extern "C" fn ds1347_probe(spi: *mut spi_device) -> c_int {
    static int ds1347_probe(struct spi_device *spi)
    {
    struct rtc_device *rtc;
    struct regmap_config config;
    struct regmap *map;
    int err;
    memset(&config, 0, sizeof(config));
    config.reg_bits = 8;
    config.val_bits = 8;
    config.read_flag_mask = 0x80;
    config.max_register = 0x3F;
    config.wr_table = &ds1347_access_table;
// spi setup with ds1347 in mode 3 and bits per word as 8
    spi.mode = SPI_MODE_3;
    spi.bits_per_word = 8;
    spi_setup(spi);
    map = devm_regmap_init_spi(spi, &config);
    if (IS_ERR(map)) {
    dev_err(&spi.dev, "ds1347 regmap init spi failed\n");
    return PTR_ERR(map);
    }
    spi_set_drvdata(spi, map);
// Disable the write protect of rtc
    err = regmap_update_bits(map, DS1347_CONTROL_REG, DS1347_WP_BIT, 0);
    if (err)
    return err;
    rtc = devm_rtc_allocate_device(&spi.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    rtc.ops = &ds1347_rtc_ops;
    rtc.range_min = RTC_TIMESTAMP_BEGIN_0000;
    rtc.range_max = RTC_TIMESTAMP_END_9999;
    return devm_rtc_register_device(rtc);
    }
    static struct spi_driver ds1347_driver = {
    .driver = {
    .name = "ds1347",
    },
    .probe = ds1347_probe,
    };
    module_spi_driver(ds1347_driver);
    MODULE_DESCRIPTION("DS1347 SPI RTC DRIVER");
    MODULE_AUTHOR("Raghavendra C Ganiga <ravi23ganiga@gmail.com>");
    MODULE_LICENSE("GPL v2");
