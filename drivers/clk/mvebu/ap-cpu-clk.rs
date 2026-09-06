//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/ap-cpu-clk.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Marvell Armada AP CPU Clock Controller
//
// Copyright (C) 2018 Marvell
//
// Omri Itach <omrii@marvell.com>
// Gregory Clement <gregory.clement@bootlin.com>
//

pub const AP806_CPU_CLUSTER0: c_int = 0;
pub const AP806_CPU_CLUSTER1: c_int = 1;
pub const AP806_CPUS_PER_CLUSTER: c_int = 2;
pub const APN806_CPU1_MASK: c_uint = 0x1;
pub const APN806_CLUSTER_NUM_OFFSET: c_int = 8;

pub const APN806_MAX_DIVIDER: c_int = 32;
//
// struct cpu_dfs_regs: CPU DFS register mapping
// @divider_reg: full integer ratio from PLL frequency to CPU clock frequency
// @force_reg: request to force new ratio regardless of relation to other clocks
// @ratio_reg: central request to switch ratios
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_dfs_regs {
    pub divider_reg: c_uint,
    pub force_reg: c_uint,
    pub ratio_reg: c_uint,
    pub ratio_state_reg: c_uint,
    pub divider_mask: c_uint,
    pub cluster_offset: c_uint,
    pub force_mask: c_uint,
    pub divider_offset: c_int,
    pub divider_ratio: c_int,
    pub ratio_offset: c_int,
    pub ratio_state_offset: c_int,
    pub ratio_state_cluster_offset: c_int,
}

// AP806 CPU DFS register mapping
pub const AP806_CA72MP2_0_PLL_CR_0_REG_OFFSET: c_uint = 0x278;
pub const AP806_CA72MP2_0_PLL_CR_1_REG_OFFSET: c_uint = 0x280;
pub const AP806_CA72MP2_0_PLL_CR_2_REG_OFFSET: c_uint = 0x284;
pub const AP806_CA72MP2_0_PLL_SR_REG_OFFSET: c_uint = 0xC94;
pub const AP806_CA72MP2_0_PLL_CR_CLUSTER_OFFSET: c_uint = 0x14;
pub const AP806_PLL_CR_0_CPU_CLK_DIV_RATIO_OFFSET: c_int = 0;
pub const AP806_PLL_CR_CPU_CLK_DIV_RATIO: c_int = 0;

    (0x3f << AP806_PLL_CR_0_CPU_CLK_DIV_RATIO_OFFSET)
pub const AP806_PLL_CR_0_CPU_CLK_RELOAD_FORCE_OFFSET: c_int = 24;

    (0x1 << AP806_PLL_CR_0_CPU_CLK_RELOAD_FORCE_OFFSET)
pub const AP806_PLL_CR_0_CPU_CLK_RELOAD_RATIO_OFFSET: c_int = 16;
pub const AP806_CA72MP2_0_PLL_RATIO_STABLE_OFFSET: c_int = 0;
pub const AP806_CA72MP2_0_PLL_RATIO_STATE: c_int = 11;
pub const STATUS_POLL_PERIOD_US: c_int = 1;
pub const STATUS_POLL_TIMEOUT_US: c_int = 1000000;

    static const struct cpu_dfs_regs ap806_dfs_regs = {
    .divider_reg = AP806_CA72MP2_0_PLL_CR_0_REG_OFFSET,
    .force_reg = AP806_CA72MP2_0_PLL_CR_1_REG_OFFSET,
    .ratio_reg = AP806_CA72MP2_0_PLL_CR_2_REG_OFFSET,
    .ratio_state_reg = AP806_CA72MP2_0_PLL_SR_REG_OFFSET,
    .divider_mask = AP806_PLL_CR_0_CPU_CLK_DIV_RATIO_MASK,
    .cluster_offset = AP806_CA72MP2_0_PLL_CR_CLUSTER_OFFSET,
    .force_mask = AP806_PLL_CR_0_CPU_CLK_RELOAD_FORCE_MASK,
    .divider_offset = AP806_PLL_CR_0_CPU_CLK_DIV_RATIO_OFFSET,
    .divider_ratio = AP806_PLL_CR_CPU_CLK_DIV_RATIO,
    .ratio_offset = AP806_PLL_CR_0_CPU_CLK_RELOAD_RATIO_OFFSET,
    .ratio_state_offset = AP806_CA72MP2_0_PLL_RATIO_STABLE_OFFSET,
    .ratio_state_cluster_offset = AP806_CA72MP2_0_PLL_RATIO_STABLE_OFFSET,
    };
