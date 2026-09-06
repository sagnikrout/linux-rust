//! Automatically rewritten from C to Rust
//! Source: samples/pfsm/pfsm-wakeup.c
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
// TPS6594 PFSM userspace example
//
// Copyright (C) 2023 BayLibre Incorporated - https://www.baylibre.com
//
// This example shows how to use PFSMs from a userspace application,
// on TI j721s2 platform. The PMIC is armed to be triggered by a RTC
// alarm to execute state transition (RETENTION to ACTIVE).
//

pub const ALARM_DELTA_SEC: c_int = 30;

pub const PMIC_NB: c_int = 3;

    static const char * const dev_pfsm[] = {PMIC_A, PMIC_B, PMIC_C};
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int i, ret, fd_rtc, fd_pfsm[PMIC_NB] = { 0 };
    struct rtc_time rtc_tm;
    let mut pmic_opt: pmic_state_opt = { 0 };
    unsigned long data;
    fd_rtc = open(RTC_A, O_RDONLY);
    if (fd_rtc < 0) {
    perror("Failed to open RTC device.");
    goto out;
    }
    for (i = 0 ; i < PMIC_NB ; i++) {
    fd_pfsm[i] = open(dev_pfsm[i], O_RDWR);
    if (fd_pfsm[i] < 0) {
    perror("Failed to open PFSM device.");
    goto out;
    }
    }
// Read RTC date/time
    ret = ioctl(fd_rtc, RTC_RD_TIME, &rtc_tm);
    if (ret < 0) {
    perror("Failed to read RTC date/time.");
    goto out;
    }
    printf("Current RTC date/time is %d-%d-%d, %02d:%02d:%02d.\n",
    rtc_tm.tm_mday, rtc_tm.tm_mon + 1, rtc_tm.tm_year + 1900,
    rtc_tm.tm_hour, rtc_tm.tm_min, rtc_tm.tm_sec);
// Set RTC alarm to ALARM_DELTA_SEC sec in the future, and check for rollover
    rtc_tm.tm_sec += ALARM_DELTA_SEC;
    if (rtc_tm.tm_sec >= 60) {
    rtc_tm.tm_sec %= 60;
    rtc_tm.tm_min++;
    }
    if (rtc_tm.tm_min == 60) {
    rtc_tm.tm_min = 0;
    rtc_tm.tm_hour++;
    }
    if (rtc_tm.tm_hour == 24)
    rtc_tm.tm_hour = 0;
    ret = ioctl(fd_rtc, RTC_ALM_SET, &rtc_tm);
    if (ret < 0) {
    perror("Failed to set RTC alarm.");
    goto out;
    }
// Enable alarm interrupts
    ret = ioctl(fd_rtc, RTC_AIE_ON, 0);
    if (ret < 0) {
    perror("Failed to enable alarm interrupts.");
    goto out;
    }
    printf("Waiting %d seconds for alarm...\n", ALARM_DELTA_SEC);
//
// Set RETENTION state with options for PMIC_C/B/A respectively.
// Since PMIC_A is master, it should be the last one to be configured.
//
    pmic_opt.ddr_retention = 1;
    for (i = PMIC_NB - 1 ; i >= 0 ; i--) {
    printf("Set RETENTION state for PMIC_%d.\n", i);
    sleep(1);
    ret = ioctl(fd_pfsm[i], PMIC_SET_RETENTION_STATE, &pmic_opt);
    if (ret < 0) {
    perror("Failed to set RETENTION state.");
    goto out_reset;
    }
    }
// This blocks until the alarm ring causes an interrupt
    ret = read(fd_rtc, &data, sizeof(unsigned long));
    if (ret < 0)
    perror("Failed to get RTC alarm.");
    else
    puts("Alarm rang.\n");
    out_reset:
    ioctl(fd_rtc, RTC_AIE_OFF, 0);
// Set ACTIVE state for PMIC_A
    ioctl(fd_pfsm[0], PMIC_SET_ACTIVE_STATE, 0);
    out:
    for (i = 0 ; i < PMIC_NB ; i++)
    if (fd_pfsm[i])
    close(fd_pfsm[i]);
    if (fd_rtc)
    close(fd_rtc);
    return 0;
    }
