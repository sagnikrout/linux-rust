//! Automatically rewritten from C to Rust
//! Source: kernel/time/time_test.c
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
// Traditional implementation of leap year evaluation, but note that long
// is a signed type and the tests do cover negative year values. So this
// can't use the is_leap_year() helper from rtc.h.
//
#[no_mangle]
unsafe extern "C" fn is_leap(year: c_long) -> bool {
    static bool is_leap(long year)
    {
    return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    }
//
// Gets the last day of a month.
//
#[no_mangle]
unsafe extern "C" fn last_day_of_month(year: c_long, month: c_int) -> c_int {
    static int last_day_of_month(long year, int month)
    {
    if (month == 2)
    return 28 + is_leap(year);
    if (month == 4 || month == 6 || month == 9 || month == 11)
    return 30;
    return 31;
    }
//
// Advances a date by one day.
//
#[no_mangle]
unsafe extern "C" fn advance_date(year: *mut c_long, month: *mut c_int, mday: *mut c_int, yday: *mut c_int) {
    static void advance_date(long *year, int *month, int *mday, int *yday)
    {
    if (*mday != last_day_of_month(*year, *month)) {
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
// yday  = 0;
    ++*year;
    }
//
// Checks every day in a 160000 years interval centered at 1970-01-01
// against the expected result.
//
#[no_mangle]
unsafe extern "C" fn time64_to_tm_test_date_range(test: *mut kunit) {
    static void time64_to_tm_test_date_range(struct kunit *test)
    {
//
// 80000 years	= (80000 / 400) * 400 years
// = (80000 / 400) * 146097 days
// = (80000 / 400) * 146097 * 86400 seconds
//
    let mut total_secs: time64_t = ((time64_t) 80000) / 400 * 146097 * 86400;
    let mut year: c_long = 1970 - 80000;
    let mut month: c_int = 1;
    let mut mdday: c_int = 1;
    let mut yday: c_int = 0;
    struct tm result;
    time64_t secs;
    s64 days;
    for (secs = -total_secs; secs <= total_secs; secs += 86400) {
    time64_to_tm(secs, 0, &result);
    days = div_s64(secs, 86400);

    year, month, mdday, yday, days
    KUNIT_ASSERT_EQ_MSG(test, year - 1900, result.tm_year, FAIL_MSG);
    KUNIT_ASSERT_EQ_MSG(test, month - 1, result.tm_mon, FAIL_MSG);
    KUNIT_ASSERT_EQ_MSG(test, mdday, result.tm_mday, FAIL_MSG);
    KUNIT_ASSERT_EQ_MSG(test, yday, result.tm_yday, FAIL_MSG);
    advance_date(&year, &month, &mdday, &yday);
    }
    }
    static struct kunit_case time_test_cases[] = {
    KUNIT_CASE_SLOW(time64_to_tm_test_date_range),
    {}
    };
    static struct kunit_suite time_test_suite = {
    .name = "time_test_cases",
    .test_cases = time_test_cases,
    };
    kunit_test_suite(time_test_suite);
    MODULE_DESCRIPTION("time unit test suite");
    MODULE_LICENSE("GPL");
