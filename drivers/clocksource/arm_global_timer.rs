//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/arm_global_timer.c
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
// drivers/clocksource/arm_global_timer.c
//
// Copyright (C) 2013 STMicroelectronics (R&D) Limited.
// Author: Stuart Menefy <stuart.menefy@st.com>
// Author: Srinivas Kandagatla <srinivas.kandagatla@st.com>
//

pub const GT_COUNTER0: c_uint = 0x00;
pub const GT_COUNTER1: c_uint = 0x04;
pub const GT_CONTROL: c_uint = 0x08;

pub const GT_INT_STATUS: c_uint = 0x0c;

pub const GT_COMP0: c_uint = 0x10;
pub const GT_COMP1: c_uint = 0x14;
pub const GT_AUTO_INC: c_uint = 0x18;
pub const MAX_F_ERR: c_int = 50;
//
// We are expecting to be clocked by the ARM peripheral clock.
//
// Note: it is assumed we are using a prescaler value of zero, so this is
// the units for all operations.
//
    static void __iomem *gt_base;
    static struct notifier_block gt_clk_rate_change_nb;
    static u32 gt_psv_new, gt_psv_bck;
    static unsigned long gt_target_rate;
    static int gt_ppi;
    static struct clock_event_device __percpu *gt_evt;
