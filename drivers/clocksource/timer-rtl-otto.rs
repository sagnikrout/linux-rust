//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-rtl-otto.c
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

pub const RTTM_DATA: c_uint = 0x0;
pub const RTTM_CNT: c_uint = 0x4;
pub const RTTM_CTRL: c_uint = 0x8;
pub const RTTM_INT: c_uint = 0xc;

//
// The Otto platform provides multiple 28 bit timers/counters with the following
// operating logic. If enabled the timer counts up. Per timer one can set a
// maximum counter value as an end marker. If end marker is reached the timer
// fires an interrupt. If the timer "overflows" by reaching the end marker or
// by adding 1 to 0x0fffffff the counter is reset to 0. When this happens and
// the timer is in operating mode COUNTER it stops. In mode TIMER it will
// continue to count up.
//
pub const RTTM_CTRL_COUNTER: c_int = 0;

pub const RTTM_BIT_COUNT: c_int = 28;
pub const RTTM_MIN_DELTA: c_int = 8;

//
// Timers are derived from the lexra bus (LXB) clock frequency. This is 175 MHz
// on RTL930x and 200 MHz on the other platforms. With 3.125 MHz choose a common
// divisor to have enough range and detail. This provides comparability between
// the different platforms.
//
pub const RTTM_TICKS_PER_SEC: c_int = 3125000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rttm_cs {
    pub to: timer_of,
    pub cs: clocksource,
}

