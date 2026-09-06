//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rtc.h
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
// Generic RTC interface.
// This version contains the part of the user interface to the Real Time Clock
// service. It is used with both the legacy mc146818 and also  EFI
// Struct rtc_time and first 12 ioctl by Paul Gortmaker, 1996 - separated out
// from <linux/mc146818rtc.h> to this file for 2.4 kernels.
//
// Copyright (C) 1999 Hewlett-Packard Co.
// Copyright (C) 1999 Stephane Eranian <eranian@hpl.hp.com>
//

extern "C" {
    pub fn rtc_month_days(month: c_uint, year: c_uint) -> c_int;
}
extern "C" {
    pub fn rtc_year_days(day: c_uint, month: c_uint, year: c_uint) -> c_int;
}
extern "C" {
    pub fn rtc_valid_tm(tm: *mut rtc_time) -> c_int;
}
extern "C" {
    pub fn rtc_tm_to_time64(tm: *mut rtc_time) -> time64_t;
}
extern "C" {
    pub fn rtc_time64_to_tm(time: time64_t, tm: *mut rtc_time);
}
extern "C" {
    pub fn rtc_tm_to_ktime(tm: rtc_time) -> ktime_t;
}
extern "C" {
    pub fn rtc_ktime_to_tm(kt: ktime_t) -> rtc_time;
}
//
// rtc_tm_sub - Return the difference in seconds.
//
extern "C" {
    pub fn rtc_tm_to_time64(rtc_tm_to_time64(rhs: lhs) -) -> return;
}

