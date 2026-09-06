//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-cadence-ttc.c
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
// This file contains driver for the Cadence Triple Timer Counter Rev 06
//
// Copyright (C) 2011-2013 Xilinx
//
// based on arch/mips/kernel/time.c timer driver
//

//
// This driver configures the 2 16/32-bit count-up timers as follows:
//
// T1: Timer 1, clocksource for generic timekeeping
// T2: Timer 2, clockevent source for hrtimers
// T3: Timer 3, <unused>
//
// The input frequency to the timer module for emulation is 2.5MHz which is
// common to all the timer channels (T1, T2, and T3). With a pre-scaler of 32,
// the timers are clocked at 78.125KHz (12.8 us resolution).
// The input frequency to the timer module in silicon is configurable and
// obtained from device tree. The pre-scaler of 32 is used.
//
// Timer Register Offset Definitions of Timer 1, Increment base address by 4
// and use same offsets for Timer 2
//
pub const TTC_CLK_CNTRL_OFFSET: c_uint = 0x00 /* Clock Control Reg, RW */;
pub const TTC_CNT_CNTRL_OFFSET: c_uint = 0x0C /* Counter Control Reg, RW */;
pub const TTC_COUNT_VAL_OFFSET: c_uint = 0x18 /* Counter Value Reg, RO */;
pub const TTC_INTR_VAL_OFFSET: c_uint = 0x24 /* Interval Count Reg, RW */;
pub const TTC_ISR_OFFSET: c_uint = 0x54 /* Interrupt Status Reg, RO */;
pub const TTC_IER_OFFSET: c_uint = 0x60 /* Interrupt Enable Reg, RW */;
pub const TTC_CNT_CNTRL_DISABLE_MASK: c_uint = 0x1;

pub const TTC_CLK_CNTRL_PSV_MASK: c_uint = 0x1e;
pub const TTC_CLK_CNTRL_PSV_SHIFT: c_int = 1;
//
// Setup the timers to use pre-scaling, using a fixed value for now that will
// work across most input frequency, but it may need to be more dynamic
//

pub const CLK_CNTRL_PRESCALE_EN: c_int = 1;

pub const MAX_F_ERR: c_int = 50;
//
// struct ttc_timer - This definition defines local timer structure
//
// @base_addr:	Base address of timer
// @freq:	Timer input clock frequency
// @clk:	Associated clock source
// @clk_rate_change_nb:	Notifier block for clock rate changes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttc_timer {
    pub base_addr: *mut void __iomem,
    pub freq: c_ulong,
    pub clk: *mut clk,
    pub clk_rate_change_nb: notifier_block,
}

    container_of(x, struct ttc_timer, clk_rate_change_nb)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttc_timer_clocksource {
    pub scale_clk_ctrl_reg_old: u32,
    pub scale_clk_ctrl_reg_new: u32,
    pub ttc: ttc_timer,
    pub cs: clocksource,
}

    container_of(x, struct ttc_timer_clocksource, cs)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttc_timer_clockevent {
    pub ttc: ttc_timer,
    pub ce: clock_event_device,
}

    container_of(x, struct ttc_timer_clockevent, ce)
    static void __iomem *ttc_sched_clock_val_reg;
//
// ttc_set_interval - Set the timer interval value
//
// @timer:	Pointer to the timer instance
// @cycles:	Timer interval ticks
//
    static void ttc_set_interval(struct ttc_timer *timer,
    unsigned long cycles)
    {
    u32 ctrl_reg;
// Disable the counter, set the counter value  and re-enable counter
    ctrl_reg = readl_relaxed(timer.base_addr + TTC_CNT_CNTRL_OFFSET);
    ctrl_reg |= TTC_CNT_CNTRL_DISABLE_MASK;
    writel_relaxed(ctrl_reg, timer.base_addr + TTC_CNT_CNTRL_OFFSET);
    writel_relaxed(cycles, timer.base_addr + TTC_INTR_VAL_OFFSET);
//
// Reset the counter (0x10) so that it starts from 0, one-shot
// mode makes this needed for timing to be right.
//
    ctrl_reg |= CNT_CNTRL_RESET;
    ctrl_reg &= ~TTC_CNT_CNTRL_DISABLE_MASK;
    writel_relaxed(ctrl_reg, timer.base_addr + TTC_CNT_CNTRL_OFFSET);
    }
