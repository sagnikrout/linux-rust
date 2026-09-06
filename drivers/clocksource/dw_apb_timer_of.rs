//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/dw_apb_timer_of.c
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
// Copyright (C) 2012 Altera Corporation
// Copyright (c) 2011 Picochip Ltd., Jamie Iles
//
// Modified from mach-picoxcell/time.c
//

    static int __init timer_get_base_and_rate(struct device_node *np,
    void __iomem **base, u32 *rate)
    {
    struct clk *timer_clk;
    struct clk *pclk;
    struct reset_control *rstc;
    int ret;
// base = of_iomap(np, 0);
    if (!*base)
    panic("Unable to map regs for %pOFn", np);
//
// Reset the timer if the reset control is available, wiping
// out the state the firmware may have left it
//
    rstc = of_reset_control_get(np, core::ptr::null_mut());
    if (!IS_ERR(rstc)) {
    reset_control_assert(rstc);
    reset_control_deassert(rstc);
    }
//
// Not all implementations use a peripheral clock, so don't panic
// if it's not present
//
    pclk = of_clk_get_by_name(np, "pclk");
    if (!IS_ERR(pclk))
    if (clk_prepare_enable(pclk))
    pr_warn("pclk for %pOFn is present, but could not be activated\n",
    np);
    if (!of_property_read_u32(np, "clock-freq", rate) ||
    !of_property_read_u32(np, "clock-frequency", rate))
    return 0;
    timer_clk = of_clk_get_by_name(np, "timer");
    if (IS_ERR(timer_clk)) {
    ret = PTR_ERR(timer_clk);
    goto out_pclk_disable;
    }
    ret = clk_prepare_enable(timer_clk);
    if (ret)
    goto out_timer_clk_put;
// rate = clk_get_rate(timer_clk);
    if (!(*rate)) {
    ret = -EINVAL;
    goto out_timer_clk_disable;
    }
    return 0;
    out_timer_clk_disable:
    clk_disable_unprepare(timer_clk);
    out_timer_clk_put:
    clk_put(timer_clk);
    out_pclk_disable:
    if (!IS_ERR(pclk)) {
    clk_disable_unprepare(pclk);
    clk_put(pclk);
    }
    iounmap(*base);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn add_clockevent(event_timer: *mut device_node) -> int __init {
    static int __init add_clockevent(struct device_node *event_timer)
    {
    void __iomem *iobase;
    struct dw_apb_clock_event_device *ced;
    u32 irq, rate;
    let mut ret: c_int = 0;
    irq = irq_of_parse_and_map(event_timer, 0);
    if (irq == 0)
    panic("No IRQ for clock event timer");
    ret = timer_get_base_and_rate(event_timer, &iobase, &rate);
    if (ret)
    return ret;
    ced = dw_apb_clockevent_init(-1, event_timer.name, 300, iobase, irq,
    rate);
    if (!ced)
    return -EINVAL;
    dw_apb_clockevent_register(ced);
    return 0;
    }
    static void __iomem *sched_io_base;
    static u32 sched_rate;
#[no_mangle]
unsafe extern "C" fn add_clocksource(source_timer: *mut device_node) -> int __init {
    static int __init add_clocksource(struct device_node *source_timer)
    {
    void __iomem *iobase;
    struct dw_apb_clocksource *cs;
    u32 rate;
    int ret;
    ret = timer_get_base_and_rate(source_timer, &iobase, &rate);
    if (ret)
    return ret;
    cs = dw_apb_clocksource_init(300, source_timer.name, iobase, rate);
    if (!cs)
    return -EINVAL;
    dw_apb_clocksource_start(cs);
    dw_apb_clocksource_register(cs);
//
// Fallback to use the clocksource as sched_clock if no separate
// timer is found. sched_io_base then points to the current_value
// register of the clocksource timer.
//
    sched_io_base = iobase + 0x04;
    sched_rate = rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn read_sched_clock() -> u64 notrace {
    static u64 notrace read_sched_clock(void)
    {
    return ~readl_relaxed(sched_io_base);
    }
    static const struct of_device_id sptimer_ids[] __initconst = {
    { .compatible = "picochip,pc3x2-rtc" },
    { /* Sentinel */ },
    };
#[no_mangle]
unsafe extern "C" fn init_sched_clock() -> void __init {
    static void __init init_sched_clock(void)
    {
    struct device_node *sched_timer;
    sched_timer = of_find_matching_node(core::ptr::null_mut(), sptimer_ids);
    if (sched_timer) {
    timer_get_base_and_rate(sched_timer, &sched_io_base,
    &sched_rate);
    of_node_put(sched_timer);
    }
    sched_clock_register(read_sched_clock, 32, sched_rate);
    }

#[no_mangle]
unsafe extern "C" fn dw_apb_delay_timer_read() -> c_ulong {
    static unsigned long dw_apb_delay_timer_read(void)
    {
    return ~readl_relaxed(sched_io_base);
    }
    static struct delay_timer dw_apb_delay_timer = {
    .read_current_timer	= dw_apb_delay_timer_read,
    };

    static int num_called;
#[no_mangle]
unsafe extern "C" fn dw_apb_timer_init(timer: *mut device_node) -> int __init {
    static int __init dw_apb_timer_init(struct device_node *timer)
    {
    let mut ret: c_int = 0;
    switch (num_called) {
    case 1:
    pr_debug("%s: found clocksource timer\n", __func__);
    ret = add_clocksource(timer);
    if (ret)
    return ret;
    init_sched_clock();

    dw_apb_delay_timer.freq = sched_rate;
    register_current_timer_delay(&dw_apb_delay_timer);

    break;
    default:
    pr_debug("%s: found clockevent timer\n", __func__);
    ret = add_clockevent(timer);
    if (ret)
    return ret;
    break;
    }
    num_called++;
    return 0;
    }
    TIMER_OF_DECLARE(pc3x2_timer, "picochip,pc3x2-timer", dw_apb_timer_init);
    TIMER_OF_DECLARE(apb_timer_osc, "snps,dw-apb-timer-osc", dw_apb_timer_init);
    TIMER_OF_DECLARE(apb_timer_sp, "snps,dw-apb-timer-sp", dw_apb_timer_init);
    TIMER_OF_DECLARE(apb_timer, "snps,dw-apb-timer", dw_apb_timer_init);