// Simple internal register functions
#[no_mangle]
pub unsafe extern "C" fn rttm_get_counter(base: *mut void __iomem) -> c_uint {
    static inline unsigned int rttm_get_counter(void __iomem *base)
    {
    return __raw_readl(base + RTTM_CNT);
    }
#[no_mangle]
pub unsafe extern "C" fn rttm_set_period(base: *mut void __iomem, period: c_uint) {
    static inline void rttm_set_period(void __iomem *base, unsigned int period)
    {
    __raw_writel(period, base + RTTM_DATA);
    }
#[no_mangle]
pub unsafe extern "C" fn rttm_disable_timer(base: *mut void __iomem) {
    static inline void rttm_disable_timer(void __iomem *base)
    {
    __raw_writel(0, base + RTTM_CTRL);
    }
#[no_mangle]
pub unsafe extern "C" fn rttm_enable_timer(base: *mut void __iomem, mode: u32, divisor: u32) {
    static inline void rttm_enable_timer(void __iomem *base, u32 mode, u32 divisor)
    {
    __raw_writel(RTTM_CTRL_ENABLE | mode | divisor, base + RTTM_CTRL);
    }
#[no_mangle]
pub unsafe extern "C" fn rttm_ack_irq(base: *mut void __iomem) {
    static inline void rttm_ack_irq(void __iomem *base)
    {
    __raw_writel(__raw_readl(base + RTTM_INT) | RTTM_INT_PENDING, base + RTTM_INT);
    }
#[no_mangle]
pub unsafe extern "C" fn rttm_enable_irq(base: *mut void __iomem) {
    static inline void rttm_enable_irq(void __iomem *base)
    {
    __raw_writel(RTTM_INT_ENABLE, base + RTTM_INT);
    }
#[no_mangle]
pub unsafe extern "C" fn rttm_disable_irq(base: *mut void __iomem) {
    static inline void rttm_disable_irq(void __iomem *base)
    {
    __raw_writel(0, base + RTTM_INT);
    }
// Aggregated control functions for kernel clock framework

    pr_debug("------------- %d %p\n",	\
    smp_processor_id(), base)
#[no_mangle]
unsafe extern "C" fn rttm_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rttm_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *clkevt = dev_id;
    struct timer_of *to = to_timer_of(clkevt);
    rttm_ack_irq(to.of_base.base);
    RTTM_DEBUG(to.of_base.base);
    clkevt.event_handler(clkevt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rttm_bounce_timer(base: *mut void __iomem, mode: u32) {
    static void rttm_bounce_timer(void __iomem *base, u32 mode)
    {
//
// When a running timer has less than ~5us left, a stop/start sequence
// might fail. While the details are unknown the most evident effect is
// that the subsequent interrupt will not be fired.
//
// As a workaround issue an intermediate restart with a very slow
// frequency of ~3kHz keeping the target counter (>=8). So the follow
// up restart will always be issued outside the critical window.
//
    rttm_disable_timer(base);
    rttm_enable_timer(base, mode, RTTM_MAX_DIVISOR);
    }
#[no_mangle]
unsafe extern "C" fn rttm_stop_timer(base: *mut void __iomem) {
    static void rttm_stop_timer(void __iomem *base)
    {
    rttm_disable_timer(base);
    rttm_ack_irq(base);
    }
#[no_mangle]
unsafe extern "C" fn rttm_start_timer(to: *mut timer_of, mode: u32) {
    static void rttm_start_timer(struct timer_of *to, u32 mode)
    {
    rttm_enable_timer(to.of_base.base, mode, to.of_clk.rate / RTTM_TICKS_PER_SEC);
    }
#[no_mangle]
unsafe extern "C" fn rttm_next_event(delta: c_ulong, clkevt: *mut clock_event_device) -> c_int {
    static int rttm_next_event(unsigned long delta, struct clock_event_device *clkevt)
    {
    struct timer_of *to = to_timer_of(clkevt);
    RTTM_DEBUG(to.of_base.base);
    rttm_bounce_timer(to.of_base.base, RTTM_CTRL_COUNTER);
    rttm_disable_timer(to.of_base.base);
    rttm_set_period(to.of_base.base, delta);
    rttm_start_timer(to, RTTM_CTRL_COUNTER);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rttm_state_oneshot(clkevt: *mut clock_event_device) -> c_int {
    static int rttm_state_oneshot(struct clock_event_device *clkevt)
    {
    struct timer_of *to = to_timer_of(clkevt);
    RTTM_DEBUG(to.of_base.base);
    rttm_bounce_timer(to.of_base.base, RTTM_CTRL_COUNTER);
    rttm_disable_timer(to.of_base.base);
    rttm_set_period(to.of_base.base, RTTM_TICKS_PER_SEC / HZ);
    rttm_start_timer(to, RTTM_CTRL_COUNTER);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rttm_state_periodic(clkevt: *mut clock_event_device) -> c_int {
    static int rttm_state_periodic(struct clock_event_device *clkevt)
    {
    struct timer_of *to = to_timer_of(clkevt);
    RTTM_DEBUG(to.of_base.base);
    rttm_bounce_timer(to.of_base.base, RTTM_CTRL_TIMER);
    rttm_disable_timer(to.of_base.base);
    rttm_set_period(to.of_base.base, RTTM_TICKS_PER_SEC / HZ);
    rttm_start_timer(to, RTTM_CTRL_TIMER);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rttm_state_shutdown(clkevt: *mut clock_event_device) -> c_int {
    static int rttm_state_shutdown(struct clock_event_device *clkevt)
    {
    struct timer_of *to = to_timer_of(clkevt);
    RTTM_DEBUG(to.of_base.base);
    rttm_stop_timer(to.of_base.base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rttm_setup_timer(base: *mut void __iomem) {
    static void rttm_setup_timer(void __iomem *base)
    {
    RTTM_DEBUG(base);
    rttm_stop_timer(base);
    rttm_set_period(base, 0);
    }
#[no_mangle]
unsafe extern "C" fn rttm_read_clocksource(cs: *mut clocksource) -> u64 {
    static u64 rttm_read_clocksource(struct clocksource *cs)
    {
    struct rttm_cs *rcs = container_of(cs, struct rttm_cs, cs);
    return rttm_get_counter(rcs.to.of_base.base);
    }
// Module initialization part.
    static DEFINE_PER_CPU(struct timer_of, rttm_to) = {
    .flags				= TIMER_OF_BASE | TIMER_OF_CLOCK | TIMER_OF_IRQ,
    .of_irq = {
    .flags			= IRQF_PERCPU | IRQF_TIMER,
    .handler		= rttm_timer_interrupt,
    },
    .clkevt = {
    .rating			= 400,
    .features		= CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    .set_state_periodic	= rttm_state_periodic,
    .set_state_shutdown	= rttm_state_shutdown,
    .set_state_oneshot	= rttm_state_oneshot,
    .set_next_event		= rttm_next_event
    },
    };
#[no_mangle]
unsafe extern "C" fn rttm_enable_clocksource(cs: *mut clocksource) -> c_int {
    static int rttm_enable_clocksource(struct clocksource *cs)
    {
    struct rttm_cs *rcs = container_of(cs, struct rttm_cs, cs);
    rttm_disable_irq(rcs.to.of_base.base);
    rttm_setup_timer(rcs.to.of_base.base);
    rttm_enable_timer(rcs.to.of_base.base, RTTM_CTRL_TIMER,
    rcs.to.of_clk.rate / RTTM_TICKS_PER_SEC);
    return 0;
    }
    static struct rttm_cs rttm_cs = {
    .to = {
    .flags	= TIMER_OF_BASE | TIMER_OF_CLOCK,
    },
    .cs = {
    .name	= "realtek_otto_timer",
    .rating	= 400,
    .mask	= CLOCKSOURCE_MASK(RTTM_BIT_COUNT),
    .flags	= CLOCK_SOURCE_IS_CONTINUOUS,
    .read	= rttm_read_clocksource,
    }
    };
#[no_mangle]
unsafe extern "C" fn rttm_read_clock() -> u64 notrace {
    static u64 notrace rttm_read_clock(void)
    {
    return rttm_get_counter(rttm_cs.to.of_base.base);
    }
#[no_mangle]
unsafe extern "C" fn rttm_cpu_starting(cpu: c_uint) -> c_int {
    static int rttm_cpu_starting(unsigned int cpu)
    {
    struct timer_of *to = per_cpu_ptr(&rttm_to, cpu);
    RTTM_DEBUG(to.of_base.base);
    to.clkevt.cpumask = cpumask_of(cpu);
    irq_force_affinity(to.of_irq.irq, to.clkevt.cpumask);
    clockevents_config_and_register(&to.clkevt, RTTM_TICKS_PER_SEC,
    RTTM_MIN_DELTA, RTTM_MAX_DELTA);
    rttm_enable_irq(to.of_base.base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rttm_probe(np: *mut device_node) -> int __init {
    static int __init rttm_probe(struct device_node *np)
    {
    unsigned int cpu, cpu_rollback;
    struct timer_of *to;
    let mut clkidx: c_uint = num_possible_cpus();
// Use the first n timers as per CPU clock event generators
    for_each_possible_cpu(cpu) {
    to = per_cpu_ptr(&rttm_to, cpu);
    to.of_irq.index = to.of_base.index = cpu;
    if (timer_of_init(np, to)) {
    pr_err("setup of timer %d failed\n", cpu);
    goto rollback;
    }
    rttm_setup_timer(to.of_base.base);
    }
// Activate the n'th + 1 timer as a stable CPU clocksource.
    to = &rttm_cs.to;
    to.of_base.index = clkidx;
    timer_of_init(np, to);
    if (rttm_cs.to.of_base.base && rttm_cs.to.of_clk.rate) {
    rttm_enable_clocksource(&rttm_cs.cs);
    clocksource_register_hz(&rttm_cs.cs, RTTM_TICKS_PER_SEC);
    sched_clock_register(rttm_read_clock, RTTM_BIT_COUNT, RTTM_TICKS_PER_SEC);
    } else
    pr_err(" setup of timer %d as clocksource failed", clkidx);
    return cpuhp_setup_state(CPUHP_AP_REALTEK_TIMER_STARTING,
    "timer/realtek:online",
    rttm_cpu_starting, core::ptr::null_mut());
    rollback:
    pr_err("timer registration failed\n");
    for_each_possible_cpu(cpu_rollback) {
    if (cpu_rollback == cpu)
    break;
    to = per_cpu_ptr(&rttm_to, cpu_rollback);
    timer_of_cleanup(to);
    }
    return -EINVAL;
    }
    TIMER_OF_DECLARE(otto_timer, "realtek,otto-timer", rttm_probe);
