//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-sun4v.c
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
// rtc-sun4v.c: Hypervisor based RTC for SUN4V systems.
//
// Author: David S. Miller
//
// Copyright (C) 2008 David S. Miller <davem@davemloft.net>
//

#[no_mangle]
unsafe extern "C" fn hypervisor_get_time() -> c_ulong {
    static unsigned long hypervisor_get_time(void)
    {
    unsigned long ret, time;
    let mut retries: c_int = 10000;
    retry:
    ret = sun4v_tod_get(&time);
    if (ret == HV_EOK)
    return time;
    if (ret == HV_EWOULDBLOCK) {
    if (--retries > 0) {
    udelay(100);
    goto retry;
    }
    pr_warn("tod_get() timed out.\n");
    return 0;
    }
    pr_warn("tod_get() not supported.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4v_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int sun4v_read_time(struct device *dev, struct rtc_time *tm)
    {
    rtc_time64_to_tm(hypervisor_get_time(), tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hypervisor_set_time(secs: c_ulong) -> c_int {
    static int hypervisor_set_time(unsigned long secs)
    {
    unsigned long ret;
    let mut retries: c_int = 10000;
    retry:
    ret = sun4v_tod_set(secs);
    if (ret == HV_EOK)
    return 0;
    if (ret == HV_EWOULDBLOCK) {
    if (--retries > 0) {
    udelay(100);
    goto retry;
    }
    pr_warn("tod_set() timed out.\n");
    return -EAGAIN;
    }
    pr_warn("tod_set() not supported.\n");
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn sun4v_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int sun4v_set_time(struct device *dev, struct rtc_time *tm)
    {
    return hypervisor_set_time(rtc_tm_to_time64(tm));
    }
    static const struct rtc_class_ops sun4v_rtc_ops = {
    .read_time	= sun4v_read_time,
    .set_time	= sun4v_set_time,
    };
#[no_mangle]
unsafe extern "C" fn sun4v_rtc_probe(pdev: *mut platform_device) -> int __init {
    static int __init sun4v_rtc_probe(struct platform_device *pdev)
    {
    struct rtc_device *rtc;
    rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    rtc.ops = &sun4v_rtc_ops;
    rtc.range_max = U64_MAX;
    platform_set_drvdata(pdev, rtc);
    return devm_rtc_register_device(rtc);
    }
    static struct platform_driver sun4v_rtc_driver = {
    .driver		= {
    .name	= "rtc-sun4v",
    },
    };
    builtin_platform_driver_probe(sun4v_rtc_driver, sun4v_rtc_probe);