//
// ttc_clock_event_interrupt - Clock event timer interrupt handler
//
// @irq:	IRQ number of the Timer
// @dev_id:	void pointer to the ttc_timer instance
//
// Returns: Always IRQ_HANDLED - success
//
#[no_mangle]
unsafe extern "C" fn ttc_clock_event_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ttc_clock_event_interrupt(int irq, void *dev_id)
    {
    struct ttc_timer_clockevent *ttce = dev_id;
    struct ttc_timer *timer = &ttce.ttc;
// Acknowledge the interrupt and call event handler
    readl_relaxed(timer.base_addr + TTC_ISR_OFFSET);
    ttce.ce.event_handler(&ttce.ce);
    return IRQ_HANDLED;
    }
//
// __ttc_clocksource_read - Reads the timer counter register
// @cs: &clocksource to read from
//
// Returns: Current timer counter register value
//
#[no_mangle]
unsafe extern "C" fn __ttc_clocksource_read(cs: *mut clocksource) -> u64 {
    static u64 __ttc_clocksource_read(struct clocksource *cs)
    {
    struct ttc_timer *timer = &to_ttc_timer_clksrc(cs).ttc;
    return (u64)readl_relaxed(timer.base_addr +
    TTC_COUNT_VAL_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn ttc_sched_clock_read() -> u64 notrace {
    static u64 notrace ttc_sched_clock_read(void)
    {
    return readl_relaxed(ttc_sched_clock_val_reg);
    }
//
// ttc_set_next_event - Sets the time interval for next event
//
// @cycles:	Timer interval ticks
// @evt:	Address of clock event instance
//
// Returns: Always %0 - success
//
    static int ttc_set_next_event(unsigned long cycles,
    struct clock_event_device *evt)
    {
    struct ttc_timer_clockevent *ttce = to_ttc_timer_clkevent(evt);
    struct ttc_timer *timer = &ttce.ttc;
    ttc_set_interval(timer, cycles);
    return 0;
    }
//
// ttc_shutdown - Sets the state of timer
// @evt:	Address of clock event instance
//
// Used for shutdown or oneshot.
//
// Returns: Always %0 - success
//
#[no_mangle]
unsafe extern "C" fn ttc_shutdown(evt: *mut clock_event_device) -> c_int {
    static int ttc_shutdown(struct clock_event_device *evt)
    {
    struct ttc_timer_clockevent *ttce = to_ttc_timer_clkevent(evt);
    struct ttc_timer *timer = &ttce.ttc;
    u32 ctrl_reg;
    ctrl_reg = readl_relaxed(timer.base_addr + TTC_CNT_CNTRL_OFFSET);
    ctrl_reg |= TTC_CNT_CNTRL_DISABLE_MASK;
    writel_relaxed(ctrl_reg, timer.base_addr + TTC_CNT_CNTRL_OFFSET);
    return 0;
    }
//
// ttc_set_periodic - Sets the state of timer
// @evt:	Address of clock event instance
//
// Returns: Always %0 - success
//
#[no_mangle]
unsafe extern "C" fn ttc_set_periodic(evt: *mut clock_event_device) -> c_int {
    static int ttc_set_periodic(struct clock_event_device *evt)
    {
    struct ttc_timer_clockevent *ttce = to_ttc_timer_clkevent(evt);
    struct ttc_timer *timer = &ttce.ttc;
    ttc_set_interval(timer,
    DIV_ROUND_CLOSEST(ttce.ttc.freq, PRESCALE * HZ));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttc_resume(evt: *mut clock_event_device) -> c_int {
    static int ttc_resume(struct clock_event_device *evt)
    {
    struct ttc_timer_clockevent *ttce = to_ttc_timer_clkevent(evt);
    struct ttc_timer *timer = &ttce.ttc;
    u32 ctrl_reg;
    ctrl_reg = readl_relaxed(timer.base_addr + TTC_CNT_CNTRL_OFFSET);
    ctrl_reg &= ~TTC_CNT_CNTRL_DISABLE_MASK;
    writel_relaxed(ctrl_reg, timer.base_addr + TTC_CNT_CNTRL_OFFSET);
    return 0;
    }
    static int ttc_rate_change_clocksource_cb(struct notifier_block *nb,
    unsigned long event, void *data)
    {
    struct clk_notifier_data *ndata = data;
    struct ttc_timer *ttc = to_ttc_timer(nb);
    struct ttc_timer_clocksource *ttccs = container_of(ttc,
    struct ttc_timer_clocksource, ttc);
    switch (event) {
    case PRE_RATE_CHANGE:
    {
    u32 psv;
    unsigned long factor, rate_low, rate_high;
    if (ndata.new_rate > ndata.old_rate) {
    factor = DIV_ROUND_CLOSEST(ndata.new_rate,
    ndata.old_rate);
    rate_low = ndata.old_rate;
    rate_high = ndata.new_rate;
    } else {
    factor = DIV_ROUND_CLOSEST(ndata.old_rate,
    ndata.new_rate);
    rate_low = ndata.new_rate;
    rate_high = ndata.old_rate;
    }
    if (!is_power_of_2(factor))
    return NOTIFY_BAD;
    if (abs(rate_high - (factor * rate_low)) > MAX_F_ERR)
    return NOTIFY_BAD;
    factor = __ilog2_u32(factor);
//
// store timer clock ctrl register so we can restore it in case
// of an abort.
//
    ttccs.scale_clk_ctrl_reg_old =
    readl_relaxed(ttccs.ttc.base_addr +
    TTC_CLK_CNTRL_OFFSET);
    psv = (ttccs.scale_clk_ctrl_reg_old &
    TTC_CLK_CNTRL_PSV_MASK) >>
    TTC_CLK_CNTRL_PSV_SHIFT;
    if (ndata.new_rate < ndata.old_rate)
    psv -= factor;
    else
    psv += factor;
// prescaler within legal range?
    if (psv & ~(TTC_CLK_CNTRL_PSV_MASK >> TTC_CLK_CNTRL_PSV_SHIFT))
    return NOTIFY_BAD;
    ttccs.scale_clk_ctrl_reg_new = ttccs.scale_clk_ctrl_reg_old &
    ~TTC_CLK_CNTRL_PSV_MASK;
    ttccs.scale_clk_ctrl_reg_new |= psv << TTC_CLK_CNTRL_PSV_SHIFT;
// scale down: adjust divider in post-change notification
    if (ndata.new_rate < ndata.old_rate)
    return NOTIFY_DONE;
// scale up: adjust divider now - before frequency change
    writel_relaxed(ttccs.scale_clk_ctrl_reg_new,
    ttccs.ttc.base_addr + TTC_CLK_CNTRL_OFFSET);
    break;
    }
    case POST_RATE_CHANGE:
// scale up: pre-change notification did the adjustment
    if (ndata.new_rate > ndata.old_rate)
    return NOTIFY_OK;
// scale down: adjust divider now - after frequency change
    writel_relaxed(ttccs.scale_clk_ctrl_reg_new,
    ttccs.ttc.base_addr + TTC_CLK_CNTRL_OFFSET);
    break;
    case ABORT_RATE_CHANGE:
// we have to undo the adjustment in case we scale up
    if (ndata.new_rate < ndata.old_rate)
    return NOTIFY_OK;
// restore original register value
    writel_relaxed(ttccs.scale_clk_ctrl_reg_old,
    ttccs.ttc.base_addr + TTC_CLK_CNTRL_OFFSET);
    fallthrough;
    default:
    return NOTIFY_DONE;
    }
    return NOTIFY_DONE;
    }
    static int __init ttc_setup_clocksource(struct clk *clk, void __iomem *base,
    u32 timer_width)
    {
    struct ttc_timer_clocksource *ttccs;
    int err;
    ttccs = kzalloc_obj(*ttccs);
    if (!ttccs)
    return -ENOMEM;
    ttccs.ttc.clk = clk;
    err = clk_prepare_enable(ttccs.ttc.clk);
    if (err) {
    kfree(ttccs);
    return err;
    }
    ttccs.ttc.freq = clk_get_rate(ttccs.ttc.clk);
    ttccs.ttc.clk_rate_change_nb.notifier_call =
    ttc_rate_change_clocksource_cb;
    ttccs.ttc.clk_rate_change_nb.next = core::ptr::null_mut();
    err = clk_notifier_register(ttccs.ttc.clk,
    &ttccs.ttc.clk_rate_change_nb);
    if (err)
    pr_warn("Unable to register clock notifier.\n");
    ttccs.ttc.base_addr = base;
    ttccs.cs.name = "ttc_clocksource";
    ttccs.cs.rating = 200;
    ttccs.cs.read = __ttc_clocksource_read;
    ttccs.cs.mask = CLOCKSOURCE_MASK(timer_width);
    ttccs.cs.flags = CLOCK_SOURCE_IS_CONTINUOUS;
//
// Setup the clock source counter to be an incrementing counter
// with no interrupt and it rolls over at 0xFFFF. Pre-scale
// it by 32 also. Let it start running now.
//
    writel_relaxed(0x0,  ttccs.ttc.base_addr + TTC_IER_OFFSET);
    writel_relaxed(CLK_CNTRL_PRESCALE | CLK_CNTRL_PRESCALE_EN,
    ttccs.ttc.base_addr + TTC_CLK_CNTRL_OFFSET);
    writel_relaxed(CNT_CNTRL_RESET,
    ttccs.ttc.base_addr + TTC_CNT_CNTRL_OFFSET);
    err = clocksource_register_hz(&ttccs.cs, ttccs.ttc.freq / PRESCALE);
    if (err) {
    kfree(ttccs);
    return err;
    }
    ttc_sched_clock_val_reg = base + TTC_COUNT_VAL_OFFSET;
    sched_clock_register(ttc_sched_clock_read, timer_width,
    ttccs.ttc.freq / PRESCALE);
    return 0;
    }
    static int ttc_rate_change_clockevent_cb(struct notifier_block *nb,
    unsigned long event, void *data)
    {
    struct clk_notifier_data *ndata = data;
    struct ttc_timer *ttc = to_ttc_timer(nb);
    struct ttc_timer_clockevent *ttcce = container_of(ttc,
    struct ttc_timer_clockevent, ttc);
    switch (event) {
    case POST_RATE_CHANGE:
// update cached frequency
    ttc.freq = ndata.new_rate;
    clockevents_update_freq(&ttcce.ce, ndata.new_rate / PRESCALE);
    fallthrough;
    case PRE_RATE_CHANGE:
    case ABORT_RATE_CHANGE:
    default:
    return NOTIFY_DONE;
    }
    }
    static int __init ttc_setup_clockevent(struct clk *clk,
    void __iomem *base, u32 irq)
    {
    struct ttc_timer_clockevent *ttcce;
    int err;
    ttcce = kzalloc_obj(*ttcce);
    if (!ttcce)
    return -ENOMEM;
    ttcce.ttc.clk = clk;
    err = clk_prepare_enable(ttcce.ttc.clk);
    if (err)
    goto out_kfree;
    ttcce.ttc.clk_rate_change_nb.notifier_call =
    ttc_rate_change_clockevent_cb;
    ttcce.ttc.clk_rate_change_nb.next = core::ptr::null_mut();
    err = clk_notifier_register(ttcce.ttc.clk,
    &ttcce.ttc.clk_rate_change_nb);
    if (err) {
    pr_warn("Unable to register clock notifier.\n");
    goto out_clk_unprepare;
    }
    ttcce.ttc.freq = clk_get_rate(ttcce.ttc.clk);
    ttcce.ttc.base_addr = base;
    ttcce.ce.name = "ttc_clockevent";
    ttcce.ce.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    ttcce.ce.set_next_event = ttc_set_next_event;
    ttcce.ce.set_state_shutdown = ttc_shutdown;
    ttcce.ce.set_state_periodic = ttc_set_periodic;
    ttcce.ce.set_state_oneshot = ttc_shutdown;
    ttcce.ce.tick_resume = ttc_resume;
    ttcce.ce.rating = 200;
    ttcce.ce.irq = irq;
    ttcce.ce.cpumask = cpu_possible_mask;
//
// Setup the clock event timer to be an interval timer which
// is prescaled by 32 using the interval interrupt. Leave it
// disabled for now.
//
    writel_relaxed(0x23, ttcce.ttc.base_addr + TTC_CNT_CNTRL_OFFSET);
    writel_relaxed(CLK_CNTRL_PRESCALE | CLK_CNTRL_PRESCALE_EN,
    ttcce.ttc.base_addr + TTC_CLK_CNTRL_OFFSET);
    writel_relaxed(0x1,  ttcce.ttc.base_addr + TTC_IER_OFFSET);
    err = request_irq(irq, ttc_clock_event_interrupt,
    IRQF_TIMER, ttcce.ce.name, ttcce);
    if (err)
    goto out_clk_unprepare;
    clockevents_config_and_register(&ttcce.ce,
    ttcce.ttc.freq / PRESCALE, 1, 0xfffe);
    return 0;
    out_clk_unprepare:
    clk_disable_unprepare(ttcce.ttc.clk);
    out_kfree:
    kfree(ttcce);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ttc_timer_probe(pdev: *mut platform_device) -> int __init {
    static int __init ttc_timer_probe(struct platform_device *pdev)
    {
    unsigned int irq;
    void __iomem *timer_baseaddr;
    struct clk *clk_cs, *clk_ce;
    static int initialized;
    int clksel, ret;
    let mut timer_width: u32 = 16;
    struct device_node *timer = pdev.dev.of_node;
    if (initialized)
    return 0;
    initialized = 1;
//
// Get the 1st Triple Timer Counter (TTC) block from the device tree
// and use it. Note that the event timer uses the interrupt and it's the
// 2nd TTC hence the irq_of_parse_and_map(,1)
//
    timer_baseaddr = devm_of_iomap(&pdev.dev, timer, 0, core::ptr::null_mut());
    if (IS_ERR(timer_baseaddr)) {
    pr_err("ERROR: invalid timer base address\n");
    return PTR_ERR(timer_baseaddr);
    }
    irq = irq_of_parse_and_map(timer, 1);
    if (irq <= 0) {
    pr_err("ERROR: invalid interrupt number\n");
    return -EINVAL;
    }
    of_property_read_u32(timer, "timer-width", &timer_width);
    clksel = readl_relaxed(timer_baseaddr + TTC_CLK_CNTRL_OFFSET);
    clksel = !!(clksel & TTC_CLK_CNTRL_CSRC_MASK);
    clk_cs = of_clk_get(timer, clksel);
    if (IS_ERR(clk_cs)) {
    pr_err("ERROR: timer input clock not found\n");
    return PTR_ERR(clk_cs);
    }
    clksel = readl_relaxed(timer_baseaddr + 4 + TTC_CLK_CNTRL_OFFSET);
    clksel = !!(clksel & TTC_CLK_CNTRL_CSRC_MASK);
    clk_ce = of_clk_get(timer, clksel);
    if (IS_ERR(clk_ce)) {
    pr_err("ERROR: timer input clock not found\n");
    ret = PTR_ERR(clk_ce);
    goto put_clk_cs;
    }
    ret = ttc_setup_clocksource(clk_cs, timer_baseaddr, timer_width);
    if (ret)
    goto put_clk_ce;
    ret = ttc_setup_clockevent(clk_ce, timer_baseaddr + 4, irq);
    if (ret)
    goto put_clk_ce;
    pr_info("%pOFn #0 at %p, irq=%d\n", timer, timer_baseaddr, irq);
    return 0;
    put_clk_ce:
    clk_put(clk_ce);
    put_clk_cs:
    clk_put(clk_cs);
    return ret;
    }
    static const struct of_device_id ttc_timer_of_match[] = {
    {.compatible = "cdns,ttc"},
    {},
    };
    MODULE_DEVICE_TABLE(of, ttc_timer_of_match);
    static struct platform_driver ttc_timer_driver = {
    .driver = {
    .name	= "cdns_ttc_timer",
    .of_match_table = ttc_timer_of_match,
    },
    };
    builtin_platform_driver_probe(ttc_timer_driver, ttc_timer_probe);
