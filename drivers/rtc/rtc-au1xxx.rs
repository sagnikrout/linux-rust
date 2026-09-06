//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-au1xxx.c
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
// Au1xxx counter0 (aka Time-Of-Year counter) RTC interface driver.
//
// Copyright (C) 2008 Manuel Lauss <mano@roarinelk.homelinux.net>
//
// All current Au1xxx SoCs have 2 counters fed by an external 32.768 kHz
// crystal. Counter 0, which keeps counting during sleep/powerdown, is
// used to count seconds since the beginning of the unix epoch.
//
// The counters must be configured and enabled by bootloader/board code;
// no checks as to whether they really get a proper 32.768kHz clock are
// made as this would take far too long.
//

// 32kHz clock enabled and detected

#[no_mangle]
unsafe extern "C" fn au1xtoy_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int au1xtoy_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    unsigned long t;
    t = alchemy_rdsys(AU1000_SYS_TOYREAD);
    rtc_time64_to_tm(t, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn au1xtoy_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int au1xtoy_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    unsigned long t;
    t = rtc_tm_to_time64(tm);
    alchemy_wrsys(t, AU1000_SYS_TOYWRITE);
// wait for the pending register write to succeed.  This can
// take up to 6 seconds...
//
    while (alchemy_rdsys(AU1000_SYS_CNTRCTRL) & SYS_CNTRL_C0S)
    msleep(1);
    return 0;
    }
    static const struct rtc_class_ops au1xtoy_rtc_ops = {
    .read_time	= au1xtoy_rtc_read_time,
    .set_time	= au1xtoy_rtc_set_time,
    };
#[no_mangle]
unsafe extern "C" fn au1xtoy_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int au1xtoy_rtc_probe(struct platform_device *pdev)
    {
    struct rtc_device *rtcdev;
    unsigned long t;
    t = alchemy_rdsys(AU1000_SYS_CNTRCTRL);
    if (!(t & CNTR_OK)) {
    dev_err(&pdev.dev, "counters not working; aborting.\n");
    return -ENODEV;
    }
// set counter0 tickrate to 1Hz if necessary
    if (alchemy_rdsys(AU1000_SYS_TOYTRIM) != 32767) {
// wait until hardware gives access to TRIM register
    t = 0x00100000;
    while ((alchemy_rdsys(AU1000_SYS_CNTRCTRL) & SYS_CNTRL_T0S) && --t)
    msleep(1);
    if (!t) {
// timed out waiting for register access; assume
// counters are unusable.
//
    dev_err(&pdev.dev, "timeout waiting for access\n");
    return -ETIMEDOUT;
    }
// set 1Hz TOY tick rate
    alchemy_wrsys(32767, AU1000_SYS_TOYTRIM);
    }
// wait until the hardware allows writes to the counter reg
    while (alchemy_rdsys(AU1000_SYS_CNTRCTRL) & SYS_CNTRL_C0S)
    msleep(1);
    rtcdev = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtcdev))
    return PTR_ERR(rtcdev);
    rtcdev.ops = &au1xtoy_rtc_ops;
    rtcdev.range_max = U32_MAX;
    platform_set_drvdata(pdev, rtcdev);
    return devm_rtc_register_device(rtcdev);
    }
    static struct platform_driver au1xrtc_driver = {
    .driver		= {
    .name	= "rtc-au1xxx",
    },
    };
    module_platform_driver_probe(au1xrtc_driver, au1xtoy_rtc_probe);
    MODULE_DESCRIPTION("Au1xxx TOY-counter-based RTC driver");
    MODULE_AUTHOR("Manuel Lauss <manuel.lauss@gmail.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:rtc-au1xxx");
