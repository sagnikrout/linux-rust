//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ntxec.c
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
// The Netronix embedded controller is a microcontroller found in some
// e-book readers designed by the original design manufacturer Netronix, Inc.
// It contains RTC, battery monitoring, system power management, and PWM
// functionality.
//
// This driver implements access to the RTC time and date.
//
// Copyright 2020 Jonathan Neuschäfer <j.neuschaefer@gmx.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntxec_rtc {
    pub dev: *mut device,
    pub ec: *mut ntxec,
}

pub const NTXEC_REG_WRITE_YEAR: c_uint = 0x10;
pub const NTXEC_REG_WRITE_MONTH: c_uint = 0x11;
pub const NTXEC_REG_WRITE_DAY: c_uint = 0x12;
pub const NTXEC_REG_WRITE_HOUR: c_uint = 0x13;
pub const NTXEC_REG_WRITE_MINUTE: c_uint = 0x14;
pub const NTXEC_REG_WRITE_SECOND: c_uint = 0x15;
pub const NTXEC_REG_READ_YEAR_MONTH: c_uint = 0x20;
pub const NTXEC_REG_READ_MDAY_HOUR: c_uint = 0x21;
pub const NTXEC_REG_READ_MINUTE_SECOND: c_uint = 0x23;
#[no_mangle]
unsafe extern "C" fn ntxec_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ntxec_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct ntxec_rtc *rtc = dev_get_drvdata(dev);
    unsigned int value;
    int res;
    retry:
    res = regmap_read(rtc.ec.regmap, NTXEC_REG_READ_MINUTE_SECOND, &value);
    if (res < 0)
    return res;
    tm.tm_min = value >> 8;
    tm.tm_sec = value & 0xff;
    res = regmap_read(rtc.ec.regmap, NTXEC_REG_READ_MDAY_HOUR, &value);
    if (res < 0)
    return res;
    tm.tm_mday = value >> 8;
    tm.tm_hour = value & 0xff;
    res = regmap_read(rtc.ec.regmap, NTXEC_REG_READ_YEAR_MONTH, &value);
    if (res < 0)
    return res;
    tm.tm_year = (value >> 8) + 100;
    tm.tm_mon = (value & 0xff) - 1;
//
// Read the minutes/seconds field again. If it changed since the first
// read, we can't assume that the values read so far are consistent,
// and should start from the beginning.
//
    res = regmap_read(rtc.ec.regmap, NTXEC_REG_READ_MINUTE_SECOND, &value);
    if (res < 0)
    return res;
    if (tm.tm_min != value >> 8 || tm.tm_sec != (value & 0xff))
    goto retry;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntxec_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ntxec_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct ntxec_rtc *rtc = dev_get_drvdata(dev);
//
// To avoid time overflows while we're writing the full date/time,
// set the seconds field to zero before doing anything else. For the
// next 59 seconds (plus however long it takes until the RTC's next
// update of the second field), the seconds field will not overflow
// into the other fields.
//
    struct reg_sequence regs[] = {
    { NTXEC_REG_WRITE_SECOND, ntxec_reg8(0) },
    { NTXEC_REG_WRITE_YEAR, ntxec_reg8(tm.tm_year - 100) },
    { NTXEC_REG_WRITE_MONTH, ntxec_reg8(tm.tm_mon + 1) },
    { NTXEC_REG_WRITE_DAY, ntxec_reg8(tm.tm_mday) },
    { NTXEC_REG_WRITE_HOUR, ntxec_reg8(tm.tm_hour) },
    { NTXEC_REG_WRITE_MINUTE, ntxec_reg8(tm.tm_min) },
    { NTXEC_REG_WRITE_SECOND, ntxec_reg8(tm.tm_sec) },
    };
    return regmap_multi_reg_write(rtc.ec.regmap, regs, ARRAY_SIZE(regs));
    }
    static const struct rtc_class_ops ntxec_rtc_ops = {
    .read_time = ntxec_read_time,
    .set_time = ntxec_set_time,
    };
#[no_mangle]
unsafe extern "C" fn ntxec_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int ntxec_rtc_probe(struct platform_device *pdev)
    {
    struct rtc_device *dev;
    struct ntxec_rtc *rtc;
    device_set_of_node_from_dev(&pdev.dev, pdev.dev.parent);
    rtc = devm_kzalloc(&pdev.dev, sizeof(*rtc), GFP_KERNEL);
    if (!rtc)
    return -ENOMEM;
    rtc.dev = &pdev.dev;
    rtc.ec = dev_get_drvdata(pdev.dev.parent);
    platform_set_drvdata(pdev, rtc);
    dev = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(dev))
    return PTR_ERR(dev);
    dev.ops = &ntxec_rtc_ops;
    dev.range_min = RTC_TIMESTAMP_BEGIN_2000;
    dev.range_max = 9025257599LL; /* 2255-12-31 23:59:59 */
    return devm_rtc_register_device(dev);
    }
    static struct platform_driver ntxec_rtc_driver = {
    .driver = {
    .name = "ntxec-rtc",
    },
    .probe = ntxec_rtc_probe,
    };
    module_platform_driver(ntxec_rtc_driver);
    MODULE_AUTHOR("Jonathan Neuschäfer <j.neuschaefer@gmx.net>");
    MODULE_DESCRIPTION("RTC driver for Netronix EC");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:ntxec-rtc");
