//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-stm32-lp.c
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
// Copyright (C) STMicroelectronics 2019 - All Rights Reserved
// Authors: Benjamin Gaignard <benjamin.gaignard@st.com> for STMicroelectronics.
// Pascal Paillet <p.paillet@st.com> for STMicroelectronics.
//

pub const CFGR_PSC_OFFSET: c_int = 9;
pub const STM32_LP_RATING: c_int = 1000;

pub const STM32_LP_MAX_PSC: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_lp_private {
    pub reg: *mut regmap,
    pub clkevt: clock_event_device,
    pub period: c_ulong,
    pub psc: u32,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub version: u32,
}

    static struct stm32_lp_private*
    to_priv(struct clock_event_device *clkevt)
    {
    return container_of(clkevt, struct stm32_lp_private, clkevt);
    }
#[no_mangle]
unsafe extern "C" fn stm32_clkevent_lp_shutdown(clkevt: *mut clock_event_device) -> c_int {
    static int stm32_clkevent_lp_shutdown(struct clock_event_device *clkevt)
    {
    struct stm32_lp_private *priv = to_priv(clkevt);
    regmap_write(priv.reg, STM32_LPTIM_CR, 0);
    regmap_write(priv.reg, STM32_LPTIM_IER, 0);
// clear pending flags
    regmap_write(priv.reg, STM32_LPTIM_ICR, STM32_LPTIM_ARRMCF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32mp25_clkevent_lp_set_evt(priv: *mut stm32_lp_private, evt: c_ulong) -> c_int {
    static int stm32mp25_clkevent_lp_set_evt(struct stm32_lp_private *priv, unsigned long evt)
    {
    int ret;
    u32 val;
    regmap_read(priv.reg, STM32_LPTIM_CR, &val);
    if (!FIELD_GET(STM32_LPTIM_ENABLE, val)) {
// Enable LPTIMER to be able to write into IER and ARR registers
    regmap_write(priv.reg, STM32_LPTIM_CR, STM32_LPTIM_ENABLE);
//
// After setting the ENABLE bit, a delay of two counter clock cycles is needed
// before the LPTIM is actually enabled. For 32KHz rate, this makes approximately
// 62.5 micro-seconds, round it up.
//
    udelay(63);
    }
// set next event counter
    regmap_write(priv.reg, STM32_LPTIM_ARR, evt);
// enable ARR interrupt
    regmap_write(priv.reg, STM32_LPTIM_IER, STM32_LPTIM_ARRMIE);
// Poll DIEROK and ARROK to ensure register access has completed
    ret = regmap_read_poll_timeout_atomic(priv.reg, STM32_LPTIM_ISR, val,
    (val & STM32_LPTIM_DIEROK_ARROK) ==
    STM32_LPTIM_DIEROK_ARROK,
    10, 500);
    if (ret) {
    dev_err(priv.dev, "access to LPTIM timed out\n");
// Disable LPTIMER
    regmap_write(priv.reg, STM32_LPTIM_CR, 0);
    return ret;
    }
// Clear DIEROK and ARROK flags
    regmap_write(priv.reg, STM32_LPTIM_ICR, STM32_LPTIM_DIEROKCF_ARROKCF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_clkevent_lp_set_evt(priv: *mut stm32_lp_private, evt: c_ulong) {
    static void stm32_clkevent_lp_set_evt(struct stm32_lp_private *priv, unsigned long evt)
    {
// disable LPTIMER to be able to write into IER register
    regmap_write(priv.reg, STM32_LPTIM_CR, 0);
// enable ARR interrupt
    regmap_write(priv.reg, STM32_LPTIM_IER, STM32_LPTIM_ARRMIE);
// enable LPTIMER to be able to write into ARR register
    regmap_write(priv.reg, STM32_LPTIM_CR, STM32_LPTIM_ENABLE);
// set next event counter
    regmap_write(priv.reg, STM32_LPTIM_ARR, evt);
    }
    static int stm32_clkevent_lp_set_timer(unsigned long evt,
    struct clock_event_device *clkevt,
    int is_periodic)
    {
    struct stm32_lp_private *priv = to_priv(clkevt);
    int ret;
    if (priv.version == STM32_LPTIM_VERR_23) {
    ret = stm32mp25_clkevent_lp_set_evt(priv, evt);
    if (ret)
    return ret;
    } else {
    stm32_clkevent_lp_set_evt(priv, evt);
    }
// start counter
    if (is_periodic)
    regmap_write(priv.reg, STM32_LPTIM_CR,
    STM32_LPTIM_CNTSTRT | STM32_LPTIM_ENABLE);
    else
    regmap_write(priv.reg, STM32_LPTIM_CR,
    STM32_LPTIM_SNGSTRT | STM32_LPTIM_ENABLE);
    return 0;
    }
    static int stm32_clkevent_lp_set_next_event(unsigned long evt,
    struct clock_event_device *clkevt)
    {
    return stm32_clkevent_lp_set_timer(evt, clkevt,
    clockevent_state_periodic(clkevt));
    }
#[no_mangle]
unsafe extern "C" fn stm32_clkevent_lp_set_periodic(clkevt: *mut clock_event_device) -> c_int {
    static int stm32_clkevent_lp_set_periodic(struct clock_event_device *clkevt)
    {
    struct stm32_lp_private *priv = to_priv(clkevt);
    return stm32_clkevent_lp_set_timer(priv.period, clkevt, true);
    }
#[no_mangle]
unsafe extern "C" fn stm32_clkevent_lp_set_oneshot(clkevt: *mut clock_event_device) -> c_int {
    static int stm32_clkevent_lp_set_oneshot(struct clock_event_device *clkevt)
    {
    struct stm32_lp_private *priv = to_priv(clkevt);
    return stm32_clkevent_lp_set_timer(priv.period, clkevt, false);
    }
#[no_mangle]
unsafe extern "C" fn stm32_clkevent_lp_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32_clkevent_lp_irq_handler(int irq, void *dev_id)
    {
    struct clock_event_device *clkevt = (struct clock_event_device *)dev_id;
    struct stm32_lp_private *priv = to_priv(clkevt);
    regmap_write(priv.reg, STM32_LPTIM_ICR, STM32_LPTIM_ARRMCF);
    if (clkevt.event_handler)
    clkevt.event_handler(clkevt);
    return IRQ_HANDLED;
    }
    static void stm32_clkevent_lp_set_prescaler(struct stm32_lp_private *priv,
    unsigned long *rate)
    {
    int i;
    for (i = 0; i <= STM32_LP_MAX_PSC; i++) {
    if (DIV_ROUND_CLOSEST(*rate, 1 << i) < STM32_TARGET_CLKRATE)
    break;
    }
    regmap_write(priv.reg, STM32_LPTIM_CFGR, i << CFGR_PSC_OFFSET);
// Adjust rate and period given the prescaler value
// rate = DIV_ROUND_CLOSEST(*rate, (1 << i));
    priv.period = DIV_ROUND_UP(*rate, HZ);
    priv.psc = i;
    }
#[no_mangle]
unsafe extern "C" fn stm32_clkevent_lp_suspend(clkevt: *mut clock_event_device) {
    static void stm32_clkevent_lp_suspend(struct clock_event_device *clkevt)
    {
    struct stm32_lp_private *priv = to_priv(clkevt);
    stm32_clkevent_lp_shutdown(clkevt);
// balance clk_prepare_enable() from the probe
    clk_disable_unprepare(priv.clk);
    }
#[no_mangle]
unsafe extern "C" fn stm32_clkevent_lp_resume(clkevt: *mut clock_event_device) {
    static void stm32_clkevent_lp_resume(struct clock_event_device *clkevt)
    {
    struct stm32_lp_private *priv = to_priv(clkevt);
    clk_prepare_enable(priv.clk);
// restore prescaler
    regmap_write(priv.reg, STM32_LPTIM_CFGR, priv.psc << CFGR_PSC_OFFSET);
    }
    static void stm32_clkevent_lp_init(struct stm32_lp_private *priv,
    struct device_node *np, unsigned long rate)
    {
    priv.clkevt.name = np.full_name;
    priv.clkevt.cpumask = cpu_possible_mask;
    priv.clkevt.features = CLOCK_EVT_FEAT_PERIODIC |
    CLOCK_EVT_FEAT_ONESHOT;
    priv.clkevt.set_state_shutdown = stm32_clkevent_lp_shutdown;
    priv.clkevt.set_state_periodic = stm32_clkevent_lp_set_periodic;
    priv.clkevt.set_state_oneshot = stm32_clkevent_lp_set_oneshot;
    priv.clkevt.set_next_event = stm32_clkevent_lp_set_next_event;
    priv.clkevt.rating = STM32_LP_RATING;
    priv.clkevt.suspend = stm32_clkevent_lp_suspend;
    priv.clkevt.resume = stm32_clkevent_lp_resume;
    priv.clkevt.owner = THIS_MODULE;
    clockevents_config_and_register(&priv.clkevt, rate, 0x1,
    STM32_LPTIM_MAX_ARR);
    }
#[no_mangle]
unsafe extern "C" fn stm32_clkevent_lp_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_clkevent_lp_probe(struct platform_device *pdev)
    {
    struct stm32_lptimer *ddata = dev_get_drvdata(pdev.dev.parent);
    struct stm32_lp_private *priv;
    unsigned long rate;
    int ret, irq;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.reg = ddata.regmap;
    priv.version = ddata.version;
    priv.clk = ddata.clk;
    ret = clk_prepare_enable(priv.clk);
    if (ret)
    return -EINVAL;
    rate = clk_get_rate(priv.clk);
    if (!rate) {
    ret = -EINVAL;
    goto out_clk_disable;
    }
    irq = platform_get_irq(to_platform_device(pdev.dev.parent), 0);
    if (irq <= 0) {
    ret = irq;
    goto out_clk_disable;
    }
    if (of_property_read_bool(pdev.dev.parent.of_node, "wakeup-source")) {
    device_set_wakeup_capable(&pdev.dev, true);
    ret = dev_pm_set_wake_irq(&pdev.dev, irq);
    if (ret)
    goto out_clk_disable;
    }
    ret = devm_request_irq(&pdev.dev, irq, stm32_clkevent_lp_irq_handler,
    IRQF_TIMER, pdev.name, &priv.clkevt);
    if (ret)
    goto out_clk_disable;
    stm32_clkevent_lp_set_prescaler(priv, &rate);
    stm32_clkevent_lp_init(priv, pdev.dev.parent.of_node, rate);
    priv.dev = &pdev.dev;
    return 0;
    out_clk_disable:
    clk_disable_unprepare(priv.clk);
    return ret;
    }
    static const struct of_device_id stm32_clkevent_lp_of_match[] = {
    { .compatible = "st,stm32-lptimer-timer", },
    {},
    };
    MODULE_DEVICE_TABLE(of, stm32_clkevent_lp_of_match);
    static struct platform_driver stm32_clkevent_lp_driver = {
    .probe  = stm32_clkevent_lp_probe,
    .driver	= {
    .name = "stm32-lptimer-timer",
    .of_match_table = stm32_clkevent_lp_of_match,
    .suppress_bind_attrs = true,
    },
    };
    module_platform_driver(stm32_clkevent_lp_driver);
    MODULE_DESCRIPTION("STMicroelectronics STM32 clockevent low power driver");
