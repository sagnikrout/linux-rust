//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/rtc.c
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
// RTC related functions
//

//
// This is a special lock that is owned by the CPU and holds the index
// register we are working with.  It is required for NMI access to the
// CMOS/RTC registers.  See arch/x86/include/asm/mc146818rtc.h for details.
//
    volatile unsigned long cmos_lock;
    EXPORT_SYMBOL(cmos_lock);

    DEFINE_SPINLOCK(rtc_lock);
    EXPORT_SYMBOL(rtc_lock);
//
// In order to set the CMOS clock precisely, mach_set_cmos_time has to be
// called 500 ms after the second nowtime has started, because when
// nowtime is written into the registers of the CMOS clock, it will
// jump to the next second precisely 500 ms later. Check the Motorola
// MC146818A or Dallas DS12887 data sheet for details.
//
#[no_mangle]
pub unsafe extern "C" fn mach_set_cmos_time(now: *const timespec64) -> c_int {
    int mach_set_cmos_time(const struct timespec64 *now)
    {
    let mut nowtime: c_ulonglong = now.tv_sec;
    struct rtc_time tm;
    let mut retval: c_int = 0;
    rtc_time64_to_tm(nowtime, &tm);
    if (!rtc_valid_tm(&tm)) {
    retval = mc146818_set_time(&tm);
    if (retval)
    printk(KERN_ERR "%s: RTC write failed with error %d\n",
    __func__, retval);
    } else {
    printk(KERN_ERR
    "%s: Invalid RTC value: write of %llx to RTC failed\n",
    __func__, nowtime);
    retval = -EINVAL;
    }
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn mach_get_cmos_time(now: *mut timespec64) {
    void mach_get_cmos_time(struct timespec64 *now)
    {
    struct rtc_time tm;
//
// If pm_trace abused the RTC as storage, set the timespec to 0,
// which tells the caller that this RTC value is unusable.
//
    if (!pm_trace_rtc_valid()) {
    now.tv_sec = now.tv_nsec = 0;
    return;
    }
    if (mc146818_get_time(&tm, 1000)) {
    pr_err("Unable to read current time from RTC\n");
    now.tv_sec = now.tv_nsec = 0;
    return;
    }
    now.tv_sec = rtc_tm_to_time64(&tm);
    now.tv_nsec = 0;
    }
// Routines for accessing the CMOS RAM/RTC.
#[no_mangle]
pub unsafe extern "C" fn rtc_cmos_read(addr: c_uchar) -> c_uchar {
    unsigned char rtc_cmos_read(unsigned char addr)
    {
    unsigned char val;
    lock_cmos_prefix(addr);
    outb(addr, RTC_PORT(0));
    val = inb(RTC_PORT(1));
    lock_cmos_suffix(addr);
    return val;
    }
    EXPORT_SYMBOL(rtc_cmos_read);
#[no_mangle]
pub unsafe extern "C" fn rtc_cmos_write(val: c_uchar, addr: c_uchar) {
    void rtc_cmos_write(unsigned char val, unsigned char addr)
    {
    lock_cmos_prefix(addr);
    outb(addr, RTC_PORT(0));
    outb(val, RTC_PORT(1));
    lock_cmos_suffix(addr);
    }
    EXPORT_SYMBOL(rtc_cmos_write);
#[no_mangle]
pub unsafe extern "C" fn update_persistent_clock64(now: timespec64) -> c_int {
    int update_persistent_clock64(struct timespec64 now)
    {
    return x86_platform.set_wallclock(&now);
    }
// not static: needed by APM
#[no_mangle]
pub unsafe extern "C" fn read_persistent_clock64(ts: *mut timespec64) {
    void read_persistent_clock64(struct timespec64 *ts)
    {
    x86_platform.get_wallclock(ts);
    }
    static struct resource rtc_resources[] = {
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
    static struct platform_device rtc_device = {
    .name		= "rtc_cmos",
    .id		= -1,
    .resource	= rtc_resources,
    .num_resources	= ARRAY_SIZE(rtc_resources),
    };
#[no_mangle]
unsafe extern "C" fn add_rtc_cmos() -> __init int {
    static __init int add_rtc_cmos(void)
    {
    if (cmos_rtc_platform_device_present)
    return 0;
    if (!x86_platform.legacy.rtc)
    return -ENODEV;
    platform_device_register(&rtc_device);
    dev_info(&rtc_device.dev, "registered fallback platform RTC device\n");
    return 0;
    }
    device_initcall(add_rtc_cmos);
