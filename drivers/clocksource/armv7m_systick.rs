//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/armv7m_systick.c
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
// Copyright (C) Maxime Coquelin 2015
// Author:  Maxime Coquelin <mcoquelin.stm32@gmail.com>
//

pub const SYST_CSR: c_uint = 0x00;
pub const SYST_RVR: c_uint = 0x04;
pub const SYST_CVR: c_uint = 0x08;
pub const SYST_CALIB: c_uint = 0x0c;

pub const SYSTICK_LOAD_RELOAD_MASK: c_uint = 0x00FFFFFF;
#[no_mangle]
unsafe extern "C" fn system_timer_of_register(np: *mut device_node) -> int __init {
    static int __init system_timer_of_register(struct device_node *np)
    {
    struct clk *clk = core::ptr::null_mut();
    void __iomem *base;
    u32 rate;
    int ret;
    base = of_iomap(np, 0);
    if (!base) {
    pr_warn("system-timer: invalid base address\n");
    return -ENXIO;
    }
    ret = of_property_read_u32(np, "clock-frequency", &rate);
    if (ret) {
    clk = of_clk_get(np, 0);
    if (IS_ERR(clk)) {
    ret = PTR_ERR(clk);
    goto out_unmap;
    }
    ret = clk_prepare_enable(clk);
    if (ret)
    goto out_clk_put;
    rate = clk_get_rate(clk);
    if (!rate) {
    ret = -EINVAL;
    goto out_clk_disable;
    }
    }
    writel_relaxed(SYSTICK_LOAD_RELOAD_MASK, base + SYST_RVR);
    writel_relaxed(SYST_CSR_ENABLE, base + SYST_CSR);
    ret = clocksource_mmio_init(base + SYST_CVR, "arm_system_timer", rate,
    200, 24, clocksource_mmio_readl_down);
    if (ret) {
    pr_err("failed to init clocksource (%d)\n", ret);
    if (clk)
    goto out_clk_disable;
    else
    goto out_unmap;
    }
    pr_info("ARM System timer initialized as clocksource\n");
    return 0;
    out_clk_disable:
    clk_disable_unprepare(clk);
    out_clk_put:
    clk_put(clk);
    out_unmap:
    iounmap(base);
    pr_warn("ARM System timer register failed (%d)\n", ret);
    return ret;
    }
    TIMER_OF_DECLARE(arm_systick, "arm,armv7m-systick",
    system_timer_of_register);
