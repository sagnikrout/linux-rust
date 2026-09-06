//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-a10-hosc.c
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
// Copyright 2013 Emilio López
//
// Emilio López <emilio@elopez.com.ar>
//

pub const SUNXI_OSC24M_GATE: c_int = 0;
    static DEFINE_SPINLOCK(hosc_lock);
#[no_mangle]
unsafe extern "C" fn sun4i_osc_clk_setup(node: *mut device_node) -> void __init {
    static void __init sun4i_osc_clk_setup(struct device_node *node)
    {
    struct clk *clk;
    struct clk_fixed_rate *fixed;
    struct clk_gate *gate;
    const char *clk_name = node.name;
    u32 rate;
    if (of_property_read_u32(node, "clock-frequency", &rate))
    return;
// allocate fixed-rate and gate clock structs
    fixed = kzalloc_obj(struct clk_fixed_rate);
    if (!fixed)
    return;
    gate = kzalloc_obj(struct clk_gate);
    if (!gate)
    goto err_free_fixed;
    of_property_read_string(node, "clock-output-names", &clk_name);
// set up gate and fixed rate properties
    gate.reg = of_iomap(node, 0);
    gate.bit_idx = SUNXI_OSC24M_GATE;
    gate.lock = &hosc_lock;
    fixed.fixed_rate = rate;
    clk = clk_register_composite(core::ptr::null_mut(), clk_name,
    core::ptr::null_mut(), 0,
    core::ptr::null_mut(), core::ptr::null_mut(),
    &fixed.hw, &clk_fixed_rate_ops,
    &gate.hw, &clk_gate_ops, 0);
    if (IS_ERR(clk))
    goto err_free_gate;
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    return;
    err_free_gate:
    kfree(gate);
    err_free_fixed:
    kfree(fixed);
    }
    CLK_OF_DECLARE(sun4i_osc, "allwinner,sun4i-a10-osc-clk", sun4i_osc_clk_setup);
