//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/clk-cpu.c
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
// Marvell MVEBU CPU clock handling.
//
// Copyright (C) 2012 Marvell
//
// Gregory CLEMENT <gregory.clement@free-electrons.com>
//

pub const SYS_CTRL_CLK_DIVIDER_CTRL_OFFSET: c_uint = 0x0;
pub const SYS_CTRL_CLK_DIVIDER_CTRL_RESET_ALL: c_uint = 0xff;
pub const SYS_CTRL_CLK_DIVIDER_CTRL_RESET_SHIFT: c_int = 8;
pub const SYS_CTRL_CLK_DIVIDER_CTRL2_OFFSET: c_uint = 0x8;
pub const SYS_CTRL_CLK_DIVIDER_CTRL2_NBCLK_RATIO_SHIFT: c_int = 16;
pub const SYS_CTRL_CLK_DIVIDER_VALUE_OFFSET: c_uint = 0xC;
pub const SYS_CTRL_CLK_DIVIDER_MASK: c_uint = 0x3F;
pub const PMU_DFS_RATIO_SHIFT: c_int = 16;
pub const PMU_DFS_RATIO_MASK: c_uint = 0x3F;
pub const MAX_CPU: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_clk {
    pub hw: clk_hw,
    pub cpu: c_int,
    pub clk_name: *const c_char,
    pub parent_name: *const c_char,
    pub reg_base: *mut void __iomem,
    pub pmu_dfs: *mut void __iomem,
}

    static struct clk **clks;
    static struct clk_onecell_data clk_data;

    static unsigned long clk_cpu_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct cpu_clk *cpuclk = to_cpu_clk(hwclk);
    u32 reg, div;
    reg = readl(cpuclk.reg_base + SYS_CTRL_CLK_DIVIDER_VALUE_OFFSET);
    div = (reg >> (cpuclk.cpu * 8)) & SYS_CTRL_CLK_DIVIDER_MASK;
    return parent_rate / div;
    }
    static int clk_cpu_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
// Valid ratio are 1:1, 1:2 and 1:3
    u32 div;
    div = req.best_parent_rate / req.rate;
    if (div == 0)
    div = 1;
#[no_mangle]
pub unsafe extern "C" fn if(3: div >) -> else {
    else if (div > 3)
    div = 3;
    req.rate = req.best_parent_rate / div;
    return 0;
    }
    static int clk_cpu_off_set_rate(struct clk_hw *hwclk, unsigned long rate,
    unsigned long parent_rate)
    {
    struct cpu_clk *cpuclk = to_cpu_clk(hwclk);
    u32 reg, div;
    u32 reload_mask;
    div = parent_rate / rate;
    reg = (readl(cpuclk.reg_base + SYS_CTRL_CLK_DIVIDER_VALUE_OFFSET)
    & (~(SYS_CTRL_CLK_DIVIDER_MASK << (cpuclk.cpu * 8))))
    | (div << (cpuclk.cpu * 8));
    writel(reg, cpuclk.reg_base + SYS_CTRL_CLK_DIVIDER_VALUE_OFFSET);
// Set clock divider reload smooth bit mask
    reload_mask = 1 << (20 + cpuclk.cpu);
    reg = readl(cpuclk.reg_base + SYS_CTRL_CLK_DIVIDER_CTRL_OFFSET)
    | reload_mask;
    writel(reg, cpuclk.reg_base + SYS_CTRL_CLK_DIVIDER_CTRL_OFFSET);
// Now trigger the clock update
    reg = readl(cpuclk.reg_base + SYS_CTRL_CLK_DIVIDER_CTRL_OFFSET)
    | 1 << 24;
    writel(reg, cpuclk.reg_base + SYS_CTRL_CLK_DIVIDER_CTRL_OFFSET);
// Wait for clocks to settle down then clear reload request
    udelay(1000);
    reg &= ~(reload_mask | 1 << 24);
    writel(reg, cpuclk.reg_base + SYS_CTRL_CLK_DIVIDER_CTRL_OFFSET);
    udelay(1000);
    return 0;
    }
    static int clk_cpu_on_set_rate(struct clk_hw *hwclk, unsigned long rate,
    unsigned long parent_rate)
    {
    u32 reg;
    unsigned long fabric_div, target_div, cur_rate;
    struct cpu_clk *cpuclk = to_cpu_clk(hwclk);
//
// PMU DFS registers are not mapped, Device Tree does not
// describes them. We cannot change the frequency dynamically.
//
    if (!cpuclk.pmu_dfs)
    return -ENODEV;
    cur_rate = clk_hw_get_rate(hwclk);
    reg = readl(cpuclk.reg_base + SYS_CTRL_CLK_DIVIDER_CTRL2_OFFSET);
    fabric_div = (reg >> SYS_CTRL_CLK_DIVIDER_CTRL2_NBCLK_RATIO_SHIFT) &
    SYS_CTRL_CLK_DIVIDER_MASK;
// Frequency is going up
    if (rate == 2 * cur_rate)
    target_div = fabric_div / 2;
// Frequency is going down
    else
    target_div = fabric_div;
    if (target_div == 0)
    target_div = 1;
    reg = readl(cpuclk.pmu_dfs);
    reg &= ~(PMU_DFS_RATIO_MASK << PMU_DFS_RATIO_SHIFT);
    reg |= (target_div << PMU_DFS_RATIO_SHIFT);
    writel(reg, cpuclk.pmu_dfs);
    reg = readl(cpuclk.reg_base + SYS_CTRL_CLK_DIVIDER_CTRL_OFFSET);
    reg |= (SYS_CTRL_CLK_DIVIDER_CTRL_RESET_ALL <<
    SYS_CTRL_CLK_DIVIDER_CTRL_RESET_SHIFT);
    writel(reg, cpuclk.reg_base + SYS_CTRL_CLK_DIVIDER_CTRL_OFFSET);
    return mvebu_pmsu_dfs_request(cpuclk.cpu);
    }
    static int clk_cpu_set_rate(struct clk_hw *hwclk, unsigned long rate,
    unsigned long parent_rate)
    {
    if (clk_hw_is_enabled(hwclk))
    return clk_cpu_on_set_rate(hwclk, rate, parent_rate);
    else
    return clk_cpu_off_set_rate(hwclk, rate, parent_rate);
    }
    static const struct clk_ops cpu_ops = {
    .recalc_rate = clk_cpu_recalc_rate,
    .determine_rate = clk_cpu_determine_rate,
    .set_rate = clk_cpu_set_rate,
    };