// AP807 CPU DFS register mapping
pub const AP807_DEVICE_GENERAL_CONTROL_10_REG_OFFSET: c_uint = 0x278;
pub const AP807_DEVICE_GENERAL_CONTROL_11_REG_OFFSET: c_uint = 0x27c;
pub const AP807_DEVICE_GENERAL_STATUS_6_REG_OFFSET: c_uint = 0xc98;
pub const AP807_CA72MP2_0_PLL_CR_CLUSTER_OFFSET: c_uint = 0x8;
pub const AP807_PLL_CR_0_CPU_CLK_DIV_RATIO_OFFSET: c_int = 18;

    (0x3f << AP807_PLL_CR_0_CPU_CLK_DIV_RATIO_OFFSET)
pub const AP807_PLL_CR_1_CPU_CLK_DIV_RATIO_OFFSET: c_int = 12;

    (0x3f << AP807_PLL_CR_1_CPU_CLK_DIV_RATIO_OFFSET)
pub const AP807_PLL_CR_CPU_CLK_DIV_RATIO: c_int = 3;
pub const AP807_PLL_CR_0_CPU_CLK_RELOAD_FORCE_OFFSET: c_int = 0;

    (0x3 << AP807_PLL_CR_0_CPU_CLK_RELOAD_FORCE_OFFSET)
pub const AP807_PLL_CR_0_CPU_CLK_RELOAD_RATIO_OFFSET: c_int = 6;
pub const AP807_CA72MP2_0_PLL_CLKDIV_RATIO_STABLE_OFFSET: c_int = 20;
pub const AP807_CA72MP2_0_PLL_CLKDIV_RATIO_STABLE_CLUSTER_OFFSET: c_int = 3;
    static const struct cpu_dfs_regs ap807_dfs_regs = {
    .divider_reg = AP807_DEVICE_GENERAL_CONTROL_10_REG_OFFSET,
    .force_reg = AP807_DEVICE_GENERAL_CONTROL_11_REG_OFFSET,
    .ratio_reg = AP807_DEVICE_GENERAL_CONTROL_11_REG_OFFSET,
    .ratio_state_reg = AP807_DEVICE_GENERAL_STATUS_6_REG_OFFSET,
    .divider_mask = AP807_PLL_CR_0_CPU_CLK_DIV_RATIO_MASK,
    .cluster_offset = AP807_CA72MP2_0_PLL_CR_CLUSTER_OFFSET,
    .force_mask = AP807_PLL_CR_0_CPU_CLK_RELOAD_FORCE_MASK,
    .divider_offset = AP807_PLL_CR_0_CPU_CLK_DIV_RATIO_OFFSET,
    .divider_ratio = AP807_PLL_CR_CPU_CLK_DIV_RATIO,
    .ratio_offset = AP807_PLL_CR_0_CPU_CLK_RELOAD_RATIO_OFFSET,
    .ratio_state_offset = AP807_CA72MP2_0_PLL_CLKDIV_RATIO_STABLE_OFFSET,
    .ratio_state_cluster_offset =
    AP807_CA72MP2_0_PLL_CLKDIV_RATIO_STABLE_CLUSTER_OFFSET
    };
