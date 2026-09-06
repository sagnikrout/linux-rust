//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/numachip.c
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
// Copyright (C) 2015 Numascale AS. All rights reserved.
//

    static DEFINE_PER_CPU(struct clock_event_device, numachip2_ced);
#[no_mangle]
unsafe extern "C" fn numachip2_timer_read(cs: *mut clocksource) -> cycles_t {
    static cycles_t numachip2_timer_read(struct clocksource *cs)
    {
    return numachip2_read64_lcsr(NUMACHIP2_TIMER_NOW);
    }
    static struct clocksource numachip2_clocksource = {
    .name            = "numachip2",
    .rating          = 295,
    .read            = numachip2_timer_read,
    .mask            = CLOCKSOURCE_MASK(64),
    .flags           = CLOCK_SOURCE_IS_CONTINUOUS,
    .mult            = 1,
    .shift           = 0,
    };
#[no_mangle]
unsafe extern "C" fn numachip2_set_next_event(delta: c_ulong, ced: *mut clock_event_device) -> c_int {
    static int numachip2_set_next_event(unsigned long delta, struct clock_event_device *ced)
    {
    numachip2_write64_lcsr(NUMACHIP2_TIMER_DEADLINE + numachip2_timer(),
    delta);
    return 0;
    }
    static const struct clock_event_device numachip2_clockevent __initconst = {
    .name            = "numachip2",
    .rating          = 400,
    .set_next_event  = numachip2_set_next_event,
    .features        = CLOCK_EVT_FEAT_ONESHOT,
    .mult            = 1,
    .shift           = 0,
    .min_delta_ns    = 1250,
    .min_delta_ticks = 1250,
    .max_delta_ns    = LONG_MAX,
    .max_delta_ticks = LONG_MAX,
    };
#[no_mangle]
unsafe extern "C" fn numachip_timer_interrupt() {
    static void numachip_timer_interrupt(void)
    {
    struct clock_event_device *ced = this_cpu_ptr(&numachip2_ced);
    ced.event_handler(ced);
    }
#[no_mangle]
unsafe extern "C" fn numachip_timer_each(work: *mut work_struct) -> __init void {
    static __init void numachip_timer_each(struct work_struct *work)
    {
    let mut local_apicid: unsigned = __this_cpu_read(x86_cpu_to_apicid) & 0xff;
    struct clock_event_device *ced = this_cpu_ptr(&numachip2_ced);
// Setup IPI vector to local core and relative timing mode
    numachip2_write64_lcsr(NUMACHIP2_TIMER_INT + numachip2_timer(),
    (3 << 22) | (X86_PLATFORM_IPI_VECTOR << 14) |
    (local_apicid << 6));
// ced = numachip2_clockevent;
    ced.cpumask = cpumask_of(smp_processor_id());
    clockevents_register_device(ced);
    }
#[no_mangle]
unsafe extern "C" fn numachip_timer_init() -> int __init {
    static int __init numachip_timer_init(void)
    {
    if (numachip_system != 2)
    return -ENODEV;
// Reset timer
    numachip2_write64_lcsr(NUMACHIP2_TIMER_RESET, 0);
    clocksource_register_hz(&numachip2_clocksource, NSEC_PER_SEC);
// Setup per-cpu clockevents
    x86_platform_ipi_callback = numachip_timer_interrupt;
    schedule_on_each_cpu(&numachip_timer_each);
    return 0;
    }
    arch_initcall(numachip_timer_init);
