//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/vDSO/vdso_test_abi.c
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
// vdso_full_test.c: Sample code to test all the timers.
// Copyright (c) 2019 Arm Ltd.
//
// Compile with:
// gcc -std=gnu99 vdso_full_test.c parse_vdso.c
//

// Macro flag: #define _GNU_SOURCE

    static const char *version;
    static const char **name;
// The same as struct __kernel_timespec
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdso_timespec64 {
    pub tv_sec: u64,
    pub tv_nsec: u64,
}

    typedef long (*vdso_gettimeofday_t)(struct timeval *tv, struct timezone *tz);
    typedef long (*vdso_clock_gettime_t)(clockid_t clk_id, struct timespec *ts);
    typedef long (*vdso_clock_gettime64_t)(clockid_t clk_id, struct vdso_timespec64 *ts);
    typedef long (*vdso_clock_getres_t)(clockid_t clk_id, struct timespec *ts);
    typedef long (*vdso_clock_getres_time64_t)(clockid_t clk_id, struct vdso_timespec64 *ts);
    typedef time_t (*vdso_time_t)(time_t *t);
    static const char * const vdso_clock_name[] = {
    [CLOCK_REALTIME]		= "CLOCK_REALTIME",
    [CLOCK_MONOTONIC]		= "CLOCK_MONOTONIC",
    [CLOCK_PROCESS_CPUTIME_ID]	= "CLOCK_PROCESS_CPUTIME_ID",
    [CLOCK_THREAD_CPUTIME_ID]	= "CLOCK_THREAD_CPUTIME_ID",
    [CLOCK_MONOTONIC_RAW]		= "CLOCK_MONOTONIC_RAW",
    [CLOCK_REALTIME_COARSE]		= "CLOCK_REALTIME_COARSE",
    [CLOCK_MONOTONIC_COARSE]	= "CLOCK_MONOTONIC_COARSE",
    [CLOCK_BOOTTIME]		= "CLOCK_BOOTTIME",
    [CLOCK_REALTIME_ALARM]		= "CLOCK_REALTIME_ALARM",
    [CLOCK_BOOTTIME_ALARM]		= "CLOCK_BOOTTIME_ALARM",
    [10 /* CLOCK_SGI_CYCLE */]	= "CLOCK_SGI_CYCLE",
    [CLOCK_TAI]			= "CLOCK_TAI",
    };
