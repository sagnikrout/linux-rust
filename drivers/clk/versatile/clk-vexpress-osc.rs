//! Automatically rewritten from C to Rust
//! Source: drivers/clk/versatile/clk-vexpress-osc.c
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
// Copyright (C) 2012 ARM Limited
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vexpress_osc {
    pub reg: *mut regmap,
    pub hw: clk_hw,
    pub rate_min: c_ulong,
    pub rate_max: c_ulong,
}

    static unsigned long vexpress_osc_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct vexpress_osc *osc = to_vexpress_osc(hw);
    u32 rate;
    regmap_read(osc.reg, 0, &rate);
    return rate;
    }
    static int vexpress_osc_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct vexpress_osc *osc = to_vexpress_osc(hw);
    if (osc.rate_min && req.rate < osc.rate_min)
    req.rate = osc.rate_min;
    if (osc.rate_max && req.rate > osc.rate_max)
    req.rate = osc.rate_max;
    return 0;
    }
    static int vexpress_osc_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct vexpress_osc *osc = to_vexpress_osc(hw);
    return regmap_write(osc.reg, 0, rate);
    }
    static const struct clk_ops vexpress_osc_ops = {
    .recalc_rate = vexpress_osc_recalc_rate,
    .determine_rate = vexpress_osc_determine_rate,
    .set_rate = vexpress_osc_set_rate,
    };
#[no_mangle]
unsafe extern "C" fn vexpress_osc_probe(pdev: *mut platform_device) -> c_int {
    static int vexpress_osc_probe(struct platform_device *pdev)
    {
    struct clk_init_data init;
    struct vexpress_osc *osc;
    u32 range[2];
    int ret;
    osc = devm_kzalloc(&pdev.dev, sizeof(*osc), GFP_KERNEL);
    if (!osc)
    return -ENOMEM;
    osc.reg = devm_regmap_init_vexpress_config(&pdev.dev);
    if (IS_ERR(osc.reg))
    return PTR_ERR(osc.reg);
    if (of_property_read_u32_array(pdev.dev.of_node, "freq-range", range,
    ARRAY_SIZE(range)) == 0) {
    osc.rate_min = range[0];
    osc.rate_max = range[1];
    }
    if (of_property_read_string(pdev.dev.of_node, "clock-output-names",
    &init.name) != 0)
    init.name = dev_name(&pdev.dev);
    init.ops = &vexpress_osc_ops;
    init.flags = 0;
    init.num_parents = 0;
    osc.hw.init = &init;
    ret = devm_clk_hw_register(&pdev.dev, &osc.hw);
    if (ret < 0)
    return ret;
    devm_of_clk_add_hw_provider(&pdev.dev, of_clk_hw_simple_get, &osc.hw);
    clk_hw_set_rate_range(&osc.hw, osc.rate_min, osc.rate_max);
    dev_dbg(&pdev.dev, "Registered clock '%s'\n", init.name);
    return 0;
    }
    static const struct of_device_id vexpress_osc_of_match[] = {
    { .compatible = "arm,vexpress-osc", },
    {}
    };
    MODULE_DEVICE_TABLE(of, vexpress_osc_of_match);
    static struct platform_driver vexpress_osc_driver = {
    .driver	= {
    .name = "vexpress-osc",
    .of_match_table = vexpress_osc_of_match,
    },
    .probe = vexpress_osc_probe,
    };
    module_platform_driver(vexpress_osc_driver);
    MODULE_DESCRIPTION("Clock driver for Versatile Express OSC clock generators");
    MODULE_LICENSE("GPL v2");
