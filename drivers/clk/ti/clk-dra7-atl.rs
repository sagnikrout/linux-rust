//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ti/clk-dra7-atl.c
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
// DRA7 ATL (Audio Tracking Logic) clock driver
//
// Copyright (C) 2013 Texas Instruments, Inc.
//
// Peter Ujfalusi <peter.ujfalusi@ti.com>
//

pub const DRA7_ATL_INSTANCES: c_int = 4;

    struct dra7_atl_clock_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dra7_atl_desc {
    pub clk: *mut clk,
    pub hw: clk_hw,
    pub cinfo: *mut dra7_atl_clock_info,
    pub id: c_int,
    pub /: *mut *mut bool probed; / the driver for the IP has been loaded,
    pub /: *mut *mut bool valid; / configured,
    pub enabled: bool,
    pub /: *mut *mut u32 bws; / Baseband Word Select Mux,
    pub /: *mut *mut u32 aws; / Audio Word Select Mux,
    pub /: *mut *mut u32 divider; / Cached divider value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dra7_atl_clock_info {
    pub dev: *mut device,
    pub iobase: *mut void __iomem,
    pub cdesc: *mut dra7_atl_desc,
}

    static inline void atl_write(struct dra7_atl_clock_info *cinfo, u32 reg,
    u32 val)
    {
    __raw_writel(val, cinfo.iobase + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn atl_read(cinfo: *mut dra7_atl_clock_info, reg: u32) -> c_int {
    static inline int atl_read(struct dra7_atl_clock_info *cinfo, u32 reg)
    {
    return __raw_readl(cinfo.iobase + reg);
    }
#[no_mangle]
unsafe extern "C" fn atl_clk_enable(hw: *mut clk_hw) -> c_int {
    static int atl_clk_enable(struct clk_hw *hw)
    {
    struct dra7_atl_desc *cdesc = to_atl_desc(hw);
    if (!cdesc.probed)
    goto out;
    if (unlikely(!cdesc.valid))
    dev_warn(cdesc.cinfo.dev, "atl%d has not been configured\n",
    cdesc.id);
    pm_runtime_get_sync(cdesc.cinfo.dev);
    atl_write(cdesc.cinfo, DRA7_ATL_ATLCR_REG(cdesc.id),
    cdesc.divider - 1);
    atl_write(cdesc.cinfo, DRA7_ATL_SWEN_REG(cdesc.id), DRA7_ATL_SWEN);
    out:
    cdesc.enabled = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atl_clk_disable(hw: *mut clk_hw) {
    static void atl_clk_disable(struct clk_hw *hw)
    {
    struct dra7_atl_desc *cdesc = to_atl_desc(hw);
    if (!cdesc.probed)
    goto out;
    atl_write(cdesc.cinfo, DRA7_ATL_SWEN_REG(cdesc.id), 0);
    pm_runtime_put_sync(cdesc.cinfo.dev);
    out:
    cdesc.enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn atl_clk_is_enabled(hw: *mut clk_hw) -> c_int {
    static int atl_clk_is_enabled(struct clk_hw *hw)
    {
    struct dra7_atl_desc *cdesc = to_atl_desc(hw);
    return cdesc.enabled;
    }
    static unsigned long atl_clk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct dra7_atl_desc *cdesc = to_atl_desc(hw);
    return parent_rate / cdesc.divider;
    }
    static int atl_clk_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    unsigned divider;
    divider = (req.best_parent_rate + req.rate / 2) / req.rate;
    if (divider > DRA7_ATL_DIVIDER_MASK + 1)
    divider = DRA7_ATL_DIVIDER_MASK + 1;
    req.rate = req.best_parent_rate / divider;
    return 0;
    }
    static int atl_clk_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct dra7_atl_desc *cdesc;
    u32 divider;
    if (!hw || !rate)
    return -EINVAL;
    cdesc = to_atl_desc(hw);
    divider = ((parent_rate + rate / 2) / rate) - 1;
    if (divider > DRA7_ATL_DIVIDER_MASK)
    divider = DRA7_ATL_DIVIDER_MASK;
    cdesc.divider = divider + 1;
    return 0;
    }
    static const struct clk_ops atl_clk_ops = {
    .enable		= atl_clk_enable,
    .disable	= atl_clk_disable,
    .is_enabled	= atl_clk_is_enabled,
    .recalc_rate	= atl_clk_recalc_rate,
    .determine_rate = atl_clk_determine_rate,
    .set_rate	= atl_clk_set_rate,
    };
#[no_mangle]
unsafe extern "C" fn of_dra7_atl_clock_setup(node: *mut device_node) -> void __init {
    static void __init of_dra7_atl_clock_setup(struct device_node *node)
    {
    struct dra7_atl_desc *clk_hw = core::ptr::null_mut();
    let mut pdata: clk_parent_data = { .index = 0 };
    let mut init: clk_init_data = { core::ptr::null_mut() };
    const char *name;
    struct clk *clk;
    clk_hw = kzalloc_obj(*clk_hw);
    if (!clk_hw) {
    pr_err("%s: could not allocate dra7_atl_desc\n", __func__);
    return;
    }
    clk_hw.hw.init = &init;
    clk_hw.divider = 1;
    name = ti_dt_clk_name(node);
    init.name = name;
    init.ops = &atl_clk_ops;
    init.flags = CLK_IGNORE_UNUSED;
    init.num_parents = of_clk_get_parent_count(node);
    if (init.num_parents != 1) {
    pr_err("%s: atl clock %pOFn must have 1 parent\n", __func__,
    node);
    goto cleanup;
    }
    init.parent_data = &pdata;
    clk = of_ti_clk_register(node, &clk_hw.hw, name);
    if (!IS_ERR(clk)) {
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    return;
    }
    cleanup:
    kfree(clk_hw);
    }
    CLK_OF_DECLARE(dra7_atl_clock, "ti,dra7-atl-clock", of_dra7_atl_clock_setup);
#[no_mangle]
unsafe extern "C" fn of_dra7_atl_clk_probe(pdev: *mut platform_device) -> c_int {
    static int of_dra7_atl_clk_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct dra7_atl_clock_info *cinfo;
    int i;
    let mut ret: c_int = 0;
    if (!node)
    return -ENODEV;
    cinfo = devm_kzalloc(&pdev.dev, sizeof(*cinfo), GFP_KERNEL);
    if (!cinfo)
    return -ENOMEM;
    cinfo.iobase = of_iomap(node, 0);
    cinfo.dev = &pdev.dev;
    pm_runtime_enable(cinfo.dev);
    pm_runtime_get_sync(cinfo.dev);
    atl_write(cinfo, DRA7_ATL_PCLKMUX_REG(0), DRA7_ATL_PCLKMUX);
    for (i = 0; i < DRA7_ATL_INSTANCES; i++) {
    struct device_node *cfg_node;
    char prop[5];
    struct dra7_atl_desc *cdesc;
    struct of_phandle_args clkspec;
    struct clk *clk;
    int rc;
    rc = of_parse_phandle_with_args(node, "ti,provided-clocks",
    core::ptr::null_mut(), i, &clkspec);
    if (rc) {
    pr_err("%s: failed to lookup atl clock %d\n", __func__,
    i);
    ret = -EINVAL;
    goto pm_put;
    }
    clk = of_clk_get_from_provider(&clkspec);
    of_node_put(clkspec.np);
    if (IS_ERR(clk)) {
    pr_err("%s: failed to get atl clock %d from provider\n",
    __func__, i);
    ret = PTR_ERR(clk);
    goto pm_put;
    }
    cdesc = to_atl_desc(__clk_get_hw(clk));
    cdesc.cinfo = cinfo;
    cdesc.id = i;
// Get configuration for the ATL instances
    snprintf(prop, sizeof(prop), "atl%u", i);
    cfg_node = of_get_child_by_name(node, prop);
    if (cfg_node) {
    ret = of_property_read_u32(cfg_node, "bws",
    &cdesc.bws);
    ret |= of_property_read_u32(cfg_node, "aws",
    &cdesc.aws);
    if (!ret) {
    cdesc.valid = true;
    atl_write(cinfo, DRA7_ATL_BWSMUX_REG(i),
    cdesc.bws);
    atl_write(cinfo, DRA7_ATL_AWSMUX_REG(i),
    cdesc.aws);
    }
    of_node_put(cfg_node);
    }
    cdesc.probed = true;
//
// Enable the clock if it has been asked prior to loading the
// hw driver
//
    if (cdesc.enabled)
    atl_clk_enable(__clk_get_hw(clk));
    }
    pm_put:
    pm_runtime_put_sync(cinfo.dev);
    return ret;
    }
    static const struct of_device_id of_dra7_atl_clk_match_tbl[] = {
    { .compatible = "ti,dra7-atl", },
    {},
    };
    static struct platform_driver dra7_atl_clk_driver = {
    .driver = {
    .name = "dra7-atl",
    .suppress_bind_attrs = true,
    .of_match_table = of_dra7_atl_clk_match_tbl,
    },
    .probe = of_dra7_atl_clk_probe,
    };
    builtin_platform_driver(dra7_atl_clk_driver);