#[no_mangle]
unsafe extern "C" fn vdso_test_gettimeofday() {
    static void vdso_test_gettimeofday(void)
    {
// Find gettimeofday.
    vdso_gettimeofday_t vdso_gettimeofday =
    (vdso_gettimeofday_t)vdso_sym(version, name[0]);
    if (!vdso_gettimeofday) {
    ksft_print_msg("Couldn't find %s\n", name[0]);
    ksft_test_result_skip("%s\n", name[0]);
    return;
    }
    struct timeval tv;
    let mut ret: c_long = VDSO_CALL(vdso_gettimeofday, 2, &tv, 0);
    if (ret == 0) {
    ksft_print_msg("The time is %lld.%06lld\n",
    (long long)tv.tv_sec, (long long)tv.tv_usec);
    ksft_test_result_pass("%s\n", name[0]);
    } else {
    ksft_test_result_fail("%s\n", name[0]);
    }
    }
#[no_mangle]
unsafe extern "C" fn vdso_test_clock_gettime64(clk_id: clockid_t) {
    static void vdso_test_clock_gettime64(clockid_t clk_id)
    {
// Find clock_gettime64.
    vdso_clock_gettime64_t vdso_clock_gettime64 =
    (vdso_clock_gettime64_t)vdso_sym(version, name[5]);
    if (!vdso_clock_gettime64) {
    ksft_print_msg("Couldn't find %s\n", name[5]);
    ksft_test_result_skip("%s %s\n", name[5],
    vdso_clock_name[clk_id]);
    return;
    }
    struct vdso_timespec64 ts;
    let mut ret: c_long = VDSO_CALL(vdso_clock_gettime64, 2, clk_id, &ts);
    if (ret == 0) {
    ksft_print_msg("The time is %lld.%06lld\n",
    (long long)ts.tv_sec, (long long)ts.tv_nsec);
    ksft_test_result_pass("%s %s\n", name[5],
    vdso_clock_name[clk_id]);
    } else {
    ksft_test_result_fail("%s %s\n", name[5],
    vdso_clock_name[clk_id]);
    }
    }
#[no_mangle]
unsafe extern "C" fn vdso_test_clock_gettime(clk_id: clockid_t) {
    static void vdso_test_clock_gettime(clockid_t clk_id)
    {
// Find clock_gettime.
    vdso_clock_gettime_t vdso_clock_gettime =
    (vdso_clock_gettime_t)vdso_sym(version, name[1]);
    if (!vdso_clock_gettime) {
    ksft_print_msg("Couldn't find %s\n", name[1]);
    ksft_test_result_skip("%s %s\n", name[1],
    vdso_clock_name[clk_id]);
    return;
    }
    struct timespec ts;
    let mut ret: c_long = VDSO_CALL(vdso_clock_gettime, 2, clk_id, &ts);
    if (ret == 0) {
    ksft_print_msg("The time is %lld.%06lld\n",
    (long long)ts.tv_sec, (long long)ts.tv_nsec);
    ksft_test_result_pass("%s %s\n", name[1],
    vdso_clock_name[clk_id]);
    } else {
    ksft_test_result_fail("%s %s\n", name[1],
    vdso_clock_name[clk_id]);
    }
    }
#[no_mangle]
unsafe extern "C" fn vdso_test_time() {
    static void vdso_test_time(void)
    {
// Find time.
    vdso_time_t vdso_time =
    (vdso_time_t)vdso_sym(version, name[2]);
    if (!vdso_time) {
    ksft_print_msg("Couldn't find %s\n", name[2]);
    ksft_test_result_skip("%s\n", name[2]);
    return;
    }
    let mut ret: c_long = VDSO_CALL(vdso_time, 1, core::ptr::null_mut());
    if (ret > 0) {
    ksft_print_msg("The time in hours since January 1, 1970 is %lld\n",
    (long long)(ret / 3600));
    ksft_test_result_pass("%s\n", name[2]);
    } else {
    ksft_test_result_fail("%s\n", name[2]);
    }
    }
#[no_mangle]
unsafe extern "C" fn vdso_test_clock_getres(clk_id: clockid_t) {
    static void vdso_test_clock_getres(clockid_t clk_id)
    {
    let mut clock_getres_fail: c_int = 0;
// Find clock_getres.
    vdso_clock_getres_t vdso_clock_getres =
    (vdso_clock_getres_t)vdso_sym(version, name[3]);
    if (!vdso_clock_getres) {
    ksft_print_msg("Couldn't find %s\n", name[3]);
    ksft_test_result_skip("%s %s\n", name[3],
    vdso_clock_name[clk_id]);
    return;
    }
    struct timespec ts, sys_ts;
    let mut ret: c_long = VDSO_CALL(vdso_clock_getres, 2, clk_id, &ts);
    if (ret == 0) {
    ksft_print_msg("The vdso resolution is %lld %lld\n",
    (long long)ts.tv_sec, (long long)ts.tv_nsec);
    } else {
    clock_getres_fail++;
    }
    ret = syscall(__NR_clock_getres, clk_id, &sys_ts);
    ksft_print_msg("The syscall resolution is %lld %lld\n",
    (long long)sys_ts.tv_sec, (long long)sys_ts.tv_nsec);
    if ((sys_ts.tv_sec != ts.tv_sec) || (sys_ts.tv_nsec != ts.tv_nsec))
    clock_getres_fail++;
    if (clock_getres_fail > 0) {
    ksft_test_result_fail("%s %s\n", name[3],
    vdso_clock_name[clk_id]);
    } else {
    ksft_test_result_pass("%s %s\n", name[3],
    vdso_clock_name[clk_id]);
    }
    }

#[no_mangle]
unsafe extern "C" fn vdso_test_clock_getres_time64(clk_id: clockid_t) {
    static void vdso_test_clock_getres_time64(clockid_t clk_id)
    {
    let mut clock_getres_fail: c_int = 0;
// Find clock_getres.
    vdso_clock_getres_time64_t vdso_clock_getres_time64 =
    (vdso_clock_getres_time64_t)vdso_sym(version, name[7]);
    if (!vdso_clock_getres_time64) {
    ksft_print_msg("Couldn't find %s\n", name[7]);
    ksft_test_result_skip("%s %s\n", name[7],
    vdso_clock_name[clk_id]);
    return;
    }
    struct vdso_timespec64 ts, sys_ts;
    let mut ret: c_long = VDSO_CALL(vdso_clock_getres_time64, 2, clk_id, &ts);
    if (ret == 0) {
    ksft_print_msg("The vdso resolution is %lld %lld\n",
    (long long)ts.tv_sec, (long long)ts.tv_nsec);
    } else {
    clock_getres_fail++;
    }
    ret = syscall(__NR_clock_getres_time64, clk_id, &sys_ts);
    ksft_print_msg("The syscall resolution is %lld %lld\n",
    (long long)sys_ts.tv_sec, (long long)sys_ts.tv_nsec);
    if ((sys_ts.tv_sec != ts.tv_sec) || (sys_ts.tv_nsec != ts.tv_nsec))
    clock_getres_fail++;
    if (clock_getres_fail > 0) {
    ksft_test_result_fail("%s %s\n", name[7],
    vdso_clock_name[clk_id]);
    } else {
    ksft_test_result_pass("%s %s\n", name[7],
    vdso_clock_name[clk_id]);
    }
    }

#[no_mangle]
unsafe extern "C" fn vdso_test_clock_getres_time64(clk_id: clockid_t) {
    static void vdso_test_clock_getres_time64(clockid_t clk_id)
    {
    ksft_test_result_skip("%s %s\n", name[7], vdso_clock_name[clk_id]);
    }

//
// This function calls vdso_test_clock_gettime and vdso_test_clock_getres
// with different values for clock_id.
//
#[no_mangle]
pub unsafe extern "C" fn vdso_test_clock(clock_id: clockid_t) {
    static inline void vdso_test_clock(clockid_t clock_id)
    {
    ksft_print_msg("clock_id: %s\n", vdso_clock_name[clock_id]);
    vdso_test_clock_gettime(clock_id);
    vdso_test_clock_gettime64(clock_id);
    vdso_test_clock_getres(clock_id);
    vdso_test_clock_getres_time64(clock_id);
    }
pub const VDSO_TEST_PLAN: c_int = 38;
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut sysinfo_ehdr: c_ulong = getauxval(AT_SYSINFO_EHDR);
    ksft_print_header();
    if (!sysinfo_ehdr)
    ksft_exit_skip("AT_SYSINFO_EHDR is not present!\n");
    ksft_set_plan(VDSO_TEST_PLAN);
    version = versions[VDSO_VERSION];
    name = (const char **)&names[VDSO_NAMES];
    ksft_print_msg("[vDSO kselftest] VDSO_VERSION: %s\n", version);
    vdso_init_from_sysinfo_ehdr(getauxval(AT_SYSINFO_EHDR));
    vdso_test_gettimeofday();
    vdso_test_clock(CLOCK_REALTIME);
    vdso_test_clock(CLOCK_BOOTTIME);
    vdso_test_clock(CLOCK_TAI);
    vdso_test_clock(CLOCK_REALTIME_COARSE);
    vdso_test_clock(CLOCK_MONOTONIC);
    vdso_test_clock(CLOCK_MONOTONIC_RAW);
    vdso_test_clock(CLOCK_MONOTONIC_COARSE);
    vdso_test_clock(CLOCK_PROCESS_CPUTIME_ID);
    vdso_test_clock(CLOCK_THREAD_CPUTIME_ID);
    vdso_test_time();
    ksft_finished();
    }
