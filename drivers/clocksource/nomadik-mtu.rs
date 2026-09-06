//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/nomadik-mtu.c
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
// Copyright (C) 2008 STMicroelectronics
// Copyright (C) 2010 Alessandro Rubini
// Copyright (C) 2010 Linus Walleij for ST-Ericsson
//

//
// The MTU device hosts four different counters, with 4 set of
// registers. These are register names.
//
pub const MTU_IMSC: c_uint = 0x00	/* Interrupt mask set/clear */;
pub const MTU_RIS: c_uint = 0x04	/* Raw interrupt status */;
pub const MTU_MIS: c_uint = 0x08	/* Masked interrupt status */;
pub const MTU_ICR: c_uint = 0x0C	/* Interrupt clear register */;
// per-timer registers take 0..3 as argument

// bits for the control register
pub const MTU_CRn_ENA: c_uint = 0x80;
pub const MTU_CRn_PERIODIC: c_uint = 0x40	/* if 0 = free-running */;
pub const MTU_CRn_PRESCALE_MASK: c_uint = 0x0c;
pub const MTU_CRn_PRESCALE_1: c_uint = 0x00;
pub const MTU_CRn_PRESCALE_16: c_uint = 0x04;
pub const MTU_CRn_PRESCALE_256: c_uint = 0x08;
pub const MTU_CRn_32BITS: c_uint = 0x02;
pub const MTU_CRn_ONESHOT: c_uint = 0x01	/* if 0 = wraps reloading from BGLR*/;
// Other registers are usual amba/primecell registers, currently not used
pub const MTU_ITCR: c_uint = 0xff0;
pub const MTU_ITOP: c_uint = 0xff4;
pub const MTU_PERIPH_ID0: c_uint = 0xfe0;
pub const MTU_PERIPH_ID1: c_uint = 0xfe4;
pub const MTU_PERIPH_ID2: c_uint = 0xfe8;
pub const MTU_PERIPH_ID3: c_uint = 0xfeC;
pub const MTU_PCELL0: c_uint = 0xff0;
pub const MTU_PCELL1: c_uint = 0xff4;
pub const MTU_PCELL2: c_uint = 0xff8;
pub const MTU_PCELL3: c_uint = 0xffC;
    static void __iomem *mtu_base;
    static bool clkevt_periodic;
    static u32 clk_prescale;
    static u32 nmdk_cycle;		/* write-once */
    static struct delay_timer mtu_delay_timer;
