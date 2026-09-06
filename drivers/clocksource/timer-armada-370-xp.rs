//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-armada-370-xp.c
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
// Marvell Armada 370/XP SoC timer handling.
//
// Copyright (C) 2012 Marvell
//
// Lior Amsalem <alior@marvell.com>
// Gregory CLEMENT <gregory.clement@free-electrons.com>
// Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
//
// Timer 0 is used as free-running clocksource, while timer 1 is
// used as clock_event_device.
//
// ---
// Clocksource driver for Armada 370 and Armada XP SoC.
// This driver implements one compatible string for each SoC, given
// each has its own characteristics:
//
// * Armada 370 has no 25 MHz fixed timer.
//
// * Armada XP cannot work properly without such 25 MHz fixed timer as
// doing otherwise leads to using a clocksource whose frequency varies
// when doing cpufreq frequency changes.
//
// See Documentation/devicetree/bindings/timer/marvell,armada-370-timer.yaml
//

//
// Timer block registers.
//
pub const TIMER_CTRL_OFF: c_uint = 0x0000;

pub const TIMER_EVENTS_STATUS: c_uint = 0x0004;

pub const TIMER0_RELOAD_OFF: c_uint = 0x0010;
pub const TIMER0_VAL_OFF: c_uint = 0x0014;
pub const TIMER1_RELOAD_OFF: c_uint = 0x0018;
pub const TIMER1_VAL_OFF: c_uint = 0x001c;
pub const LCL_TIMER_EVENTS_STATUS: c_uint = 0x0028;
// Global timers are connected to the coherency fabric clock, and the
    below divider reduces their incrementing frequency. */
pub const TIMER_DIVIDER_SHIFT: c_int = 5;

//
// SoC-specific data.
//
    static void __iomem *timer_base, *local_base;
    static unsigned int timer_clk;
    let mut timer25Mhz: static bool = true;
    static u32 enable_mask;
//
// Number of timer ticks per jiffy.
//
    static u32 ticks_per_jiffy;
    static struct clock_event_device __percpu *armada_370_xp_evt;