//
// For these RTC methods the device parameter is the physical device
// on whatever bus holds the hardware (I2C, Platform, SPI, etc), which
// was passed to rtc_device_register().  Its driver_data normally holds
// device state, including the rtc_device pointer for the RTC.
//
// Most of these methods are called with rtc_device.ops_lock held,
// through the rtc_*(struct rtc_device *, ...) calls.
//
// The (current) exceptions are mostly filesystem hooks:
// - the proc() hook for procfs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtc_class_ops {
    pub long): *mut *mut *mut int (ioctl)(struct device , unsigned int, unsigned,
    pub ): *mut *mut *mut int (read_time)(struct device , struct rtc_time,
    pub ): *mut *mut *mut int (set_time)(struct device , struct rtc_time,
    pub ): *mut *mut *mut int (read_alarm)(struct device , struct rtc_wkalrm,
    pub ): *mut *mut *mut int (set_alarm)(struct device , struct rtc_wkalrm,
    pub ): *mut *mut *mut int (proc)(struct device , struct seq_file,
    pub enabled): *mut *mut *mut int (alarm_irq_enable)(struct device , unsigned int,
    pub offset): *mut *mut *mut int (read_offset)(struct device , long,
    pub offset): *mut *mut *mut int (set_offset)(struct device , long,
    pub param): *mut *mut *mut int (param_get)(struct device , struct rtc_param,
    pub param): *mut *mut *mut int (param_set)(struct device , struct rtc_param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtc_timer {
    pub node: timerqueue_node,
    pub period: ktime_t,
    pub func: Option<unsafe extern "C" fn()>,
    pub rtc: *mut rtc_device,
    pub enabled: c_int,
}

// flags
pub const RTC_DEV_BUSY: c_int = 0;
pub const RTC_NO_CDEV: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtc_device {
    pub dev: device,
    pub owner: *mut module,
    pub id: c_int,
    pub ops: *const rtc_class_ops,
    pub ops_lock: mutex,
    pub char_dev: cdev,
    pub flags: c_ulong,
    pub irq_data: c_ulong,
    pub irq_lock: spinlock_t,
    pub irq_queue: wait_queue_head_t,
    pub async_queue: *mut fasync_struct,
    pub irq_freq: c_int,
    pub max_user_freq: c_int,
    pub timerqueue: timerqueue_head,
    pub aie_timer: rtc_timer,
    pub uie_rtctimer: rtc_timer,
    pub /: *mut *mut hrtimer pie_timer; / sub second exp, so needs hrtimer,
    pub pie_enabled: c_int,
    pub irqwork: work_struct,
//
// This offset specifies the update timing of the RTC.
//
// tsched     t1 write(t2.tv_sec - 1sec))  t2 RTC increments seconds
//
// The offset defines how tsched is computed so that the write to
// the RTC (t2.tv_sec - 1sec) is correct versus the time required
// for the transport of the write and the time which the RTC needs
// to increment seconds the first time after the write (t2).
//
// For direct accessible RTCs tsched ~= t1 because the write time
// is negligible. For RTCs behind slow busses the transport time is
// significant and has to be taken into account.
//
// The time between the write (t1) and the first increment after
// the write (t2) is RTC specific. For a MC146818 RTC it's 500ms,
// for many others it's exactly 1 second. Consult the datasheet.
//
// The value of this offset is also used to calculate the to be
// written value (t2.tv_sec - 1sec) at tsched.
//
// The default value for this is NSEC_PER_SEC + 10 msec default
// transport time. The offset can be adjusted by drivers so the
// calculation for the to be written value at tsched becomes
// correct:
//
// newval = tsched + set_offset_nsec - NSEC_PER_SEC
// and  (tsched + set_offset_nsec) % NSEC_PER_SEC == 0
//
    pub set_offset_nsec: c_ulong,
    pub features: [c_ulong; BITS_TO_LONGS(RTC_FEATURE_CNT)],
    pub range_min: time64_t,
    pub range_max: timeu64_t,
    pub alarm_offset_max: timeu64_t,
    pub start_secs: time64_t,
    pub offset_secs: time64_t,
    pub set_start_time: bool,

    pub uie_task: work_struct,
    pub uie_timer: timer_list,
// Those fields are protected by rtc->irq_lock
    pub oldsecs: c_uint,
    pub uie_irq_active:1: c_uint,
    pub stop_uie_polling:1: c_uint,
    pub uie_task_active:1: c_uint,
    pub uie_timer_active:1: c_uint,

}

// useful timestamps

extern "C" {
    pub fn __devm_rtc_register_device(owner: *mut module, rtc: *mut rtc_device) -> c_int;
}
extern "C" {
    pub fn rtc_read_time(rtc: *mut rtc_device, tm: *mut rtc_time) -> c_int;
}
extern "C" {
    pub fn rtc_set_time(rtc: *mut rtc_device, tm: *mut rtc_time) -> c_int;
}
extern "C" {
    pub fn __rtc_read_alarm(rtc: *mut rtc_device, alarm: *mut rtc_wkalrm) -> c_int;
}
extern "C" {
    pub fn rtc_class_close(rtc: *mut rtc_device);
}
extern "C" {
    pub fn rtc_irq_set_state(rtc: *mut rtc_device, enabled: c_int) -> c_int;
}
extern "C" {
    pub fn rtc_irq_set_freq(rtc: *mut rtc_device, freq: c_int) -> c_int;
}
extern "C" {
    pub fn rtc_update_irq_enable(rtc: *mut rtc_device, enabled: c_uint) -> c_int;
}
extern "C" {
    pub fn rtc_alarm_irq_enable(rtc: *mut rtc_device, enabled: c_uint) -> c_int;
}
extern "C" {
    pub fn rtc_handle_legacy_irq(rtc: *mut rtc_device, num: c_int, mode: c_int);
}
extern "C" {
    pub fn rtc_aie_update_irq(rtc: *mut rtc_device);
}
extern "C" {
    pub fn rtc_uie_update_irq(rtc: *mut rtc_device);
}
extern "C" {
    pub fn rtc_pie_update_irq(timer: *mut hrtimer) -> hrtimer_restart;
}
extern "C" {
    pub fn rtc_timer_cancel(rtc: *mut rtc_device, timer: *mut rtc_timer);
}
extern "C" {
    pub fn rtc_read_offset(rtc: *mut rtc_device, offset: *mut c_long) -> c_int;
}
extern "C" {
    pub fn rtc_set_offset(rtc: *mut rtc_device, offset: c_long) -> c_int;
}
extern "C" {
    pub fn rtc_timer_do_work(work: *mut work_struct);
}
//
// rtc_bound_alarmtime() - Return alarm time bound by rtc limit
// @rtc: Pointer to rtc device structure
// @requested: Requested alarm timeout
//
// Return: Alarm timeout bound by maximum alarm time supported by rtc.
//
extern "C" {
    pub fn ms_to_ktime(MSEC_PER_SEC: *mut *mut rtc->alarm_offset_max) -> return;
}

extern "C" {
    pub fn rtc_add_group(rtc: *mut rtc_device, grp: *const attribute_group) -> c_int;
}
extern "C" {
    pub fn rtc_add_groups(rtc: *mut rtc_device, grps: *const attribute_group) -> c_int;
}

