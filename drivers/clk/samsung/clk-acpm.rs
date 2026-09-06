//! Automatically rewritten from C to Rust
//! Source: drivers/clk/samsung/clk-acpm.c
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
// Samsung Exynos ACPM protocol based clock driver.
//
// Copyright 2025 Linaro Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpm_clk {
    pub id: u32,
    pub hw: clk_hw,
    pub mbox_chan_id: c_uint,
    pub handle: *mut acpm_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpm_clk_variant {
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpm_clk_driver_data {
    pub clks: *const acpm_clk_variant,
    pub nr_clks: c_uint,
    pub mbox_chan_id: c_uint,
}

    {						\
    .name		= cname,		\
    }
    static const struct acpm_clk_variant gs101_acpm_clks[] = {
    ACPM_CLK("mif"),
    ACPM_CLK("int"),
    ACPM_CLK("cpucl0"),
    ACPM_CLK("cpucl1"),
    ACPM_CLK("cpucl2"),
    ACPM_CLK("g3d"),
    ACPM_CLK("g3dl2"),
    ACPM_CLK("tpu"),
    ACPM_CLK("intcam"),
    ACPM_CLK("tnr"),
    ACPM_CLK("cam"),
    ACPM_CLK("mfc"),
    ACPM_CLK("disp"),
    ACPM_CLK("bo"),
    };
    static const struct acpm_clk_driver_data acpm_clk_gs101 = {
    .clks = gs101_acpm_clks,
    .nr_clks = ARRAY_SIZE(gs101_acpm_clks),
    .mbox_chan_id = 0,
    };
    static unsigned long acpm_clk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct acpm_clk *clk = to_acpm_clk(hw);
    return clk.handle.ops.dvfs.get_rate(clk.handle, clk.mbox_chan_id,
    clk.id);
    }
    static int acpm_clk_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct acpm_clk *clk = to_acpm_clk(hw);
    return clk.handle.ops.dvfs.set_rate(clk.handle, clk.mbox_chan_id,
    clk.id, rate);
    }
    static const struct clk_ops acpm_clk_ops = {
    .recalc_rate = acpm_clk_recalc_rate,
    .determine_rate = clk_determine_rate_noop,
    .set_rate = acpm_clk_set_rate,
    };
    static int acpm_clk_register(struct device *dev, struct acpm_clk *aclk,
    const char *name)
    {
    let mut init: clk_init_data = {};
    init.name = name;
    init.ops = &acpm_clk_ops;
    aclk.hw.init = &init;
    return devm_clk_hw_register(dev, &aclk.hw);
    }
#[no_mangle]
unsafe extern "C" fn acpm_clk_probe(pdev: *mut platform_device) -> c_int {
    static int acpm_clk_probe(struct platform_device *pdev)
    {
    struct acpm_handle *acpm_handle;
    struct clk_hw_onecell_data *clk_data;
    struct clk_hw **hws;
    struct device *dev = &pdev.dev;
    struct acpm_clk *aclks;
    unsigned int mbox_chan_id;
    int i, err, count;
    acpm_handle = devm_acpm_get_by_node(dev, dev.parent.of_node);
    if (IS_ERR(acpm_handle))
    return dev_err_probe(dev, PTR_ERR(acpm_handle),
    "Failed to get acpm handle\n");
    count = acpm_clk_gs101.nr_clks;
    mbox_chan_id = acpm_clk_gs101.mbox_chan_id;
    clk_data = devm_kzalloc(dev, struct_size(clk_data, hws, count),
    GFP_KERNEL);
    if (!clk_data)
    return -ENOMEM;
    clk_data.num = count;
    hws = clk_data.hws;
    aclks = devm_kcalloc(dev, count, sizeof(*aclks), GFP_KERNEL);
    if (!aclks)
    return -ENOMEM;
    for (i = 0; i < count; i++) {
    struct acpm_clk *aclk = &aclks[i];
//
// The code assumes the clock IDs start from zero,
// are sequential and do not have gaps.
//
    aclk.id = i;
    aclk.handle = acpm_handle;
    aclk.mbox_chan_id = mbox_chan_id;
    hws[i] = &aclk.hw;
    err = acpm_clk_register(dev, aclk,
    acpm_clk_gs101.clks[i].name);
    if (err)
    return dev_err_probe(dev, err,
    "Failed to register clock\n");
    }
    return devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get,
    clk_data);
    }
    static const struct platform_device_id acpm_clk_id[] = {
    { .name = "gs101-acpm-clk" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, acpm_clk_id);
    static struct platform_driver acpm_clk_driver = {
    .driver	= {
    .name = "acpm-clocks",
    },
    .probe = acpm_clk_probe,
    .id_table = acpm_clk_id,
    };
    module_platform_driver(acpm_clk_driver);
    MODULE_AUTHOR("Tudor Ambarus <tudor.ambarus@linaro.org>");
    MODULE_DESCRIPTION("Samsung Exynos ACPM clock driver");
    MODULE_LICENSE("GPL");
