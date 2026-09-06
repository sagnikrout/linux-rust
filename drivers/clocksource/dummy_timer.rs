//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/dummy_timer.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/drivers/clocksource/dummy_timer.c
//
// Copyright (C) 2013 ARM Ltd.
// All Rights Reserved
//

    static DEFINE_PER_CPU(struct clock_event_device, dummy_timer_evt);
#[no_mangle]
unsafe extern "C" fn dummy_timer_starting_cpu(cpu: c_uint) -> c_int {
    static int dummy_timer_starting_cpu(unsigned int cpu)
    {
    struct clock_event_device *evt = per_cpu_ptr(&dummy_timer_evt, cpu);
    evt.name	= "dummy_timer";
    evt.features	= CLOCK_EVT_FEAT_PERIODIC |
    CLOCK_EVT_FEAT_ONESHOT |
    CLOCK_EVT_FEAT_DUMMY;
    evt.rating	= 100;
    evt.cpumask	= cpumask_of(cpu);
    clockevents_register_device(evt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dummy_timer_register() -> int __init {
    static int __init dummy_timer_register(void)
    {
    return cpuhp_setup_state(CPUHP_AP_DUMMY_TIMER_STARTING,
    "clockevents/dummy_timer:starting",
    dummy_timer_starting_cpu, core::ptr::null_mut());
    }
    early_initcall(dummy_timer_register);
