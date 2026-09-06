//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-starfire.c
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


// rtc-starfire.c: Starfire platform RTC driver.
//
// Author: David S. Miller
// License: GPL
//
// Copyright (C) 2008 David S. Miller <davem@davemloft.net>
//

#[no_mangle]
unsafe extern "C" fn starfire_get_time() -> u32 {
    static u32 starfire_get_time(void)
    {
    static char obp_gettod[32];
    static u32 unix_tod;
    sprintf(obp_gettod, "h# %08x unix-gettod",
    (unsigned int) (long) &unix_tod);
    prom_feval(obp_gettod);
    return unix_tod;
    }
#[no_mangle]
unsafe extern "C" fn starfire_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int starfire_read_time(struct device *dev, struct rtc_time *tm)
    {
    rtc_time64_to_tm(starfire_get_time(), tm);
    return 0;
    }
    static const struct rtc_class_ops starfire_rtc_ops = {
    .read_time	= starfire_read_time,
    };
#[no_mangle]
unsafe extern "C" fn starfire_rtc_probe(pdev: *mut platform_device) -> int __init {
    static int __init starfire_rtc_probe(struct platform_device *pdev)
    {
    struct rtc_device *rtc;
    rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    rtc.ops = &starfire_rtc_ops;
    rtc.range_max = U32_MAX;
    platform_set_drvdata(pdev, rtc);
    return devm_rtc_register_device(rtc);
    }
    static struct platform_driver starfire_rtc_driver = {
    .driver		= {
    .name	= "rtc-starfire",
    },
    };
    builtin_platform_driver_probe(starfire_rtc_driver, starfire_rtc_probe);
