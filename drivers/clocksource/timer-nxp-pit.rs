//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-nxp-pit.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2012-2013 Freescale Semiconductor, Inc.
// Copyright 2018,2021-2025 NXP
//

//
// Each pit takes 0x10 Bytes register space
//
pub const PIT0_OFFSET: c_uint = 0x100;

pub const PITCVAL_OFFSET: c_uint = 0x04;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pit_timer {
    pub clksrc_base: *mut void __iomem,
    pub clkevt_base: *mut void __iomem,
    pub ced: clock_event_device,
    pub cs: clocksource,
    pub rate: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pit_timer_data {
    pub max_pit_instances: c_int,
}

    static DEFINE_PER_CPU(struct pit_timer *, pit_timers);
//
// Global structure for multiple PITs initialization
//
    static int pit_instances;
    let mut max_pit_instances: static int = 1;
    static void __iomem *sched_clock_base;
    static inline struct pit_timer *ced_to_pit(struct clock_event_device *ced)
    {
    return container_of(ced, struct pit_timer, ced);
    }
    static inline struct pit_timer *cs_to_pit(struct clocksource *cs)
    {
    return container_of(cs, struct pit_timer, cs);
    }
#[no_mangle]
pub unsafe extern "C" fn pit_module_enable(base: *mut void __iomem) {
    static inline void pit_module_enable(void __iomem *base)
    {
    writel(0, PITMCR(base));
    }
#[no_mangle]
pub unsafe extern "C" fn pit_module_disable(base: *mut void __iomem) {
    static inline void pit_module_disable(void __iomem *base)
    {
    writel(PITMCR_MDIS, PITMCR(base));
    }
#[no_mangle]
pub unsafe extern "C" fn pit_timer_enable(base: *mut void __iomem, tie: bool) {
    static inline void pit_timer_enable(void __iomem *base, bool tie)
    {
    let mut val: u32 = PITTCTRL_TEN | (tie ? PITTCTRL_TIE : 0);
    writel(val, PITTCTRL(base));
    }
#[no_mangle]
pub unsafe extern "C" fn pit_timer_disable(base: *mut void __iomem) {
    static inline void pit_timer_disable(void __iomem *base)
    {
    writel(0, PITTCTRL(base));
    }
#[no_mangle]
pub unsafe extern "C" fn pit_timer_set_counter(base: *mut void __iomem, cnt: c_uint) {
    static inline void pit_timer_set_counter(void __iomem *base, unsigned int cnt)
    {
    writel(cnt, PITLDVAL(base));
    }
#[no_mangle]
pub unsafe extern "C" fn pit_timer_irqack(pit: *mut pit_timer) {
    static inline void pit_timer_irqack(struct pit_timer *pit)
    {
    writel(PITTFLG_TIF, PITTFLG(pit.clkevt_base));
    }
#[no_mangle]
unsafe extern "C" fn pit_read_sched_clock() -> u64 notrace {
    static u64 notrace pit_read_sched_clock(void)
    {
    return ~readl(sched_clock_base);
    }
#[no_mangle]
unsafe extern "C" fn pit_timer_clocksource_read(cs: *mut clocksource) -> u64 {
    static u64 pit_timer_clocksource_read(struct clocksource *cs)
    {
    struct pit_timer *pit = cs_to_pit(cs);
    return (u64)~readl(PITCVAL(pit.clksrc_base));
    }
    static int pit_clocksource_init(struct pit_timer *pit, const char *name,
    void __iomem *base, unsigned long rate)
    {
//
// The channels 0 and 1 can be chained to build a 64-bit
// timer. Let's use the channel 2 as a clocksource and leave
// the channels 0 and 1 unused for anyone else who needs them
//
    pit.clksrc_base = base + PIT_CH(2);
    pit.cs.name = name;
    pit.cs.rating = 300;
    pit.cs.read = pit_timer_clocksource_read;
    pit.cs.mask = CLOCKSOURCE_MASK(32);
    pit.cs.flags = CLOCK_SOURCE_IS_CONTINUOUS;
// set the max load value and start the clock source counter
    pit_timer_disable(pit.clksrc_base);
    pit_timer_set_counter(pit.clksrc_base, ~0);
    pit_timer_enable(pit.clksrc_base, 0);
    sched_clock_base = pit.clksrc_base + PITCVAL_OFFSET;
    sched_clock_register(pit_read_sched_clock, 32, rate);
    return clocksource_register_hz(&pit.cs, rate);
    }
#[no_mangle]
unsafe extern "C" fn pit_set_next_event(delta: c_ulong, ced: *mut clock_event_device) -> c_int {
    static int pit_set_next_event(unsigned long delta, struct clock_event_device *ced)
    {
    struct pit_timer *pit = ced_to_pit(ced);
//
// set a new value to PITLDVAL register will not restart the timer,
// to abort the current cycle and start a timer period with the new
// value, the timer must be disabled and enabled again.
// and the PITLAVAL should be set to delta minus one according to pit
// hardware requirement.
//
    pit_timer_disable(pit.clkevt_base);
    pit_timer_set_counter(pit.clkevt_base, delta - 1);
    pit_timer_enable(pit.clkevt_base, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pit_shutdown(ced: *mut clock_event_device) -> c_int {
    static int pit_shutdown(struct clock_event_device *ced)
    {
    struct pit_timer *pit = ced_to_pit(ced);
    pit_timer_disable(pit.clkevt_base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pit_set_periodic(ced: *mut clock_event_device) -> c_int {
    static int pit_set_periodic(struct clock_event_device *ced)
    {
    struct pit_timer *pit = ced_to_pit(ced);
    pit_set_next_event(pit.rate / HZ, ced);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pit_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pit_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *ced = dev_id;
    struct pit_timer *pit = ced_to_pit(ced);
    pit_timer_irqack(pit);
//
// pit hardware doesn't support oneshot, it will generate an interrupt
// and reload the counter value from PITLDVAL when PITCVAL reach zero,
// and start the counter again. So software need to disable the timer
// to stop the counter loop in ONESHOT mode.
//
    if (likely(clockevent_state_oneshot(ced)))
    pit_timer_disable(pit.clkevt_base);
    ced.event_handler(ced);
    return IRQ_HANDLED;
    }
    static int pit_clockevent_per_cpu_init(struct pit_timer *pit, const char *name,
    void __iomem *base, unsigned long rate,
    int irq, unsigned int cpu)
    {
    int ret;
//
// The channels 0 and 1 can be chained to build a 64-bit
// timer. Let's use the channel 3 as a clockevent and leave
// the channels 0 and 1 unused for anyone else who needs them
//
    pit.clkevt_base = base + PIT_CH(3);
    pit.rate = rate;
    pit_timer_disable(pit.clkevt_base);
    pit_timer_irqack(pit);
    ret = request_irq(irq, pit_timer_interrupt, IRQF_TIMER | IRQF_NOBALANCING,
    name, &pit.ced);
    if (ret)
    return ret;
    pit.ced.cpumask = cpumask_of(cpu);
    pit.ced.irq = irq;
    pit.ced.name = name;
    pit.ced.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    pit.ced.set_state_shutdown = pit_shutdown;
    pit.ced.set_state_periodic = pit_set_periodic;
    pit.ced.set_next_event	= pit_set_next_event;
    pit.ced.rating	= 300;
    per_cpu(pit_timers, cpu) = pit;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pit_clockevent_per_cpu_exit(pit: *mut pit_timer, cpu: c_uint) {
    static void pit_clockevent_per_cpu_exit(struct pit_timer *pit, unsigned int cpu)
    {
    pit_timer_disable(pit.clkevt_base);
    free_irq(pit.ced.irq, &pit.ced);
    per_cpu(pit_timers, cpu) = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pit_clockevent_starting_cpu(cpu: c_uint) -> c_int {
    static int pit_clockevent_starting_cpu(unsigned int cpu)
    {
    struct pit_timer *pit = per_cpu(pit_timers, cpu);
    int ret;
    if (!pit)
    return 0;
    ret = irq_force_affinity(pit.ced.irq, cpumask_of(cpu));
    if (ret) {
    pit_clockevent_per_cpu_exit(pit, cpu);
    return ret;
    }
//
// The value for the LDVAL register trigger is calculated as:
// LDVAL trigger = (period / clock period) - 1
// The pit is a 32-bit down count timer, when the counter value
// reaches 0, it will generate an interrupt, thus the minimal
// LDVAL trigger value is 1. And then the min_delta is
// minimal LDVAL trigger value + 1, and the max_delta is full 32-bit.
//
    clockevents_config_and_register(&pit.ced, pit.rate, 2, 0xffffffff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pit_timer_init(np: *mut device_node) -> c_int {
    static int pit_timer_init(struct device_node *np)
    {
    struct pit_timer *pit;
    struct clk *pit_clk;
    void __iomem *timer_base;
    const char *name = of_node_full_name(np);
    unsigned long clk_rate;
    int irq, ret;
    pit = kzalloc_obj(*pit);
    if (!pit)
    return -ENOMEM;
    ret = -ENXIO;
    timer_base = of_iomap(np, 0);
    if (!timer_base) {
    pr_err("Failed to iomap\n");
    goto out_kfree;
    }
    ret = -EINVAL;
    irq = irq_of_parse_and_map(np, 0);
    if (irq <= 0) {
    pr_err("Failed to irq_of_parse_and_map\n");
    goto out_iounmap;
    }
    pit_clk = of_clk_get(np, 0);
    if (IS_ERR(pit_clk)) {
    ret = PTR_ERR(pit_clk);
    goto out_irq_dispose_mapping;
    }
    ret = clk_prepare_enable(pit_clk);
    if (ret)
    goto out_clk_put;
    clk_rate = clk_get_rate(pit_clk);
    pit_module_disable(timer_base);
    ret = pit_clocksource_init(pit, name, timer_base, clk_rate);
    if (ret) {
    pr_err("Failed to initialize clocksource '%pOF'\n", np);
    goto out_pit_module_disable;
    }
    ret = pit_clockevent_per_cpu_init(pit, name, timer_base, clk_rate, irq, pit_instances);
    if (ret) {
    pr_err("Failed to initialize clockevent '%pOF'\n", np);
    goto out_pit_clocksource_unregister;
    }
// enable the pit module
    pit_module_enable(timer_base);
    pit_instances++;
    if (pit_instances == max_pit_instances) {
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "PIT timer:starting",
    pit_clockevent_starting_cpu, core::ptr::null_mut());
    if (ret < 0) {
    pit_clockevent_per_cpu_exit(pit, pit_instances);
    goto out_pit_clocksource_unregister;
    }
    }
    return 0;
    out_pit_clocksource_unregister:
    clocksource_unregister(&pit.cs);
    out_pit_module_disable:
    pit_module_disable(timer_base);
    clk_disable_unprepare(pit_clk);
    out_clk_put:
    clk_put(pit_clk);
    out_irq_dispose_mapping:
    irq_dispose_mapping(irq);
    out_iounmap:
    iounmap(timer_base);
    out_kfree:
    kfree(pit);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pit_timer_probe(pdev: *mut platform_device) -> c_int {
    static int pit_timer_probe(struct platform_device *pdev)
    {
    const struct pit_timer_data *pit_timer_data;
    pit_timer_data = of_device_get_match_data(&pdev.dev);
    if (pit_timer_data)
    max_pit_instances = pit_timer_data.max_pit_instances;
    return pit_timer_init(pdev.dev.of_node);
    }
    let mut s32g2_data: static struct pit_timer_data = { .max_pit_instances = 2 };
    static const struct of_device_id pit_timer_of_match[] = {
    { .compatible = "nxp,s32g2-pit", .data = &s32g2_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, pit_timer_of_match);
    static struct platform_driver nxp_pit_driver = {
    .driver = {
    .name = "nxp-pit",
    .of_match_table = pit_timer_of_match,
    .suppress_bind_attrs = true,
    },
    .probe = pit_timer_probe,
    };
    builtin_platform_driver(nxp_pit_driver);
    TIMER_OF_DECLARE(vf610, "fsl,vf610-pit", pit_timer_init);
