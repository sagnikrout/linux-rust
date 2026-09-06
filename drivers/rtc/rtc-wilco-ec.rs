//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-wilco-ec.c
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
// RTC interface for Wilco Embedded Controller with R/W abilities
//
// Copyright 2018 Google LLC
//
// The corresponding platform device is typically registered in
// drivers/platform/chrome/wilco_ec/core.c
//

pub const EC_COMMAND_CMOS: c_uint = 0x7c;
pub const EC_CMOS_TOD_WRITE: c_uint = 0x02;
pub const EC_CMOS_TOD_READ: c_uint = 0x08;
// Message sent to the EC to request the current time.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_rtc_read_request {
    pub command: u8,
    pub reserved: u8,
    pub param: u8,
    pub __packed: },
    static struct ec_rtc_read_request read_rq = {
    .command = EC_COMMAND_CMOS,
    .param = EC_CMOS_TOD_READ,
}

//
// struct ec_rtc_read_response - Format of RTC returned by EC.
// @reserved: Unused byte
// @second: Second value (0..59)
// @minute: Minute value (0..59)
// @hour: Hour value (0..23)
// @day: Day value (1..31)
// @month: Month value (1..12)
// @year: Year value (full year % 100)
// @century: Century value (full year / 100)
//
// All values are presented in binary (not BCD).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_rtc_read_response {
    pub reserved: u8,
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: u8,
    pub century: u8,
    pub __packed: },
//
// struct ec_rtc_write_request - Format of RTC sent to the EC.
// @command: Always EC_COMMAND_CMOS
// @reserved: Unused byte
// @param: Always EC_CMOS_TOD_WRITE
// @century: Century value (full year / 100)
// @year: Year value (full year % 100)
// @month: Month value (1..12)
// @day: Day value (1..31)
// @hour: Hour value (0..23)
// @minute: Minute value (0..59)
// @second: Second value (0..59)
// @weekday: Day of the week (0=Saturday)
//
// All values are presented in BCD.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_rtc_write_request {
    pub command: u8,
    pub reserved: u8,
    pub param: u8,
    pub century: u8,
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub weekday: u8,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn wilco_ec_rtc_read(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int wilco_ec_rtc_read(struct device *dev, struct rtc_time *tm)
    {
    pub dev_get_drvdata(dev->parent): *mut *mut wilco_ec_device ec =,
    pub rtc: ec_rtc_read_response,
    pub msg: wilco_ec_message,
    pub ret: c_int,
    pub sizeof(msg)): memset(&msg, 0,,
    pub WILCO_EC_MSG_LEGACY: msg.type =,
    pub &read_rq: msg.request_data =,
    pub sizeof(read_rq): msg.request_size =,
    pub &rtc: msg.response_data =,
    pub sizeof(rtc): msg.response_size =,
    pub &msg): ret = wilco_ec_mailbox(ec,,
    if (ret < 0)
    pub ret: return,
    pub rtc.second: tm->tm_sec =,
    pub rtc.minute: tm->tm_min =,
    pub rtc.hour: tm->tm_hour =,
    pub rtc.day: tm->tm_mday =,
    pub 1: tm->tm_mon = rtc.month -,
    pub 1900: *mut *mut tm->tm_year = rtc.year + (rtc.century  100) -,
// Ignore other tm fields, man rtc says userspace shouldn't use them.
    if (rtc_valid_tm(tm)) {
    pub tm): dev_err(dev, "Time from RTC is invalid: %ptRr\n",,
    pub -EIO: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn wilco_ec_rtc_write(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int wilco_ec_rtc_write(struct device *dev, struct rtc_time *tm)
    {
    pub dev_get_drvdata(dev->parent): *mut *mut wilco_ec_device ec =,
    pub rtc: ec_rtc_write_request,
    pub msg: wilco_ec_message,
    pub 1900: int year = tm->tm_year +,
//
// Convert from 0=Sunday to 0=Saturday for the EC
// We DO need to set weekday because the EC controls battery charging
// schedules that depend on the day of the week.
//
    pub 1: int wday = tm->tm_wday == 6 ? 0 : tm->tm_wday +,
    pub ret: c_int,
    pub EC_COMMAND_CMOS: rtc.command =,
    pub EC_CMOS_TOD_WRITE: rtc.param =,
    pub 100): rtc.century = bin2bcd(year /,
    pub 100): rtc.year = bin2bcd(year %,
    pub 1): rtc.month = bin2bcd(tm->tm_mon +,
    pub bin2bcd(tm->tm_mday): rtc.day =,
    pub bin2bcd(tm->tm_hour): rtc.hour =,
    pub bin2bcd(tm->tm_min): rtc.minute =,
    pub bin2bcd(tm->tm_sec): rtc.second =,
    pub bin2bcd(wday): rtc.weekday =,
    pub sizeof(msg)): memset(&msg, 0,,
    pub WILCO_EC_MSG_LEGACY: msg.type =,
    pub &rtc: msg.request_data =,
    pub sizeof(rtc): msg.request_size =,
    pub &msg): ret = wilco_ec_mailbox(ec,,
    if (ret < 0)
    pub ret: return,
    pub 0: return,
    }
    static const struct rtc_class_ops wilco_ec_rtc_ops = {
    .read_time = wilco_ec_rtc_read,
    .set_time = wilco_ec_rtc_write,
}

#[no_mangle]
unsafe extern "C" fn wilco_ec_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int wilco_ec_rtc_probe(struct platform_device *pdev)
    {
    struct rtc_device *rtc;
    rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    rtc.ops = &wilco_ec_rtc_ops;
// EC only supports this century
    rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    rtc.range_max = RTC_TIMESTAMP_END_2099;
    rtc.owner = THIS_MODULE;
    return devm_rtc_register_device(rtc);
    }
    static struct platform_driver wilco_ec_rtc_driver = {
    .driver = {
    .name = "rtc-wilco-ec",
    },
    .probe = wilco_ec_rtc_probe,
    };
    module_platform_driver(wilco_ec_rtc_driver);
    MODULE_ALIAS("platform:rtc-wilco-ec");
    MODULE_AUTHOR("Nick Crews <ncrews@chromium.org>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Wilco EC RTC driver");