//
// To get the value from the Global Timer Counter register proceed as follows:
// 1. Read the upper 32-bit timer counter register
// 2. Read the lower 32-bit timer counter register
// 3. Read the upper 32-bit timer counter register again. If the value is
// different to the 32-bit upper value read previously, go back to step 2.
// Otherwise the 64-bit timer counter value is correct.
//
#[no_mangle]
unsafe extern "C" fn _gt_counter_read() -> u64 notrace {
    static u64 notrace _gt_counter_read(void)
    {
    u64 counter;
    u32 lower;
    u32 upper, old_upper;
    upper = readl_relaxed(gt_base + GT_COUNTER1);
    do {
    old_upper = upper;
    lower = readl_relaxed(gt_base + GT_COUNTER0);
    upper = readl_relaxed(gt_base + GT_COUNTER1);
    } while (upper != old_upper);
    counter = upper;
    counter <<= 32;
    counter |= lower;
    return counter;
    }
#[no_mangle]
unsafe extern "C" fn gt_counter_read() -> u64 {
    static u64 gt_counter_read(void)
    {
    return _gt_counter_read();
    }
//
// To ensure that updates to comparator value register do not set the
// Interrupt Status Register proceed as follows:
// 1. Clear the Comp Enable bit in the Timer Control Register.
// 2. Write the lower 32-bit Comparator Value Register.
// 3. Write the upper 32-bit Comparator Value Register.
// 4. Set the Comp Enable bit and, if necessary, the IRQ enable bit.
//
#[no_mangle]
unsafe extern "C" fn gt_compare_set(delta: c_ulong, periodic: c_int) {
    static void gt_compare_set(unsigned long delta, int periodic)
    {
    let mut counter: u64 = gt_counter_read();
    unsigned long ctrl;
    counter += delta;
    ctrl = readl(gt_base + GT_CONTROL);
    ctrl &= ~(GT_CONTROL_COMP_ENABLE | GT_CONTROL_IRQ_ENABLE |
    GT_CONTROL_AUTO_INC);
    ctrl |= GT_CONTROL_TIMER_ENABLE;
    writel_relaxed(ctrl, gt_base + GT_CONTROL);
    writel_relaxed(lower_32_bits(counter), gt_base + GT_COMP0);
    writel_relaxed(upper_32_bits(counter), gt_base + GT_COMP1);
    if (periodic) {
    writel_relaxed(delta, gt_base + GT_AUTO_INC);
    ctrl |= GT_CONTROL_AUTO_INC;
    }
    ctrl |= GT_CONTROL_COMP_ENABLE | GT_CONTROL_IRQ_ENABLE;
    writel_relaxed(ctrl, gt_base + GT_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn gt_clockevent_shutdown(evt: *mut clock_event_device) -> c_int {
    static int gt_clockevent_shutdown(struct clock_event_device *evt)
    {
    unsigned long ctrl;
    ctrl = readl(gt_base + GT_CONTROL);
    ctrl &= ~(GT_CONTROL_COMP_ENABLE | GT_CONTROL_IRQ_ENABLE |
    GT_CONTROL_AUTO_INC);
    writel(ctrl, gt_base + GT_CONTROL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gt_clockevent_set_periodic(evt: *mut clock_event_device) -> c_int {
    static int gt_clockevent_set_periodic(struct clock_event_device *evt)
    {
    gt_compare_set(DIV_ROUND_CLOSEST(gt_target_rate, HZ), 1);
    return 0;
    }
    static int gt_clockevent_set_next_event(unsigned long evt,
    struct clock_event_device *unused)
    {
    gt_compare_set(evt, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gt_clockevent_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t gt_clockevent_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    if (!(readl_relaxed(gt_base + GT_INT_STATUS) &
    GT_INT_STATUS_EVENT_FLAG))
    return IRQ_NONE;
//
// ERRATA 740657( Global Timer can send 2 interrupts for
// the same event in single-shot mode)
// Workaround:
// Either disable single-shot mode.
// Or
// Modify the Interrupt Handler to avoid the
// offending sequence. This is achieved by clearing
// the Global Timer flag _after_ having incremented
// the Comparator register	value to a higher value.
//
    if (clockevent_state_oneshot(evt))
    gt_compare_set(ULONG_MAX, 0);
    writel_relaxed(GT_INT_STATUS_EVENT_FLAG, gt_base + GT_INT_STATUS);
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn gt_starting_cpu(cpu: c_uint) -> c_int {
    static int gt_starting_cpu(unsigned int cpu)
    {
    struct clock_event_device *clk = this_cpu_ptr(gt_evt);
    clk.name = "arm_global_timer";
    clk.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT |
    CLOCK_EVT_FEAT_PERCPU;
    clk.set_state_shutdown = gt_clockevent_shutdown;
    clk.set_state_periodic = gt_clockevent_set_periodic;
    clk.set_state_oneshot = gt_clockevent_shutdown;
    clk.set_state_oneshot_stopped = gt_clockevent_shutdown;
    clk.set_next_event = gt_clockevent_set_next_event;
    clk.cpumask = cpumask_of(cpu);
    clk.rating = 300;
    clk.irq = gt_ppi;
    clockevents_config_and_register(clk, gt_target_rate,
    1, 0xffffffff);
    enable_percpu_irq(clk.irq, IRQ_TYPE_NONE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gt_dying_cpu(cpu: c_uint) -> c_int {
    static int gt_dying_cpu(unsigned int cpu)
    {
    struct clock_event_device *clk = this_cpu_ptr(gt_evt);
    disable_percpu_irq(clk.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gt_clocksource_read(cs: *mut clocksource) -> u64 {
    static u64 gt_clocksource_read(struct clocksource *cs)
    {
    return gt_counter_read();
    }
#[no_mangle]
unsafe extern "C" fn gt_resume(cs: *mut clocksource) {
    static void gt_resume(struct clocksource *cs)
    {
    unsigned long ctrl;
    ctrl = readl(gt_base + GT_CONTROL);
    if (!(ctrl & GT_CONTROL_TIMER_ENABLE))
// re-enable timer on resume
    writel(GT_CONTROL_TIMER_ENABLE, gt_base + GT_CONTROL);
    }
    static struct clocksource gt_clocksource = {
    .name	= "arm_global_timer",
    .rating	= 300,
    .read	= gt_clocksource_read,
    .mask	= CLOCKSOURCE_MASK(64),
    .flags	= CLOCK_SOURCE_IS_CONTINUOUS,
    .resume = gt_resume,
    };

#[no_mangle]
unsafe extern "C" fn gt_sched_clock_read() -> u64 notrace {
    static u64 notrace gt_sched_clock_read(void)
    {
    return _gt_counter_read();
    }

#[no_mangle]
unsafe extern "C" fn gt_read_long() -> c_ulong {
    static unsigned long gt_read_long(void)
    {
    return readl_relaxed(gt_base + GT_COUNTER0);
    }
    static struct delay_timer gt_delay_timer = {
    .read_current_timer = gt_read_long,
    };
#[no_mangle]
unsafe extern "C" fn gt_write_presc(psv: u32) {
    static void gt_write_presc(u32 psv)
    {
    u32 reg;
    reg = readl(gt_base + GT_CONTROL);
    reg &= ~GT_CONTROL_PRESCALER_MASK;
    reg |= FIELD_PREP(GT_CONTROL_PRESCALER_MASK, psv);
    writel(reg, gt_base + GT_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn gt_read_presc() -> u32 {
    static u32 gt_read_presc(void)
    {
    u32 reg;
    reg = readl(gt_base + GT_CONTROL);
    return FIELD_GET(GT_CONTROL_PRESCALER_MASK, reg);
    }
#[no_mangle]
unsafe extern "C" fn gt_delay_timer_init() -> void __init {
    static void __init gt_delay_timer_init(void)
    {
    gt_delay_timer.freq = gt_target_rate;
    register_current_timer_delay(&gt_delay_timer);
    }
#[no_mangle]
unsafe extern "C" fn gt_clocksource_init(psv: c_uint) -> int __init {
    static int __init gt_clocksource_init(unsigned int psv)
    {
    writel(0, gt_base + GT_CONTROL);
    writel(0, gt_base + GT_COUNTER0);
    writel(0, gt_base + GT_COUNTER1);
// set prescaler and enable timer on all the cores
    writel(FIELD_PREP(GT_CONTROL_PRESCALER_MASK, psv - 1) |
    GT_CONTROL_TIMER_ENABLE, gt_base + GT_CONTROL);

    sched_clock_register(gt_sched_clock_read, 64, gt_target_rate);

    return clocksource_register_hz(&gt_clocksource, gt_target_rate);
    }
    static int gt_clk_rate_change_cb(struct notifier_block *nb,
    unsigned long event, void *data)
    {
    struct clk_notifier_data *ndata = data;
    switch (event) {
    case PRE_RATE_CHANGE:
    {
    unsigned long psv;
    psv = DIV_ROUND_CLOSEST(ndata.new_rate, gt_target_rate);
    if (!psv ||
    abs(gt_target_rate - (ndata.new_rate / psv)) > MAX_F_ERR)
    return NOTIFY_BAD;
    psv--;
// prescaler within legal range?
    if (!FIELD_FIT(GT_CONTROL_PRESCALER_MASK, psv))
    return NOTIFY_BAD;
//
// store timer clock ctrl register so we can restore it in case
// of an abort.
//
    gt_psv_bck = gt_read_presc();
    gt_psv_new = psv;
// scale down: adjust divider in post-change notification
    if (ndata.new_rate < ndata.old_rate)
    return NOTIFY_DONE;
// scale up: adjust divider now - before frequency change
    gt_write_presc(psv);
    break;
    }
    case POST_RATE_CHANGE:
// scale up: pre-change notification did the adjustment
    if (ndata.new_rate > ndata.old_rate)
    return NOTIFY_OK;
// scale down: adjust divider now - after frequency change
    gt_write_presc(gt_psv_new);
    break;
    case ABORT_RATE_CHANGE:
// we have to undo the adjustment in case we scale up
    if (ndata.new_rate < ndata.old_rate)
    return NOTIFY_OK;
// restore original register value
    gt_write_presc(gt_psv_bck);
    break;
    default:
    return NOTIFY_DONE;
    }
    return NOTIFY_DONE;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gt_prescaler_config {
    pub compatible: *const c_char,
    pub prescaler: c_ulong,
}

    static const struct gt_prescaler_config gt_prescaler_configs[] = {
//
// On am43 the global timer clock is a child of the clock used for CPU
// OPPs, so the initial prescaler has to be compatible with all OPPs
// which are 300, 600, 720, 800 and 1000 with a fixed divider of 2, this
// gives us a GCD of 10. Initial frequency is 1000, so the prescaler is
// 50.
//
    { .compatible = "ti,am43", .prescaler = 50 },
    { .compatible = "xlnx,zynq-7000", .prescaler = 2 },
    { .compatible = core::ptr::null_mut() }
    };
#[no_mangle]
unsafe extern "C" fn gt_get_initial_prescaler_value(np: *mut device_node) -> c_ulong {
    static unsigned long gt_get_initial_prescaler_value(struct device_node *np)
    {
    const struct gt_prescaler_config *config;
    if (CONFIG_ARM_GT_INITIAL_PRESCALER_VAL != 0)
    return CONFIG_ARM_GT_INITIAL_PRESCALER_VAL;
    for (config = gt_prescaler_configs; config.compatible; config++) {
    if (of_machine_is_compatible(config.compatible))
    return config.prescaler;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn global_timer_of_register(np: *mut device_node) -> int __init {
    static int __init global_timer_of_register(struct device_node *np)
    {
    struct clk *gt_clk;
    static unsigned long gt_clk_rate;
    int err;
    unsigned long psv;
//
// In A9 r2p0 the comparators for each processor with the global timer
// fire when the timer value is greater than or equal to. In previous
// revisions the comparators fired when the timer value was equal to.
//
    if (read_cpuid_part() == ARM_CPU_PART_CORTEX_A9
    && (read_cpuid_id() & 0xf0000f) < 0x200000) {
    pr_warn("global-timer: non support for this cpu version.\n");
    return -ENOSYS;
    }
    gt_ppi = irq_of_parse_and_map(np, 0);
    if (!gt_ppi) {
    pr_warn("global-timer: unable to parse irq\n");
    return -EINVAL;
    }
    gt_base = of_iomap(np, 0);
    if (!gt_base) {
    pr_warn("global-timer: invalid base address\n");
    return -ENXIO;
    }
    gt_clk = of_clk_get(np, 0);
    if (!IS_ERR(gt_clk)) {
    err = clk_prepare_enable(gt_clk);
    if (err)
    goto out_unmap;
    } else {
    pr_warn("global-timer: clk not found\n");
    err = -EINVAL;
    goto out_unmap;
    }
    psv = gt_get_initial_prescaler_value(np);
    gt_clk_rate = clk_get_rate(gt_clk);
    gt_target_rate = gt_clk_rate / psv;
    gt_clk_rate_change_nb.notifier_call =
    gt_clk_rate_change_cb;
    err = clk_notifier_register(gt_clk, &gt_clk_rate_change_nb);
    if (err) {
    pr_warn("Unable to register clock notifier\n");
    goto out_clk;
    }
    gt_evt = alloc_percpu(struct clock_event_device);
    if (!gt_evt) {
    pr_warn("global-timer: can't allocate memory\n");
    err = -ENOMEM;
    goto out_clk_nb;
    }
    err = request_percpu_irq(gt_ppi, gt_clockevent_interrupt,
    "gt", gt_evt);
    if (err) {
    pr_warn("global-timer: can't register interrupt %d (%d)\n",
    gt_ppi, err);
    goto out_free;
    }
// Register and immediately configure the timer on the boot CPU
    err = gt_clocksource_init(psv);
    if (err)
    goto out_irq;
    err = cpuhp_setup_state(CPUHP_AP_ARM_GLOBAL_TIMER_STARTING,
    "clockevents/arm/global_timer:starting",
    gt_starting_cpu, gt_dying_cpu);
    if (err)
    goto out_irq;
    gt_delay_timer_init();
    return 0;
    out_irq:
    free_percpu_irq(gt_ppi, gt_evt);
    out_free:
    free_percpu(gt_evt);
    out_clk_nb:
    clk_notifier_unregister(gt_clk, &gt_clk_rate_change_nb);
    out_clk:
    clk_disable_unprepare(gt_clk);
    out_unmap:
    iounmap(gt_base);
    WARN(err, "ARM Global timer register failed (%d)\n", err);
    return err;
    }
// Only tested on r2p2 and r3p0
    TIMER_OF_DECLARE(arm_gt, "arm,cortex-a9-global-timer",
    global_timer_of_register);