//
// struct ap806_clk: CPU cluster clock controller instance
// @cluster: Cluster clock controller index
// @clk_name: Cluster clock controller name
// @dev : Cluster clock device
// @hw: HW specific structure of Cluster clock controller
// @pll_cr_base: CA72MP2 Register base (Device Sample at Reset register)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_cpu_clk {
    pub cluster: c_uint,
    pub clk_name: *const c_char,
    pub dev: *mut device,
    pub hw: clk_hw,
    pub pll_cr_base: *mut regmap,
    pub pll_regs: *const cpu_dfs_regs,
}

    static unsigned long ap_cpu_clk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct ap_cpu_clk *clk = to_ap_cpu_clk(hw);
    unsigned int cpu_clkdiv_reg;
    int cpu_clkdiv_ratio;
    cpu_clkdiv_reg = clk.pll_regs.divider_reg +
    (clk.cluster * clk.pll_regs.cluster_offset);
    regmap_read(clk.pll_cr_base, cpu_clkdiv_reg, &cpu_clkdiv_ratio);
    cpu_clkdiv_ratio &= clk.pll_regs.divider_mask;
    cpu_clkdiv_ratio >>= clk.pll_regs.divider_offset;
    return parent_rate / cpu_clkdiv_ratio;
    }
    static int ap_cpu_clk_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct ap_cpu_clk *clk = to_ap_cpu_clk(hw);
    int ret, reg, divider = parent_rate / rate;
    unsigned int cpu_clkdiv_reg, cpu_force_reg, cpu_ratio_reg, stable_bit;
    cpu_clkdiv_reg = clk.pll_regs.divider_reg +
    (clk.cluster * clk.pll_regs.cluster_offset);
    cpu_force_reg = clk.pll_regs.force_reg +
    (clk.cluster * clk.pll_regs.cluster_offset);
    cpu_ratio_reg = clk.pll_regs.ratio_reg +
    (clk.cluster * clk.pll_regs.cluster_offset);
    regmap_read(clk.pll_cr_base, cpu_clkdiv_reg, &reg);
    reg &= ~(clk.pll_regs.divider_mask);
    reg |= (divider << clk.pll_regs.divider_offset);
//
// AP807 CPU divider has two channels with ratio 1:3 and divider_ratio
// is 1. Otherwise, in the case of the AP806, divider_ratio is 0.
//
    if (clk.pll_regs.divider_ratio) {
    reg &= ~(AP807_PLL_CR_1_CPU_CLK_DIV_RATIO_MASK);
    reg |= ((divider * clk.pll_regs.divider_ratio) <<
    AP807_PLL_CR_1_CPU_CLK_DIV_RATIO_OFFSET);
    }
    regmap_write(clk.pll_cr_base, cpu_clkdiv_reg, reg);
    regmap_update_bits(clk.pll_cr_base, cpu_force_reg,
    clk.pll_regs.force_mask,
    clk.pll_regs.force_mask);
    regmap_update_bits(clk.pll_cr_base, cpu_ratio_reg,
    BIT(clk.pll_regs.ratio_offset),
    BIT(clk.pll_regs.ratio_offset));
    stable_bit = BIT(clk.pll_regs.ratio_state_offset +
    clk.cluster *
    clk.pll_regs.ratio_state_cluster_offset);
    ret = regmap_read_poll_timeout(clk.pll_cr_base,
    clk.pll_regs.ratio_state_reg, reg,
    reg & stable_bit, STATUS_POLL_PERIOD_US,
    STATUS_POLL_TIMEOUT_US);
    if (ret)
    return ret;
    regmap_update_bits(clk.pll_cr_base, cpu_ratio_reg,
    BIT(clk.pll_regs.ratio_offset), 0);
    return 0;
    }
    static int ap_cpu_clk_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    let mut divider: c_int = req.best_parent_rate / req.rate;
    divider = min(divider, APN806_MAX_DIVIDER);
    req.rate = req.best_parent_rate / divider;
    return 0;
    }
    static const struct clk_ops ap_cpu_clk_ops = {
    .recalc_rate	= ap_cpu_clk_recalc_rate,
    .determine_rate = ap_cpu_clk_determine_rate,
    .set_rate	= ap_cpu_clk_set_rate,
    };
