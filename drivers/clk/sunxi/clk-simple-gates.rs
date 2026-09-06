//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-simple-gates.c
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
// Copyright 2015 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

    static DEFINE_SPINLOCK(gates_lock);
    static void __init sunxi_simple_gates_setup(struct device_node *node,
    const int protected[],
    int nprotected)
    {
    struct clk_onecell_data *clk_data;
    const char *clk_parent, *clk_name;
    struct resource res;
    void __iomem *clk_reg;
    void __iomem *reg;
    int number, i = 0, j;
    u8 clk_bit;
    u32 index;
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(reg))
    return;
    clk_parent = of_clk_get_parent_name(node, 0);
    clk_data = kmalloc_obj(struct clk_onecell_data);
    if (!clk_data)
    goto err_unmap;
    number = of_property_count_u32_elems(node, "clock-indices");
    of_property_read_u32_index(node, "clock-indices", number - 1, &number);
    clk_data.clks = kzalloc_objs(struct clk *, number + 1);
    if (!clk_data.clks)
    goto err_free_data;
    of_property_for_each_u32(node, "clock-indices", index) {
    of_property_read_string_index(node, "clock-output-names",
    i, &clk_name);
    clk_reg = reg + 4 * (index / 32);
    clk_bit = index % 32;
    clk_data.clks[index] = clk_register_gate(core::ptr::null_mut(), clk_name,
    clk_parent, 0,
    clk_reg,
    clk_bit,
    0, &gates_lock);
    i++;
    if (IS_ERR(clk_data.clks[index])) {
    WARN_ON(true);
    continue;
    }
    for (j = 0; j < nprotected; j++)
    if (protected[j] == index)
    clk_prepare_enable(clk_data.clks[index]);
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
#[no_mangle]
unsafe extern "C" fn sunxi_simple_gates_init(node: *mut device_node) -> void __init {
    static void __init sunxi_simple_gates_init(struct device_node *node)
    {
    sunxi_simple_gates_setup(node, core::ptr::null_mut(), 0);
    }
    CLK_OF_DECLARE(sun4i_a10_gates, "allwinner,sun4i-a10-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun4i_a10_apb0, "allwinner,sun4i-a10-apb0-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun4i_a10_apb1, "allwinner,sun4i-a10-apb1-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun4i_a10_axi, "allwinner,sun4i-a10-axi-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun5i_a10s_apb0, "allwinner,sun5i-a10s-apb0-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun5i_a10s_apb1, "allwinner,sun5i-a10s-apb1-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun5i_a13_apb0, "allwinner,sun5i-a13-apb0-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun5i_a13_apb1, "allwinner,sun5i-a13-apb1-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun6i_a31_ahb1, "allwinner,sun6i-a31-ahb1-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun6i_a31_apb1, "allwinner,sun6i-a31-apb1-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun6i_a31_apb2, "allwinner,sun6i-a31-apb2-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun7i_a20_apb0, "allwinner,sun7i-a20-apb0-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun7i_a20_apb1, "allwinner,sun7i-a20-apb1-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun8i_a23_ahb1, "allwinner,sun8i-a23-ahb1-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun8i_a23_apb1, "allwinner,sun8i-a23-apb1-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun8i_a23_apb2, "allwinner,sun8i-a23-apb2-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun8i_a33_ahb1, "allwinner,sun8i-a33-ahb1-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun8i_a83t_apb0, "allwinner,sun8i-a83t-apb0-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun9i_a80_ahb0, "allwinner,sun9i-a80-ahb0-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun9i_a80_ahb1, "allwinner,sun9i-a80-ahb1-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun9i_a80_ahb2, "allwinner,sun9i-a80-ahb2-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun9i_a80_apb0, "allwinner,sun9i-a80-apb0-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun9i_a80_apb1, "allwinner,sun9i-a80-apb1-gates-clk",
    sunxi_simple_gates_init);
    CLK_OF_DECLARE(sun9i_a80_apbs, "allwinner,sun9i-a80-apbs-gates-clk",
    sunxi_simple_gates_init);
    static const int sun4i_a10_ahb_critical_clocks[] __initconst = {
    14,	/* ahb_sdram */
    };
#[no_mangle]
unsafe extern "C" fn sun4i_a10_ahb_init(node: *mut device_node) -> void __init {
    static void __init sun4i_a10_ahb_init(struct device_node *node)
    {
    sunxi_simple_gates_setup(node, sun4i_a10_ahb_critical_clocks,
    ARRAY_SIZE(sun4i_a10_ahb_critical_clocks));
    }
    CLK_OF_DECLARE(sun4i_a10_ahb, "allwinner,sun4i-a10-ahb-gates-clk",
    sun4i_a10_ahb_init);
    CLK_OF_DECLARE(sun5i_a10s_ahb, "allwinner,sun5i-a10s-ahb-gates-clk",
    sun4i_a10_ahb_init);
    CLK_OF_DECLARE(sun5i_a13_ahb, "allwinner,sun5i-a13-ahb-gates-clk",
    sun4i_a10_ahb_init);
    CLK_OF_DECLARE(sun7i_a20_ahb, "allwinner,sun7i-a20-ahb-gates-clk",
    sun4i_a10_ahb_init);
    static const int sun4i_a10_dram_critical_clocks[] __initconst = {
    15,	/* dram_output */
    };
#[no_mangle]
unsafe extern "C" fn sun4i_a10_dram_init(node: *mut device_node) -> void __init {
    static void __init sun4i_a10_dram_init(struct device_node *node)
    {
    sunxi_simple_gates_setup(node, sun4i_a10_dram_critical_clocks,
    ARRAY_SIZE(sun4i_a10_dram_critical_clocks));
    }
    CLK_OF_DECLARE(sun4i_a10_dram, "allwinner,sun4i-a10-dram-gates-clk",
    sun4i_a10_dram_init);
