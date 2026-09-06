//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-sun9i-core.c
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

//
// sun9i_a80_get_pll4_factors() - calculates n, p, m factors for PLL4
// PLL4 rate is calculated as follows
// rate = (parent_rate * n >> p) / (m + 1);
// parent_rate is always 24MHz
//
// p and m are named div1 and div2 in Allwinner's SDK
//
#[no_mangle]
unsafe extern "C" fn sun9i_a80_get_pll4_factors(req: *mut factors_request) {
    static void sun9i_a80_get_pll4_factors(struct factors_request *req)
    {
    int n;
    let mut m: c_int = 1;
    let mut p: c_int = 1;
// Normalize value to a 6 MHz multiple (24 MHz / 4)
    n = DIV_ROUND_UP(req.rate, 6000000);
// If n is too large switch to steps of 12 MHz
    if (n > 255) {
    m = 0;
    n = (n + 1) / 2;
    }
// If n is still too large switch to steps of 24 MHz
    if (n > 255) {
    p = 0;
    n = (n + 1) / 2;
    }
// n must be between 12 and 255
    if (n > 255)
    n = 255;
#[no_mangle]
pub unsafe extern "C" fn if(12: n <) -> else {
    else if (n < 12)
    n = 12;
    req.rate = ((24000000 * n) >> p) / (m + 1);
    req.n = n;
    req.m = m;
    req.p = p;
    }
    static const struct clk_factors_config sun9i_a80_pll4_config = {
    .mshift = 18,
    .mwidth = 1,
    .nshift = 8,
    .nwidth = 8,
    .pshift = 16,
    .pwidth = 1,
    };
    static const struct factors_data sun9i_a80_pll4_data __initconst = {
    .enable = 31,
    .table = &sun9i_a80_pll4_config,
    .getter = sun9i_a80_get_pll4_factors,
    };
    static DEFINE_SPINLOCK(sun9i_a80_pll4_lock);
#[no_mangle]
unsafe extern "C" fn sun9i_a80_pll4_setup(node: *mut device_node) -> void __init {
    static void __init sun9i_a80_pll4_setup(struct device_node *node)
    {
    void __iomem *reg;
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(reg)) {
    pr_err("Could not get registers for a80-pll4-clk: %pOFn\n",
    node);
    return;
    }
    sunxi_factors_register(node, &sun9i_a80_pll4_data,
    &sun9i_a80_pll4_lock, reg);
    }
    CLK_OF_DECLARE(sun9i_a80_pll4, "allwinner,sun9i-a80-pll4-clk", sun9i_a80_pll4_setup);
//
// sun9i_a80_get_gt_factors() - calculates m factor for GT
// GT rate is calculated as follows
// rate = parent_rate / (m + 1);
//
#[no_mangle]
unsafe extern "C" fn sun9i_a80_get_gt_factors(req: *mut factors_request) {
    static void sun9i_a80_get_gt_factors(struct factors_request *req)
    {
    u32 div;
    if (req.parent_rate < req.rate)
    req.rate = req.parent_rate;
    div = DIV_ROUND_UP(req.parent_rate, req.rate);
// maximum divider is 4
    if (div > 4)
    div = 4;
    req.rate = req.parent_rate / div;
    req.m = div;
    }
    static const struct clk_factors_config sun9i_a80_gt_config = {
    .mshift = 0,
    .mwidth = 2,
    };
    static const struct factors_data sun9i_a80_gt_data __initconst = {
    .mux = 24,
    .muxmask = BIT(1) | BIT(0),
    .table = &sun9i_a80_gt_config,
    .getter = sun9i_a80_get_gt_factors,
    };
    static DEFINE_SPINLOCK(sun9i_a80_gt_lock);
#[no_mangle]
unsafe extern "C" fn sun9i_a80_gt_setup(node: *mut device_node) -> void __init {
    static void __init sun9i_a80_gt_setup(struct device_node *node)
    {
    void __iomem *reg;
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(reg)) {
    pr_err("Could not get registers for a80-gt-clk: %pOFn\n",
    node);
    return;
    }
// The GT bus clock needs to be always enabled
    sunxi_factors_register_critical(node, &sun9i_a80_gt_data,
    &sun9i_a80_gt_lock, reg);
    }
    CLK_OF_DECLARE(sun9i_a80_gt, "allwinner,sun9i-a80-gt-clk", sun9i_a80_gt_setup);
//
// sun9i_a80_get_ahb_factors() - calculates p factor for AHB0/1/2
// AHB rate is calculated as follows
// rate = parent_rate >> p;
//
#[no_mangle]
unsafe extern "C" fn sun9i_a80_get_ahb_factors(req: *mut factors_request) {
    static void sun9i_a80_get_ahb_factors(struct factors_request *req)
    {
    u32 _p;
    if (req.parent_rate < req.rate)
    req.rate = req.parent_rate;
    _p = order_base_2(DIV_ROUND_UP(req.parent_rate, req.rate));
// maximum p is 3
    if (_p > 3)
    _p = 3;
    req.rate = req.parent_rate >> _p;
    req.p = _p;
    }
    static const struct clk_factors_config sun9i_a80_ahb_config = {
    .pshift = 0,
    .pwidth = 2,
    };
    static const struct factors_data sun9i_a80_ahb_data __initconst = {
    .mux = 24,
    .muxmask = BIT(1) | BIT(0),
    .table = &sun9i_a80_ahb_config,
    .getter = sun9i_a80_get_ahb_factors,
    };
    static DEFINE_SPINLOCK(sun9i_a80_ahb_lock);
#[no_mangle]
unsafe extern "C" fn sun9i_a80_ahb_setup(node: *mut device_node) -> void __init {
    static void __init sun9i_a80_ahb_setup(struct device_node *node)
    {
    void __iomem *reg;
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(reg)) {
    pr_err("Could not get registers for a80-ahb-clk: %pOFn\n",
    node);
    return;
    }
    sunxi_factors_register(node, &sun9i_a80_ahb_data,
    &sun9i_a80_ahb_lock, reg);
    }
    CLK_OF_DECLARE(sun9i_a80_ahb, "allwinner,sun9i-a80-ahb-clk", sun9i_a80_ahb_setup);
    static const struct factors_data sun9i_a80_apb0_data __initconst = {
    .mux = 24,
    .muxmask = BIT(0),
    .table = &sun9i_a80_ahb_config,
    .getter = sun9i_a80_get_ahb_factors,
    };
    static DEFINE_SPINLOCK(sun9i_a80_apb0_lock);
#[no_mangle]
unsafe extern "C" fn sun9i_a80_apb0_setup(node: *mut device_node) -> void __init {
    static void __init sun9i_a80_apb0_setup(struct device_node *node)
    {
    void __iomem *reg;
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(reg)) {
    pr_err("Could not get registers for a80-apb0-clk: %pOFn\n",
    node);
    return;
    }
    sunxi_factors_register(node, &sun9i_a80_apb0_data,
    &sun9i_a80_apb0_lock, reg);
    }
    CLK_OF_DECLARE(sun9i_a80_apb0, "allwinner,sun9i-a80-apb0-clk", sun9i_a80_apb0_setup);
//
// sun9i_a80_get_apb1_factors() - calculates m, p factors for APB1
// APB1 rate is calculated as follows
// rate = (parent_rate >> p) / (m + 1);
//
#[no_mangle]
unsafe extern "C" fn sun9i_a80_get_apb1_factors(req: *mut factors_request) {
    static void sun9i_a80_get_apb1_factors(struct factors_request *req)
    {
    u32 div;
    if (req.parent_rate < req.rate)
    req.rate = req.parent_rate;
    div = DIV_ROUND_UP(req.parent_rate, req.rate);
// Highest possible divider is 256 (p = 3, m = 31)
    if (div > 256)
    div = 256;
    req.p = order_base_2(div);
    req.m = (req.parent_rate >> req.p) - 1;
    req.rate = (req.parent_rate >> req.p) / (req.m + 1);
    }
    static const struct clk_factors_config sun9i_a80_apb1_config = {
    .mshift = 0,
    .mwidth = 5,
    .pshift = 16,
    .pwidth = 2,
    };
    static const struct factors_data sun9i_a80_apb1_data __initconst = {
    .mux = 24,
    .muxmask = BIT(0),
    .table = &sun9i_a80_apb1_config,
    .getter = sun9i_a80_get_apb1_factors,
    };
    static DEFINE_SPINLOCK(sun9i_a80_apb1_lock);
#[no_mangle]
unsafe extern "C" fn sun9i_a80_apb1_setup(node: *mut device_node) -> void __init {
    static void __init sun9i_a80_apb1_setup(struct device_node *node)
    {
    void __iomem *reg;
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(reg)) {
    pr_err("Could not get registers for a80-apb1-clk: %pOFn\n",
    node);
    return;
    }
    sunxi_factors_register(node, &sun9i_a80_apb1_data,
    &sun9i_a80_apb1_lock, reg);
    }
    CLK_OF_DECLARE(sun9i_a80_apb1, "allwinner,sun9i-a80-apb1-clk", sun9i_a80_apb1_setup);