//
// Override the global weak sched_clock symbol with this
// local implementation which uses the clocksource to get some
// better resolution when scheduling the kernel.
//
#[no_mangle]
unsafe extern "C" fn nomadik_read_sched_clock() -> u64 notrace {
    static u64 notrace nomadik_read_sched_clock(void)
    {
    if (unlikely(!mtu_base))
    return 0;
    return -readl(mtu_base + MTU_VAL(0));
    }
#[no_mangle]
unsafe extern "C" fn nmdk_timer_read_current_timer() -> c_ulong {
    static unsigned long nmdk_timer_read_current_timer(void)
    {
    return ~readl_relaxed(mtu_base + MTU_VAL(0));
    }
// Clockevent device: use one-shot mode
#[no_mangle]
unsafe extern "C" fn nmdk_clkevt_next(evt: c_ulong, ev: *mut clock_event_device) -> c_int {
    static int nmdk_clkevt_next(unsigned long evt, struct clock_event_device *ev)
    {
    writel(1 << 1, mtu_base + MTU_IMSC);
    writel(evt, mtu_base + MTU_LR(1));
// Load highest value, enable device, enable interrupts
    writel(MTU_CRn_ONESHOT | clk_prescale |
    MTU_CRn_32BITS | MTU_CRn_ENA,
    mtu_base + MTU_CR(1));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nmdk_clkevt_reset() {
    static void nmdk_clkevt_reset(void)
    {
    if (clkevt_periodic) {
// Timer: configure load and background-load, and fire it up
    writel(nmdk_cycle, mtu_base + MTU_LR(1));
    writel(nmdk_cycle, mtu_base + MTU_BGLR(1));
    writel(MTU_CRn_PERIODIC | clk_prescale |
    MTU_CRn_32BITS | MTU_CRn_ENA,
    mtu_base + MTU_CR(1));
    writel(1 << 1, mtu_base + MTU_IMSC);
    } else {
// Generate an interrupt to start the clockevent again
    (void) nmdk_clkevt_next(nmdk_cycle, core::ptr::null_mut());
    }
    }
#[no_mangle]
unsafe extern "C" fn nmdk_clkevt_shutdown(evt: *mut clock_event_device) -> c_int {
    static int nmdk_clkevt_shutdown(struct clock_event_device *evt)
    {
    writel(0, mtu_base + MTU_IMSC);
// disable timer
    writel(0, mtu_base + MTU_CR(1));
// load some high default value
    writel(0xffffffff, mtu_base + MTU_LR(1));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nmdk_clkevt_set_oneshot(evt: *mut clock_event_device) -> c_int {
    static int nmdk_clkevt_set_oneshot(struct clock_event_device *evt)
    {
    clkevt_periodic = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nmdk_clkevt_set_periodic(evt: *mut clock_event_device) -> c_int {
    static int nmdk_clkevt_set_periodic(struct clock_event_device *evt)
    {
    clkevt_periodic = true;
    nmdk_clkevt_reset();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nmdk_clksrc_reset() {
    static void nmdk_clksrc_reset(void)
    {
// Disable
    writel(0, mtu_base + MTU_CR(0));
// ClockSource: configure load and background-load, and fire it up
    writel(nmdk_cycle, mtu_base + MTU_LR(0));
    writel(nmdk_cycle, mtu_base + MTU_BGLR(0));
    writel(clk_prescale | MTU_CRn_32BITS | MTU_CRn_ENA,
    mtu_base + MTU_CR(0));
    }
#[no_mangle]
unsafe extern "C" fn nmdk_clkevt_resume(cedev: *mut clock_event_device) {
    static void nmdk_clkevt_resume(struct clock_event_device *cedev)
    {
    nmdk_clkevt_reset();
    nmdk_clksrc_reset();
    }
    static struct clock_event_device nmdk_clkevt = {
    .name			= "mtu_1",
    .features		= CLOCK_EVT_FEAT_ONESHOT |
    CLOCK_EVT_FEAT_PERIODIC |
    CLOCK_EVT_FEAT_DYNIRQ,
    .rating			= 200,
    .set_state_shutdown	= nmdk_clkevt_shutdown,
    .set_state_periodic	= nmdk_clkevt_set_periodic,
    .set_state_oneshot	= nmdk_clkevt_set_oneshot,
    .set_next_event		= nmdk_clkevt_next,
    .resume			= nmdk_clkevt_resume,
    };
//
// IRQ Handler for timer 1 of the MTU block.
//
#[no_mangle]
unsafe extern "C" fn nmdk_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t nmdk_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evdev = dev_id;
    writel(1 << 1, mtu_base + MTU_ICR); /* Interrupt clear reg */
    evdev.event_handler(evdev);
    return IRQ_HANDLED;
    }
    static int __init nmdk_timer_init(void __iomem *base, int irq,
    struct clk *pclk, struct clk *clk)
    {
    unsigned long rate;
    int ret;
    int min_ticks;
    mtu_base = base;
    BUG_ON(clk_prepare_enable(pclk));
    BUG_ON(clk_prepare_enable(clk));
//
// Tick rate is 2.4MHz for Nomadik and 2.4Mhz, 100MHz or 133 MHz
// for ux500, and in one specific Ux500 case 32768 Hz.
//
// Use a divide-by-16 counter if the tick rate is more than 32MHz.
// At 32 MHz, the timer (with 32 bit counter) can be programmed
// to wake-up at a max 127s a head in time. Dividing a 2.4 MHz timer
// with 16 gives too low timer resolution.
//
    rate = clk_get_rate(clk);
    if (rate > 32000000) {
    rate /= 16;
    clk_prescale = MTU_CRn_PRESCALE_16;
    } else {
    clk_prescale = MTU_CRn_PRESCALE_1;
    }
// Cycles for periodic mode
    nmdk_cycle = DIV_ROUND_CLOSEST(rate, HZ);
// Timer 0 is the free running clocksource
    nmdk_clksrc_reset();
    ret = clocksource_mmio_init(mtu_base + MTU_VAL(0), "mtu_0",
    rate, 200, 32, clocksource_mmio_readl_down);
    if (ret) {
    pr_err("timer: failed to initialize clock source %s\n", "mtu_0");
    return ret;
    }
    sched_clock_register(nomadik_read_sched_clock, 32, rate);
// Timer 1 is used for events, register irq and clockevents
    if (request_irq(irq, nmdk_timer_interrupt, IRQF_TIMER,
    "Nomadik Timer Tick", &nmdk_clkevt))
    pr_err("%s: request_irq() failed\n", "Nomadik Timer Tick");
    nmdk_clkevt.cpumask = cpumask_of(0);
    nmdk_clkevt.irq = irq;
    if (rate < 100000)
    min_ticks = 5;
    else
    min_ticks = 2;
    clockevents_config_and_register(&nmdk_clkevt, rate, min_ticks,
    0xffffffffU);
    mtu_delay_timer.read_current_timer = &nmdk_timer_read_current_timer;
    mtu_delay_timer.freq = rate;
    register_current_timer_delay(&mtu_delay_timer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nmdk_timer_of_init(node: *mut device_node) -> int __init {
    static int __init nmdk_timer_of_init(struct device_node *node)
    {
    struct clk *pclk;
    struct clk *clk;
    void __iomem *base;
    int irq;
    base = of_iomap(node, 0);
    if (!base) {
    pr_err("Can't remap registers\n");
    return -ENXIO;
    }
    pclk = of_clk_get_by_name(node, "apb_pclk");
    if (IS_ERR(pclk)) {
    pr_err("could not get apb_pclk\n");
    return PTR_ERR(pclk);
    }
    clk = of_clk_get_by_name(node, "timclk");
    if (IS_ERR(clk)) {
    pr_err("could not get timclk\n");
    return PTR_ERR(clk);
    }
    irq = irq_of_parse_and_map(node, 0);
    if (irq <= 0) {
    pr_err("Can't parse IRQ\n");
    return -EINVAL;
    }
    return nmdk_timer_init(base, irq, pclk, clk);
    }
    TIMER_OF_DECLARE(nomadik_mtu, "st,nomadik-mtu",
    nmdk_timer_of_init);