#[no_mangle]
unsafe extern "C" fn ap_cpu_clock_probe(pdev: *mut platform_device) -> c_int {
    static int ap_cpu_clock_probe(struct platform_device *pdev)
    {
    int ret, nclusters = 0, cluster_index = 0;
    struct device *dev = &pdev.dev;
    struct device_node *dn, *np = dev.of_node;
    struct clk_hw_onecell_data *ap_cpu_data;
    struct ap_cpu_clk *ap_cpu_clk;
    struct regmap *regmap;
    regmap = syscon_node_to_regmap(np.parent);
    if (IS_ERR(regmap)) {
    pr_err("cannot get pll_cr_base regmap\n");
    return PTR_ERR(regmap);
    }
//
// AP806 has 4 cpus and DFS for AP806 is controlled per
// cluster (2 CPUs per cluster), cpu0 and cpu1 are fixed to
// cluster0 while cpu2 and cpu3 are fixed to cluster1 whether
// they are enabled or not.  Since cpu0 is the boot cpu, then
// cluster0 must exist.  If cpu2 or cpu3 is enabled, cluster1
// will exist and the cluster number is 2; otherwise the
// cluster number is 1.
//
    nclusters = 1;
    for_each_of_cpu_node(dn) {
    u64 cpu;
    cpu = of_get_cpu_hwid(dn, 0);
    if (WARN_ON(cpu == OF_BAD_ADDR)) {
    of_node_put(dn);
    return -EINVAL;
    }
// If cpu2 or cpu3 is enabled
    if (cpu & APN806_CLUSTER_NUM_MASK) {
    nclusters = 2;
    of_node_put(dn);
    break;
    }
    }
//
// DFS for AP806 is controlled per cluster (2 CPUs per cluster),
// so allocate structs per cluster
//
    ap_cpu_clk = devm_kcalloc(dev, nclusters, sizeof(*ap_cpu_clk),
    GFP_KERNEL);
    if (!ap_cpu_clk)
    return -ENOMEM;
    ap_cpu_data = devm_kzalloc(dev, struct_size(ap_cpu_data, hws,
    nclusters),
    GFP_KERNEL);
    if (!ap_cpu_data)
    return -ENOMEM;
    for_each_of_cpu_node(dn) {
    char *clk_name = "cpu-cluster-0";
    struct clk_init_data init;
    const char *parent_name;
    struct clk *parent;
    u64 cpu;
    cpu = of_get_cpu_hwid(dn, 0);
    if (WARN_ON(cpu == OF_BAD_ADDR)) {
    of_node_put(dn);
    return -EINVAL;
    }
    cluster_index = cpu & APN806_CLUSTER_NUM_MASK;
    cluster_index >>= APN806_CLUSTER_NUM_OFFSET;
// Initialize once for one cluster
    if (ap_cpu_data.hws[cluster_index])
    continue;
    parent = of_clk_get(np, cluster_index);
    if (IS_ERR(parent)) {
    dev_err(dev, "Could not get the clock parent\n");
    of_node_put(dn);
    return -EINVAL;
    }
    parent_name =  __clk_get_name(parent);
    clk_name[12] += cluster_index;
    ap_cpu_clk[cluster_index].clk_name =
    ap_cp_unique_name(dev, np.parent, clk_name);
    ap_cpu_clk[cluster_index].cluster = cluster_index;
    ap_cpu_clk[cluster_index].pll_cr_base = regmap;
    ap_cpu_clk[cluster_index].hw.init = &init;
    ap_cpu_clk[cluster_index].dev = dev;
    ap_cpu_clk[cluster_index].pll_regs = of_device_get_match_data(&pdev.dev);
    init.name = ap_cpu_clk[cluster_index].clk_name;
    init.ops = &ap_cpu_clk_ops;
    init.num_parents = 1;
    init.parent_names = &parent_name;
    ret = devm_clk_hw_register(dev, &ap_cpu_clk[cluster_index].hw);
    if (ret) {
    of_node_put(dn);
    return ret;
    }
    ap_cpu_data.hws[cluster_index] = &ap_cpu_clk[cluster_index].hw;
    }
    ap_cpu_data.num = cluster_index + 1;
    ret = of_clk_add_hw_provider(np, of_clk_hw_onecell_get, ap_cpu_data);
    if (ret)
    dev_err(dev, "failed to register OF clock provider\n");
    return ret;
    }
    static const struct of_device_id ap_cpu_clock_of_match[] = {
    {
    .compatible = "marvell,ap806-cpu-clock",
    .data = &ap806_dfs_regs,
    },
    {
    .compatible = "marvell,ap807-cpu-clock",
    .data = &ap807_dfs_regs,
    },
    { }
    };
    static struct platform_driver ap_cpu_clock_driver = {
    .probe = ap_cpu_clock_probe,
    .driver		= {
    .name	= "marvell-ap-cpu-clock",
    .of_match_table = ap_cpu_clock_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(ap_cpu_clock_driver);
