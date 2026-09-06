//! Automatically rewritten from C to Rust
//! Source: drivers/clk/renesas/r8a78000-cpg.c
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


// SPDX-License-Identifier: GPL-2.0
//
// R-Car X5H Clock Pulse Generator
//
// Copyright (C) 2026 Glider bv
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_map {
    pub /: *mut *mut int dt_id; / DT binding clock ID or -1 sentinel,
    pub /: *mut *mut u32 fw_id; / FIXED_CLK() ID,
}

    enum fixed_clk {
    FIXED_CLK_66M,
    FIXED_CLK_266M,
    NUM_FIXED_CLKS
    };
    static const unsigned long fixed_clk_rates[NUM_FIXED_CLKS] = {
    [FIXED_CLK_66M] = 66666000,
    [FIXED_CLK_266M] = 266660000,
    };

//
// struct r8a78000_cpg_priv - Clock Pulse Generator Private Data
//
// @dev: CPG device
// @map: Mapping from DT clock IDs to fixed-rate clocks
// @fixed_hws: Fixed rate clocks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a78000_cpg_priv {
    pub dev: *mut device,
    pub map: *const clk_map,
    pub fixed_hws: [*mut clk_hw; NUM_FIXED_CLKS],
}

    static const struct clk_map *clk_map_find(const struct clk_map *map, u32 id)
    {
    if (!map)
    return core::ptr::null_mut();
    for (; map.dt_id >= 0; map++) {
    if (map.dt_id == id)
    return map;
    }
    return core::ptr::null_mut();
    }
    static struct clk_hw *r8a78000_clk_get(struct of_phandle_args *spec,
    void *data)
    {
    struct r8a78000_cpg_priv *priv = data;
    struct device *dev = priv.dev;
    const struct clk_map *map;
    struct clk_hw *hw;
    u32 id;
    if (spec.args_count != 1)
    return ERR_PTR(-EINVAL);
    id = spec.args[0];
    map = clk_map_find(priv.map, id);
    if (!map) {
    dev_err(dev, "Unknown clock %u\n", id);
    return ERR_PTR(-ENOENT);
    }
    dev_dbg(dev, "Mapping DT clock %u to fixed clock %u\n", id, map.fw_id);
    hw = priv.fixed_hws[map.fw_id];
    dev_dbg(dev, "clock %u is %s at %lu Hz\n", id, clk_hw_get_name(hw),
    clk_hw_get_rate(hw));
    return hw;
    }
#[no_mangle]
unsafe extern "C" fn register_fixed_clks(priv: *mut r8a78000_cpg_priv) -> c_int {
    static int register_fixed_clks(struct r8a78000_cpg_priv *priv)
    {
    struct device *dev = priv.dev;
    unsigned long rate;
    struct clk_hw *hw;
    const char *name;
    for (unsigned int i = 0; i < ARRAY_SIZE(fixed_clk_rates); i++) {
    rate = fixed_clk_rates[i];
    name = devm_kasprintf(dev, GFP_KERNEL, "cpg-%lu", rate);
    if (!name)
    return -ENOMEM;
    hw = devm_clk_hw_register_fixed_rate(dev, name, core::ptr::null_mut(), 0, rate);
    if (IS_ERR(hw))
    return PTR_ERR(hw);
    priv.fixed_hws[i] = hw;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn r8a78000_cpg_probe(pdev: *mut platform_device) -> c_int {
    static int r8a78000_cpg_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct r8a78000_cpg_priv *priv;
    const struct clk_map *map;
    int ret;
    map = of_device_get_match_data(dev);
    if (!map)
    return -ENODEV;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.map = map;
    ret = register_fixed_clks(priv);
    if (ret)
    return ret;
    return devm_of_clk_add_hw_provider(dev, r8a78000_clk_get, priv);
    }
    static const struct clk_map r8a78000_cpg_default[] = {
    { R8A78000_CPG_SGASYNCD4_PERW_BUS,	FIXED_CLK(266M) },
    { R8A78000_CPG_SGASYNCD16_PERW_BUS,	FIXED_CLK(66M) },
    { -1 }
    };
    static const struct of_device_id r8a78000_cpg_match[] = {
    {
    .compatible = "renesas,r8a78000-cpg",
    .data = &r8a78000_cpg_default,
    },
    { /* sentinel */ }
    };
    static struct platform_driver r8a78000_cpg_driver = {
    .probe = r8a78000_cpg_probe,
    .driver = {
    .name = "r8a78000-cpg",
    .of_match_table = r8a78000_cpg_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(r8a78000_cpg_driver)
    MODULE_DESCRIPTION("R-Car X5H CPG Driver");
