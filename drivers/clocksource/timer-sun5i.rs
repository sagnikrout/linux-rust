//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-sun5i.c
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
// Allwinner SoCs hstimer driver.
//
// Copyright (C) 2013 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

pub const TIMER_IRQ_EN_REG: c_uint = 0x00;

pub const TIMER_IRQ_ST_REG: c_uint = 0x04;

pub const TIMER_SYNC_TICKS: c_int = 3;
//
// struct sunxi_timer_quirks - Differences between SoC variants.
//
// @from_ctl_base_offset: offset applied from ctl register onwards
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_timer_quirks {
    pub from_ctl_base_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun5i_timer {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub clk_rate_cb: notifier_block,
    pub ticks_per_jiffy: u32,
    pub clksrc: clocksource,
    pub clkevt: clock_event_device,
    pub quirks: *const sunxi_timer_quirks,
}

    container_of(x, struct sun5i_timer, clk_rate_cb)

    container_of(x, struct sun5i_timer, clksrc)

    container_of(x, struct sun5i_timer, clkevt)
//
// When we disable a timer, we need to wait at least for 2 cycles of
// the timer source clock. We will use for that the clocksource timer
// that is already setup and runs at the same frequency than the other
// timers, and we never will be disabled.
//
#[no_mangle]
unsafe extern "C" fn sun5i_clkevt_sync(ce: *mut sun5i_timer) {
    static void sun5i_clkevt_sync(struct sun5i_timer *ce)
    {
    let mut offset: u32 = ce.quirks.from_ctl_base_offset;
    let mut old: u32 = readl(ce.base + TIMER_CNTVAL_LO_REG(1, offset));
    while ((old - readl(ce.base + TIMER_CNTVAL_LO_REG(1, offset))) <
    TIMER_SYNC_TICKS)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn sun5i_clkevt_time_stop(ce: *mut sun5i_timer, timer: u8) {
    static void sun5i_clkevt_time_stop(struct sun5i_timer *ce, u8 timer)
    {
    let mut offset: u32 = ce.quirks.from_ctl_base_offset;
    let mut val: u32 = readl(ce.base + TIMER_CTL_REG(timer, offset));
    writel(val & ~TIMER_CTL_ENABLE,
    ce.base + TIMER_CTL_REG(timer, offset));
    sun5i_clkevt_sync(ce);
    }
#[no_mangle]
unsafe extern "C" fn sun5i_clkevt_time_setup(ce: *mut sun5i_timer, timer: u8, delay: u32) {
    static void sun5i_clkevt_time_setup(struct sun5i_timer *ce, u8 timer, u32 delay)
    {
    let mut offset: u32 = ce.quirks.from_ctl_base_offset;
    writel(delay, ce.base + TIMER_INTVAL_LO_REG(timer, offset));
    }
#[no_mangle]
unsafe extern "C" fn sun5i_clkevt_time_start(ce: *mut sun5i_timer, timer: u8, periodic: bool) {
    static void sun5i_clkevt_time_start(struct sun5i_timer *ce, u8 timer, bool periodic)
    {
    let mut offset: u32 = ce.quirks.from_ctl_base_offset;
    let mut val: u32 = readl(ce.base + TIMER_CTL_REG(timer, offset));
    if (periodic)
    val &= ~TIMER_CTL_ONESHOT;
    else
    val |= TIMER_CTL_ONESHOT;
    writel(val | TIMER_CTL_ENABLE | TIMER_CTL_RELOAD,
    ce.base + TIMER_CTL_REG(timer, offset));
    }
#[no_mangle]
unsafe extern "C" fn sun5i_clkevt_shutdown(clkevt: *mut clock_event_device) -> c_int {
    static int sun5i_clkevt_shutdown(struct clock_event_device *clkevt)
    {
    struct sun5i_timer *ce = clkevt_to_sun5i_timer(clkevt);
    sun5i_clkevt_time_stop(ce, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun5i_clkevt_set_oneshot(clkevt: *mut clock_event_device) -> c_int {
    static int sun5i_clkevt_set_oneshot(struct clock_event_device *clkevt)
    {
    struct sun5i_timer *ce = clkevt_to_sun5i_timer(clkevt);
    sun5i_clkevt_time_stop(ce, 0);
    sun5i_clkevt_time_start(ce, 0, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun5i_clkevt_set_periodic(clkevt: *mut clock_event_device) -> c_int {
    static int sun5i_clkevt_set_periodic(struct clock_event_device *clkevt)
    {
    struct sun5i_timer *ce = clkevt_to_sun5i_timer(clkevt);
    sun5i_clkevt_time_stop(ce, 0);
    sun5i_clkevt_time_setup(ce, 0, ce.ticks_per_jiffy);
    sun5i_clkevt_time_start(ce, 0, true);
    return 0;
    }
    static int sun5i_clkevt_next_event(unsigned long evt,
    struct clock_event_device *clkevt)
    {
    struct sun5i_timer *ce = clkevt_to_sun5i_timer(clkevt);
    sun5i_clkevt_time_stop(ce, 0);
    sun5i_clkevt_time_setup(ce, 0, evt - TIMER_SYNC_TICKS);
    sun5i_clkevt_time_start(ce, 0, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun5i_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sun5i_timer_interrupt(int irq, void *dev_id)
    {
    struct sun5i_timer *ce = dev_id;
    writel(0x1, ce.base + TIMER_IRQ_ST_REG);
    ce.clkevt.event_handler(&ce.clkevt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sun5i_clksrc_read(clksrc: *mut clocksource) -> u64 {
    static u64 sun5i_clksrc_read(struct clocksource *clksrc)
    {
    struct sun5i_timer *cs = clksrc_to_sun5i_timer(clksrc);
    let mut offset: u32 = cs.quirks.from_ctl_base_offset;
    return ~readl(cs.base + TIMER_CNTVAL_LO_REG(1, offset));
    }
    static int sun5i_rate_cb(struct notifier_block *nb,
    unsigned long event, void *data)
    {
    struct clk_notifier_data *ndata = data;
    struct sun5i_timer *cs = nb_to_sun5i_timer(nb);
    switch (event) {
    case PRE_RATE_CHANGE:
    clocksource_unregister(&cs.clksrc);
    break;
    case POST_RATE_CHANGE:
    clocksource_register_hz(&cs.clksrc, ndata.new_rate);
    clockevents_update_freq(&cs.clkevt, ndata.new_rate);
    cs.ticks_per_jiffy = DIV_ROUND_UP(ndata.new_rate, HZ);
    break;
    default:
    break;
    }
    return NOTIFY_DONE;
    }
    static int sun5i_setup_clocksource(struct platform_device *pdev,
    unsigned long rate)
    {
    struct sun5i_timer *cs = platform_get_drvdata(pdev);
    let mut offset: u32 = cs.quirks.from_ctl_base_offset;
    void __iomem *base = cs.base;
    int ret;
    writel(~0, base + TIMER_INTVAL_LO_REG(1, offset));
    writel(TIMER_CTL_ENABLE | TIMER_CTL_RELOAD,
    base + TIMER_CTL_REG(1, offset));
    cs.clksrc.name = pdev.dev.of_node.name;
    cs.clksrc.rating = 340;
    cs.clksrc.read = sun5i_clksrc_read;
    cs.clksrc.mask = CLOCKSOURCE_MASK(32);
    cs.clksrc.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    cs.clksrc.owner = THIS_MODULE;
    ret = clocksource_register_hz(&cs.clksrc, rate);
    if (ret) {
    dev_err(&pdev.dev, "Couldn't register clock source.\n");
    return ret;
    }
    return 0;
    }
    static int sun5i_setup_clockevent(struct platform_device *pdev,
    unsigned long rate, int irq)
    {
    struct device *dev = &pdev.dev;
    struct sun5i_timer *ce = platform_get_drvdata(pdev);
    void __iomem *base = ce.base;
    int ret;
    u32 val;
    ce.clkevt.name = dev.of_node.name;
    ce.clkevt.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    ce.clkevt.set_next_event = sun5i_clkevt_next_event;
    ce.clkevt.set_state_shutdown = sun5i_clkevt_shutdown;
    ce.clkevt.set_state_periodic = sun5i_clkevt_set_periodic;
    ce.clkevt.set_state_oneshot = sun5i_clkevt_set_oneshot;
    ce.clkevt.tick_resume = sun5i_clkevt_shutdown;
    ce.clkevt.rating = 340;
    ce.clkevt.irq = irq;
    ce.clkevt.cpumask = cpu_possible_mask;
    ce.clkevt.owner = THIS_MODULE;
// Enable timer0 interrupt
    val = readl(base + TIMER_IRQ_EN_REG);
    writel(val | TIMER_IRQ_EN(0), base + TIMER_IRQ_EN_REG);
    clockevents_config_and_register(&ce.clkevt, rate,
    TIMER_SYNC_TICKS, 0xffffffff);
    ret = devm_request_irq(dev, irq, sun5i_timer_interrupt,
    IRQF_TIMER | IRQF_IRQPOLL,
    "sun5i_timer0", ce);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun5i_timer_probe(pdev: *mut platform_device) -> c_int {
    static int sun5i_timer_probe(struct platform_device *pdev)
    {
    const struct sunxi_timer_quirks *quirks;
    struct device *dev = &pdev.dev;
    struct sun5i_timer *st;
    struct reset_control *rstc;
    void __iomem *timer_base;
    struct clk *clk;
    unsigned long rate;
    int irq, ret;
    st = devm_kzalloc(dev, sizeof(*st), GFP_KERNEL);
    if (!st)
    return -ENOMEM;
    platform_set_drvdata(pdev, st);
    timer_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(timer_base)) {
    dev_err(dev, "Can't map registers\n");
    return PTR_ERR(timer_base);
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk)) {
    dev_err(dev, "Can't get timer clock\n");
    return PTR_ERR(clk);
    }
    rate = clk_get_rate(clk);
    if (!rate) {
    dev_err(dev, "Couldn't get parent clock rate\n");
    return -EINVAL;
    }
    quirks = of_device_get_match_data(&pdev.dev);
    if (!quirks) {
    dev_err(&pdev.dev, "Failed to determine the quirks to use\n");
    return -ENODEV;
    }
    st.base = timer_base;
    st.ticks_per_jiffy = DIV_ROUND_UP(rate, HZ);
    st.clk = clk;
    st.clk_rate_cb.notifier_call = sun5i_rate_cb;
    st.clk_rate_cb.next = core::ptr::null_mut();
    st.quirks = quirks;
    ret = devm_clk_notifier_register(dev, clk, &st.clk_rate_cb);
    if (ret) {
    dev_err(dev, "Unable to register clock notifier.\n");
    return ret;
    }
    rstc = devm_reset_control_get_optional_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(rstc))
    return dev_err_probe(dev, PTR_ERR(rstc),
    "failed to get reset\n");
    if (rstc)
    reset_control_deassert(rstc);
    ret = sun5i_setup_clocksource(pdev, rate);
    if (ret)
    return ret;
    ret = sun5i_setup_clockevent(pdev, rate, irq);
    if (ret)
    goto err_unreg_clocksource;
    return 0;
    err_unreg_clocksource:
    clocksource_unregister(&st.clksrc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sun5i_timer_remove(pdev: *mut platform_device) {
    static void sun5i_timer_remove(struct platform_device *pdev)
    {
    struct sun5i_timer *st = platform_get_drvdata(pdev);
    clocksource_unregister(&st.clksrc);
    }
    static const struct sunxi_timer_quirks sun5i_sun7i_hstimer_quirks = {
    .from_ctl_base_offset = 0x0,
    };
    static const struct sunxi_timer_quirks sun20i_d1_hstimer_quirks = {
    .from_ctl_base_offset = 0x10,
    };
    static const struct of_device_id sun5i_timer_of_match[] = {
    {
    .compatible = "allwinner,sun5i-a13-hstimer",
    .data = &sun5i_sun7i_hstimer_quirks,
    },
    {
    .compatible = "allwinner,sun7i-a20-hstimer",
    .data = &sun5i_sun7i_hstimer_quirks,
    },
    {
    .compatible = "allwinner,sun20i-d1-hstimer",
    .data = &sun20i_d1_hstimer_quirks,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, sun5i_timer_of_match);
    static struct platform_driver sun5i_timer_driver = {
    .probe		= sun5i_timer_probe,
    .remove		= sun5i_timer_remove,
    .driver	= {
    .name	= "sun5i-timer",
    .of_match_table = sun5i_timer_of_match,
    .suppress_bind_attrs = true,
    },
    };
    module_platform_driver(sun5i_timer_driver);
