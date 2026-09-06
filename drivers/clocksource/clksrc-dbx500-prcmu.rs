//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/clksrc-dbx500-prcmu.c
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
// Copyright (C) ST-Ericsson SA 2011
//
// Author: Mattias Wallin <mattias.wallin@stericsson.com> for ST-Ericsson
// Author: Sundar Iyer for ST-Ericsson
// sched_clock implementation is based on:
// plat-nomadik/timer.c Linus Walleij <linus.walleij@stericsson.com>
//
// DBx500-PRCMU Timer
// The PRCMU has 5 timers which are available in a always-on
// power domain.  We use the Timer 4 for our always-on clock
// source on DB8500.
//

pub const RATE_32K: c_int = 32768;
pub const TIMER_MODE_CONTINUOUS: c_uint = 0x1;
pub const TIMER_DOWNCOUNT_VAL: c_uint = 0xffffffff;
pub const PRCMU_TIMER_REF: c_int = 0;
pub const PRCMU_TIMER_DOWNCOUNT: c_uint = 0x4;
pub const PRCMU_TIMER_MODE: c_uint = 0x8;
    static void __iomem *clksrc_dbx500_timer_base;
#[no_mangle]
unsafe extern "C" fn clksrc_dbx500_prcmu_read(cs: *mut clocksource) -> u64 notrace {
    static u64 notrace clksrc_dbx500_prcmu_read(struct clocksource *cs)
    {
    void __iomem *base = clksrc_dbx500_timer_base;
    u32 count, count2;
    do {
    count = readl_relaxed(base + PRCMU_TIMER_DOWNCOUNT);
    count2 = readl_relaxed(base + PRCMU_TIMER_DOWNCOUNT);
    } while (count2 != count);
// Negate because the timer is a decrementing counter
    return ~count;
    }
    static struct clocksource clocksource_dbx500_prcmu = {
    .name		= "dbx500-prcmu-timer",
    .rating		= 100,
    .read		= clksrc_dbx500_prcmu_read,
    .mask		= CLOCKSOURCE_MASK(32),
    .flags		= CLOCK_SOURCE_IS_CONTINUOUS | CLOCK_SOURCE_SUSPEND_NONSTOP,
    };
#[no_mangle]
unsafe extern "C" fn clksrc_dbx500_prcmu_init(node: *mut device_node) -> int __init {
    static int __init clksrc_dbx500_prcmu_init(struct device_node *node)
    {
    clksrc_dbx500_timer_base = of_iomap(node, 0);
//
// The A9 sub system expects the timer to be configured as
// a continuous looping timer.
// The PRCMU should configure it but if it for some reason
// don't we do it here.
//
    if (readl(clksrc_dbx500_timer_base + PRCMU_TIMER_MODE) !=
    TIMER_MODE_CONTINUOUS) {
    writel(TIMER_MODE_CONTINUOUS,
    clksrc_dbx500_timer_base + PRCMU_TIMER_MODE);
    writel(TIMER_DOWNCOUNT_VAL,
    clksrc_dbx500_timer_base + PRCMU_TIMER_REF);
    }
    return clocksource_register_hz(&clocksource_dbx500_prcmu, RATE_32K);
    }
    TIMER_OF_DECLARE(dbx500_prcmu, "stericsson,db8500-prcmu-timer-4",
    clksrc_dbx500_prcmu_init);
