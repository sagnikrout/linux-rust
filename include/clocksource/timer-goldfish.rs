//! Automatically rewritten from C Header to Rust Module
//! Source: include/clocksource/timer-goldfish.h
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
// goldfish-timer clocksource
// Registers definition for the goldfish-timer device
//
// TIMER_TIME_LOW	 get low bits of current time and update TIMER_TIME_HIGH
// TIMER_TIME_HIGH	 get high bits of time at last TIMER_TIME_LOW read
// TIMER_ALARM_LOW	 set low bits of alarm and activate it
// TIMER_ALARM_HIGH	 set high bits of next alarm
// TIMER_IRQ_ENABLED	 enable alarm interrupt
// TIMER_CLEAR_ALARM	 disarm an existing alarm
// TIMER_ALARM_STATUS	 alarm status (running or not)
// TIMER_CLEAR_INTERRUPT clear interrupt
//
pub const TIMER_TIME_LOW: c_uint = 0x00;
pub const TIMER_TIME_HIGH: c_uint = 0x04;
pub const TIMER_ALARM_LOW: c_uint = 0x08;
pub const TIMER_ALARM_HIGH: c_uint = 0x0c;
pub const TIMER_IRQ_ENABLED: c_uint = 0x10;
pub const TIMER_CLEAR_ALARM: c_uint = 0x14;
pub const TIMER_ALARM_STATUS: c_uint = 0x18;
pub const TIMER_CLEAR_INTERRUPT: c_uint = 0x1c;
extern "C" {
    pub fn goldfish_timer_init(irq: c_int, base: *mut void __iomem) -> c_int;
}
