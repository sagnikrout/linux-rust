//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-tegra.c
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
// Copyright (C) 2010 Google, Inc.
//
// Author:
// Colin Cross <ccross@google.com>
//

pub const RTC_SECONDS: c_uint = 0x08;
pub const RTC_SHADOW_SECONDS: c_uint = 0x0c;
pub const RTC_MILLISECONDS: c_uint = 0x10;
pub const TIMERUS_CNTR_1US: c_uint = 0x10;
pub const TIMERUS_USEC_CFG: c_uint = 0x14;
pub const TIMERUS_CNTR_FREEZE: c_uint = 0x4c;
pub const TIMER_PTV: c_uint = 0x0;

pub const TIMER_PCR: c_uint = 0x4;

pub const TIMER1_BASE: c_uint = 0x00;
pub const TIMER2_BASE: c_uint = 0x08;
pub const TIMER3_BASE: c_uint = 0x50;
pub const TIMER4_BASE: c_uint = 0x58;
pub const TIMER10_BASE: c_uint = 0x90;
pub const TIMER1_IRQ_IDX: c_int = 0;
pub const TIMER10_IRQ_IDX: c_int = 10;
pub const TIMER_1MHz: c_int = 1000000;
    static u32 usec_config;
    static void __iomem *timer_reg_base;
    static int tegra_timer_set_next_event(unsigned long cycles,
    struct clock_event_device *evt)
    {
    void __iomem *reg_base = timer_of_base(to_timer_of(evt));
//
// Tegra's timer uses n+1 scheme for the counter, i.e. timer will
// fire after one tick if 0 is loaded.
//
// The minimum and maximum numbers of oneshot ticks are defined
// by clockevents_config_and_register(1, 0x1fffffff + 1) invocation
// below in the code. Hence the cycles (ticks) can't be outside of
// a range supportable by hardware.
//
    writel_relaxed(TIMER_PTV_EN | (cycles - 1), reg_base + TIMER_PTV);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_timer_shutdown(evt: *mut clock_event_device) -> c_int {
    static int tegra_timer_shutdown(struct clock_event_device *evt)
    {
    void __iomem *reg_base = timer_of_base(to_timer_of(evt));
    writel_relaxed(0, reg_base + TIMER_PTV);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_timer_set_periodic(evt: *mut clock_event_device) -> c_int {
    static int tegra_timer_set_periodic(struct clock_event_device *evt)
    {
    void __iomem *reg_base = timer_of_base(to_timer_of(evt));
    let mut period: c_ulong = timer_of_period(to_timer_of(evt));
    writel_relaxed(TIMER_PTV_EN | TIMER_PTV_PER | (period - 1),
    reg_base + TIMER_PTV);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_timer_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t tegra_timer_isr(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    void __iomem *reg_base = timer_of_base(to_timer_of(evt));
    writel_relaxed(TIMER_PCR_INTR_CLR, reg_base + TIMER_PCR);
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tegra_timer_suspend(evt: *mut clock_event_device) {
    static void tegra_timer_suspend(struct clock_event_device *evt)
    {
    void __iomem *reg_base = timer_of_base(to_timer_of(evt));
    writel_relaxed(TIMER_PCR_INTR_CLR, reg_base + TIMER_PCR);
    }
#[no_mangle]
unsafe extern "C" fn tegra_timer_resume(evt: *mut clock_event_device) {
    static void tegra_timer_resume(struct clock_event_device *evt)
    {
    writel_relaxed(usec_config, timer_reg_base + TIMERUS_USEC_CFG);
    }
    static DEFINE_PER_CPU(struct timer_of, tegra_to) = {
    .flags = TIMER_OF_CLOCK | TIMER_OF_BASE,
    .clkevt = {
    .name = "tegra_timer",
    .features = CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_PERIODIC,
    .set_next_event = tegra_timer_set_next_event,
    .set_state_shutdown = tegra_timer_shutdown,
    .set_state_periodic = tegra_timer_set_periodic,
    .set_state_oneshot = tegra_timer_shutdown,
    .tick_resume = tegra_timer_shutdown,
    .suspend = tegra_timer_suspend,
    .resume = tegra_timer_resume,
    },
    };
#[no_mangle]
unsafe extern "C" fn tegra_timer_setup(cpu: c_uint) -> c_int {
    static int tegra_timer_setup(unsigned int cpu)
    {
    struct timer_of *to = per_cpu_ptr(&tegra_to, cpu);
    writel_relaxed(0, timer_of_base(to) + TIMER_PTV);
    writel_relaxed(TIMER_PCR_INTR_CLR, timer_of_base(to) + TIMER_PCR);
    irq_force_affinity(to.clkevt.irq, cpumask_of(cpu));
    enable_irq(to.clkevt.irq);
//
// Tegra's timer uses n+1 scheme for the counter, i.e. timer will
// fire after one tick if 0 is loaded and thus minimum number of
// ticks is 1. In result both of the clocksource's tick limits are
// higher than a minimum and maximum that hardware register can
// take by 1, this is then taken into account by set_next_event
// callback.
//
    clockevents_config_and_register(&to.clkevt, timer_of_rate(to),
    1, /* min */
    0x1fffffff + 1); /* max 29 bits + 1 */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_timer_stop(cpu: c_uint) -> c_int {
    static int tegra_timer_stop(unsigned int cpu)
    {
    struct timer_of *to = per_cpu_ptr(&tegra_to, cpu);
    disable_irq_nosync(to.clkevt.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_read_sched_clock() -> u64 notrace {
    static u64 notrace tegra_read_sched_clock(void)
    {
    return readl_relaxed(timer_reg_base + TIMERUS_CNTR_1US);
    }

#[no_mangle]
unsafe extern "C" fn tegra_delay_timer_read_counter_long() -> c_ulong {
    static unsigned long tegra_delay_timer_read_counter_long(void)
    {
    return readl_relaxed(timer_reg_base + TIMERUS_CNTR_1US);
    }
    static struct delay_timer tegra_delay_timer = {
    .read_current_timer = tegra_delay_timer_read_counter_long,
    .freq = TIMER_1MHz,
    };

    static struct timer_of suspend_rtc_to = {
    .flags = TIMER_OF_BASE | TIMER_OF_CLOCK,
    };
//
// tegra_rtc_read - Reads the Tegra RTC registers
// Care must be taken that this function is not called while the
// tegra_rtc driver could be executing to avoid race conditions
// on the RTC shadow register
//
#[no_mangle]
unsafe extern "C" fn tegra_rtc_read_ms(cs: *mut clocksource) -> u64 {
    static u64 tegra_rtc_read_ms(struct clocksource *cs)
    {
    void __iomem *reg_base = timer_of_base(&suspend_rtc_to);
    let mut ms: u32 = readl_relaxed(reg_base + RTC_MILLISECONDS);
    let mut s: u32 = readl_relaxed(reg_base + RTC_SHADOW_SECONDS);
    return (u64)s * MSEC_PER_SEC + ms;
    }
    static struct clocksource suspend_rtc_clocksource = {
    .name	= "tegra_suspend_timer",
    .rating	= 200,
    .read	= tegra_rtc_read_ms,
    .mask	= CLOCKSOURCE_MASK(32),
    .flags	= CLOCK_SOURCE_IS_CONTINUOUS | CLOCK_SOURCE_SUSPEND_NONSTOP,
    };
#[no_mangle]
pub unsafe extern "C" fn tegra_base_for_cpu(cpu: c_int, tegra20: bool) -> c_uint {
    static inline unsigned int tegra_base_for_cpu(int cpu, bool tegra20)
    {
    if (tegra20) {
    switch (cpu) {
    case 0:
    return TIMER1_BASE;
    case 1:
    return TIMER2_BASE;
    case 2:
    return TIMER3_BASE;
    default:
    return TIMER4_BASE;
    }
    }
    return TIMER10_BASE + cpu * 8;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_irq_idx_for_cpu(cpu: c_int, tegra20: bool) -> c_uint {
    static inline unsigned int tegra_irq_idx_for_cpu(int cpu, bool tegra20)
    {
    if (tegra20)
    return TIMER1_IRQ_IDX + cpu;
    return TIMER10_IRQ_IDX + cpu;
    }
    static inline unsigned long tegra_rate_for_timer(struct timer_of *to,
    bool tegra20)
    {
//
// TIMER1-9 are fixed to 1MHz, TIMER10-13 are running off the
// parent clock.
//
    if (tegra20)
    return TIMER_1MHz;
    return timer_of_rate(to);
    }
    static int __init tegra_init_timer(struct device_node *np, bool tegra20,
    int rating)
    {
    struct timer_of *to;
    int cpu, ret;
    to = this_cpu_ptr(&tegra_to);
    ret = timer_of_init(np, to);
    if (ret)
    goto out;
    timer_reg_base = timer_of_base(to);
//
// Configure microsecond timers to have 1MHz clock
// Config register is 0xqqww, where qq is "dividend", ww is "divisor"
// Uses n+1 scheme
//
    switch (timer_of_rate(to)) {
    case 12000000:
    usec_config = 0x000b; /* (11+1)/(0+1) */
    break;
    case 12800000:
    usec_config = 0x043f; /* (63+1)/(4+1) */
    break;
    case 13000000:
    usec_config = 0x000c; /* (12+1)/(0+1) */
    break;
    case 16800000:
    usec_config = 0x0453; /* (83+1)/(4+1) */
    break;
    case 19200000:
    usec_config = 0x045f; /* (95+1)/(4+1) */
    break;
    case 26000000:
    usec_config = 0x0019; /* (25+1)/(0+1) */
    break;
    case 38400000:
    usec_config = 0x04bf; /* (191+1)/(4+1) */
    break;
    case 48000000:
    usec_config = 0x002f; /* (47+1)/(0+1) */
    break;
    default:
    ret = -EINVAL;
    goto out;
    }
    writel_relaxed(usec_config, timer_reg_base + TIMERUS_USEC_CFG);
    for_each_possible_cpu(cpu) {
    struct timer_of *cpu_to = per_cpu_ptr(&tegra_to, cpu);
    let mut flags: c_ulong = IRQF_TIMER | IRQF_NOBALANCING;
    let mut rate: c_ulong = tegra_rate_for_timer(to, tegra20);
    let mut base: c_uint = tegra_base_for_cpu(cpu, tegra20);
    let mut idx: c_uint = tegra_irq_idx_for_cpu(cpu, tegra20);
    let mut irq: c_uint = irq_of_parse_and_map(np, idx);
    if (!irq) {
    pr_err("failed to map irq for cpu%d\n", cpu);
    ret = -EINVAL;
    goto out_irq;
    }
    cpu_to.clkevt.irq = irq;
    cpu_to.clkevt.rating = rating;
    cpu_to.clkevt.cpumask = cpumask_of(cpu);
    cpu_to.of_base.base = timer_reg_base + base;
    cpu_to.of_clk.period = rate / HZ;
    cpu_to.of_clk.rate = rate;
    irq_set_status_flags(cpu_to.clkevt.irq, IRQ_NOAUTOEN);
    ret = request_irq(cpu_to.clkevt.irq, tegra_timer_isr, flags,
    cpu_to.clkevt.name, &cpu_to.clkevt);
    if (ret) {
    pr_err("failed to set up irq for cpu%d: %d\n",
    cpu, ret);
    irq_dispose_mapping(cpu_to.clkevt.irq);
    cpu_to.clkevt.irq = 0;
    goto out_irq;
    }
    }
    sched_clock_register(tegra_read_sched_clock, 32, TIMER_1MHz);
    ret = clocksource_mmio_init(timer_reg_base + TIMERUS_CNTR_1US,
    "timer_us", TIMER_1MHz, 300, 32,
    clocksource_mmio_readl_up);
    if (ret)
    pr_err("failed to register clocksource: %d\n", ret);

    register_current_timer_delay(&tegra_delay_timer);

    ret = cpuhp_setup_state(CPUHP_AP_TEGRA_TIMER_STARTING,
    "AP_TEGRA_TIMER_STARTING", tegra_timer_setup,
    tegra_timer_stop);
    if (ret)
    pr_err("failed to set up cpu hp state: %d\n", ret);
    return ret;
    out_irq:
    for_each_possible_cpu(cpu) {
    struct timer_of *cpu_to;
    cpu_to = per_cpu_ptr(&tegra_to, cpu);
    if (cpu_to.clkevt.irq) {
    free_irq(cpu_to.clkevt.irq, &cpu_to.clkevt);
    irq_dispose_mapping(cpu_to.clkevt.irq);
    }
    }
    to.of_base.base = timer_reg_base;
    out:
    timer_of_cleanup(to);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tegra210_init_timer(np: *mut device_node) -> int __init {
    static int __init tegra210_init_timer(struct device_node *np)
    {
//
// Arch-timer can't survive across power cycle of CPU core and
// after CPUPORESET signal due to a system design shortcoming,
// hence tegra-timer is more preferable on Tegra210.
//
    return tegra_init_timer(np, false, 460);
    }
    TIMER_OF_DECLARE(tegra210_timer, "nvidia,tegra210-timer", tegra210_init_timer);
#[no_mangle]
unsafe extern "C" fn tegra20_init_timer(np: *mut device_node) -> int __init {
    static int __init tegra20_init_timer(struct device_node *np)
    {
    int rating;
//
// Tegra20 and Tegra30 have Cortex A9 CPU that has a TWD timer,
// that timer runs off the CPU clock and hence is subjected to
// a jitter caused by DVFS clock rate changes. Tegra-timer is
// more preferable for older Tegra's, while later SoC generations
// have arch-timer as a main per-CPU timer and it is not affected
// by DVFS changes.
//
    if (of_machine_is_compatible("nvidia,tegra20") ||
    of_machine_is_compatible("nvidia,tegra30"))
    rating = 460;
    else
    rating = 330;
    return tegra_init_timer(np, true, rating);
    }
    TIMER_OF_DECLARE(tegra20_timer, "nvidia,tegra20-timer", tegra20_init_timer);
#[no_mangle]
unsafe extern "C" fn tegra20_init_rtc(np: *mut device_node) -> int __init {
    static int __init tegra20_init_rtc(struct device_node *np)
    {
    int ret;
    ret = timer_of_init(np, &suspend_rtc_to);
    if (ret)
    return ret;
    return clocksource_register_hz(&suspend_rtc_clocksource, 1000);
    }
    TIMER_OF_DECLARE(tegra20_rtc, "nvidia,tegra20-rtc", tegra20_init_rtc);
