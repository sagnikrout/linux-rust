//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-spacemit-p1.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Driver for the RTC found in the SpacemiT P1 PMIC
//
// Copyright (C) 2025 by RISCstar Solutions Corporation.  All rights reserved.
//

//
// Six consecutive 1-byte registers hold the seconds, minutes, hours,
// day-of-month, month, and year (respectively).
//
// The range of values in these registers is:
// seconds	0-59
// minutes	0-59
// hours	0-59
// day	0-30 (struct tm is 1-31)
// month	0-11
// year	years since 2000 (struct tm is since 1900)
//
// Note that the day and month must be converted after reading and
// before writing.
//
pub const RTC_TIME: c_uint = 0x0d	/* Offset of the seconds register */;
pub const RTC_CTRL: c_uint = 0x1d;

// Number of attempts to read a consistent time stamp before giving up

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p1_rtc {
    pub regmap: *mut regmap,
    pub rtc: *mut rtc_device,
}

//
// The P1 hardware documentation states that the register values are
// latched to ensure a consistent time snapshot within the registers,
// but these are in fact unstable due to a bug in the hardware design.
// So we loop until we get two identical readings.
//
#[no_mangle]
unsafe extern "C" fn p1_rtc_read_time(dev: *mut device, t: *mut rtc_time) -> c_int {
    static int p1_rtc_read_time(struct device *dev, struct rtc_time *t)
    {
    struct p1_rtc *p1 = dev_get_drvdata(dev);
    struct regmap *regmap = p1.regmap;
    let mut count: u32 = RTC_READ_TRIES;
    u8 seconds;
    u8 time[6];
    int ret;
    ret = regmap_test_bits(regmap, RTC_CTRL, RTC_EN);
    if (ret <= 0)
    return ret ?: -EINVAL;	/* RTC is disabled or error */
    ret = regmap_bulk_read(regmap, RTC_TIME, time, sizeof(time));
    if (ret)
    return ret;
    do {
    seconds = time[0];
    ret = regmap_bulk_read(regmap, RTC_TIME, time, sizeof(time));
    if (ret)
    return ret;
    } while (time[0] != seconds && --count);
    if (!count)
    return -EIO;		/* Unable to get a consistent result */
    t.tm_sec = time[0] & GENMASK(5, 0);
    t.tm_min = time[1] & GENMASK(5, 0);
    t.tm_hour = time[2] & GENMASK(4, 0);
    t.tm_mday = (time[3] & GENMASK(4, 0)) + 1;
    t.tm_mon = time[4] & GENMASK(3, 0);
    t.tm_year = (time[5] & GENMASK(5, 0)) + 100;
    return 0;
    }
//
// The P1 hardware documentation states that values in the registers are
// latched so when written they represent a consistent time snapshot.
// Nevertheless, this is not guaranteed by the implementation, so we must
// disable the RTC while updating it.
//
#[no_mangle]
unsafe extern "C" fn p1_rtc_set_time(dev: *mut device, t: *mut rtc_time) -> c_int {
    static int p1_rtc_set_time(struct device *dev, struct rtc_time *t)
    {
    struct p1_rtc *p1 = dev_get_drvdata(dev);
    struct regmap *regmap = p1.regmap;
    u8 time[6];
    int ret;
    time[0] = t.tm_sec;
    time[1] = t.tm_min;
    time[2] = t.tm_hour;
    time[3] = t.tm_mday - 1;
    time[4] = t.tm_mon;
    time[5] = t.tm_year - 100;
// Disable the RTC to update; re-enable again when done
    ret = regmap_clear_bits(regmap, RTC_CTRL, RTC_EN);
    if (ret)
    return ret;
// If something goes wrong, leave the RTC disabled
    ret = regmap_bulk_write(regmap, RTC_TIME, time, sizeof(time));
    if (ret)
    return ret;
    return regmap_set_bits(regmap, RTC_CTRL, RTC_EN);
    }
    static const struct rtc_class_ops p1_rtc_class_ops = {
    .read_time = p1_rtc_read_time,
    .set_time = p1_rtc_set_time,
    };
#[no_mangle]
unsafe extern "C" fn p1_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int p1_rtc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rtc_device *rtc;
    struct p1_rtc *p1;
    p1 = devm_kzalloc(dev, sizeof(*p1), GFP_KERNEL);
    if (!p1)
    return -ENOMEM;
    dev_set_drvdata(dev, p1);
    p1.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!p1.regmap)
    return dev_err_probe(dev, -ENODEV, "failed to get regmap\n");
    rtc = devm_rtc_allocate_device(dev);
    if (IS_ERR(rtc))
    return dev_err_probe(dev, PTR_ERR(rtc),
    "error allocating device\n");
    p1.rtc = rtc;
    rtc.ops = &p1_rtc_class_ops;
    rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    rtc.range_max = RTC_TIMESTAMP_END_2063;
    clear_bit(RTC_FEATURE_ALARM, rtc.features);
    clear_bit(RTC_FEATURE_UPDATE_INTERRUPT, rtc.features);
    return devm_rtc_register_device(rtc);
    }
    static struct platform_driver p1_rtc_driver = {
    .probe = p1_rtc_probe,
    .driver = {
    .name = MOD_NAME,
    },
    };
    module_platform_driver(p1_rtc_driver);
    MODULE_DESCRIPTION("SpacemiT P1 RTC driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" MOD_NAME);
