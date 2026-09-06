//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-em3027.c
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
// An rtc/i2c driver for the EM Microelectronic EM3027
// Copyright 2011 CompuLab, Ltd.
//
// Author: Mike Rapoport <mike@compulab.co.il>
//
// Based on rtc-ds1672.c by Alessandro Zummo <a.zummo@towertech.it>
//

// Registers
pub const EM3027_REG_ON_OFF_CTRL: c_uint = 0x00;
pub const EM3027_REG_IRQ_CTRL: c_uint = 0x01;
pub const EM3027_REG_IRQ_FLAGS: c_uint = 0x02;
pub const EM3027_REG_STATUS: c_uint = 0x03;
pub const EM3027_REG_RST_CTRL: c_uint = 0x04;
pub const EM3027_REG_WATCH_SEC: c_uint = 0x08;
pub const EM3027_REG_WATCH_MIN: c_uint = 0x09;
pub const EM3027_REG_WATCH_HOUR: c_uint = 0x0a;
pub const EM3027_REG_WATCH_DATE: c_uint = 0x0b;
pub const EM3027_REG_WATCH_DAY: c_uint = 0x0c;
pub const EM3027_REG_WATCH_MON: c_uint = 0x0d;
pub const EM3027_REG_WATCH_YEAR: c_uint = 0x0e;
pub const EM3027_REG_ALARM_SEC: c_uint = 0x10;
pub const EM3027_REG_ALARM_MIN: c_uint = 0x11;
pub const EM3027_REG_ALARM_HOUR: c_uint = 0x12;
pub const EM3027_REG_ALARM_DATE: c_uint = 0x13;
pub const EM3027_REG_ALARM_DAY: c_uint = 0x14;
pub const EM3027_REG_ALARM_MON: c_uint = 0x15;
pub const EM3027_REG_ALARM_YEAR: c_uint = 0x16;
    static struct i2c_driver em3027_driver;
#[no_mangle]
unsafe extern "C" fn em3027_get_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int em3027_get_time(struct device *dev, struct rtc_time *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    let mut addr: c_uchar = EM3027_REG_WATCH_SEC;
    unsigned char buf[7];
    struct i2c_msg msgs[] = {
    {/* setup read addr */
    .addr = client.addr,
    .len = 1,
    .buf = &addr
    },
    {/* read time/date */
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = 7,
    .buf = buf
    },
    };
// read time/date registers
    if ((i2c_transfer(client.adapter, &msgs[0], 2)) != 2) {
    dev_err(&client.dev, "%s: read error\n", __func__);
    return -EIO;
    }
    tm.tm_sec	= bcd2bin(buf[0]);
    tm.tm_min	= bcd2bin(buf[1]);
    tm.tm_hour	= bcd2bin(buf[2]);
    tm.tm_mday	= bcd2bin(buf[3]);
    tm.tm_wday	= bcd2bin(buf[4]);
    tm.tm_mon	= bcd2bin(buf[5]) - 1;
    tm.tm_year	= bcd2bin(buf[6]) + 100;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn em3027_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int em3027_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    unsigned char buf[8];
    struct i2c_msg msg = {
    .addr = client.addr,
    .len = 8,
    .buf = buf,	/* write time/date */
    };
    buf[0] = EM3027_REG_WATCH_SEC;
    buf[1] = bin2bcd(tm.tm_sec);
    buf[2] = bin2bcd(tm.tm_min);
    buf[3] = bin2bcd(tm.tm_hour);
    buf[4] = bin2bcd(tm.tm_mday);
    buf[5] = bin2bcd(tm.tm_wday);
    buf[6] = bin2bcd(tm.tm_mon + 1);
    buf[7] = bin2bcd(tm.tm_year % 100);
// write time/date registers
    if ((i2c_transfer(client.adapter, &msg, 1)) != 1) {
    dev_err(&client.dev, "%s: write error\n", __func__);
    return -EIO;
    }
    return 0;
    }
    static const struct rtc_class_ops em3027_rtc_ops = {
    .read_time = em3027_get_time,
    .set_time = em3027_set_time,
    };
#[no_mangle]
unsafe extern "C" fn em3027_probe(client: *mut i2c_client) -> c_int {
    static int em3027_probe(struct i2c_client *client)
    {
    struct rtc_device *rtc;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -ENODEV;
    rtc = devm_rtc_device_register(&client.dev, em3027_driver.driver.name,
    &em3027_rtc_ops, THIS_MODULE);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    i2c_set_clientdata(client, rtc);
    return 0;
    }
    static const struct i2c_device_id em3027_id[] = {
    { .name = "em3027" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, em3027_id);

    static const struct of_device_id em3027_of_match[] = {
    { .compatible = "emmicro,em3027", },
    {}
    };
    MODULE_DEVICE_TABLE(of, em3027_of_match);

    static struct i2c_driver em3027_driver = {
    .driver = {
    .name = "rtc-em3027",
    .of_match_table = of_match_ptr(em3027_of_match),
    },
    .probe = em3027_probe,
    .id_table = em3027_id,
    };
    module_i2c_driver(em3027_driver);
    MODULE_AUTHOR("Mike Rapoport <mike@compulab.co.il>");
    MODULE_DESCRIPTION("EM Microelectronic EM3027 RTC driver");
    MODULE_LICENSE("GPL");
