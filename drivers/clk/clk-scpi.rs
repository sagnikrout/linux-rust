//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-scpi.c
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
// System Control and Power Interface (SCPI) Protocol based clock driver
//
// Copyright (C) 2015 ARM Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scpi_clk {
    pub id: u32,
    pub hw: clk_hw,
    pub info: *mut scpi_dvfs_info,
    pub scpi_ops: *mut scpi_ops,
}

    static struct platform_device *cpufreq_dev;
    static unsigned long scpi_clk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct scpi_clk *clk = to_scpi_clk(hw);
    return clk.scpi_ops.clk_get_val(clk.id);
    }
    static int scpi_clk_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct scpi_clk *clk = to_scpi_clk(hw);
    return clk.scpi_ops.clk_set_val(clk.id, rate);
    }
    static const struct clk_ops scpi_clk_ops = {
    .recalc_rate = scpi_clk_recalc_rate,
    .determine_rate = clk_determine_rate_noop,
    .set_rate = scpi_clk_set_rate,
    };
// find closest match to given frequency in OPP table
#[no_mangle]
unsafe extern "C" fn __scpi_dvfs_round_rate(clk: *mut scpi_clk, rate: c_ulong) -> c_long {
    static long __scpi_dvfs_round_rate(struct scpi_clk *clk, unsigned long rate)
    {
    int idx;
    let mut fmin: c_ulong = 0, fmax = ~0, ftmp;
    const struct scpi_opp *opp = clk.info.opps;
    for (idx = 0; idx < clk.info.count; idx++, opp++) {
    ftmp = opp.freq;
    if (ftmp >= rate) {
    if (ftmp <= fmax)
    fmax = ftmp;
    break;
    } else if (ftmp >= fmin) {
    fmin = ftmp;
    }
    }
    return fmax != ~0 ? fmax : fmin;
    }
    static unsigned long scpi_dvfs_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct scpi_clk *clk = to_scpi_clk(hw);
    let mut idx: c_int = clk.scpi_ops.dvfs_get_idx(clk.id);
    const struct scpi_opp *opp;
    if (idx < 0)
    return 0;
    opp = clk.info.opps + idx;
    return opp.freq;
    }
    static int scpi_dvfs_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct scpi_clk *clk = to_scpi_clk(hw);
    req.rate = __scpi_dvfs_round_rate(clk, req.rate);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __scpi_find_dvfs_index(clk: *mut scpi_clk, rate: c_ulong) -> c_int {
    static int __scpi_find_dvfs_index(struct scpi_clk *clk, unsigned long rate)
    {
    int idx, max_opp = clk.info.count;
    const struct scpi_opp *opp = clk.info.opps;
    for (idx = 0; idx < max_opp; idx++, opp++)
    if (opp.freq == rate)
    return idx;
    return -EINVAL;
    }
    static int scpi_dvfs_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct scpi_clk *clk = to_scpi_clk(hw);
    let mut ret: c_int = __scpi_find_dvfs_index(clk, rate);
    if (ret < 0)
    return ret;
    return clk.scpi_ops.dvfs_set_idx(clk.id, (u8)ret);
    }
    static const struct clk_ops scpi_dvfs_ops = {
    .recalc_rate = scpi_dvfs_recalc_rate,
    .determine_rate = scpi_dvfs_determine_rate,
    .set_rate = scpi_dvfs_set_rate,
    };
    static const struct of_device_id scpi_clk_match[] __maybe_unused = {
    { .compatible = "arm,scpi-dvfs-clocks", .data = &scpi_dvfs_ops, },
    { .compatible = "arm,scpi-variable-clocks", .data = &scpi_clk_ops, },
    {}
    };
    static int
    scpi_clk_ops_init(struct device *dev, const struct of_device_id *match,
    struct scpi_clk *sclk, const char *name)
    {
    struct clk_init_data init;
    let mut min: c_ulong = 0, max = 0;
    int ret;
    init.name = name;
    init.flags = 0;
    init.num_parents = 0;
    init.ops = match.data;
    sclk.hw.init = &init;
    sclk.scpi_ops = get_scpi_ops();
    if (init.ops == &scpi_dvfs_ops) {
    sclk.info = sclk.scpi_ops.dvfs_get_info(sclk.id);
    if (IS_ERR(sclk.info))
    return PTR_ERR(sclk.info);
    } else if (init.ops == &scpi_clk_ops) {
    if (sclk.scpi_ops.clk_get_range(sclk.id, &min, &max) || !max)
    return -EINVAL;
    } else {
    return -EINVAL;
    }
    ret = devm_clk_hw_register(dev, &sclk.hw);
    if (!ret && max)
    clk_hw_set_rate_range(&sclk.hw, min, max);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scpi_clk_data {
    pub clk: *mut scpi_clk,
    pub clk_num: c_uint,
}

    static struct clk_hw *
    scpi_of_clk_src_get(struct of_phandle_args *clkspec, void *data)
    {
    struct scpi_clk *sclk;
    struct scpi_clk_data *clk_data = data;
    let mut idx: c_uint = clkspec.args[0], count;
    for (count = 0; count < clk_data.clk_num; count++) {
    sclk = clk_data.clk[count];
    if (idx == sclk.id)
    return &sclk.hw;
    }
    return ERR_PTR(-EINVAL);
    }
    static int scpi_clk_add(struct device *dev, struct device_node *np,
    const struct of_device_id *match)
    {
    int idx, count, err;
    struct scpi_clk_data *clk_data;
    count = of_property_count_strings(np, "clock-output-names");
    if (count < 0) {
    dev_err(dev, "%pOFn: invalid clock output count\n", np);
    return -EINVAL;
    }
    clk_data = devm_kmalloc(dev, sizeof(*clk_data), GFP_KERNEL);
    if (!clk_data)
    return -ENOMEM;
    clk_data.clk_num = count;
    clk_data.clk = devm_kcalloc(dev, count, sizeof(*clk_data.clk),
    GFP_KERNEL);
    if (!clk_data.clk)
    return -ENOMEM;
    for (idx = 0; idx < count; idx++) {
    struct scpi_clk *sclk;
    const char *name;
    u32 val;
    sclk = devm_kzalloc(dev, sizeof(*sclk), GFP_KERNEL);
    if (!sclk)
    return -ENOMEM;
    if (of_property_read_string_index(np, "clock-output-names",
    idx, &name)) {
    dev_err(dev, "invalid clock name @ %pOFn\n", np);
    return -EINVAL;
    }
    if (of_property_read_u32_index(np, "clock-indices",
    idx, &val)) {
    dev_err(dev, "invalid clock index @ %pOFn\n", np);
    return -EINVAL;
    }
    sclk.id = val;
    err = scpi_clk_ops_init(dev, match, sclk, name);
    if (err) {
    dev_err(dev, "failed to register clock '%s'\n", name);
    return err;
    }
    dev_dbg(dev, "Registered clock '%s'\n", name);
    clk_data.clk[idx] = sclk;
    }
    return of_clk_add_hw_provider(np, scpi_of_clk_src_get, clk_data);
    }
#[no_mangle]
unsafe extern "C" fn scpi_clocks_remove(pdev: *mut platform_device) {
    static void scpi_clocks_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *child, *np = dev.of_node;
    if (cpufreq_dev) {
    platform_device_unregister(cpufreq_dev);
    cpufreq_dev = core::ptr::null_mut();
    }
    for_each_available_child_of_node(np, child)
    of_clk_del_provider(child);
    }
#[no_mangle]
unsafe extern "C" fn scpi_clocks_probe(pdev: *mut platform_device) -> c_int {
    static int scpi_clocks_probe(struct platform_device *pdev)
    {
    int ret;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    const struct of_device_id *match;
    if (!get_scpi_ops())
    return -ENXIO;
    for_each_available_child_of_node_scoped(np, child) {
    match = of_match_node(scpi_clk_match, child);
    if (!match)
    continue;
    ret = scpi_clk_add(dev, child, match);
    if (ret) {
    scpi_clocks_remove(pdev);
    return ret;
    }
    if (match.data != &scpi_dvfs_ops)
    continue;
// Add the virtual cpufreq device if it's DVFS clock provider
    cpufreq_dev = platform_device_register_simple("scpi-cpufreq",
    -1, core::ptr::null_mut(), 0);
    if (IS_ERR(cpufreq_dev))
    pr_warn("unable to register cpufreq device");
    }
    return 0;
    }
    static const struct of_device_id scpi_clocks_ids[] = {
    { .compatible = "arm,scpi-clocks", },
    {}
    };
    MODULE_DEVICE_TABLE(of, scpi_clocks_ids);
    static struct platform_driver scpi_clocks_driver = {
    .driver	= {
    .name = "scpi_clocks",
    .of_match_table = scpi_clocks_ids,
    },
    .probe = scpi_clocks_probe,
    .remove = scpi_clocks_remove,
    };
    module_platform_driver(scpi_clocks_driver);
    MODULE_AUTHOR("Sudeep Holla <sudeep.holla@arm.com>");
    MODULE_DESCRIPTION("ARM SCPI clock driver");
    MODULE_LICENSE("GPL v2");
