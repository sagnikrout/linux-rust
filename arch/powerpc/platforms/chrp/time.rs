//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/chrp/time.c
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
// Copyright (C) 1991, 1992, 1995  Linus Torvalds
//
// Adapted for PowerPC (PReP) by Gary Thomas
// Modified by Cort Dougan (cort@cs.nmt.edu).
// Copied and modified from arch/i386/kernel/time.c
//

pub const NVRAM_AS0: c_uint = 0x74;
pub const NVRAM_AS1: c_uint = 0x75;
pub const NVRAM_DATA: c_uint = 0x77;
    let mut nvram_as1: static int = NVRAM_AS1;
    let mut nvram_as0: static int = NVRAM_AS0;
    let mut nvram_data: static int = NVRAM_DATA;
#[no_mangle]
pub unsafe extern "C" fn chrp_time_init() -> long __init {
    long __init chrp_time_init(void)
    {
    struct device_node *rtcs;
    struct resource r;
    int base;
    rtcs = of_find_compatible_node(core::ptr::null_mut(), "rtc", "pnpPNP,b00");
    if (rtcs == core::ptr::null_mut())
    rtcs = of_find_compatible_node(core::ptr::null_mut(), "rtc", "ds1385-rtc");
    if (rtcs == core::ptr::null_mut())
    return 0;
    if (of_address_to_resource(rtcs, 0, &r)) {
    of_node_put(rtcs);
    return 0;
    }
    of_node_put(rtcs);
    base = r.start;
    nvram_as1 = 0;
    nvram_as0 = base;
    nvram_data = base + 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chrp_cmos_clock_read(addr: c_int) -> c_int {
    static int chrp_cmos_clock_read(int addr)
    {
    if (nvram_as1 != 0)
    outb(addr>>8, nvram_as1);
    outb(addr, nvram_as0);
    return (inb(nvram_data));
    }
#[no_mangle]
unsafe extern "C" fn chrp_cmos_clock_write(val: c_ulong, addr: c_int) {
    static void chrp_cmos_clock_write(unsigned long val, int addr)
    {
    if (nvram_as1 != 0)
    outb(addr>>8, nvram_as1);
    outb(addr, nvram_as0);
    outb(val, nvram_data);
    return;
    }
//
// Set the hardware clock. -- Cort
//
#[no_mangle]
pub unsafe extern "C" fn chrp_set_rtc_time(tmarg: *mut rtc_time) -> c_int {
    int chrp_set_rtc_time(struct rtc_time *tmarg)
    {
    unsigned char save_control, save_freq_select;
    let mut tm: rtc_time = *tmarg;
    spin_lock(&rtc_lock);
    save_control = chrp_cmos_clock_read(RTC_CONTROL); /* tell the clock it's being set */
    chrp_cmos_clock_write((save_control|RTC_SET), RTC_CONTROL);
    save_freq_select = chrp_cmos_clock_read(RTC_FREQ_SELECT); /* stop and reset prescaler */
    chrp_cmos_clock_write((save_freq_select|RTC_DIV_RESET2), RTC_FREQ_SELECT);
    if (!(save_control & RTC_DM_BINARY) || RTC_ALWAYS_BCD) {
    tm.tm_sec = bin2bcd(tm.tm_sec);
    tm.tm_min = bin2bcd(tm.tm_min);
    tm.tm_hour = bin2bcd(tm.tm_hour);
    tm.tm_mon = bin2bcd(tm.tm_mon);
    tm.tm_mday = bin2bcd(tm.tm_mday);
    tm.tm_year = bin2bcd(tm.tm_year);
    }
    chrp_cmos_clock_write(tm.tm_sec,RTC_SECONDS);
    chrp_cmos_clock_write(tm.tm_min,RTC_MINUTES);
    chrp_cmos_clock_write(tm.tm_hour,RTC_HOURS);
    chrp_cmos_clock_write(tm.tm_mon,RTC_MONTH);
    chrp_cmos_clock_write(tm.tm_mday,RTC_DAY_OF_MONTH);
    chrp_cmos_clock_write(tm.tm_year,RTC_YEAR);
// The following flags have to be released exactly in this order,
// otherwise the DS12887 (popular MC146818A clone with integrated
// battery and quartz) will not reset the oscillator and will not
// update precisely 500 ms later. You won't find this mentioned in
// the Dallas Semiconductor data sheets, but who believes data
// sheets anyway ...                           -- Markus Kuhn
//
    chrp_cmos_clock_write(save_control, RTC_CONTROL);
    chrp_cmos_clock_write(save_freq_select, RTC_FREQ_SELECT);
    spin_unlock(&rtc_lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn chrp_get_rtc_time(tm: *mut rtc_time) {
    void chrp_get_rtc_time(struct rtc_time *tm)
    {
    unsigned int year, mon, day, hour, min, sec;
    do {
    sec = chrp_cmos_clock_read(RTC_SECONDS);
    min = chrp_cmos_clock_read(RTC_MINUTES);
    hour = chrp_cmos_clock_read(RTC_HOURS);
    day = chrp_cmos_clock_read(RTC_DAY_OF_MONTH);
    mon = chrp_cmos_clock_read(RTC_MONTH);
    year = chrp_cmos_clock_read(RTC_YEAR);
    } while (sec != chrp_cmos_clock_read(RTC_SECONDS));
    if (!(chrp_cmos_clock_read(RTC_CONTROL) & RTC_DM_BINARY) || RTC_ALWAYS_BCD) {
    sec = bcd2bin(sec);
    min = bcd2bin(min);
    hour = bcd2bin(hour);
    day = bcd2bin(day);
    mon = bcd2bin(mon);
    year = bcd2bin(year);
    }
    if (year < 70)
    year += 100;
    tm.tm_sec = sec;
    tm.tm_min = min;
    tm.tm_hour = hour;
    tm.tm_mday = day;
    tm.tm_mon = mon;
    tm.tm_year = year;
    }
