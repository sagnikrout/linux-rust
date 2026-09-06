//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/qoriq-cpufreq.c
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
// Copyright 2013 Freescale Semiconductor, Inc.
//
// CPU Frequency Scaling driver for Freescale QorIQ SoCs.
//

//
// struct cpu_data
// @pclk: the parent clock of cpu
// @table: frequency table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_data {
    pub pclk: *mut clk,
    pub table: *mut cpufreq_frequency_table,
}

//
// struct soc_data - SoC specific data
// @flags: SOC_xxx
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_data {
    pub flags: u32,
}

#[no_mangle]
unsafe extern "C" fn get_bus_freq() -> u32 {
    static u32 get_bus_freq(void)
    {
    struct device_node *soc;
    u32 sysfreq;
    struct clk *pltclk;
    int ret;
// get platform freq by searching bus-frequency property
    soc = of_find_node_by_type(core::ptr::null_mut(), "soc");
    if (soc) {
    ret = of_property_read_u32(soc, "bus-frequency", &sysfreq);
    of_node_put(soc);
    if (!ret)
    return sysfreq;
    }
// get platform freq by its clock name
    pltclk = clk_get(core::ptr::null_mut(), "cg-pll0-div1");
    if (IS_ERR(pltclk)) {
    pr_err("%s: can't get bus frequency %ld\n",
    __func__, PTR_ERR(pltclk));
    return PTR_ERR(pltclk);
    }
    return clk_get_rate(pltclk);
    }
    static struct clk *cpu_to_clk(int cpu)
    {
    struct device_node *np;
    struct clk *clk;
    if (!cpu_present(cpu))
    return core::ptr::null_mut();
    np = of_get_cpu_node(cpu, core::ptr::null_mut());
    if (!np)
    return core::ptr::null_mut();
    clk = of_clk_get(np, 0);
    of_node_put(np);
    return clk;
    }
// traverse cpu nodes to get cpu mask of sharing clock wire
#[no_mangle]
unsafe extern "C" fn set_affected_cpus(policy: *mut cpufreq_policy) {
    static void set_affected_cpus(struct cpufreq_policy *policy)
    {
    struct cpumask *dstp = policy.cpus;
    struct clk *clk;
    int i;
    for_each_present_cpu(i) {
    clk = cpu_to_clk(i);
    if (IS_ERR(clk)) {
    pr_err("%s: no clock for cpu %d\n", __func__, i);
    continue;
    }
    if (clk_is_match(policy.clk, clk))
    cpumask_set_cpu(i, dstp);
    }
    }
// reduce the duplicated frequencies in frequency table
    static void freq_table_redup(struct cpufreq_frequency_table *freq_table,
    int count)
    {
    int i, j;
    for (i = 1; i < count; i++) {
    for (j = 0; j < i; j++) {
    if (freq_table[j].frequency == CPUFREQ_ENTRY_INVALID ||
    freq_table[j].frequency !=
    freq_table[i].frequency)
    continue;
    freq_table[i].frequency = CPUFREQ_ENTRY_INVALID;
    break;
    }
    }
    }
// sort the frequencies in frequency table in descenting order
    static void freq_table_sort(struct cpufreq_frequency_table *freq_table,
    int count)
    {
    int i, j, ind;
    unsigned int freq, max_freq;
    struct cpufreq_frequency_table table;
    for (i = 0; i < count - 1; i++) {
    max_freq = freq_table[i].frequency;
    ind = i;
    for (j = i + 1; j < count; j++) {
    freq = freq_table[j].frequency;
    if (freq == CPUFREQ_ENTRY_INVALID ||
    freq <= max_freq)
    continue;
    ind = j;
    max_freq = freq;
    }
    if (ind != i) {
// exchange the frequencies
    table.driver_data = freq_table[i].driver_data;
    table.frequency = freq_table[i].frequency;
    freq_table[i].driver_data = freq_table[ind].driver_data;
    freq_table[i].frequency = freq_table[ind].frequency;
    freq_table[ind].driver_data = table.driver_data;
    freq_table[ind].frequency = table.frequency;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn qoriq_cpufreq_cpu_init(policy: *mut cpufreq_policy) -> c_int {
    static int qoriq_cpufreq_cpu_init(struct cpufreq_policy *policy)
    {
    struct device_node *np;
    int i, count;
    u32 freq;
    struct clk *clk;
    const struct clk_hw *hwclk;
    struct cpufreq_frequency_table *table;
    struct cpu_data *data;
    let mut cpu: c_uint = policy.cpu;
    u64 u64temp;
    np = of_get_cpu_node(cpu, core::ptr::null_mut());
    if (!np)
    return -ENODEV;
    data = kzalloc_obj(*data);
    if (!data)
    goto err_np;
    policy.clk = of_clk_get(np, 0);
    if (IS_ERR(policy.clk)) {
    pr_err("%s: no clock information\n", __func__);
    goto err_nomem2;
    }
    hwclk = __clk_get_hw(policy.clk);
    count = clk_hw_get_num_parents(hwclk);
    data.pclk = kzalloc_objs(struct clk *, count);
    if (!data.pclk)
    goto err_nomem2;
    table = kzalloc_objs(*table, count + 1);
    if (!table)
    goto err_pclk;
    for (i = 0; i < count; i++) {
    clk = clk_hw_get_parent_by_index(hwclk, i).clk;
    data.pclk[i] = clk;
    freq = clk_get_rate(clk);
    table[i].frequency = freq / 1000;
    table[i].driver_data = i;
    }
    freq_table_redup(table, count);
    freq_table_sort(table, count);
    table[i].frequency = CPUFREQ_TABLE_END;
    policy.freq_table = table;
    data.table = table;
// update ->cpus if we have cluster, no harm if not
    set_affected_cpus(policy);
    policy.driver_data = data;
// Minimum transition latency is 12 platform clocks
    u64temp = 12ULL * NSEC_PER_SEC;
    do_div(u64temp, get_bus_freq());
    policy.cpuinfo.transition_latency = u64temp + 1;
    of_node_put(np);
    return 0;
    err_pclk:
    kfree(data.pclk);
    err_nomem2:
    kfree(data);
    err_np:
    of_node_put(np);
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn qoriq_cpufreq_cpu_exit(policy: *mut cpufreq_policy) {
    static void qoriq_cpufreq_cpu_exit(struct cpufreq_policy *policy)
    {
    struct cpu_data *data = policy.driver_data;
    kfree(data.pclk);
    kfree(data.table);
    kfree(data);
    policy.driver_data = core::ptr::null_mut();
    }
    static int qoriq_cpufreq_target(struct cpufreq_policy *policy,
    unsigned int index)
    {
    struct clk *parent;
    struct cpu_data *data = policy.driver_data;
    parent = data.pclk[data.table[index].driver_data];
    return clk_set_parent(policy.clk, parent);
    }
    static struct cpufreq_driver qoriq_cpufreq_driver = {
    .name		= "qoriq_cpufreq",
    .flags		= CPUFREQ_CONST_LOOPS |
    CPUFREQ_IS_COOLING_DEV,
    .init		= qoriq_cpufreq_cpu_init,
    .exit		= qoriq_cpufreq_cpu_exit,
    .verify		= cpufreq_generic_frequency_table_verify,
    .target_index	= qoriq_cpufreq_target,
    .get		= cpufreq_generic_get,
    };
    static const struct of_device_id qoriq_cpufreq_blacklist[] = {
// e6500 cannot use cpufreq due to erratum A-008083
    { .compatible = "fsl,b4420-clockgen", },
    { .compatible = "fsl,b4860-clockgen", },
    { .compatible = "fsl,t2080-clockgen", },
    { .compatible = "fsl,t4240-clockgen", },
    {}
    };
#[no_mangle]
unsafe extern "C" fn qoriq_cpufreq_probe(pdev: *mut platform_device) -> c_int {
    static int qoriq_cpufreq_probe(struct platform_device *pdev)
    {
    int ret;
    struct device_node *np;
    np = of_find_matching_node(core::ptr::null_mut(), qoriq_cpufreq_blacklist);
    if (np) {
    of_node_put(np);
    dev_info(&pdev.dev, "Disabling due to erratum A-008083");
    return -ENODEV;
    }
    ret = cpufreq_register_driver(&qoriq_cpufreq_driver);
    if (ret)
    return ret;
    dev_info(&pdev.dev, "Freescale QorIQ CPU frequency scaling driver\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qoriq_cpufreq_remove(pdev: *mut platform_device) {
    static void qoriq_cpufreq_remove(struct platform_device *pdev)
    {
    cpufreq_unregister_driver(&qoriq_cpufreq_driver);
    }
    static struct platform_driver qoriq_cpufreq_platform_driver = {
    .driver = {
    .name = "qoriq-cpufreq",
    },
    .probe = qoriq_cpufreq_probe,
    .remove = qoriq_cpufreq_remove,
    };
    module_platform_driver(qoriq_cpufreq_platform_driver);
    MODULE_ALIAS("platform:qoriq-cpufreq");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Tang Yuantian <Yuantian.Tang@freescale.com>");
    MODULE_DESCRIPTION("cpufreq driver for Freescale QorIQ series SoCs");