#[no_mangle]
unsafe extern "C" fn local_timer_ctrl_clrset(clr: u32, set: u32) {
    static void local_timer_ctrl_clrset(u32 clr, u32 set)
    {
    writel((readl(local_base + TIMER_CTRL_OFF) & ~clr) | set,
    local_base + TIMER_CTRL_OFF);
    }
#[no_mangle]
unsafe extern "C" fn armada_370_xp_read_sched_clock() -> u64 notrace {
    static u64 notrace armada_370_xp_read_sched_clock(void)
    {
    return ~readl(timer_base + TIMER0_VAL_OFF);
    }
//
// Clockevent handling.
//
    static int
    armada_370_xp_clkevt_next_event(unsigned long delta,
    struct clock_event_device *dev)
    {
//
// Clear clockevent timer interrupt.
//
    writel(TIMER0_CLR_MASK, local_base + LCL_TIMER_EVENTS_STATUS);
//
// Setup new clockevent timer value.
//
    writel(delta, local_base + TIMER0_VAL_OFF);
//
// Enable the timer.
//
    local_timer_ctrl_clrset(TIMER0_RELOAD_EN, enable_mask);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada_370_xp_clkevt_shutdown(evt: *mut clock_event_device) -> c_int {
    static int armada_370_xp_clkevt_shutdown(struct clock_event_device *evt)
    {
//
// Disable timer.
//
    local_timer_ctrl_clrset(TIMER0_EN, 0);
//
// ACK pending timer interrupt.
//
    writel(TIMER0_CLR_MASK, local_base + LCL_TIMER_EVENTS_STATUS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada_370_xp_clkevt_set_periodic(evt: *mut clock_event_device) -> c_int {
    static int armada_370_xp_clkevt_set_periodic(struct clock_event_device *evt)
    {
//
// Setup timer to fire at 1/HZ intervals.
//
    writel(ticks_per_jiffy - 1, local_base + TIMER0_RELOAD_OFF);
    writel(ticks_per_jiffy - 1, local_base + TIMER0_VAL_OFF);
//
// Enable timer.
//
    local_timer_ctrl_clrset(0, TIMER0_RELOAD_EN | enable_mask);
    return 0;
    }
    static int armada_370_xp_clkevt_irq;
#[no_mangle]
unsafe extern "C" fn armada_370_xp_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t armada_370_xp_timer_interrupt(int irq, void *dev_id)
    {
//
// ACK timer interrupt and call event handler.
//
    struct clock_event_device *evt = dev_id;
    writel(TIMER0_CLR_MASK, local_base + LCL_TIMER_EVENTS_STATUS);
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
//
// Setup the local clock events for a CPU.
//
#[no_mangle]
unsafe extern "C" fn armada_370_xp_timer_starting_cpu(cpu: c_uint) -> c_int {
    static int armada_370_xp_timer_starting_cpu(unsigned int cpu)
    {
    struct clock_event_device *evt = per_cpu_ptr(armada_370_xp_evt, cpu);
    let mut clr: u32 = 0, set = 0;
    if (timer25Mhz)
    set = TIMER0_25MHZ;
    else
    clr = TIMER0_25MHZ;
    local_timer_ctrl_clrset(clr, set);
    evt.name		= "armada_370_xp_per_cpu_tick";
    evt.features		= CLOCK_EVT_FEAT_ONESHOT |
    CLOCK_EVT_FEAT_PERIODIC;
    evt.shift		= 32;
    evt.rating		= 300;
    evt.set_next_event	= armada_370_xp_clkevt_next_event;
    evt.set_state_shutdown	= armada_370_xp_clkevt_shutdown;
    evt.set_state_periodic	= armada_370_xp_clkevt_set_periodic;
    evt.set_state_oneshot	= armada_370_xp_clkevt_shutdown;
    evt.tick_resume	= armada_370_xp_clkevt_shutdown;
    evt.irq		= armada_370_xp_clkevt_irq;
    evt.cpumask		= cpumask_of(cpu);
    clockevents_config_and_register(evt, timer_clk, 1, 0xfffffffe);
    enable_percpu_irq(evt.irq, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada_370_xp_timer_dying_cpu(cpu: c_uint) -> c_int {
    static int armada_370_xp_timer_dying_cpu(unsigned int cpu)
    {
    struct clock_event_device *evt = per_cpu_ptr(armada_370_xp_evt, cpu);
    disable_percpu_irq(evt.irq);
    return 0;
    }
    static u32 timer0_ctrl_reg, timer0_local_ctrl_reg;
#[no_mangle]
unsafe extern "C" fn armada_370_xp_timer_suspend(data: *mut c_void) -> c_int {
    static int armada_370_xp_timer_suspend(void *data)
    {
    timer0_ctrl_reg = readl(timer_base + TIMER_CTRL_OFF);
    timer0_local_ctrl_reg = readl(local_base + TIMER_CTRL_OFF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada_370_xp_timer_resume(data: *mut c_void) {
    static void armada_370_xp_timer_resume(void *data)
    {
    writel(0xffffffff, timer_base + TIMER0_VAL_OFF);
    writel(0xffffffff, timer_base + TIMER0_RELOAD_OFF);
    writel(timer0_ctrl_reg, timer_base + TIMER_CTRL_OFF);
    writel(timer0_local_ctrl_reg, local_base + TIMER_CTRL_OFF);
    }
    static const struct syscore_ops armada_370_xp_timer_syscore_ops = {
    .suspend	= armada_370_xp_timer_suspend,
    .resume		= armada_370_xp_timer_resume,
    };
    static struct syscore armada_370_xp_timer_syscore = {
    .ops = &armada_370_xp_timer_syscore_ops,
    };
#[no_mangle]
unsafe extern "C" fn armada_370_delay_timer_read() -> c_ulong {
    static unsigned long armada_370_delay_timer_read(void)
    {
    return ~readl(timer_base + TIMER0_VAL_OFF);
    }
    static struct delay_timer armada_370_delay_timer = {
    .read_current_timer = armada_370_delay_timer_read,
    };
#[no_mangle]
unsafe extern "C" fn armada_370_xp_timer_common_init(np: *mut device_node) -> int __init {
    static int __init armada_370_xp_timer_common_init(struct device_node *np)
    {
    let mut clr: u32 = 0, set = 0;
    int res;
    timer_base = of_iomap(np, 0);
    if (!timer_base) {
    pr_err("Failed to iomap\n");
    return -ENXIO;
    }
    local_base = of_iomap(np, 1);
    if (!local_base) {
    pr_err("Failed to iomap\n");
    return -ENXIO;
    }
    if (timer25Mhz) {
    set = TIMER0_25MHZ;
    enable_mask = TIMER0_EN;
    } else {
    clr = TIMER0_25MHZ;
    enable_mask = TIMER0_EN | TIMER0_DIV(TIMER_DIVIDER_SHIFT);
    }
    atomic_io_modify(timer_base + TIMER_CTRL_OFF, clr | set, set);
    local_timer_ctrl_clrset(clr, set);
//
// We use timer 0 as clocksource, and private(local) timer 0
// for clockevents
//
    armada_370_xp_clkevt_irq = irq_of_parse_and_map(np, 4);
    ticks_per_jiffy = (timer_clk + HZ / 2) / HZ;
//
// Setup free-running clocksource timer (interrupts
// disabled).
//
    writel(0xffffffff, timer_base + TIMER0_VAL_OFF);
    writel(0xffffffff, timer_base + TIMER0_RELOAD_OFF);
    atomic_io_modify(timer_base + TIMER_CTRL_OFF,
    TIMER0_RELOAD_EN | enable_mask,
    TIMER0_RELOAD_EN | enable_mask);
    armada_370_delay_timer.freq = timer_clk;
    register_current_timer_delay(&armada_370_delay_timer);
//
// Set scale and timer for sched_clock.
//
    sched_clock_register(armada_370_xp_read_sched_clock, 32, timer_clk);
    res = clocksource_mmio_init(timer_base + TIMER0_VAL_OFF,
    "armada_370_xp_clocksource",
    timer_clk, 300, 32, clocksource_mmio_readl_down);
    if (res) {
    pr_err("Failed to initialize clocksource mmio\n");
    return res;
    }
    armada_370_xp_evt = alloc_percpu(struct clock_event_device);
    if (!armada_370_xp_evt)
    return -ENOMEM;
//
// Setup clockevent timer (interrupt-driven).
//
    res = request_percpu_irq(armada_370_xp_clkevt_irq,
    armada_370_xp_timer_interrupt,
    "armada_370_xp_per_cpu_tick",
    armada_370_xp_evt);
// Immediately configure the timer on the boot CPU
    if (res) {
    pr_err("Failed to request percpu irq\n");
    return res;
    }
    res = cpuhp_setup_state(CPUHP_AP_ARMADA_TIMER_STARTING,
    "clockevents/armada:starting",
    armada_370_xp_timer_starting_cpu,
    armada_370_xp_timer_dying_cpu);
    if (res) {
    pr_err("Failed to setup hotplug state and timer\n");
    return res;
    }
    register_syscore(&armada_370_xp_timer_syscore);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada_xp_timer_init(np: *mut device_node) -> int __init {
    static int __init armada_xp_timer_init(struct device_node *np)
    {
    struct clk *clk = of_clk_get_by_name(np, "fixed");
    int ret;
    if (IS_ERR(clk)) {
    pr_err("Failed to get clock\n");
    return PTR_ERR(clk);
    }
    ret = clk_prepare_enable(clk);
    if (ret)
    return ret;
    timer_clk = clk_get_rate(clk);
    ret = armada_370_xp_timer_common_init(np);
    if (ret)
    clk_disable_unprepare(clk);
    return ret;
    }
    TIMER_OF_DECLARE(armada_xp, "marvell,armada-xp-timer",
    armada_xp_timer_init);
#[no_mangle]
unsafe extern "C" fn armada_375_timer_init(np: *mut device_node) -> int __init {
    static int __init armada_375_timer_init(struct device_node *np)
    {
    struct clk *clk;
    int ret;
    clk = of_clk_get_by_name(np, "fixed");
    if (!IS_ERR(clk)) {
    ret = clk_prepare_enable(clk);
    if (ret)
    return ret;
    timer_clk = clk_get_rate(clk);
    } else {
//
// This fallback is required in order to retain proper
// devicetree backwards compatibility.
//
    clk = of_clk_get(np, 0);
// Must have at least a clock
    if (IS_ERR(clk)) {
    pr_err("Failed to get clock\n");
    return PTR_ERR(clk);
    }
    ret = clk_prepare_enable(clk);
    if (ret)
    return ret;
    timer_clk = clk_get_rate(clk) / TIMER_DIVIDER;
    timer25Mhz = false;
    }
    ret = armada_370_xp_timer_common_init(np);
    if (ret)
    clk_disable_unprepare(clk);
    return ret;
    }
    TIMER_OF_DECLARE(armada_375, "marvell,armada-375-timer",
    armada_375_timer_init);
#[no_mangle]
unsafe extern "C" fn armada_370_timer_init(np: *mut device_node) -> int __init {
    static int __init armada_370_timer_init(struct device_node *np)
    {
    struct clk *clk;
    int ret;
    clk = of_clk_get(np, 0);
    if (IS_ERR(clk)) {
    pr_err("Failed to get clock\n");
    return PTR_ERR(clk);
    }
    ret = clk_prepare_enable(clk);
    if (ret)
    return ret;
    timer_clk = clk_get_rate(clk) / TIMER_DIVIDER;
    timer25Mhz = false;
    ret = armada_370_xp_timer_common_init(np);
    if (ret)
    clk_disable_unprepare(clk);
    return ret;
    }
    TIMER_OF_DECLARE(armada_370, "marvell,armada-370-timer",
    armada_370_timer_init);
