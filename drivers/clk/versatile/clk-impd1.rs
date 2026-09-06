//! Automatically rewritten from C to Rust
//! Source: drivers/clk/versatile/clk-impd1.c
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
// Clock driver for the ARM Integrator/IM-PD1 board
// Copyright (C) 2012-2013 Linus Walleij
//

pub const IMPD1_OSC1: c_uint = 0x00;
pub const IMPD1_OSC2: c_uint = 0x04;
pub const IMPD1_LOCK: c_uint = 0x08;
//
// There are two VCO's on the IM-PD1
//
    static const struct icst_params impd1_vco1_params = {
    .ref		= 24000000,	/* 24 MHz */
    .vco_max	= ICST525_VCO_MAX_3V,
    .vco_min	= ICST525_VCO_MIN,
    .vd_min		= 12,
    .vd_max		= 519,
    .rd_min		= 3,
    .rd_max		= 120,
    .s2div		= icst525_s2div,
    .idx2s		= icst525_idx2s,
    };
    static const struct clk_icst_desc impd1_icst1_desc = {
    .params = &impd1_vco1_params,
    .vco_offset = IMPD1_OSC1,
    .lock_offset = IMPD1_LOCK,
    };
    static const struct icst_params impd1_vco2_params = {
    .ref		= 24000000,	/* 24 MHz */
    .vco_max	= ICST525_VCO_MAX_3V,
    .vco_min	= ICST525_VCO_MIN,
    .vd_min		= 12,
    .vd_max		= 519,
    .rd_min		= 3,
    .rd_max		= 120,
    .s2div		= icst525_s2div,
    .idx2s		= icst525_idx2s,
    };
    static const struct clk_icst_desc impd1_icst2_desc = {
    .params = &impd1_vco2_params,
    .vco_offset = IMPD1_OSC2,
    .lock_offset = IMPD1_LOCK,
    };
    static int integrator_impd1_clk_spawn(struct device *dev,
    struct device_node *parent,
    struct device_node *np)
    {
    struct regmap *map;
    struct clk *clk = ERR_PTR(-EINVAL);
    const char *name = np.name;
    const char *parent_name;
    const struct clk_icst_desc *desc;
    int ret;
    map = syscon_node_to_regmap(parent);
    if (IS_ERR(map)) {
    pr_err("no regmap for syscon IM-PD1 ICST clock parent\n");
    return PTR_ERR(map);
    }
    if (of_device_is_compatible(np, "arm,impd1-vco1")) {
    desc = &impd1_icst1_desc;
    } else if (of_device_is_compatible(np, "arm,impd1-vco2")) {
    desc = &impd1_icst2_desc;
    } else {
    dev_err(dev, "not a clock node %s\n", name);
    return -ENODEV;
    }
    of_property_read_string(np, "clock-output-names", &name);
    parent_name = of_clk_get_parent_name(np, 0);
    clk = icst_clk_setup(core::ptr::null_mut(), desc, name, parent_name, map,
    ICST_INTEGRATOR_IM_PD1);
    if (!IS_ERR(clk)) {
    of_clk_add_provider(np, of_clk_src_simple_get, clk);
    ret = 0;
    } else {
    dev_err(dev, "error setting up IM-PD1 ICST clock\n");
    ret = PTR_ERR(clk);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn integrator_impd1_clk_probe(pdev: *mut platform_device) -> c_int {
    static int integrator_impd1_clk_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    let mut ret: c_int = 0;
    for_each_available_child_of_node_scoped(np, child) {
    ret = integrator_impd1_clk_spawn(dev, np, child);
    if (ret)
    break;
    }
    return ret;
    }
    static const struct of_device_id impd1_syscon_match[] = {
    { .compatible = "arm,im-pd1-syscon", },
    {}
    };
    MODULE_DEVICE_TABLE(of, impd1_syscon_match);
    static struct platform_driver impd1_clk_driver = {
    .driver = {
    .name = "impd1-clk",
    .of_match_table = impd1_syscon_match,
    },
    .probe  = integrator_impd1_clk_probe,
    };
    builtin_platform_driver(impd1_clk_driver);
    MODULE_AUTHOR("Linus Walleij <linusw@kernel.org>");
    MODULE_DESCRIPTION("Arm IM-PD1 module clock driver");
    MODULE_LICENSE("GPL v2");
