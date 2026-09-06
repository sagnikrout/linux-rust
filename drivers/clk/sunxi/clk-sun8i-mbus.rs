//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-sun8i-mbus.c
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
// Copyright 2014 Chen-Yu Tsai
//
// Chen-Yu Tsai <wens@csie.org>
//

pub const SUN8I_MBUS_ENABLE: c_int = 31;
pub const SUN8I_MBUS_MUX_SHIFT: c_int = 24;
pub const SUN8I_MBUS_MUX_MASK: c_uint = 0x3;
pub const SUN8I_MBUS_DIV_SHIFT: c_int = 0;
pub const SUN8I_MBUS_DIV_WIDTH: c_int = 3;
pub const SUN8I_MBUS_MAX_PARENTS: c_int = 4;
    static DEFINE_SPINLOCK(sun8i_a23_mbus_lock);
#[no_mangle]
unsafe extern "C" fn sun8i_a23_mbus_setup(node: *mut device_node) -> void __init {
    static void __init sun8i_a23_mbus_setup(struct device_node *node)
    {
    let mut num_parents: c_int = of_clk_get_parent_count(node);
    const char **parents;
    const char *clk_name = node.name;
    struct resource res;
    struct clk_divider *div;
    struct clk_gate *gate;
    struct clk_mux *mux;
    struct clk *clk;
    void __iomem *reg;
    int err;
    parents = kcalloc(num_parents, sizeof(*parents), GFP_KERNEL);
    if (!parents)
    return;
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(reg)) {
    pr_err("Could not get registers for sun8i-mbus-clk\n");
    goto err_free_parents;
    }
    div = kzalloc_obj(*div);
    if (!div)
    goto err_unmap;
    mux = kzalloc_obj(*mux);
    if (!mux)
    goto err_free_div;
    gate = kzalloc_obj(*gate);
    if (!gate)
    goto err_free_mux;
    of_property_read_string(node, "clock-output-names", &clk_name);
    of_clk_parent_fill(node, parents, num_parents);
    gate.reg = reg;
    gate.bit_idx = SUN8I_MBUS_ENABLE;
    gate.lock = &sun8i_a23_mbus_lock;
    div.reg = reg;
    div.shift = SUN8I_MBUS_DIV_SHIFT;
    div.width = SUN8I_MBUS_DIV_WIDTH;
    div.lock = &sun8i_a23_mbus_lock;
    mux.reg = reg;
    mux.shift = SUN8I_MBUS_MUX_SHIFT;
    mux.mask = SUN8I_MBUS_MUX_MASK;
    mux.lock = &sun8i_a23_mbus_lock;
// The MBUS clocks needs to be always enabled
    clk = clk_register_composite(core::ptr::null_mut(), clk_name, parents, num_parents,
    &mux.hw, &clk_mux_ops,
    &div.hw, &clk_divider_ops,
    &gate.hw, &clk_gate_ops,
    CLK_IS_CRITICAL);
    if (IS_ERR(clk))
    goto err_free_gate;
    err = of_clk_add_provider(node, of_clk_src_simple_get, clk);
    if (err)
    goto err_unregister_clk;
    kfree(parents); /* parents is deep copied */
    return;
    err_unregister_clk:
// TODO: The composite clock stuff will leak a bit here.
    clk_unregister(clk);
    err_free_gate:
    kfree(gate);
    err_free_mux:
    kfree(mux);
    err_free_div:
    kfree(div);
    err_unmap:
    iounmap(reg);
    of_address_to_resource(node, 0, &res);
    release_mem_region(res.start, resource_size(&res));
    err_free_parents:
    kfree(parents);
    }
    CLK_OF_DECLARE(sun8i_a23_mbus, "allwinner,sun8i-a23-mbus-clk", sun8i_a23_mbus_setup);
