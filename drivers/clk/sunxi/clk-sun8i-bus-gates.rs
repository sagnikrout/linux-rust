//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-sun8i-bus-gates.c
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
// Copyright (C) 2015 Jens Kuske <jenskuske@gmail.com>
//
// Based on clk-simple-gates.c, which is:
// Copyright 2015 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

    static DEFINE_SPINLOCK(gates_lock);
#[no_mangle]
unsafe extern "C" fn sun8i_h3_bus_gates_init(node: *mut device_node) -> void __init {
    static void __init sun8i_h3_bus_gates_init(struct device_node *node)
    {
    static const char * const names[] = { "ahb1", "ahb2", "apb1", "apb2" };
    enum { AHB1, AHB2, APB1, APB2, PARENT_MAX } clk_parent;
    const char *parents[PARENT_MAX];
    struct clk_onecell_data *clk_data;
    const char *clk_name;
    struct resource res;
    void __iomem *clk_reg;
    void __iomem *reg;
    int number, i;
    u8 clk_bit;
    int index;
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(reg))
    return;
    for (i = 0; i < ARRAY_SIZE(names); i++) {
    int idx = of_property_match_string(node, "clock-names",
    names[i]);
    if (idx < 0)
    return;
    parents[i] = of_clk_get_parent_name(node, idx);
    }
    clk_data = kmalloc_obj(struct clk_onecell_data);
    if (!clk_data)
    goto err_unmap;
    number = of_property_count_u32_elems(node, "clock-indices");
    of_property_read_u32_index(node, "clock-indices", number - 1, &number);
    clk_data.clks = kzalloc_objs(struct clk *, number + 1);
    if (!clk_data.clks)
    goto err_free_data;
    i = 0;
    of_property_for_each_u32(node, "clock-indices", index) {
    of_property_read_string_index(node, "clock-output-names",
    i, &clk_name);
    if (index == 17 || (index >= 29 && index <= 31))
    clk_parent = AHB2;
#[no_mangle]
pub unsafe extern "C" fn if(128: index <= 63 || index >=) -> else {
    else if (index <= 63 || index >= 128)
    clk_parent = AHB1;
#[no_mangle]
pub unsafe extern "C" fn if(95: index >= 64 && index <=) -> else {
    else if (index >= 64 && index <= 95)
    clk_parent = APB1;
#[no_mangle]
pub unsafe extern "C" fn if(127: index >= 96 && index <=) -> else {
    else if (index >= 96 && index <= 127)
    clk_parent = APB2;
    else {
    WARN_ON(true);
    continue;
    }
    clk_reg = reg + 4 * (index / 32);
    clk_bit = index % 32;
    clk_data.clks[index] = clk_register_gate(core::ptr::null_mut(), clk_name,
    parents[clk_parent],
    0, clk_reg, clk_bit,
    0, &gates_lock);
    i++;
    if (IS_ERR(clk_data.clks[index])) {
    WARN_ON(true);
    continue;
    }
    }
    clk_data.clk_num = number + 1;
    of_clk_add_provider(node, of_clk_src_onecell_get, clk_data);
    return;
    err_free_data:
    kfree(clk_data);
    err_unmap:
    iounmap(reg);
    of_address_to_resource(node, 0, &res);
    release_mem_region(res.start, resource_size(&res));
    }
    CLK_OF_DECLARE(sun8i_h3_bus_gates, "allwinner,sun8i-h3-bus-gates-clk",
    sun8i_h3_bus_gates_init);
    CLK_OF_DECLARE(sun8i_a83t_bus_gates, "allwinner,sun8i-a83t-bus-gates-clk",
    sun8i_h3_bus_gates_init);
