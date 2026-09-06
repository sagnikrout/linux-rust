//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpi_tad.c
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
// ACPI Time and Alarm (TAD) Device Driver
//
// Copyright (C) 2018 - 2026 Intel Corporation
// Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
//
// This driver is based on ACPI 6.6, Section 9.17.
//
// Provided are sysfs attributes, available under the TAD platform device,
// allowing user space to manage the AC and DC wakeup timers of the TAD:
// set and read their values, set and check their expire timer wake policies,
// check and clear their status and check the capabilities of the TAD reported
// by AML.  The DC timer attributes are only present if the TAD supports a
// separate DC alarm timer.
//
// The wakeup events handling and power management of the TAD is expected to
// be taken care of by the ACPI PM domain attached to its platform device.
//
// If the TAD supports the get/set real time features, as indicated by the
// capability mask returned by _GCP under the TAD object, additional sysfs
// attributes are created allowing the real time to be set and read and an RTC
// class device is registered under the TAD platform device.
//

    MODULE_DESCRIPTION("ACPI Time and Alarm (TAD) Device Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Rafael J. Wysocki");
// ACPI TAD capability flags (ACPI 6.6, Section 9.17.2)

// ACPI TAD alarm timer selection

// Special value for disabled timer or expired timer wake policy.

// ACPI TAD RTC
pub const ACPI_TAD_TZ_UNSPEC: c_int = 2047;
pub const ACPI_TAD_TIME_ISDST: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_tad_driver_data {
    pub capabilities: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_tad_rt {
    pub /: *mut *mut u16 year; / 1900 - 9999,
    pub /: *mut *mut u8 month; / 1 - 12,
    pub /: *mut *mut u8 day; / 1 - 31,
    pub /: *mut *mut u8 hour; / 0 - 23,
    pub /: *mut *mut u8 minute; / 0 - 59,
    pub /: *mut *mut u8 second; / 0 - 59,
    pub /: *mut *mut u8 valid; / 0 (failed) or 1 (success) for reads, 0 for writes,
    pub /: *mut *mut u16 msec; / 1 - 1000,
    pub /: *mut *mut s16 tz; / -1440 to 1440 or 2047 (unspecified),
    pub daylight: u8,
    pub /: *mut *mut u8 padding[3]; / must be 0,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn acpi_tad_rt_is_invalid(rt: *mut acpi_tad_rt) -> bool {
    static bool acpi_tad_rt_is_invalid(struct acpi_tad_rt *rt)
    {
    return rt.year < 1900 || rt.year > 9999 ||
    rt.month < 1 || rt.month > 12 ||
    rt.hour > 23 || rt.minute > 59 || rt.second > 59 ||
    rt.tz < -1440 ||
    (rt.tz > 1440 && rt.tz != ACPI_TAD_TZ_UNSPEC) ||
    pub 3: rt->daylight >,
    }
    pub DEFINE_MUTEX(acpi_tad_aml_lock): static,
#[no_mangle]
unsafe extern "C" fn acpi_tad_set_real_time(dev: *mut device, rt: *mut acpi_tad_rt) -> c_int {
    static int acpi_tad_set_real_time(struct device *dev, struct acpi_tad_rt *rt)
    {
    pub ACPI_HANDLE(dev): acpi_handle handle =,
    union acpi_object args[] = {
    { .type = ACPI_TYPE_BUFFER, },
}

    struct acpi_object_list arg_list = {
    .pointer = args,
    .count = ARRAY_SIZE(args),
    };
    unsigned long long retval;
    acpi_status status;
    if (acpi_tad_rt_is_invalid(rt))
    return -EINVAL;
    rt.valid = 0;
    rt.msec = 0;
    memset(rt.padding, 0, 3);
    args[0].buffer.pointer = (u8 *)rt;
    args[0].buffer.length = sizeof(*rt);
    PM_RUNTIME_ACQUIRE(dev, pm);
    if (PM_RUNTIME_ACQUIRE_ERR(&pm))
    return -ENXIO;
    guard(mutex)(&acpi_tad_aml_lock);
    status = acpi_evaluate_integer(handle, "_SRT", &arg_list, &retval);
    if (ACPI_FAILURE(status) || retval)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_tad_evaluate_grt(dev: *mut device, rt: *mut acpi_tad_rt) -> c_int {
    static int acpi_tad_evaluate_grt(struct device *dev, struct acpi_tad_rt *rt)
    {
    let mut handle: acpi_handle = ACPI_HANDLE(dev);
    let mut output: acpi_buffer = { ACPI_ALLOCATE_BUFFER };
    acpi_status status;
    let mut ret: c_int = -EIO;
    guard(mutex)(&acpi_tad_aml_lock);
    status = acpi_evaluate_object(handle, "_GRT", core::ptr::null_mut(), &output);
    if (ACPI_SUCCESS(status)) {
    union acpi_object *out_obj;
    out_obj = output.pointer;
    if (out_obj.type == ACPI_TYPE_BUFFER &&
    out_obj.buffer.length == sizeof(*rt)) {
    struct acpi_tad_rt *data;
    data = (struct acpi_tad_rt *)(out_obj.buffer.pointer);
    if (data.valid) {
    memcpy(rt, data, sizeof(*rt));
    ret = 0;
    }
    }
    }
    ACPI_FREE(output.pointer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __acpi_tad_get_real_time(dev: *mut device, rt: *mut acpi_tad_rt) -> c_int {
    static int __acpi_tad_get_real_time(struct device *dev, struct acpi_tad_rt *rt)
    {
    int ret;
    ret = acpi_tad_evaluate_grt(dev, rt);
    if (ret)
    return ret;
    if (acpi_tad_rt_is_invalid(rt))
    return -ENODATA;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_tad_get_real_time(dev: *mut device, rt: *mut acpi_tad_rt) -> c_int {
    static int acpi_tad_get_real_time(struct device *dev, struct acpi_tad_rt *rt)
    {
    PM_RUNTIME_ACQUIRE(dev, pm);
    if (PM_RUNTIME_ACQUIRE_ERR(&pm))
    return -ENXIO;
    return __acpi_tad_get_real_time(dev, rt);
    }
    static int __acpi_tad_wake_set(struct device *dev, char *method, u32 timer_id,
    u32 value)
    {
    let mut handle: acpi_handle = ACPI_HANDLE(dev);
    union acpi_object args[] = {
    { .type = ACPI_TYPE_INTEGER, },
    { .type = ACPI_TYPE_INTEGER, },
    };
    struct acpi_object_list arg_list = {
    .pointer = args,
    .count = ARRAY_SIZE(args),
    };
    unsigned long long retval;
    acpi_status status;
    args[0].integer.value = timer_id;
    args[1].integer.value = value;
    guard(mutex)(&acpi_tad_aml_lock);
    status = acpi_evaluate_integer(handle, method, &arg_list, &retval);
    if (ACPI_FAILURE(status) || retval)
    return -EIO;
    return 0;
    }
    static int __acpi_tad_wake_read(struct device *dev, char *method, u32 timer_id,
    unsigned long long *retval)
    {
    let mut handle: acpi_handle = ACPI_HANDLE(dev);
    union acpi_object args[] = {
    { .type = ACPI_TYPE_INTEGER, },
    };
    struct acpi_object_list arg_list = {
    .pointer = args,
    .count = ARRAY_SIZE(args),
    };
    acpi_status status;
    args[0].integer.value = timer_id;
    guard(mutex)(&acpi_tad_aml_lock);
    status = acpi_evaluate_integer(handle, method, &arg_list, retval);
    if (ACPI_FAILURE(status))
    return -EIO;
    return 0;
    }
// sysfs interface
    static char *acpi_tad_rt_next_field(char *s, int *val)
    {
    char *p;
    p = strchr(s, ':');
    if (!p)
    return core::ptr::null_mut();
// p = '\0';
    if (kstrtoint(s, 10, val))
    return core::ptr::null_mut();
    return p + 1;
    }
    static ssize_t time_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct acpi_tad_rt rt;
    int val, ret;
    char *s;
    char *str __free(kfree) = kmemdup_nul(buf, count, GFP_KERNEL);
    if (!str)
    return -ENOMEM;
    s = acpi_tad_rt_next_field(str, &val);
    if (!s)
    return -ENODATA;
    rt.year = val;
    s = acpi_tad_rt_next_field(s, &val);
    if (!s)
    return -ENODATA;
    rt.month = val;
    s = acpi_tad_rt_next_field(s, &val);
    if (!s)
    return -ENODATA;
    rt.day = val;
    s = acpi_tad_rt_next_field(s, &val);
    if (!s)
    return -ENODATA;
    rt.hour = val;
    s = acpi_tad_rt_next_field(s, &val);
    if (!s)
    return -ENODATA;
    rt.minute = val;
    s = acpi_tad_rt_next_field(s, &val);
    if (!s)
    return -ENODATA;
    rt.second = val;
    s = acpi_tad_rt_next_field(s, &val);
    if (!s)
    return -ENODATA;
    rt.tz = val;
    if (kstrtoint(s, 10, &val))
    return -ENODATA;
    rt.daylight = val;
    ret = acpi_tad_set_real_time(dev, &rt);
    if (ret)
    return ret;
    return count;
    }
    static ssize_t time_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct acpi_tad_rt rt;
    int ret;
    ret = acpi_tad_get_real_time(dev, &rt);
    if (ret)
    return ret;
    return sysfs_emit(buf, "%u:%u:%u:%u:%u:%u:%d:%u\n",
    rt.year, rt.month, rt.day, rt.hour, rt.minute, rt.second,
    rt.tz, rt.daylight);
    }
    static DEVICE_ATTR_RW(time);
    static int acpi_tad_wake_set(struct device *dev, char *method, u32 timer_id,
    u32 value)
    {
    PM_RUNTIME_ACQUIRE(dev, pm);
    if (PM_RUNTIME_ACQUIRE_ERR(&pm))
    return -ENXIO;
    return __acpi_tad_wake_set(dev, method, timer_id, value);
    }
    static int acpi_tad_wake_write(struct device *dev, const char *buf, char *method,
    u32 timer_id, const char *specval)
    {
    u32 value;
    if (sysfs_streq(buf, specval)) {
    value = ACPI_TAD_WAKE_DISABLED;
    } else {
    let mut ret: c_int = kstrtou32(buf, 0, &value);
    if (ret)
    return ret;
    if (value == ACPI_TAD_WAKE_DISABLED)
    return -EINVAL;
    }
    return acpi_tad_wake_set(dev, method, timer_id, value);
    }
    static ssize_t acpi_tad_wake_read(struct device *dev, char *buf, char *method,
    u32 timer_id, const char *specval)
    {
    unsigned long long retval;
    int ret;
    PM_RUNTIME_ACQUIRE(dev, pm);
    if (PM_RUNTIME_ACQUIRE_ERR(&pm))
    return -ENXIO;
    ret = __acpi_tad_wake_read(dev, method, timer_id, &retval);
    if (ret)
    return ret;
    if ((u32)retval == ACPI_TAD_WAKE_DISABLED)
    return sprintf(buf, "%s\n", specval);
    return sprintf(buf, "%u\n", (u32)retval);
    }
    static const char *alarm_specval = "disabled";
    static int acpi_tad_alarm_write(struct device *dev, const char *buf,
    u32 timer_id)
    {
    return acpi_tad_wake_write(dev, buf, "_STV", timer_id, alarm_specval);
    }
#[no_mangle]
unsafe extern "C" fn acpi_tad_alarm_read(dev: *mut device, buf: *mut c_char, timer_id: u32) -> isize {
    static ssize_t acpi_tad_alarm_read(struct device *dev, char *buf, u32 timer_id)
    {
    return acpi_tad_wake_read(dev, buf, "_TIV", timer_id, alarm_specval);
    }
    static const char *policy_specval = "never";
    static int acpi_tad_policy_write(struct device *dev, const char *buf,
    u32 timer_id)
    {
    return acpi_tad_wake_write(dev, buf, "_STP", timer_id, policy_specval);
    }
#[no_mangle]
unsafe extern "C" fn acpi_tad_policy_read(dev: *mut device, buf: *mut c_char, timer_id: u32) -> isize {
    static ssize_t acpi_tad_policy_read(struct device *dev, char *buf, u32 timer_id)
    {
    return acpi_tad_wake_read(dev, buf, "_TIP", timer_id, policy_specval);
    }
#[no_mangle]
unsafe extern "C" fn acpi_tad_clear_status(dev: *mut device, timer_id: u32) -> c_int {
    static int acpi_tad_clear_status(struct device *dev, u32 timer_id)
    {
    let mut handle: acpi_handle = ACPI_HANDLE(dev);
    union acpi_object args[] = {
    { .type = ACPI_TYPE_INTEGER, },
    };
    struct acpi_object_list arg_list = {
    .pointer = args,
    .count = ARRAY_SIZE(args),
    };
    unsigned long long retval;
    acpi_status status;
    args[0].integer.value = timer_id;
    PM_RUNTIME_ACQUIRE(dev, pm);
    if (PM_RUNTIME_ACQUIRE_ERR(&pm))
    return -ENXIO;
    guard(mutex)(&acpi_tad_aml_lock);
    status = acpi_evaluate_integer(handle, "_CWS", &arg_list, &retval);
    if (ACPI_FAILURE(status) || retval)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_tad_status_write(dev: *mut device, buf: *const c_char, timer_id: u32) -> c_int {
    static int acpi_tad_status_write(struct device *dev, const char *buf, u32 timer_id)
    {
    int ret, value;
    ret = kstrtoint(buf, 0, &value);
    if (ret)
    return ret;
    if (value)
    return -EINVAL;
    return acpi_tad_clear_status(dev, timer_id);
    }
#[no_mangle]
unsafe extern "C" fn acpi_tad_status_read(dev: *mut device, buf: *mut c_char, timer_id: u32) -> isize {
    static ssize_t acpi_tad_status_read(struct device *dev, char *buf, u32 timer_id)
    {
    let mut handle: acpi_handle = ACPI_HANDLE(dev);
    union acpi_object args[] = {
    { .type = ACPI_TYPE_INTEGER, },
    };
    struct acpi_object_list arg_list = {
    .pointer = args,
    .count = ARRAY_SIZE(args),
    };
    unsigned long long retval;
    acpi_status status;
    args[0].integer.value = timer_id;
    PM_RUNTIME_ACQUIRE(dev, pm);
    if (PM_RUNTIME_ACQUIRE_ERR(&pm))
    return -ENXIO;
    guard(mutex)(&acpi_tad_aml_lock);
    status = acpi_evaluate_integer(handle, "_GWS", &arg_list, &retval);
    if (ACPI_FAILURE(status))
    return -EIO;
    return sprintf(buf, "0x%02X\n", (u32)retval);
    }
    static ssize_t caps_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct acpi_tad_driver_data *dd = dev_get_drvdata(dev);
    return sysfs_emit(buf, "0x%02X\n", dd.capabilities);
    }
    static DEVICE_ATTR_RO(caps);
    static ssize_t ac_alarm_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    let mut ret: c_int = acpi_tad_alarm_write(dev, buf, ACPI_TAD_AC_TIMER);
    return ret ? ret : count;
    }
    static ssize_t ac_alarm_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return acpi_tad_alarm_read(dev, buf, ACPI_TAD_AC_TIMER);
    }
    static DEVICE_ATTR_RW(ac_alarm);
    static ssize_t ac_policy_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    let mut ret: c_int = acpi_tad_policy_write(dev, buf, ACPI_TAD_AC_TIMER);
    return ret ? ret : count;
    }
    static ssize_t ac_policy_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return acpi_tad_policy_read(dev, buf, ACPI_TAD_AC_TIMER);
    }
    static DEVICE_ATTR_RW(ac_policy);
    static ssize_t ac_status_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    let mut ret: c_int = acpi_tad_status_write(dev, buf, ACPI_TAD_AC_TIMER);
    return ret ? ret : count;
    }
    static ssize_t ac_status_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return acpi_tad_status_read(dev, buf, ACPI_TAD_AC_TIMER);
    }
    static DEVICE_ATTR_RW(ac_status);
    static ssize_t dc_alarm_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    let mut ret: c_int = acpi_tad_alarm_write(dev, buf, ACPI_TAD_DC_TIMER);
    return ret ? ret : count;
    }
    static ssize_t dc_alarm_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return acpi_tad_alarm_read(dev, buf, ACPI_TAD_DC_TIMER);
    }
    static DEVICE_ATTR_RW(dc_alarm);
    static ssize_t dc_policy_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    let mut ret: c_int = acpi_tad_policy_write(dev, buf, ACPI_TAD_DC_TIMER);
    return ret ? ret : count;
    }
    static ssize_t dc_policy_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return acpi_tad_policy_read(dev, buf, ACPI_TAD_DC_TIMER);
    }
    static DEVICE_ATTR_RW(dc_policy);
    static ssize_t dc_status_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    let mut ret: c_int = acpi_tad_status_write(dev, buf, ACPI_TAD_DC_TIMER);
    return ret ? ret : count;
    }
    static ssize_t dc_status_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return acpi_tad_status_read(dev, buf, ACPI_TAD_DC_TIMER);
    }
    static DEVICE_ATTR_RW(dc_status);
    static struct attribute *acpi_tad_attrs[] = {
    &dev_attr_caps.attr,
    &dev_attr_ac_alarm.attr,
    &dev_attr_ac_policy.attr,
    &dev_attr_ac_status.attr,
    &dev_attr_dc_alarm.attr,
    &dev_attr_dc_policy.attr,
    &dev_attr_dc_status.attr,
    &dev_attr_time.attr,
    core::ptr::null_mut(),
    };
    static umode_t acpi_tad_attr_is_visible(struct kobject *kobj,
    struct attribute *a, int n)
    {
    struct acpi_tad_driver_data *dd = dev_get_drvdata(kobj_to_dev(kobj));
    if (a == &dev_attr_caps.attr)
    return a.mode;
    if ((dd.capabilities & ACPI_TAD_AC_WAKE) &&
    (a == &dev_attr_ac_alarm.attr || a == &dev_attr_ac_policy.attr ||
    a == &dev_attr_ac_status.attr))
    return a.mode;
    if ((dd.capabilities & ACPI_TAD_DC_WAKE) &&
    (a == &dev_attr_dc_alarm.attr || a == &dev_attr_dc_policy.attr ||
    a == &dev_attr_dc_status.attr))
    return a.mode;
    if ((dd.capabilities & ACPI_TAD_RT) && a == &dev_attr_time.attr)
    return a.mode;
    return 0;
    }
    static const struct attribute_group acpi_tad_group = {
    .attrs	= acpi_tad_attrs,
    .is_visible = acpi_tad_attr_is_visible,
    };
    __ATTRIBUTE_GROUPS(acpi_tad);

// RTC class device interface
#[no_mangle]
unsafe extern "C" fn acpi_tad_rt_to_tm(rt: *mut acpi_tad_rt, tm: *mut rtc_time) {
    static void acpi_tad_rt_to_tm(struct acpi_tad_rt *rt, struct rtc_time *tm)
    {
    tm.tm_year = rt.year - 1900;
    tm.tm_mon = rt.month - 1;
    tm.tm_mday = rt.day;
    tm.tm_hour = rt.hour;
    tm.tm_min = rt.minute;
    tm.tm_sec = rt.second;
    tm.tm_isdst = rt.daylight == ACPI_TAD_TIME_ISDST;
    }
#[no_mangle]
unsafe extern "C" fn acpi_tad_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int acpi_tad_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct acpi_tad_rt rt;
    rt.year = tm.tm_year + 1900;
    rt.month = tm.tm_mon + 1;
    rt.day = tm.tm_mday;
    rt.hour = tm.tm_hour;
    rt.minute = tm.tm_min;
    rt.second = tm.tm_sec;
    rt.tz = ACPI_TAD_TZ_UNSPEC;
    rt.daylight = ACPI_TAD_TIME_ISDST * !!tm.tm_isdst;
    return acpi_tad_set_real_time(dev, &rt);
    }
#[no_mangle]
unsafe extern "C" fn acpi_tad_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int acpi_tad_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct acpi_tad_rt rt;
    int ret;
    ret = acpi_tad_get_real_time(dev, &rt);
    if (ret)
    return ret;
    acpi_tad_rt_to_tm(&rt, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_tad_rtc_set_alarm(dev: *mut device, t: *mut rtc_wkalrm) -> c_int {
    static int acpi_tad_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *t)
    {
    struct acpi_tad_driver_data *dd = dev_get_drvdata(dev);
    let mut value: i64 = ACPI_TAD_WAKE_DISABLED;
    struct rtc_time tm_now;
    struct acpi_tad_rt rt;
    int ret;
    PM_RUNTIME_ACQUIRE(dev, pm);
    if (PM_RUNTIME_ACQUIRE_ERR(&pm))
    return -ENXIO;
    if (t.enabled) {
//
// The value to pass to _STV is expected to be the number of
// seconds between the time when the timer is programmed and the
// time when it expires represented as a 32-bit integer.
//
    ret = __acpi_tad_get_real_time(dev, &rt);
    if (ret)
    return ret;
    acpi_tad_rt_to_tm(&rt, &tm_now);
    value = rtc_tm_to_time64(&t.time) - rtc_tm_to_time64(&tm_now);
    if (value <= 0 || value >= U32_MAX)
    return -EINVAL;
    }
    ret = __acpi_tad_wake_set(dev, "_STV", ACPI_TAD_AC_TIMER, value);
    if (ret && t.enabled)
    return ret;
//
// If a separate DC alarm timer is supported, set it to the same value
// as the AC alarm timer.
//
    if (dd.capabilities & ACPI_TAD_DC_WAKE) {
    ret = __acpi_tad_wake_set(dev, "_STV", ACPI_TAD_DC_TIMER, value);
    if (ret && t.enabled) {
    __acpi_tad_wake_set(dev, "_STV", ACPI_TAD_AC_TIMER,
    ACPI_TAD_WAKE_DISABLED);
    return ret;
    }
    }
// Assume success if the alarm is being disabled.
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_tad_rtc_read_alarm(dev: *mut device, t: *mut rtc_wkalrm) -> c_int {
    static int acpi_tad_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *t)
    {
    unsigned long long retval;
    struct rtc_time tm_now;
    struct acpi_tad_rt rt;
    int ret;
    PM_RUNTIME_ACQUIRE(dev, pm);
    if (PM_RUNTIME_ACQUIRE_ERR(&pm))
    return -ENXIO;
    ret = __acpi_tad_get_real_time(dev, &rt);
    if (ret)
    return ret;
    acpi_tad_rt_to_tm(&rt, &tm_now);
//
// Assume that the alarm was set by acpi_tad_rtc_set_alarm(), so the AC
// and DC alarm timer settings are the same and it is sufficient to read
// the former.
//
// The value returned by _TIV should be the number of seconds till the
// expiration of the timer, represented as a 32-bit integer, or the
// special ACPI_TAD_WAKE_DISABLED value meaning that the timer has
// been disabled.
//
    ret = __acpi_tad_wake_read(dev, "_TIV", ACPI_TAD_AC_TIMER, &retval);
    if (ret)
    return ret;
    if (retval > U32_MAX)
    return -ENODATA;
    t.pending = 0;
    if (retval != ACPI_TAD_WAKE_DISABLED) {
    t.enabled = 1;
    rtc_time64_to_tm(rtc_tm_to_time64(&tm_now) + retval, &t.time);
    } else {
    t.enabled = 0;
    t.time = tm_now;
    }
    return 0;
    }
    static const struct rtc_class_ops acpi_tad_rtc_ops = {
    .read_time = acpi_tad_rtc_read_time,
    .set_time = acpi_tad_rtc_set_time,
    .set_alarm = acpi_tad_rtc_set_alarm,
    .read_alarm = acpi_tad_rtc_read_alarm,
    };
#[no_mangle]
unsafe extern "C" fn acpi_tad_register_rtc(dev: *mut device, caps: c_ulonglong) {
    static void acpi_tad_register_rtc(struct device *dev, unsigned long long caps)
    {
    struct rtc_device *rtc;
    rtc = devm_rtc_allocate_device(dev);
    if (IS_ERR(rtc))
    return;
    rtc.range_min = mktime64(1900,  1,  1,  0,  0,  0);
    rtc.range_max = mktime64(9999, 12, 31, 23, 59, 59);
    rtc.ops = &acpi_tad_rtc_ops;
    if (!(caps & ACPI_TAD_AC_WAKE))
    clear_bit(RTC_FEATURE_ALARM, rtc.features);
    devm_rtc_register_device(rtc);
    }

    static inline void acpi_tad_register_rtc(struct device *dev,
    unsigned long long caps) {}

// Platform driver interface
#[no_mangle]
unsafe extern "C" fn acpi_tad_disable_timer(dev: *mut device, timer_id: u32) -> c_int {
    static int acpi_tad_disable_timer(struct device *dev, u32 timer_id)
    {
    return acpi_tad_wake_set(dev, "_STV", timer_id, ACPI_TAD_WAKE_DISABLED);
    }
#[no_mangle]
unsafe extern "C" fn acpi_tad_remove(data: *mut c_void) {
    static void acpi_tad_remove(void *data)
    {
    struct device *dev = data;
    struct acpi_tad_driver_data *dd = dev_get_drvdata(dev);
    device_init_wakeup(dev, false);
    scoped_guard(pm_runtime_noresume, dev) {
    if (dd.capabilities & ACPI_TAD_AC_WAKE) {
    acpi_tad_disable_timer(dev, ACPI_TAD_AC_TIMER);
    acpi_tad_clear_status(dev, ACPI_TAD_AC_TIMER);
    }
    if (dd.capabilities & ACPI_TAD_DC_WAKE) {
    acpi_tad_disable_timer(dev, ACPI_TAD_DC_TIMER);
    acpi_tad_clear_status(dev, ACPI_TAD_DC_TIMER);
    }
    }
    pm_runtime_suspend(dev);
    pm_runtime_disable(dev);
    }
#[no_mangle]
unsafe extern "C" fn acpi_tad_probe(pdev: *mut platform_device) -> c_int {
    static int acpi_tad_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct acpi_tad_driver_data *dd;
    acpi_handle handle;
    acpi_status status;
    unsigned long long caps;
    int ret;
    handle = ACPI_HANDLE(dev);
    if (!handle)
    return -ENODEV;
//
// Initialization failure messages are mostly about firmware issues, so
// print them at the "info" level.
//
    status = acpi_evaluate_integer(handle, "_GCP", core::ptr::null_mut(), &caps);
    if (ACPI_FAILURE(status)) {
    dev_info(dev, "Unable to get capabilities\n");
    return -ENODEV;
    }
    if (!acpi_has_method(handle, "_PRW")) {
    dev_info(dev, "Missing _PRW\n");
    caps &= ~(ACPI_TAD_AC_WAKE | ACPI_TAD_DC_WAKE);
    }
    if (!(caps & ACPI_TAD_AC_WAKE))
    caps &= ~ACPI_TAD_DC_WAKE;
    dd = devm_kzalloc(dev, sizeof(*dd), GFP_KERNEL);
    if (!dd)
    return -ENOMEM;
    dd.capabilities = caps;
    dev_set_drvdata(dev, dd);
//
// Assume that the ACPI PM domain has been attached to the device and
// simply enable system wakeup and runtime PM and put the device into
// runtime suspend.  Everything else should be taken care of by the ACPI
// PM domain callbacks.
//
    if (caps & ACPI_TAD_AC_WAKE) {
    device_init_wakeup(dev, true);
    dev_pm_set_driver_flags(dev, DPM_FLAG_SMART_SUSPEND |
    DPM_FLAG_MAY_SKIP_RESUME);
    }
//
// The platform bus type probe callback tells the ACPI PM domain to
// power up the device, so set the runtime PM status of it to "active".
//
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    pm_runtime_suspend(dev);
//
// acpi_tad_remove() needs to run after unregistering the RTC class
// device to avoid racing with the latter's callbacks.
//
    ret = devm_add_action_or_reset(&pdev.dev, acpi_tad_remove, &pdev.dev);
    if (ret)
    return ret;
    if (caps & ACPI_TAD_RT)
    acpi_tad_register_rtc(dev, caps);
    return 0;
    }
    static const struct acpi_device_id acpi_tad_ids[] = {
    {"ACPI000E", 0},
    {}
    };
    static struct platform_driver acpi_tad_driver = {
    .driver = {
    .name = "acpi-tad",
    .acpi_match_table = acpi_tad_ids,
    .dev_groups = acpi_tad_groups,
    },
    .probe = acpi_tad_probe,
    };
    MODULE_DEVICE_TABLE(acpi, acpi_tad_ids);
    module_platform_driver(acpi_tad_driver);
