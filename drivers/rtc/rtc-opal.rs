//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-opal.c
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
// IBM OPAL RTC driver
// Copyright (C) 2014 IBM
//

#[no_mangle]
unsafe extern "C" fn opal_to_tm(y_m_d: u32, h_m_s_ms: u64, tm: *mut rtc_time) {
    static void opal_to_tm(u32 y_m_d, u64 h_m_s_ms, struct rtc_time *tm)
    {
    tm.tm_year = ((bcd2bin(y_m_d >> 24) * 100) +
    bcd2bin((y_m_d >> 16) & 0xff)) - 1900;
    tm.tm_mon  = bcd2bin((y_m_d >> 8) & 0xff) - 1;
    tm.tm_mday = bcd2bin(y_m_d & 0xff);
    tm.tm_hour = bcd2bin((h_m_s_ms >> 56) & 0xff);
    tm.tm_min  = bcd2bin((h_m_s_ms >> 48) & 0xff);
    tm.tm_sec  = bcd2bin((h_m_s_ms >> 40) & 0xff);
    tm.tm_wday = -1;
    }
#[no_mangle]
unsafe extern "C" fn tm_to_opal(tm: *mut rtc_time, y_m_d: *mut u32, h_m_s_ms: *mut u64) {
    static void tm_to_opal(struct rtc_time *tm, u32 *y_m_d, u64 *h_m_s_ms)
    {
// y_m_d |= ((u32)bin2bcd((tm->tm_year + 1900) / 100)) << 24;
// y_m_d |= ((u32)bin2bcd((tm->tm_year + 1900) % 100)) << 16;
// y_m_d |= ((u32)bin2bcd((tm->tm_mon + 1))) << 8;
// y_m_d |= ((u32)bin2bcd(tm->tm_mday));
// h_m_s_ms |= ((u64)bin2bcd(tm->tm_hour)) << 56;
// h_m_s_ms |= ((u64)bin2bcd(tm->tm_min)) << 48;
// h_m_s_ms |= ((u64)bin2bcd(tm->tm_sec)) << 40;
    }
#[no_mangle]
unsafe extern "C" fn opal_get_rtc_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int opal_get_rtc_time(struct device *dev, struct rtc_time *tm)
    {
    let mut rc: i64 = OPAL_BUSY;
    let mut retries: c_int = 10;
    u32 y_m_d;
    u64 h_m_s_ms;
    __be32 __y_m_d;
    __be64 __h_m_s_ms;
    while (rc == OPAL_BUSY || rc == OPAL_BUSY_EVENT) {
    rc = opal_rtc_read(&__y_m_d, &__h_m_s_ms);
    if (rc == OPAL_BUSY_EVENT) {
    msleep(OPAL_BUSY_DELAY_MS);
    opal_poll_events(core::ptr::null_mut());
    } else if (rc == OPAL_BUSY) {
    msleep(OPAL_BUSY_DELAY_MS);
    } else if (rc == OPAL_HARDWARE || rc == OPAL_INTERNAL_ERROR) {
    if (retries--) {
    msleep(10); /* Wait 10ms before retry */
    rc = OPAL_BUSY; /* go around again */
    }
    }
    }
    if (rc != OPAL_SUCCESS)
    return -EIO;
    y_m_d = be32_to_cpu(__y_m_d);
    h_m_s_ms = be64_to_cpu(__h_m_s_ms);
    opal_to_tm(y_m_d, h_m_s_ms, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn opal_set_rtc_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int opal_set_rtc_time(struct device *dev, struct rtc_time *tm)
    {
    let mut rc: i64 = OPAL_BUSY;
    let mut retries: c_int = 10;
    let mut y_m_d: u32 = 0;
    let mut h_m_s_ms: u64 = 0;
    tm_to_opal(tm, &y_m_d, &h_m_s_ms);
    while (rc == OPAL_BUSY || rc == OPAL_BUSY_EVENT) {
    rc = opal_rtc_write(y_m_d, h_m_s_ms);
    if (rc == OPAL_BUSY_EVENT) {
    msleep(OPAL_BUSY_DELAY_MS);
    opal_poll_events(core::ptr::null_mut());
    } else if (rc == OPAL_BUSY) {
    msleep(OPAL_BUSY_DELAY_MS);
    } else if (rc == OPAL_HARDWARE || rc == OPAL_INTERNAL_ERROR) {
    if (retries--) {
    msleep(10); /* Wait 10ms before retry */
    rc = OPAL_BUSY; /* go around again */
    }
    }
    }
    let mut rc: return = = OPAL_SUCCESS ? 0 : -EIO;
    }
//
// TPO	Timed Power-On
//
// TPO get/set OPAL calls care about the hour and min and to make it consistent
// with the rtc utility time conversion functions, we use the 'u64' to store
// its value and perform bit shift by 32 before use..
//
#[no_mangle]
unsafe extern "C" fn opal_get_tpo_time(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int opal_get_tpo_time(struct device *dev, struct rtc_wkalrm *alarm)
    {
    __be32 __y_m_d, __h_m;
    struct opal_msg msg;
    int rc, token;
    u64 h_m_s_ms;
    u32 y_m_d;
    token = opal_async_get_token_interruptible();
    if (token < 0) {
    if (token != -ERESTARTSYS)
    pr_err("Failed to get the async token\n");
    return token;
    }
    rc = opal_tpo_read(token, &__y_m_d, &__h_m);
    if (rc != OPAL_ASYNC_COMPLETION) {
    rc = -EIO;
    goto exit;
    }
    rc = opal_async_wait_response(token, &msg);
    if (rc) {
    rc = -EIO;
    goto exit;
    }
    rc = opal_get_async_rc(msg);
    if (rc != OPAL_SUCCESS) {
    rc = -EIO;
    goto exit;
    }
    y_m_d = be32_to_cpu(__y_m_d);
    h_m_s_ms = ((u64)be32_to_cpu(__h_m) << 32);
// check if no alarm is set
    if (y_m_d == 0 && h_m_s_ms == 0) {
    pr_debug("No alarm is set\n");
    rc = -ENOENT;
    goto exit;
    } else {
    pr_debug("Alarm set to %x %llx\n", y_m_d, h_m_s_ms);
    }
    opal_to_tm(y_m_d, h_m_s_ms, &alarm.time);
    exit:
    opal_async_release_token(token);
    return rc;
    }
// Set Timed Power-On
#[no_mangle]
unsafe extern "C" fn opal_set_tpo_time(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int opal_set_tpo_time(struct device *dev, struct rtc_wkalrm *alarm)
    {
    let mut h_m_s_ms: u64 = 0;
    struct opal_msg msg;
    let mut y_m_d: u32 = 0;
    int token, rc;
// if alarm is enabled
    if (alarm.enabled) {
    tm_to_opal(&alarm.time, &y_m_d, &h_m_s_ms);
    pr_debug("Alarm set to %x %llx\n", y_m_d, h_m_s_ms);
    } else {
    pr_debug("Alarm getting disabled\n");
    }
    token = opal_async_get_token_interruptible();
    if (token < 0) {
    if (token != -ERESTARTSYS)
    pr_err("Failed to get the async token\n");
    return token;
    }
// TPO, we care about hour and minute
    rc = opal_tpo_write(token, y_m_d,
    (u32)((h_m_s_ms >> 32) & 0xffff0000));
    if (rc != OPAL_ASYNC_COMPLETION) {
    rc = -EIO;
    goto exit;
    }
    rc = opal_async_wait_response(token, &msg);
    if (rc) {
    rc = -EIO;
    goto exit;
    }
    rc = opal_get_async_rc(msg);
    if (rc != OPAL_SUCCESS)
    rc = -EIO;
    exit:
    opal_async_release_token(token);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn opal_tpo_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int opal_tpo_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    let mut alarm: rtc_wkalrm = { .enabled = 0 };
//
// TPO is automatically enabled when opal_set_tpo_time() is called with
// non-zero rtc-time. We only handle disable case which needs to be
// explicitly told to opal.
//
    return enabled ? 0 : opal_set_tpo_time(dev, &alarm);
    }
    static const struct rtc_class_ops opal_rtc_ops = {
    .read_time	= opal_get_rtc_time,
    .set_time	= opal_set_rtc_time,
    .read_alarm	= opal_get_tpo_time,
    .set_alarm	= opal_set_tpo_time,
    .alarm_irq_enable = opal_tpo_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn opal_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int opal_rtc_probe(struct platform_device *pdev)
    {
    struct rtc_device *rtc;
    rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    if (pdev.dev.of_node &&
    (of_property_read_bool(pdev.dev.of_node, "wakeup-source") ||
    of_property_read_bool(pdev.dev.of_node, "has-tpo")/* legacy */))
    device_set_wakeup_capable(&pdev.dev, true);
    else
    clear_bit(RTC_FEATURE_ALARM, rtc.features);
    rtc.ops = &opal_rtc_ops;
    rtc.range_min = RTC_TIMESTAMP_BEGIN_0000;
    rtc.range_max = RTC_TIMESTAMP_END_9999;
    clear_bit(RTC_FEATURE_UPDATE_INTERRUPT, rtc.features);
    return devm_rtc_register_device(rtc);
    }
    static const struct of_device_id opal_rtc_match[] = {
    {
    .compatible	= "ibm,opal-rtc",
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, opal_rtc_match);
    static const struct platform_device_id opal_rtc_driver_ids[] = {
    {
    .name		= "opal-rtc",
    },
    { }
    };
    MODULE_DEVICE_TABLE(platform, opal_rtc_driver_ids);
    static struct platform_driver opal_rtc_driver = {
    .probe		= opal_rtc_probe,
    .id_table	= opal_rtc_driver_ids,
    .driver		= {
    .name		= DRVNAME,
    .of_match_table	= opal_rtc_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn opal_rtc_init() -> int __init {
    static int __init opal_rtc_init(void)
    {
    if (!firmware_has_feature(FW_FEATURE_OPAL))
    return -ENODEV;
    return platform_driver_register(&opal_rtc_driver);
    }
#[no_mangle]
unsafe extern "C" fn opal_rtc_exit() -> void __exit {
    static void __exit opal_rtc_exit(void)
    {
    platform_driver_unregister(&opal_rtc_driver);
    }
    MODULE_AUTHOR("Neelesh Gupta <neelegup@linux.vnet.ibm.com>");
    MODULE_DESCRIPTION("IBM OPAL RTC driver");
    MODULE_LICENSE("GPL");
    module_init(opal_rtc_init);
    module_exit(opal_rtc_exit);
