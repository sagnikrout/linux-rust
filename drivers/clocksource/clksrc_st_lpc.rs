//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/clksrc_st_lpc.c
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
// Clocksource using the Low Power Timer found in the Low Power Controller (LPC)
//
// Copyright (C) 2015 STMicroelectronics – All Rights Reserved
//
// Author(s): Francesco Virlinzi <francesco.virlinzi@st.com>
// Ajit Pal Singh <ajitpal.singh@st.com>
//

// Low Power Timer
pub const LPC_LPT_LSB_OFF: c_uint = 0x400;
pub const LPC_LPT_MSB_OFF: c_uint = 0x404;
pub const LPC_LPT_START_OFF: c_uint = 0x408;
    static struct st_clksrc_ddata {
    struct clk		*clk;
    void __iomem		*base;
    } ddata;
#[no_mangle]
unsafe extern "C" fn st_clksrc_reset() -> void __init {
    static void __init st_clksrc_reset(void)
    {
    writel_relaxed(0, ddata.base + LPC_LPT_START_OFF);
    writel_relaxed(0, ddata.base + LPC_LPT_MSB_OFF);
    writel_relaxed(0, ddata.base + LPC_LPT_LSB_OFF);
    writel_relaxed(1, ddata.base + LPC_LPT_START_OFF);
    }
#[no_mangle]
unsafe extern "C" fn st_clksrc_sched_clock_read() -> u64 notrace {
    static u64 notrace st_clksrc_sched_clock_read(void)
    {
    return (u64)readl_relaxed(ddata.base + LPC_LPT_LSB_OFF);
    }
#[no_mangle]
unsafe extern "C" fn st_clksrc_init() -> int __init {
    static int __init st_clksrc_init(void)
    {
    unsigned long rate;
    int ret;
    st_clksrc_reset();
    rate = clk_get_rate(ddata.clk);
    sched_clock_register(st_clksrc_sched_clock_read, 32, rate);
    ret = clocksource_mmio_init(ddata.base + LPC_LPT_LSB_OFF,
    "clksrc-st-lpc", rate, 300, 32,
    clocksource_mmio_readl_up);
    if (ret) {
    pr_err("clksrc-st-lpc: Failed to register clocksource\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_clksrc_setup_clk(np: *mut device_node) -> int __init {
    static int __init st_clksrc_setup_clk(struct device_node *np)
    {
    struct clk *clk;
    clk = of_clk_get(np, 0);
    if (IS_ERR(clk)) {
    pr_err("clksrc-st-lpc: Failed to get LPC clock\n");
    return PTR_ERR(clk);
    }
    if (clk_prepare_enable(clk)) {
    pr_err("clksrc-st-lpc: Failed to enable LPC clock\n");
    return -EINVAL;
    }
    if (!clk_get_rate(clk)) {
    pr_err("clksrc-st-lpc: Failed to get LPC clock rate\n");
    clk_disable_unprepare(clk);
    return -EINVAL;
    }
    ddata.clk = clk;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_clksrc_of_register(np: *mut device_node) -> int __init {
    static int __init st_clksrc_of_register(struct device_node *np)
    {
    int ret;
    uint32_t mode;
    ret = of_property_read_u32(np, "st,lpc-mode", &mode);
    if (ret) {
    pr_err("clksrc-st-lpc: An LPC mode must be provided\n");
    return ret;
    }
// LPC can either run as a Clocksource or in RTC or WDT mode
    if (mode != ST_LPC_MODE_CLKSRC)
    return 0;
    ddata.base = of_iomap(np, 0);
    if (!ddata.base) {
    pr_err("clksrc-st-lpc: Unable to map iomem\n");
    return -ENXIO;
    }
    ret = st_clksrc_setup_clk(np);
    if (ret) {
    iounmap(ddata.base);
    return ret;
    }
    ret = st_clksrc_init();
    if (ret) {
    clk_disable_unprepare(ddata.clk);
    clk_put(ddata.clk);
    iounmap(ddata.base);
    return ret;
    }
    pr_info("clksrc-st-lpc: clocksource initialised - running @ %luHz\n",
    clk_get_rate(ddata.clk));
    return ret;
    }
    TIMER_OF_DECLARE(ddata, "st,stih407-lpc", st_clksrc_of_register);
