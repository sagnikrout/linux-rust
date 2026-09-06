//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-ti-32k.c
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
// timer-ti-32k.c - OMAP2 32k Timer Support
//
// Copyright (C) 2009 Nokia Corporation
//
// Update to use new clocksource/clockevent layers
// Author: Kevin Hilman, MontaVista Software, Inc. <source@mvista.com>
// Copyright (C) 2007 MontaVista Software, Inc.
//
// Original driver:
// Copyright (C) 2005 Nokia Corporation
// Author: Paul Mundt <paul.mundt@nokia.com>
// Juha Yrjölä <juha.yrjola@nokia.com>
// OMAP Dual-mode timer framework support by Timo Teras
//
// Some parts based off of TI's 24xx code:
//
// Copyright (C) 2004-2009 Texas Instruments, Inc.
//
// Roughly modelled after the OMAP1 MPU timer code.
// Added OMAP4 support - Santosh Shilimkar <santosh.shilimkar@ti.com>
//
// Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com
//

//
// 32KHz clocksource ... always available, on pretty most chips except
// OMAP 730 and 1510.  Other timers could be used as clocksources, with
// higher resolution in free-running counter modes (e.g. 12 MHz xtal),
// but systems won't necessarily want to spend resources that way.
//
pub const OMAP2_32KSYNCNT_REV_OFF: c_uint = 0x0;

pub const OMAP2_32KSYNCNT_CR_OFF_LOW: c_uint = 0x10;
pub const OMAP2_32KSYNCNT_CR_OFF_HIGH: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_32k {
    pub base: *mut void __iomem,
    pub counter: *mut void __iomem,
    pub cs: clocksource,
}

    static inline struct ti_32k *to_ti_32k(struct clocksource *cs)
    {
    return container_of(cs, struct ti_32k, cs);
    }
#[no_mangle]
unsafe extern "C" fn ti_32k_read_cycles(cs: *mut clocksource) -> u64 notrace {
    static u64 notrace ti_32k_read_cycles(struct clocksource *cs)
    {
    struct ti_32k *ti = to_ti_32k(cs);
    return (u64)readl_relaxed(ti.counter);
    }
    static struct ti_32k ti_32k_timer = {
    .cs = {
    .name		= "32k_counter",
    .rating		= 250,
    .read		= ti_32k_read_cycles,
    .mask		= CLOCKSOURCE_MASK(32),
    .flags		= CLOCK_SOURCE_IS_CONTINUOUS,
    },
    };
#[no_mangle]
unsafe extern "C" fn omap_32k_read_sched_clock() -> u64 notrace {
    static u64 notrace omap_32k_read_sched_clock(void)
    {
    return ti_32k_read_cycles(&ti_32k_timer.cs);
    }
    static void __init ti_32k_timer_enable_clock(struct device_node *np,
    const char *name)
    {
    struct clk *clock;
    int error;
    clock = of_clk_get_by_name(np.parent, name);
    if (IS_ERR(clock)) {
// Only some SoCs have a separate interface clock
    if (PTR_ERR(clock) == -EINVAL && !strncmp("ick", name, 3))
    return;
    pr_warn("%s: could not get clock %s %li\n",
    __func__, name, PTR_ERR(clock));
    return;
    }
    error = clk_prepare_enable(clock);
    if (error) {
    pr_warn("%s: could not enable %s: %i\n",
    __func__, name, error);
    return;
    }
    }
    static void __init ti_32k_timer_module_init(struct device_node *np,
    void __iomem *base)
    {
    void __iomem *sysc = base + 4;
    if (!of_device_is_compatible(np.parent, "ti,sysc"))
    return;
    ti_32k_timer_enable_clock(np, "fck");
    ti_32k_timer_enable_clock(np, "ick");
//
// Force idle module as wkup domain is active with MPU.
// No need to tag the module disabled for ti-sysc probe.
//
    writel_relaxed(0, sysc);
    }
#[no_mangle]
unsafe extern "C" fn ti_32k_timer_init(np: *mut device_node) -> int __init {
    static int __init ti_32k_timer_init(struct device_node *np)
    {
    int ret;
    ti_32k_timer.base = of_iomap(np, 0);
    if (!ti_32k_timer.base) {
    pr_err("Can't ioremap 32k timer base\n");
    return -ENXIO;
    }
    if (!of_machine_is_compatible("ti,am43"))
    ti_32k_timer.cs.flags |= CLOCK_SOURCE_SUSPEND_NONSTOP;
    ti_32k_timer.counter = ti_32k_timer.base;
    ti_32k_timer_module_init(np, ti_32k_timer.base);
//
// 32k sync Counter IP register offsets vary between the highlander
// version and the legacy ones.
//
// The 'SCHEME' bits(30-31) of the revision register is used to identify
// the version.
//
    if (readl_relaxed(ti_32k_timer.base + OMAP2_32KSYNCNT_REV_OFF) &
    OMAP2_32KSYNCNT_REV_SCHEME)
    ti_32k_timer.counter += OMAP2_32KSYNCNT_CR_OFF_HIGH;
    else
    ti_32k_timer.counter += OMAP2_32KSYNCNT_CR_OFF_LOW;
    pr_info("OMAP clocksource: 32k_counter at 32768 Hz\n");
    ret = clocksource_register_hz(&ti_32k_timer.cs, 32768);
    if (ret) {
    pr_err("32k_counter: can't register clocksource\n");
    return ret;
    }
    sched_clock_register(omap_32k_read_sched_clock, 32, 32768);
    return 0;
    }
    TIMER_OF_DECLARE(ti_32k_timer, "ti,omap-counter32k",
    ti_32k_timer_init);
