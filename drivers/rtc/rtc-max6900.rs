//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-max6900.c
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
// rtc class driver for the Maxim MAX6900 chip
//
// Copyright (c) 2007 MontaVista, Software, Inc.
//
// Author: Dale Farnsworth <dale@farnsworth.org>
//
// based on previously existing rtc class drivers
//

//
// register indices
//

// register 8 is undocumented

pub const MAX6900_REG_LEN: c_int = 10;

//
// register read/write commands
//
pub const MAX6900_REG_CONTROL_WRITE: c_uint = 0x8e;
pub const MAX6900_REG_CENTURY_WRITE: c_uint = 0x92;
pub const MAX6900_REG_CENTURY_READ: c_uint = 0x93;
pub const MAX6900_REG_RESERVED_READ: c_uint = 0x96;
pub const MAX6900_REG_BURST_WRITE: c_uint = 0xbe;
pub const MAX6900_REG_BURST_READ: c_uint = 0xbf;

    static struct i2c_driver max6900_driver;
#[no_mangle]
unsafe extern "C" fn max6900_i2c_read_regs(client: *mut i2c_client, buf: *mut u8) -> c_int {
    static int max6900_i2c_read_regs(struct i2c_client *client, u8 *buf)
    {
    u8 reg_burst_read[1] = { MAX6900_REG_BURST_READ };
    u8 reg_century_read[1] = { MAX6900_REG_CENTURY_READ };
    struct i2c_msg msgs[4] = {
    {
    .addr = client.addr,
    .flags = 0,	/* write */
    .len = sizeof(reg_burst_read),
    .buf = reg_burst_read}
    ,
    {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = MAX6900_BURST_LEN,
    .buf = buf}
    ,
    {
    .addr = client.addr,
    .flags = 0,	/* write */
    .len = sizeof(reg_century_read),
    .buf = reg_century_read}
    ,
    {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = sizeof(buf[MAX6900_REG_CENTURY]),
    .buf = &buf[MAX6900_REG_CENTURY]
    }
    };
    int rc;
    rc = i2c_transfer(client.adapter, msgs, ARRAY_SIZE(msgs));
    if (rc != ARRAY_SIZE(msgs)) {
    dev_err(&client.dev, "%s: register read failed\n", __func__);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max6900_i2c_write_regs(client: *mut i2c_client, buf: *const u8) -> c_int {
    static int max6900_i2c_write_regs(struct i2c_client *client, u8 const *buf)
    {
    u8 i2c_century_buf[1 + 1] = { MAX6900_REG_CENTURY_WRITE };
    struct i2c_msg century_msgs[1] = {
    {
    .addr = client.addr,
    .flags = 0,	/* write */
    .len = sizeof(i2c_century_buf),
    .buf = i2c_century_buf}
    };
    u8 i2c_burst_buf[MAX6900_BURST_LEN + 1] = { MAX6900_REG_BURST_WRITE };
    struct i2c_msg burst_msgs[1] = {
    {
    .addr = client.addr,
    .flags = 0,	/* write */
    .len = sizeof(i2c_burst_buf),
    .buf = i2c_burst_buf}
    };
    int rc;
//
// We have to make separate calls to i2c_transfer because of
// the need to delay after each write to the chip.  Also,
// we write the century byte first, since we set the write-protect
// bit as part of the burst write.
//
    i2c_century_buf[1] = buf[MAX6900_REG_CENTURY];
    rc = i2c_transfer(client.adapter, century_msgs,
    ARRAY_SIZE(century_msgs));
    if (rc != ARRAY_SIZE(century_msgs))
    goto write_failed;
    msleep(MAX6900_IDLE_TIME_AFTER_WRITE);
    memcpy(&i2c_burst_buf[1], buf, MAX6900_BURST_LEN);
    rc = i2c_transfer(client.adapter, burst_msgs, ARRAY_SIZE(burst_msgs));
    if (rc != ARRAY_SIZE(burst_msgs))
    goto write_failed;
    msleep(MAX6900_IDLE_TIME_AFTER_WRITE);
    return 0;
    write_failed:
    dev_err(&client.dev, "%s: register write failed\n", __func__);
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn max6900_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int max6900_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    int rc;
    u8 regs[MAX6900_REG_LEN];
    rc = max6900_i2c_read_regs(client, regs);
    if (rc < 0)
    return rc;
    tm.tm_sec = bcd2bin(regs[MAX6900_REG_SC]);
    tm.tm_min = bcd2bin(regs[MAX6900_REG_MN]);
    tm.tm_hour = bcd2bin(regs[MAX6900_REG_HR] & 0x3f);
    tm.tm_mday = bcd2bin(regs[MAX6900_REG_DT]);
    tm.tm_mon = bcd2bin(regs[MAX6900_REG_MO]) - 1;
    tm.tm_year = bcd2bin(regs[MAX6900_REG_YR]) +
    bcd2bin(regs[MAX6900_REG_CENTURY]) * 100 - 1900;
    tm.tm_wday = bcd2bin(regs[MAX6900_REG_DW]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max6900_i2c_clear_write_protect(client: *mut i2c_client) -> c_int {
    static int max6900_i2c_clear_write_protect(struct i2c_client *client)
    {
    return i2c_smbus_write_byte_data(client, MAX6900_REG_CONTROL_WRITE, 0);
    }
#[no_mangle]
unsafe extern "C" fn max6900_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int max6900_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    u8 regs[MAX6900_REG_LEN];
    int rc;
    rc = max6900_i2c_clear_write_protect(client);
    if (rc < 0)
    return rc;
    regs[MAX6900_REG_SC] = bin2bcd(tm.tm_sec);
    regs[MAX6900_REG_MN] = bin2bcd(tm.tm_min);
    regs[MAX6900_REG_HR] = bin2bcd(tm.tm_hour);
    regs[MAX6900_REG_DT] = bin2bcd(tm.tm_mday);
    regs[MAX6900_REG_MO] = bin2bcd(tm.tm_mon + 1);
    regs[MAX6900_REG_DW] = bin2bcd(tm.tm_wday);
    regs[MAX6900_REG_YR] = bin2bcd(tm.tm_year % 100);
    regs[MAX6900_REG_CENTURY] = bin2bcd((tm.tm_year + 1900) / 100);
// set write protect
    regs[MAX6900_REG_CT] = MAX6900_REG_CT_WP;
    rc = max6900_i2c_write_regs(client, regs);
    if (rc < 0)
    return rc;
    return 0;
    }
    static const struct rtc_class_ops max6900_rtc_ops = {
    .read_time = max6900_rtc_read_time,
    .set_time = max6900_rtc_set_time,
    };
#[no_mangle]
unsafe extern "C" fn max6900_probe(client: *mut i2c_client) -> c_int {
    static int max6900_probe(struct i2c_client *client)
    {
    struct rtc_device *rtc;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -ENODEV;
    rtc = devm_rtc_device_register(&client.dev, max6900_driver.driver.name,
    &max6900_rtc_ops, THIS_MODULE);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    i2c_set_clientdata(client, rtc);
    return 0;
    }
    static const struct i2c_device_id max6900_id[] = {
    { .name = "max6900" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max6900_id);
    static struct i2c_driver max6900_driver = {
    .driver = {
    .name = "rtc-max6900",
    },
    .probe = max6900_probe,
    .id_table = max6900_id,
    };
    module_i2c_driver(max6900_driver);
    MODULE_DESCRIPTION("Maxim MAX6900 RTC driver");
    MODULE_AUTHOR("Dale Farnsworth <dale@farnsworth.org>");
    MODULE_LICENSE("GPL");
