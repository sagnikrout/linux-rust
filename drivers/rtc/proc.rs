//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/proc.c
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
// RTC subsystem, proc interface
//
// Copyright (C) 2005-06 Tower Technologies
// Author: Alessandro Zummo <a.zummo@towertech.it>
//
// based on arch/arm/common/rtctime.c
//

pub const NAME_SIZE: c_int = 10;

#[no_mangle]
unsafe extern "C" fn is_rtc_hctosys(rtc: *mut rtc_device) -> bool {
    static bool is_rtc_hctosys(struct rtc_device *rtc)
    {
    int size;
    char name[NAME_SIZE];
    size = snprintf(name, NAME_SIZE, "rtc%d", rtc.id);
    if (size >= NAME_SIZE)
    return false;
    return !strncmp(name, CONFIG_RTC_HCTOSYS_DEVICE, NAME_SIZE);
    }

#[no_mangle]
unsafe extern "C" fn is_rtc_hctosys(rtc: *mut rtc_device) -> bool {
    static bool is_rtc_hctosys(struct rtc_device *rtc)
    {
    return (rtc.id == 0);
    }

#[no_mangle]
unsafe extern "C" fn rtc_proc_show(seq: *mut seq_file, offset: *mut c_void) -> c_int {
    static int rtc_proc_show(struct seq_file *seq, void *offset)
    {
    int err;
    struct rtc_device *rtc = seq.private;
    const struct rtc_class_ops *ops = rtc.ops;
    struct rtc_wkalrm alrm;
    struct rtc_time tm;
    err = rtc_read_time(rtc, &tm);
    if (err == 0) {
    seq_printf(seq,
    "rtc_time\t: %ptRt\n"
    "rtc_date\t: %ptRd\n",
    &tm, &tm);
    }
    err = rtc_read_alarm(rtc, &alrm);
    if (err == 0) {
    seq_printf(seq, "alrm_time\t: %ptRt\n", &alrm.time);
    seq_printf(seq, "alrm_date\t: %ptRd\n", &alrm.time);
    seq_printf(seq, "alarm_IRQ\t: %s\n",
    alrm.enabled ? "yes" : "no");
    seq_printf(seq, "alrm_pending\t: %s\n",
    alrm.pending ? "yes" : "no");
    seq_printf(seq, "update IRQ enabled\t: %s\n",
    (rtc.uie_rtctimer.enabled) ? "yes" : "no");
    seq_printf(seq, "periodic IRQ enabled\t: %s\n",
    (rtc.pie_enabled) ? "yes" : "no");
    seq_printf(seq, "periodic IRQ frequency\t: %d\n",
    rtc.irq_freq);
    seq_printf(seq, "max user IRQ frequency\t: %d\n",
    rtc.max_user_freq);
    }
    seq_printf(seq, "24hr\t\t: yes\n");
    if (ops.proc)
    ops.proc(rtc.dev.parent, seq);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rtc_proc_add_device(rtc: *mut rtc_device) {
    void rtc_proc_add_device(struct rtc_device *rtc)
    {
    if (is_rtc_hctosys(rtc))
    proc_create_single_data("driver/rtc", 0, core::ptr::null_mut(), rtc_proc_show,
    rtc);
    }
#[no_mangle]
pub unsafe extern "C" fn rtc_proc_del_device(rtc: *mut rtc_device) {
    void rtc_proc_del_device(struct rtc_device *rtc)
    {
    if (is_rtc_hctosys(rtc))
    remove_proc_entry("driver/rtc", core::ptr::null_mut());
    }