#[no_mangle]
unsafe extern "C" fn of_cpu_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_cpu_clk_setup(struct device_node *node)
    {
    struct cpu_clk *cpuclk;
    void __iomem *clock_complex_base = of_iomap(node, 0);
    void __iomem *pmu_dfs_base = of_iomap(node, 1);
    let mut ncpus: c_int = num_possible_cpus();
    int cpu;
    if (clock_complex_base == core::ptr::null_mut()) {
    pr_err("%s: clock-complex base register not set\n",
    __func__);
    return;
    }
    if (pmu_dfs_base == core::ptr::null_mut())
    pr_warn("%s: pmu-dfs base register not set, dynamic frequency scaling not available\n",
    __func__);
    cpuclk = kzalloc_objs(*cpuclk, ncpus);
    if (WARN_ON(!cpuclk))
    goto cpuclk_out;
    clks = kzalloc_objs(*clks, ncpus);
    if (WARN_ON(!clks))
    goto clks_out;
    for_each_possible_cpu(cpu) {
    struct clk_init_data init;
    struct clk *clk;
    char *clk_name = kzalloc(5, GFP_KERNEL);
    if (WARN_ON(!clk_name))
    goto bail_out;
    sprintf(clk_name, "cpu%d", cpu);
    cpuclk[cpu].parent_name = of_clk_get_parent_name(node, 0);
    cpuclk[cpu].clk_name = clk_name;
    cpuclk[cpu].cpu = cpu;
    cpuclk[cpu].reg_base = clock_complex_base;
    if (pmu_dfs_base)
    cpuclk[cpu].pmu_dfs = pmu_dfs_base + 4 * cpu;
    cpuclk[cpu].hw.init = &init;
    init.name = cpuclk[cpu].clk_name;
    init.ops = &cpu_ops;
    init.flags = 0;
    init.parent_names = &cpuclk[cpu].parent_name;
    init.num_parents = 1;
    clk = clk_register(core::ptr::null_mut(), &cpuclk[cpu].hw);
    if (WARN_ON(IS_ERR(clk)))
    goto bail_out;
    clks[cpu] = clk;
    }
    clk_data.clk_num = MAX_CPU;
    clk_data.clks = clks;
    of_clk_add_provider(node, of_clk_src_onecell_get, &clk_data);
    return;
    bail_out:
    kfree(clks);
    while(ncpus--)
    kfree(cpuclk[ncpus].clk_name);
    clks_out:
    kfree(cpuclk);
    cpuclk_out:
    iounmap(clock_complex_base);
    }
    CLK_OF_DECLARE(armada_xp_cpu_clock, "marvell,armada-xp-cpu-clock",
    of_cpu_clk_setup);
#[no_mangle]
unsafe extern "C" fn of_mv98dx3236_cpu_clk_setup(node: *mut device_node) -> void __init {
    static void __init of_mv98dx3236_cpu_clk_setup(struct device_node *node)
    {
    of_clk_add_provider(node, of_clk_src_simple_get, core::ptr::null_mut());
    }
    CLK_OF_DECLARE(mv98dx3236_cpu_clock, "marvell,mv98dx3236-cpu-clock",
    of_mv98dx3236_cpu_clk_setup);
