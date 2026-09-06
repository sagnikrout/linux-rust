//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-rtc.c
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
// PowerNV Real Time Clock.
//
// Copyright 2011 IBM Corp.
//

#[no_mangle]
unsafe extern "C" fn opal_to_tm(y_m_d: u32, h_m_s_ms: u64, tm: *mut rtc_time) -> void __init {
    static void __init opal_to_tm(u32 y_m_d, u64 h_m_s_ms, struct rtc_time *tm)
    {
    tm.tm_year	= ((bcd2bin(y_m_d >> 24) * 100) +
    bcd2bin((y_m_d >> 16) & 0xff)) - 1900;
    tm.tm_mon	= bcd2bin((y_m_d >> 8) & 0xff) - 1;
    tm.tm_mday	= bcd2bin(y_m_d & 0xff);
    tm.tm_hour	= bcd2bin((h_m_s_ms >> 56) & 0xff);
    tm.tm_min	= bcd2bin((h_m_s_ms >> 48) & 0xff);
    tm.tm_sec	= bcd2bin((h_m_s_ms >> 40) & 0xff);
    tm.tm_wday     = -1;
    }
#[no_mangle]
pub unsafe extern "C" fn opal_get_boot_time() -> time64_t __init {
    time64_t __init opal_get_boot_time(void)
    {
    struct rtc_time tm;
    u32 y_m_d;
    u64 h_m_s_ms;
    __be32 __y_m_d;
    __be64 __h_m_s_ms;
    let mut rc: c_long = OPAL_BUSY;
    if (!opal_check_token(OPAL_RTC_READ))
    return 0;
    while (rc == OPAL_BUSY || rc == OPAL_BUSY_EVENT) {
    rc = opal_rtc_read(&__y_m_d, &__h_m_s_ms);
    if (rc == OPAL_BUSY_EVENT) {
    mdelay(OPAL_BUSY_DELAY_MS);
    opal_poll_events(core::ptr::null_mut());
    } else if (rc == OPAL_BUSY) {
    mdelay(OPAL_BUSY_DELAY_MS);
    }
    }
    if (rc != OPAL_SUCCESS)
    return 0;
    y_m_d = be32_to_cpu(__y_m_d);
    h_m_s_ms = be64_to_cpu(__h_m_s_ms);
    opal_to_tm(y_m_d, h_m_s_ms, &tm);
    return rtc_tm_to_time64(&tm);
    }
#[no_mangle]
unsafe extern "C" fn opal_time_init() -> __init int {
    static __init int opal_time_init(void)
    {
    struct platform_device *pdev;
    struct device_node *rtc;
    rtc = of_find_node_by_path("/ibm,opal/rtc");
    if (rtc) {
    pdev = of_platform_device_create(rtc, "opal-rtc", core::ptr::null_mut());
    of_node_put(rtc);
    } else {
    if (opal_check_token(OPAL_RTC_READ) ||
    opal_check_token(OPAL_READ_TPO))
    pdev = platform_device_register_simple("opal-rtc", -1,
    core::ptr::null_mut(), 0);
    else
    return -ENODEV;
    }
    return PTR_ERR_OR_ZERO(pdev);
    }
    machine_subsys_initcall(powernv, opal_time_init);
