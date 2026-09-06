//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-test.c
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
// An RTC test device/driver
// Copyright (C) 2005 Tower Technologies
// Author: Alessandro Zummo <a.zummo@towertech.it>
//

pub const MAX_RTC_TEST: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtc_test_data {
    pub rtc: *mut rtc_device,
    pub offset: time64_t,
    pub alarm: timer_list,
    pub alarm_en: bool,
}

    static struct platform_device *pdev[MAX_RTC_TEST];
#[no_mangle]
unsafe extern "C" fn test_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int test_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct rtc_test_data *rtd = dev_get_drvdata(dev);
    time64_t alarm;
    alarm = (rtd.alarm.expires - jiffies) / HZ;
    alarm += ktime_get_real_seconds() + rtd.offset;
    rtc_time64_to_tm(alarm, &alrm.time);
    alrm.enabled = rtd.alarm_en;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int test_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct rtc_test_data *rtd = dev_get_drvdata(dev);
    ktime_t timeout;
    u64 expires;
    timeout = rtc_tm_to_time64(&alrm.time) - ktime_get_real_seconds();
    timeout -= rtd.offset;
    timer_delete(&rtd.alarm);
    expires = jiffies + timeout * HZ;
    if (expires > U32_MAX)
    expires = U32_MAX;
    rtd.alarm.expires = expires;
    if (alrm.enabled)
    add_timer(&rtd.alarm);
    rtd.alarm_en = alrm.enabled;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int test_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct rtc_test_data *rtd = dev_get_drvdata(dev);
    rtc_time64_to_tm(ktime_get_real_seconds() + rtd.offset, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int test_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct rtc_test_data *rtd = dev_get_drvdata(dev);
    rtd.offset = rtc_tm_to_time64(tm) - ktime_get_real_seconds();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_rtc_alarm_irq_enable(dev: *mut device, enable: c_uint) -> c_int {
    static int test_rtc_alarm_irq_enable(struct device *dev, unsigned int enable)
    {
    struct rtc_test_data *rtd = dev_get_drvdata(dev);
    rtd.alarm_en = enable;
    if (enable)
    add_timer(&rtd.alarm);
    else
    timer_delete(&rtd.alarm);
    return 0;
    }
    static const struct rtc_class_ops test_rtc_ops_noalm = {
    .read_time = test_rtc_read_time,
    .set_time = test_rtc_set_time,
    .alarm_irq_enable = test_rtc_alarm_irq_enable,
    };
    static const struct rtc_class_ops test_rtc_ops = {
    .read_time = test_rtc_read_time,
    .set_time = test_rtc_set_time,
    .read_alarm = test_rtc_read_alarm,
    .set_alarm = test_rtc_set_alarm,
    .alarm_irq_enable = test_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn test_rtc_alarm_handler(t: *mut timer_list) {
    static void test_rtc_alarm_handler(struct timer_list *t)
    {
    struct rtc_test_data *rtd = timer_container_of(rtd, t, alarm);
    rtc_update_irq(rtd.rtc, 1, RTC_AF | RTC_IRQF);
    }
#[no_mangle]
unsafe extern "C" fn test_probe(plat_dev: *mut platform_device) -> c_int {
    static int test_probe(struct platform_device *plat_dev)
    {
    struct rtc_test_data *rtd;
    rtd = devm_kzalloc(&plat_dev.dev, sizeof(*rtd), GFP_KERNEL);
    if (!rtd)
    return -ENOMEM;
    platform_set_drvdata(plat_dev, rtd);
    rtd.rtc = devm_rtc_allocate_device(&plat_dev.dev);
    if (IS_ERR(rtd.rtc))
    return PTR_ERR(rtd.rtc);
    switch (plat_dev.id) {
    case 0:
    rtd.rtc.ops = &test_rtc_ops_noalm;
    break;
    default:
    rtd.rtc.ops = &test_rtc_ops;
    device_init_wakeup(&plat_dev.dev, true);
    }
    timer_setup(&rtd.alarm, test_rtc_alarm_handler, 0);
    rtd.alarm.expires = 0;
    return devm_rtc_register_device(rtd.rtc);
    }
    static struct platform_driver test_driver = {
    .probe	= test_probe,
    .driver = {
    .name = "rtc-test",
    },
    };
#[no_mangle]
unsafe extern "C" fn test_init() -> int __init {
    static int __init test_init(void)
    {
    int i, err;
    err = platform_driver_register(&test_driver);
    if (err)
    return err;
    err = -ENOMEM;
    for (i = 0; i < MAX_RTC_TEST; i++) {
    pdev[i] = platform_device_alloc("rtc-test", i);
    if (!pdev[i])
    goto exit_free_mem;
    }
    for (i = 0; i < MAX_RTC_TEST; i++) {
    err = platform_device_add(pdev[i]);
    if (err)
    goto exit_device_del;
    }
    return 0;
    exit_device_del:
    for (; i > 0; i--)
    platform_device_del(pdev[i - 1]);
    exit_free_mem:
    for (i = 0; i < MAX_RTC_TEST; i++)
    platform_device_put(pdev[i]);
    platform_driver_unregister(&test_driver);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test_exit() -> void __exit {
    static void __exit test_exit(void)
    {
    int i;
    for (i = 0; i < MAX_RTC_TEST; i++)
    platform_device_unregister(pdev[i]);
    platform_driver_unregister(&test_driver);
    }
    MODULE_AUTHOR("Alessandro Zummo <a.zummo@towertech.it>");
    MODULE_DESCRIPTION("RTC test driver/device");
    MODULE_LICENSE("GPL v2");
    module_init(test_init);
    module_exit(test_exit);
