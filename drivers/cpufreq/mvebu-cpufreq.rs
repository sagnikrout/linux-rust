//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/mvebu-cpufreq.c
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
// CPUFreq support for Armada 370/XP platforms.
//
// Copyright (C) 2012-2016 Marvell
//
// Yehuda Yitschak <yehuday@marvell.com>
// Gregory Clement <gregory.clement@free-electrons.com>
// Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
//

#[no_mangle]
unsafe extern "C" fn armada_xp_pmsu_cpufreq_init() -> int __init {
    static int __init armada_xp_pmsu_cpufreq_init(void)
    {
    struct device_node *np;
    struct resource res;
    int ret, cpu;
    if (!of_machine_is_compatible("marvell,armadaxp"))
    return 0;
//
// In order to have proper cpufreq handling, we need to ensure
// that the Device Tree description of the CPU clock includes
// the definition of the PMU DFS registers. If not, we do not
// register the clock notifier and the cpufreq driver. This
// piece of code is only for compatibility with old Device
// Trees.
//
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "marvell,armada-xp-cpu-clock");
    if (!np)
    return 0;
    ret = of_address_to_resource(np, 1, &res);
    if (ret) {
    pr_warn(FW_WARN "not enabling cpufreq, deprecated armada-xp-cpu-clock binding\n");
    of_node_put(np);
    return 0;
    }
    of_node_put(np);
//
// For each CPU, this loop registers the operating points
// supported (which are the nominal CPU frequency and half of
// it), and registers the clock notifier that will take care
// of doing the PMSU part of a frequency transition.
//
    for_each_present_cpu(cpu) {
    struct device *cpu_dev;
    struct clk *clk;
    int ret;
    cpu_dev = get_cpu_device(cpu);
    if (!cpu_dev) {
    pr_err("Cannot get CPU %d\n", cpu);
    continue;
    }
    clk = clk_get(cpu_dev, core::ptr::null_mut());
    if (IS_ERR(clk)) {
    pr_err("Cannot get clock for CPU %d\n", cpu);
    return PTR_ERR(clk);
    }
    ret = dev_pm_opp_add(cpu_dev, clk_get_rate(clk), 0);
    if (ret) {
    clk_put(clk);
    return ret;
    }
    ret = dev_pm_opp_add(cpu_dev, clk_get_rate(clk) / 2, 0);
    if (ret) {
    dev_pm_opp_remove(cpu_dev, clk_get_rate(clk));
    clk_put(clk);
    dev_err(cpu_dev, "Failed to register OPPs\n");
    return ret;
    }
    ret = dev_pm_opp_set_sharing_cpus(cpu_dev,
    cpumask_of(cpu_dev.id));
    if (ret)
    dev_err(cpu_dev, "%s: failed to mark OPPs as shared: %d\n",
    __func__, ret);
    clk_put(clk);
    }
    platform_device_register_simple("cpufreq-dt", -1, core::ptr::null_mut(), 0);
    return 0;
    }
    device_initcall(armada_xp_pmsu_cpufreq_init);
