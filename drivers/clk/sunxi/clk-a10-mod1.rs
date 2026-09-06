//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-a10-mod1.c
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

    static DEFINE_SPINLOCK(mod1_lock);
pub const SUN4I_MOD1_ENABLE: c_int = 31;
pub const SUN4I_MOD1_MUX: c_int = 16;
pub const SUN4I_MOD1_MUX_WIDTH: c_int = 2;
pub const SUN4I_MOD1_MAX_PARENTS: c_int = 4;
#[no_mangle]
unsafe extern "C" fn sun4i_mod1_clk_setup(node: *mut device_node) -> void __init {
    static void __init sun4i_mod1_clk_setup(struct device_node *node)
    {
    struct clk *clk;
    struct clk_mux *mux;
    struct clk_gate *gate;
    const char *parents[4];
    const char *clk_name = node.name;
    void __iomem *reg;
    int i;
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(reg))
    return;
    mux = kzalloc_obj(*mux);
    if (!mux)
    goto err_unmap;
    gate = kzalloc_obj(*gate);
    if (!gate)
    goto err_free_mux;
    of_property_read_string(node, "clock-output-names", &clk_name);
    i = of_clk_parent_fill(node, parents, SUN4I_MOD1_MAX_PARENTS);
    gate.reg = reg;
    gate.bit_idx = SUN4I_MOD1_ENABLE;
    gate.lock = &mod1_lock;
    mux.reg = reg;
    mux.shift = SUN4I_MOD1_MUX;
    mux.mask = BIT(SUN4I_MOD1_MUX_WIDTH) - 1;
    mux.lock = &mod1_lock;
    clk = clk_register_composite(core::ptr::null_mut(), clk_name, parents, i,
    &mux.hw, &clk_mux_ops,
    core::ptr::null_mut(), core::ptr::null_mut(),
    &gate.hw, &clk_gate_ops, CLK_SET_RATE_PARENT);
    if (IS_ERR(clk))
    goto err_free_gate;
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    return;
    err_free_gate:
    kfree(gate);
    err_free_mux:
    kfree(mux);
    err_unmap:
    iounmap(reg);
    }
    CLK_OF_DECLARE(sun4i_mod1, "allwinner,sun4i-a10-mod1-clk",
    sun4i_mod1_clk_setup);
