//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-clps711x.c
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
// Cirrus Logic CLPS711X CLK driver
//
// Copyright (C) 2014 Alexander Shiyan <shc_work@mail.ru>
//

    static const struct clk_div_table spi_div_table[] = {
    { .val = 0, .div = 32, },
    { .val = 1, .div = 8, },
    { .val = 2, .div = 2, },
    { .val = 3, .div = 1, },
    { /* sentinel */ }
    };
    static const struct clk_div_table timer_div_table[] = {
    { .val = 0, .div = 256, },
    { .val = 1, .div = 1, },
    { /* sentinel */ }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clps711x_clk {
    pub lock: spinlock_t,
    pub clk_data: clk_hw_onecell_data,
}

#[no_mangle]
unsafe extern "C" fn clps711x_clk_init_dt(np: *mut device_node) -> void __init {
    static void __init clps711x_clk_init_dt(struct device_node *np)
    {
    u32 tmp, f_cpu, f_pll, f_bus, f_tim, f_pwm, f_spi, fref = 0;
    struct clps711x_clk *clps711x_clk;
    void __iomem *base;
    WARN_ON(of_property_read_u32(np, "startup-frequency", &fref));
    base = of_iomap(np, 0);
    BUG_ON(!base);
    clps711x_clk = kzalloc_flex(*clps711x_clk, clk_data.hws,
    CLPS711X_CLK_MAX);
    BUG_ON(!clps711x_clk);
    spin_lock_init(&clps711x_clk.lock);
// Read PLL multiplier value and sanity check
    tmp = readl(base + CLPS711X_PLLR) >> 24;
    if (((tmp >= 10) && (tmp <= 50)) || !fref)
    f_pll = DIV_ROUND_UP(CLPS711X_OSC_FREQ * tmp, 2);
    else
    f_pll = fref;
    tmp = readl(base + CLPS711X_SYSFLG2);
    if (tmp & SYSFLG2_CKMODE) {
    f_cpu = CLPS711X_EXT_FREQ;
    f_bus = CLPS711X_EXT_FREQ;
    f_spi = DIV_ROUND_CLOSEST(CLPS711X_EXT_FREQ, 96);
    f_pll = 0;
    f_pwm = DIV_ROUND_CLOSEST(CLPS711X_EXT_FREQ, 128);
    } else {
    f_cpu = f_pll;
    if (f_cpu > 36864000)
    f_bus = DIV_ROUND_UP(f_cpu, 2);
    else
    f_bus = 36864000 / 2;
    f_spi = DIV_ROUND_CLOSEST(f_cpu, 576);
    f_pwm = DIV_ROUND_CLOSEST(f_cpu, 768);
    }
    if (tmp & SYSFLG2_CKMODE) {
    if (readl(base + CLPS711X_SYSCON2) & SYSCON2_OSTB)
    f_tim = DIV_ROUND_CLOSEST(CLPS711X_EXT_FREQ, 26);
    else
    f_tim = DIV_ROUND_CLOSEST(CLPS711X_EXT_FREQ, 24);
    } else
    f_tim = DIV_ROUND_CLOSEST(f_cpu, 144);
    tmp = readl(base + CLPS711X_SYSCON1);
// Timer1 in free running mode.
// Counter will wrap around to 0xffff when it underflows
// and will continue to count down.
//
    tmp &= ~(SYSCON1_TC1M | SYSCON1_TC1S);
// Timer2 in prescale mode.
// Value written is automatically re-loaded when
// the counter underflows.
//
    tmp |= SYSCON1_TC2M | SYSCON1_TC2S;
    writel(tmp, base + CLPS711X_SYSCON1);
    clps711x_clk.clk_data.hws[CLPS711X_CLK_DUMMY] =
    clk_hw_register_fixed_rate(core::ptr::null_mut(), "dummy", core::ptr::null_mut(), 0, 0);
    clps711x_clk.clk_data.hws[CLPS711X_CLK_CPU] =
    clk_hw_register_fixed_rate(core::ptr::null_mut(), "cpu", core::ptr::null_mut(), 0, f_cpu);
    clps711x_clk.clk_data.hws[CLPS711X_CLK_BUS] =
    clk_hw_register_fixed_rate(core::ptr::null_mut(), "bus", core::ptr::null_mut(), 0, f_bus);
    clps711x_clk.clk_data.hws[CLPS711X_CLK_PLL] =
    clk_hw_register_fixed_rate(core::ptr::null_mut(), "pll", core::ptr::null_mut(), 0, f_pll);
    clps711x_clk.clk_data.hws[CLPS711X_CLK_TIMERREF] =
    clk_hw_register_fixed_rate(core::ptr::null_mut(), "timer_ref", core::ptr::null_mut(), 0, f_tim);
    clps711x_clk.clk_data.hws[CLPS711X_CLK_TIMER1] =
    clk_hw_register_divider_table(core::ptr::null_mut(), "timer1", "timer_ref", 0,
    base + CLPS711X_SYSCON1, 5, 1, 0,
    timer_div_table, &clps711x_clk.lock);
    clps711x_clk.clk_data.hws[CLPS711X_CLK_TIMER2] =
    clk_hw_register_divider_table(core::ptr::null_mut(), "timer2", "timer_ref", 0,
    base + CLPS711X_SYSCON1, 7, 1, 0,
    timer_div_table, &clps711x_clk.lock);
    clps711x_clk.clk_data.hws[CLPS711X_CLK_PWM] =
    clk_hw_register_fixed_rate(core::ptr::null_mut(), "pwm", core::ptr::null_mut(), 0, f_pwm);
    clps711x_clk.clk_data.hws[CLPS711X_CLK_SPIREF] =
    clk_hw_register_fixed_rate(core::ptr::null_mut(), "spi_ref", core::ptr::null_mut(), 0, f_spi);
    clps711x_clk.clk_data.hws[CLPS711X_CLK_SPI] =
    clk_hw_register_divider_table(core::ptr::null_mut(), "spi", "spi_ref", 0,
    base + CLPS711X_SYSCON1, 16, 2, 0,
    spi_div_table, &clps711x_clk.lock);
    clps711x_clk.clk_data.hws[CLPS711X_CLK_UART] =
    clk_hw_register_fixed_factor(core::ptr::null_mut(), "uart", "bus", 0, 1, 10);
    clps711x_clk.clk_data.hws[CLPS711X_CLK_TICK] =
    clk_hw_register_fixed_rate(core::ptr::null_mut(), "tick", core::ptr::null_mut(), 0, 64);
    for (tmp = 0; tmp < CLPS711X_CLK_MAX; tmp++)
    if (IS_ERR(clps711x_clk.clk_data.hws[tmp]))
    pr_err("clk %i: register failed with %ld\n",
    tmp, PTR_ERR(clps711x_clk.clk_data.hws[tmp]));
    clps711x_clk.clk_data.num = CLPS711X_CLK_MAX;
    of_clk_add_hw_provider(np, of_clk_hw_onecell_get,
    &clps711x_clk.clk_data);
    }
    CLK_OF_DECLARE(clps711x, "cirrus,ep7209-clk", clps711x_clk_init_dt);
