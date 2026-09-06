//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ps3.c
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
// PS3 RTC Driver
//
// Copyright 2009 Sony Corporation
//

#[no_mangle]
unsafe extern "C" fn read_rtc() -> u64 {
    static u64 read_rtc(void)
    {
    int result;
    u64 rtc_val;
    u64 tb_val;
    result = lv1_get_rtc(&rtc_val, &tb_val);
    BUG_ON(result);
    return rtc_val;
    }
#[no_mangle]
unsafe extern "C" fn ps3_get_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ps3_get_time(struct device *dev, struct rtc_time *tm)
    {
    rtc_time64_to_tm(read_rtc() + ps3_os_area_get_rtc_diff(), tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps3_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ps3_set_time(struct device *dev, struct rtc_time *tm)
    {
    ps3_os_area_set_rtc_diff(rtc_tm_to_time64(tm) - read_rtc());
    return 0;
    }
    static const struct rtc_class_ops ps3_rtc_ops = {
    .read_time = ps3_get_time,
    .set_time = ps3_set_time,
    };
#[no_mangle]
unsafe extern "C" fn ps3_rtc_probe(dev: *mut platform_device) -> int __init {
    static int __init ps3_rtc_probe(struct platform_device *dev)
    {
    struct rtc_device *rtc;
    rtc = devm_rtc_allocate_device(&dev.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    rtc.ops = &ps3_rtc_ops;
    rtc.range_max = U64_MAX;
    platform_set_drvdata(dev, rtc);
    return devm_rtc_register_device(rtc);
    }
    static struct platform_driver ps3_rtc_driver = {
    .driver = {
    .name = "rtc-ps3",
    },
    };
    module_platform_driver_probe(ps3_rtc_driver, ps3_rtc_probe);
    MODULE_AUTHOR("Sony Corporation");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("ps3 RTC driver");
    MODULE_ALIAS("platform:rtc-ps3");
