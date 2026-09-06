//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-cpu.c
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
// Copyright (c) 2014 Lucas Stach <l.stach@pengutronix.de>, Pengutronix
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_cpu {
    pub hw: clk_hw,
    pub div: *mut clk,
    pub mux: *mut clk,
    pub pll: *mut clk,
    pub step: *mut clk,
}

    static inline struct clk_cpu *to_clk_cpu(struct clk_hw *hw)
    {
    return container_of(hw, struct clk_cpu, hw);
    }
    static unsigned long clk_cpu_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_cpu *cpu = to_clk_cpu(hw);
    return clk_get_rate(cpu.div);
    }
    static int clk_cpu_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_cpu *cpu = to_clk_cpu(hw);
    req.rate = clk_round_rate(cpu.pll, req.rate);
    return 0;
    }
    static int clk_cpu_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_cpu *cpu = to_clk_cpu(hw);
    int ret;
// switch to PLL bypass clock
    ret = clk_set_parent(cpu.mux, cpu.step);
    if (ret)
    return ret;
// reprogram PLL
    ret = clk_set_rate(cpu.pll, rate);
    if (ret) {
    clk_set_parent(cpu.mux, cpu.pll);
    return ret;
    }
// switch back to PLL clock
    clk_set_parent(cpu.mux, cpu.pll);
// Ensure the divider is what we expect
    clk_set_rate(cpu.div, rate);
    return 0;
    }
    static const struct clk_ops clk_cpu_ops = {
    .recalc_rate	= clk_cpu_recalc_rate,
    .determine_rate = clk_cpu_determine_rate,
    .set_rate	= clk_cpu_set_rate,
    };
    struct clk_hw *imx_clk_hw_cpu(const char *name, const char *parent_name,
    struct clk *div, struct clk *mux, struct clk *pll,
    struct clk *step)
    {
    struct clk_cpu *cpu;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    cpu = kzalloc_obj(*cpu);
    if (!cpu)
    return ERR_PTR(-ENOMEM);
    cpu.div = div;
    cpu.mux = mux;
    cpu.pll = pll;
    cpu.step = step;
    init.name = name;
    init.ops = &clk_cpu_ops;
    init.flags = CLK_IS_CRITICAL;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    cpu.hw.init = &init;
    hw = &cpu.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(cpu);
    return ERR_PTR(ret);
    }
    return hw;
    }
    EXPORT_SYMBOL_GPL(imx_clk_hw_cpu);
