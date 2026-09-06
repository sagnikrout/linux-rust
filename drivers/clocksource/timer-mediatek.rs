//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-mediatek.c
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
// Mediatek SoCs General-Purpose Timer handling.
//
// Copyright (C) 2014 Matthias Brugger
//
// Matthias Brugger <matthias.bgg@gmail.com>
//

// gpt
pub const GPT_IRQ_EN_REG: c_uint = 0x00;

pub const GPT_IRQ_ACK_REG: c_uint = 0x08;

// system timer

//
// SYST_CON_EN: Clock enable. Shall be set to
// - Start timer countdown.
// - Allow timeout ticks being updated.
// - Allow changing interrupt status,like clear irq pending.
//
// SYST_CON_IRQ_EN: Set to enable interrupt.
//
// SYST_CON_IRQ_CLR: Set to clear interrupt.
//

    static void __iomem *gpt_sched_reg __read_mostly;
#[no_mangle]
unsafe extern "C" fn mtk_syst_ack_irq(to: *mut timer_of) {
    static void mtk_syst_ack_irq(struct timer_of *to)
    {
// Clear and disable interrupt
    writel(SYST_CON_EN, SYST_CON_REG(to));
    writel(SYST_CON_IRQ_CLR | SYST_CON_EN, SYST_CON_REG(to));
    }
#[no_mangle]
unsafe extern "C" fn mtk_syst_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mtk_syst_handler(int irq, void *dev_id)
    {
    struct clock_event_device *clkevt = dev_id;
    struct timer_of *to = to_timer_of(clkevt);
    mtk_syst_ack_irq(to);
    clkevt.event_handler(clkevt);
    return IRQ_HANDLED;
    }
    static int mtk_syst_clkevt_next_event(unsigned long ticks,
    struct clock_event_device *clkevt)
    {
    struct timer_of *to = to_timer_of(clkevt);
// Enable clock to allow timeout tick update later
    writel(SYST_CON_EN, SYST_CON_REG(to));
//
// Write new timeout ticks. Timer shall start countdown
// after timeout ticks are updated.
//
    writel(ticks, SYST_VAL_REG(to));
// Enable interrupt
    writel(SYST_CON_EN | SYST_CON_IRQ_EN, SYST_CON_REG(to));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_syst_clkevt_shutdown(clkevt: *mut clock_event_device) -> c_int {
    static int mtk_syst_clkevt_shutdown(struct clock_event_device *clkevt)
    {
// Clear any irq
    mtk_syst_ack_irq(to_timer_of(clkevt));
// Disable timer
    writel(0, SYST_CON_REG(to_timer_of(clkevt)));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_syst_clkevt_resume(clkevt: *mut clock_event_device) -> c_int {
    static int mtk_syst_clkevt_resume(struct clock_event_device *clkevt)
    {
    return mtk_syst_clkevt_shutdown(clkevt);
    }
#[no_mangle]
unsafe extern "C" fn mtk_syst_clkevt_oneshot(clkevt: *mut clock_event_device) -> c_int {
    static int mtk_syst_clkevt_oneshot(struct clock_event_device *clkevt)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_gpt_read_sched_clock() -> u64 notrace {
    static u64 notrace mtk_gpt_read_sched_clock(void)
    {
    return readl_relaxed(gpt_sched_reg);
    }
#[no_mangle]
unsafe extern "C" fn mtk_gpt_clkevt_time_stop(to: *mut timer_of, timer: u8) {
    static void mtk_gpt_clkevt_time_stop(struct timer_of *to, u8 timer)
    {
    u32 val;
    val = readl(timer_of_base(to) + GPT_CTRL_REG(timer));
    writel(val & ~GPT_CTRL_ENABLE, timer_of_base(to) +
    GPT_CTRL_REG(timer));
    }
    static void mtk_gpt_clkevt_time_setup(struct timer_of *to,
    unsigned long delay, u8 timer)
    {
    writel(delay, timer_of_base(to) + GPT_CMP_REG(timer));
    }
    static void mtk_gpt_clkevt_time_start(struct timer_of *to,
    bool periodic, u8 timer)
    {
    u32 val;
// Acknowledge interrupt
    writel(GPT_IRQ_ACK(timer), timer_of_base(to) + GPT_IRQ_ACK_REG);
    val = readl(timer_of_base(to) + GPT_CTRL_REG(timer));
// Clear 2 bit timer operation mode field
    val &= ~GPT_CTRL_OP(0x3);
    if (periodic)
    val |= GPT_CTRL_OP(GPT_CTRL_OP_REPEAT);
    else
    val |= GPT_CTRL_OP(GPT_CTRL_OP_ONESHOT);
    writel(val | GPT_CTRL_ENABLE | GPT_CTRL_CLEAR,
    timer_of_base(to) + GPT_CTRL_REG(timer));
    }
#[no_mangle]
unsafe extern "C" fn mtk_gpt_clkevt_shutdown(clk: *mut clock_event_device) -> c_int {
    static int mtk_gpt_clkevt_shutdown(struct clock_event_device *clk)
    {
    mtk_gpt_clkevt_time_stop(to_timer_of(clk), TIMER_CLK_EVT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_gpt_clkevt_set_periodic(clk: *mut clock_event_device) -> c_int {
    static int mtk_gpt_clkevt_set_periodic(struct clock_event_device *clk)
    {
    struct timer_of *to = to_timer_of(clk);
    mtk_gpt_clkevt_time_stop(to, TIMER_CLK_EVT);
    mtk_gpt_clkevt_time_setup(to, to.of_clk.period, TIMER_CLK_EVT);
    mtk_gpt_clkevt_time_start(to, true, TIMER_CLK_EVT);
    return 0;
    }
    static int mtk_gpt_clkevt_next_event(unsigned long event,
    struct clock_event_device *clk)
    {
    struct timer_of *to = to_timer_of(clk);
    mtk_gpt_clkevt_time_stop(to, TIMER_CLK_EVT);
    mtk_gpt_clkevt_time_setup(to, event, TIMER_CLK_EVT);
    mtk_gpt_clkevt_time_start(to, false, TIMER_CLK_EVT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_gpt_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mtk_gpt_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *clkevt = (struct clock_event_device *)dev_id;
    struct timer_of *to = to_timer_of(clkevt);
// Acknowledge timer0 irq
    writel(GPT_IRQ_ACK(TIMER_CLK_EVT), timer_of_base(to) + GPT_IRQ_ACK_REG);
    clkevt.event_handler(clkevt);
    return IRQ_HANDLED;
    }
    static void
#[no_mangle]
pub unsafe extern "C" fn mtk_gpt_setup(to: *mut timer_of, timer: u8, option: u8) -> __init {
    __init mtk_gpt_setup(struct timer_of *to, u8 timer, u8 option)
    {
    writel(GPT_CTRL_CLEAR | GPT_CTRL_DISABLE,
    timer_of_base(to) + GPT_CTRL_REG(timer));
    writel(GPT_CLK_SRC(GPT_CLK_SRC_SYS13M) | GPT_CLK_DIV1,
    timer_of_base(to) + GPT_CLK_REG(timer));
    writel(0x0, timer_of_base(to) + GPT_CMP_REG(timer));
    writel(GPT_CTRL_OP(option) | GPT_CTRL_ENABLE,
    timer_of_base(to) + GPT_CTRL_REG(timer));
    }
#[no_mangle]
unsafe extern "C" fn mtk_gpt_enable_irq(to: *mut timer_of, timer: u8) {
    static void mtk_gpt_enable_irq(struct timer_of *to, u8 timer)
    {
    u32 val;
// Disable all interrupts
    writel(0x0, timer_of_base(to) + GPT_IRQ_EN_REG);
// Acknowledge all spurious pending interrupts
    writel(0x3f, timer_of_base(to) + GPT_IRQ_ACK_REG);
    val = readl(timer_of_base(to) + GPT_IRQ_EN_REG);
    writel(val | GPT_IRQ_ENABLE(timer),
    timer_of_base(to) + GPT_IRQ_EN_REG);
    }
#[no_mangle]
unsafe extern "C" fn mtk_gpt_resume(clk: *mut clock_event_device) {
    static void mtk_gpt_resume(struct clock_event_device *clk)
    {
    struct timer_of *to = to_timer_of(clk);
    mtk_gpt_enable_irq(to, TIMER_CLK_EVT);
    }
#[no_mangle]
unsafe extern "C" fn mtk_gpt_suspend(clk: *mut clock_event_device) {
    static void mtk_gpt_suspend(struct clock_event_device *clk)
    {
    struct timer_of *to = to_timer_of(clk);
// Disable all interrupts
    writel(0x0, timer_of_base(to) + GPT_IRQ_EN_REG);
//
// This is called with interrupts disabled,
// so we need to ack any interrupt that is pending
// or for example ATF will prevent a suspend from completing.
//
    writel(0x3f, timer_of_base(to) + GPT_IRQ_ACK_REG);
    }
    static struct timer_of to = {
    .flags = TIMER_OF_IRQ | TIMER_OF_BASE | TIMER_OF_CLOCK,
    .clkevt = {
    .name = "mtk-clkevt",
    .rating = 300,
    .cpumask = cpu_possible_mask,
    },
    .of_irq = {
    .flags = IRQF_TIMER | IRQF_IRQPOLL,
    },
    };
#[no_mangle]
unsafe extern "C" fn mtk_syst_init(node: *mut device_node) -> int __init {
    static int __init mtk_syst_init(struct device_node *node)
    {
    int ret;
    to.clkevt.features = CLOCK_EVT_FEAT_DYNIRQ | CLOCK_EVT_FEAT_ONESHOT;
    to.clkevt.set_state_shutdown = mtk_syst_clkevt_shutdown;
    to.clkevt.set_state_oneshot = mtk_syst_clkevt_oneshot;
    to.clkevt.tick_resume = mtk_syst_clkevt_resume;
    to.clkevt.set_next_event = mtk_syst_clkevt_next_event;
    to.of_irq.handler = mtk_syst_handler;
    ret = timer_of_init(node, &to);
    if (ret)
    return ret;
    clockevents_config_and_register(&to.clkevt, timer_of_rate(&to),
    TIMER_SYNC_TICKS, 0xffffffff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_gpt_init(node: *mut device_node) -> int __init {
    static int __init mtk_gpt_init(struct device_node *node)
    {
    int ret;
    to.clkevt.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    to.clkevt.set_state_shutdown = mtk_gpt_clkevt_shutdown;
    to.clkevt.set_state_periodic = mtk_gpt_clkevt_set_periodic;
    to.clkevt.set_state_oneshot = mtk_gpt_clkevt_shutdown;
    to.clkevt.tick_resume = mtk_gpt_clkevt_shutdown;
    to.clkevt.set_next_event = mtk_gpt_clkevt_next_event;
    to.clkevt.suspend = mtk_gpt_suspend;
    to.clkevt.resume = mtk_gpt_resume;
    to.of_irq.handler = mtk_gpt_interrupt;
    ret = timer_of_init(node, &to);
    if (ret)
    return ret;
// Configure clock source
    mtk_gpt_setup(&to, TIMER_CLK_SRC, GPT_CTRL_OP_FREERUN);
    clocksource_mmio_init(timer_of_base(&to) + GPT_CNT_REG(TIMER_CLK_SRC),
    node.name, timer_of_rate(&to), 300, 32,
    clocksource_mmio_readl_up);
    gpt_sched_reg = timer_of_base(&to) + GPT_CNT_REG(TIMER_CLK_SRC);
    sched_clock_register(mtk_gpt_read_sched_clock, 32, timer_of_rate(&to));
// Configure clock event
    mtk_gpt_setup(&to, TIMER_CLK_EVT, GPT_CTRL_OP_REPEAT);
    clockevents_config_and_register(&to.clkevt, timer_of_rate(&to),
    TIMER_SYNC_TICKS, 0xffffffff);
    mtk_gpt_enable_irq(&to, TIMER_CLK_EVT);
    return 0;
    }
    TIMER_OF_DECLARE(mtk_mt6577, "mediatek,mt6577-timer", mtk_gpt_init);
    TIMER_OF_DECLARE(mtk_mt6765, "mediatek,mt6765-timer", mtk_syst_init);
