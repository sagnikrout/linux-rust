//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/olpc/olpc-xo1-rtc.c
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
// Support for OLPC XO-1 Real Time Clock (RTC)
//
// Copyright (C) 2011 One Laptop per Child
//

#[no_mangle]
unsafe extern "C" fn rtc_wake_on(dev: *mut device) {
    static void rtc_wake_on(struct device *dev)
    {
    olpc_xo1_pm_wakeup_set(CS5536_PM_RTC);
    }
#[no_mangle]
unsafe extern "C" fn rtc_wake_off(dev: *mut device) {
    static void rtc_wake_off(struct device *dev)
    {
    olpc_xo1_pm_wakeup_clear(CS5536_PM_RTC);
    }
    static struct resource rtc_platform_resource[] = {
    [0] = {
    .start	= RTC_PORT(0),
    .end	= RTC_PORT(1),
    .flags	= IORESOURCE_IO,
    },
    [1] = {
    .start	= RTC_IRQ,
    .end	= RTC_IRQ,
    .flags	= IORESOURCE_IRQ,
    }
    };
    static struct cmos_rtc_board_info rtc_info = {
    .rtc_day_alarm = 0,
    .rtc_mon_alarm = 0,
    .rtc_century = 0,
    .wake_on = rtc_wake_on,
    .wake_off = rtc_wake_off,
    };
    static struct platform_device xo1_rtc_device = {
    .name = "rtc_cmos",
    .id = -1,
    .num_resources = ARRAY_SIZE(rtc_platform_resource),
    .dev.platform_data = &rtc_info,
    .resource = rtc_platform_resource,
    };
#[no_mangle]
unsafe extern "C" fn xo1_rtc_init() -> int __init {
    static int __init xo1_rtc_init(void)
    {
    int r;
    struct device_node *node;
    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "olpc,xo1-rtc");
    if (!node)
    return 0;
    of_node_put(node);
    pr_info("olpc-xo1-rtc: Initializing OLPC XO-1 RTC\n");
    rdmsrq(MSR_RTC_DOMA_OFFSET, rtc_info.rtc_day_alarm);
    rdmsrq(MSR_RTC_MONA_OFFSET, rtc_info.rtc_mon_alarm);
    rdmsrq(MSR_RTC_CEN_OFFSET, rtc_info.rtc_century);
    r = platform_device_register(&xo1_rtc_device);
    if (r)
    return r;
    x86_platform.legacy.rtc = 0;
    device_init_wakeup(&xo1_rtc_device.dev, 1);
    return 0;
    }
    arch_initcall(xo1_rtc_init);
