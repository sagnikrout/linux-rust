//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/test_rtc_lib.c
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


// SPDX-License-Identifier: LGPL-2.1+

//
// Advance a date by one day.
//
#[no_mangle]
unsafe extern "C" fn advance_date(year: *mut c_int, month: *mut c_int, mday: *mut c_int, yday: *mut c_int, wday: *mut c_int) {
    static void advance_date(int *year, int *month, int *mday, int *yday, int *wday)
    {
// wday = (*wday + 1) % 7;
    if (*mday != rtc_month_days(*month - 1, *year)) {
    ++*mday;
    ++*yday;
    return;
    }
// mday = 1;
    if (*month != 12) {
    ++*month;
    ++*yday;
    return;
    }
// month = 1;
// yday  = 1;
    ++*year;
    }
//
// Check every day in specified number of years interval starting on 1970-01-01
// against the expected result.
//
#[no_mangle]
unsafe extern "C" fn rtc_time64_to_tm_test_date_range(test: *mut kunit, years: c_int) {
    static void rtc_time64_to_tm_test_date_range(struct kunit *test, int years)
    {
//
// years	= (years / 400) * 400 years
// = (years / 400) * 146097 days
// = (years / 400) * 146097 * 86400 seconds
//
    let mut total_secs: time64_t = ((time64_t)years) / 400 * 146097 * 86400;
    let mut year: c_int = 1900;
    let mut month: c_int = 1;
    let mut mday: c_int = 1;
    let mut yday: c_int = 1;
    int wday	= 1; /* Jan 1st 1900 was a Monday */
    struct rtc_time result;
    time64_t secs;
    let mut sec_offset: time64_t = RTC_TIMESTAMP_BEGIN_1900 + ((1 * 60) + 2) * 60 + 3;
    for (secs = 0; secs <= total_secs; secs += 86400) {
    rtc_time64_to_tm(secs + sec_offset, &result);

    year, month, mday, yday, wday, secs + sec_offset
    KUNIT_ASSERT_EQ_MSG(test, year - 1900, result.tm_year, FAIL_MSG);
    KUNIT_ASSERT_EQ_MSG(test, month - 1, result.tm_mon, FAIL_MSG);
    KUNIT_ASSERT_EQ_MSG(test, mday, result.tm_mday, FAIL_MSG);
    KUNIT_ASSERT_EQ_MSG(test, yday, result.tm_yday, FAIL_MSG);
    KUNIT_ASSERT_EQ_MSG(test, 1, result.tm_hour, FAIL_MSG);
    KUNIT_ASSERT_EQ_MSG(test, 2, result.tm_min, FAIL_MSG);
    KUNIT_ASSERT_EQ_MSG(test, 3, result.tm_sec, FAIL_MSG);
    KUNIT_ASSERT_EQ_MSG(test, wday, result.tm_wday, FAIL_MSG);
    advance_date(&year, &month, &mday, &yday, &wday);
    }
    }
//
// Checks every day in a 160000 years interval starting on 1900-01-01
// against the expected result.
//
#[no_mangle]
unsafe extern "C" fn rtc_time64_to_tm_test_date_range_160000(test: *mut kunit) {
    static void rtc_time64_to_tm_test_date_range_160000(struct kunit *test)
    {
    rtc_time64_to_tm_test_date_range(test, 160000);
    }
//
// Checks every day in a 1000 years interval starting on 1900-01-01
// against the expected result.
//
#[no_mangle]
unsafe extern "C" fn rtc_time64_to_tm_test_date_range_1000(test: *mut kunit) {
    static void rtc_time64_to_tm_test_date_range_1000(struct kunit *test)
    {
    rtc_time64_to_tm_test_date_range(test, 1000);
    }
    static struct kunit_case rtc_lib_test_cases[] = {
    KUNIT_CASE(rtc_time64_to_tm_test_date_range_1000),
    KUNIT_CASE_SLOW(rtc_time64_to_tm_test_date_range_160000),
    {}
    };
    static struct kunit_suite rtc_lib_test_suite = {
    .name = "rtc_lib_test_cases",
    .test_cases = rtc_lib_test_cases,
    };
    kunit_test_suite(rtc_lib_test_suite);
    MODULE_DESCRIPTION("KUnit test for RTC lib functions");
    MODULE_LICENSE("GPL");
