//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-88pm886.c
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
// Time is calculated as the sum of a 32-bit read-only advancing counter and a
// writeable constant offset stored in the chip's spare registers.
//
#[no_mangle]
unsafe extern "C" fn pm886_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pm886_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct regmap *regmap = dev_get_drvdata(dev);
    u32 time;
    u32 buf;
    int ret;
    ret = regmap_bulk_read(regmap, PM886_REG_RTC_SPARE1, &buf, 4);
    if (ret)
    return ret;
    time = buf;
    ret = regmap_bulk_read(regmap, PM886_REG_RTC_CNT1, &buf, 4);
    if (ret)
    return ret;
    time += buf;
    rtc_time64_to_tm(time, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm886_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pm886_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct regmap *regmap = dev_get_drvdata(dev);
    u32 buf;
    int ret;
    ret = regmap_bulk_read(regmap, PM886_REG_RTC_CNT1, &buf, 4);
    if (ret)
    return ret;
    buf = rtc_tm_to_time64(tm) - buf;
    return regmap_bulk_write(regmap, PM886_REG_RTC_SPARE1, &buf, 4);
    }
    static const struct rtc_class_ops pm886_rtc_ops = {
    .read_time = pm886_rtc_read_time,
    .set_time = pm886_rtc_set_time,
    };
#[no_mangle]
unsafe extern "C" fn pm886_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int pm886_rtc_probe(struct platform_device *pdev)
    {
    struct pm886_chip *chip = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    struct rtc_device *rtc;
    int ret;
    platform_set_drvdata(pdev, chip.regmap);
    rtc = devm_rtc_allocate_device(dev);
    if (IS_ERR(rtc))
    return dev_err_probe(dev, PTR_ERR(rtc),
    "Failed to allocate RTC device\n");
    rtc.ops = &pm886_rtc_ops;
    rtc.range_max = U32_MAX;
    ret = devm_rtc_register_device(rtc);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register RTC device\n");
    return 0;
    }
    static const struct platform_device_id pm886_rtc_id_table[] = {
    { .name = "88pm886-rtc" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, pm886_rtc_id_table);
    static struct platform_driver pm886_rtc_driver = {
    .driver = {
    .name = "88pm886-rtc",
    },
    .probe = pm886_rtc_probe,
    .id_table = pm886_rtc_id_table,
    };
    module_platform_driver(pm886_rtc_driver);
    MODULE_DESCRIPTION("Marvell 88PM886 RTC driver");
    MODULE_AUTHOR("Karel Balej <balejk@matfyz.cz>");
    MODULE_LICENSE("GPL");
