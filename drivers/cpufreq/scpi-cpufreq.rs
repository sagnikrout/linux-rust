//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/scpi-cpufreq.c
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
// System Control and Power Interface (SCPI) based CPUFreq Interface driver
//
// Copyright (C) 2015 ARM Ltd.
// Sudeep Holla <sudeep.holla@arm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scpi_data {
    pub clk: *mut clk,
    pub cpu_dev: *mut device,
}

    static struct scpi_ops *scpi_ops;
#[no_mangle]
unsafe extern "C" fn scpi_cpufreq_get_rate(cpu: c_uint) -> c_uint {
    static unsigned int scpi_cpufreq_get_rate(unsigned int cpu)
    {
    struct cpufreq_policy *policy;
    struct scpi_data *priv;
    unsigned long rate;
    policy = cpufreq_cpu_get_raw(cpu);
    if (unlikely(!policy))
    return 0;
    priv = policy.driver_data;
    rate = clk_get_rate(priv.clk);
    return rate / 1000;
    }
    static int
    scpi_cpufreq_set_target(struct cpufreq_policy *policy, unsigned int index)
    {
    let mut freq_khz: c_ulong = policy.freq_table[index].frequency;
    struct scpi_data *priv = policy.driver_data;
    let mut rate: c_ulong = freq_khz * 1000;
    int ret;
    ret = clk_set_rate(priv.clk, rate);
    if (ret)
    return ret;
    if (clk_get_rate(priv.clk) / 1000 != freq_khz)
    return -EIO;
    return 0;
    }
    static int
    scpi_get_sharing_cpus(struct device *cpu_dev, struct cpumask *cpumask)
    {
    int cpu, domain, tdomain;
    struct device *tcpu_dev;
    domain = scpi_ops.device_domain_id(cpu_dev);
    if (domain < 0)
    return domain;
    for_each_present_cpu(cpu) {
    if (cpu == cpu_dev.id)
    continue;
    tcpu_dev = get_cpu_device(cpu);
    if (!tcpu_dev)
    continue;
    tdomain = scpi_ops.device_domain_id(tcpu_dev);
    if (tdomain == domain)
    cpumask_set_cpu(cpu, cpumask);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scpi_cpufreq_init(policy: *mut cpufreq_policy) -> c_int {
    static int scpi_cpufreq_init(struct cpufreq_policy *policy)
    {
    int ret;
    unsigned int latency;
    struct device *cpu_dev;
    struct scpi_data *priv;
    struct cpufreq_frequency_table *freq_table;
    cpu_dev = get_cpu_device(policy.cpu);
    if (!cpu_dev) {
    pr_err("failed to get cpu%d device\n", policy.cpu);
    return -ENODEV;
    }
    ret = scpi_ops.add_opps_to_device(cpu_dev);
    if (ret) {
    dev_warn(cpu_dev, "failed to add opps to the device\n");
    return ret;
    }
    ret = scpi_get_sharing_cpus(cpu_dev, policy.cpus);
    if (ret) {
    dev_warn(cpu_dev, "failed to get sharing cpumask\n");
    return ret;
    }
    ret = dev_pm_opp_set_sharing_cpus(cpu_dev, policy.cpus);
    if (ret) {
    dev_err(cpu_dev, "%s: failed to mark OPPs as shared: %d\n",
    __func__, ret);
    return ret;
    }
    ret = dev_pm_opp_get_opp_count(cpu_dev);
    if (ret <= 0) {
    dev_dbg(cpu_dev, "OPP table is not ready, deferring probe\n");
    ret = -EPROBE_DEFER;
    goto out_free_opp;
    }
    priv = kzalloc_obj(*priv);
    if (!priv) {
    ret = -ENOMEM;
    goto out_free_opp;
    }
    ret = dev_pm_opp_init_cpufreq_table(cpu_dev, &freq_table);
    if (ret) {
    dev_err(cpu_dev, "failed to init cpufreq table: %d\n", ret);
    goto out_free_priv;
    }
    priv.cpu_dev = cpu_dev;
    priv.clk = clk_get(cpu_dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk)) {
    dev_err(cpu_dev, "%s: Failed to get clk for cpu: %d\n",
    __func__, cpu_dev.id);
    ret = PTR_ERR(priv.clk);
    goto out_free_cpufreq_table;
    }
    policy.driver_data = priv;
    policy.freq_table = freq_table;
// scpi allows DVFS request for any domain from any CPU
    policy.dvfs_possible_from_any_cpu = true;
    latency = scpi_ops.get_transition_latency(cpu_dev);
    if (!latency)
    latency = CPUFREQ_DEFAULT_TRANSITION_LATENCY_NS;
    policy.cpuinfo.transition_latency = latency;
    policy.fast_switch_possible = false;
    return 0;
    out_free_cpufreq_table:
    dev_pm_opp_free_cpufreq_table(cpu_dev, &freq_table);
    out_free_priv:
    kfree(priv);
    out_free_opp:
    dev_pm_opp_remove_all_dynamic(cpu_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn scpi_cpufreq_exit(policy: *mut cpufreq_policy) {
    static void scpi_cpufreq_exit(struct cpufreq_policy *policy)
    {
    struct scpi_data *priv = policy.driver_data;
    clk_put(priv.clk);
    dev_pm_opp_free_cpufreq_table(priv.cpu_dev, &policy.freq_table);
    dev_pm_opp_remove_all_dynamic(priv.cpu_dev);
    kfree(priv);
    }
    static struct cpufreq_driver scpi_cpufreq_driver = {
    .name	= "scpi-cpufreq",
    .flags	= CPUFREQ_HAVE_GOVERNOR_PER_POLICY |
    CPUFREQ_NEED_INITIAL_FREQ_CHECK |
    CPUFREQ_IS_COOLING_DEV,
    .verify	= cpufreq_generic_frequency_table_verify,
    .get	= scpi_cpufreq_get_rate,
    .init	= scpi_cpufreq_init,
    .exit	= scpi_cpufreq_exit,
    .target_index	= scpi_cpufreq_set_target,
    .register_em	= cpufreq_register_em_with_opp,
    };
#[no_mangle]
unsafe extern "C" fn scpi_cpufreq_probe(pdev: *mut platform_device) -> c_int {
    static int scpi_cpufreq_probe(struct platform_device *pdev)
    {
    int ret;
    scpi_ops = get_scpi_ops();
    if (!scpi_ops)
    return -EIO;
    ret = cpufreq_register_driver(&scpi_cpufreq_driver);
    if (ret)
    dev_err(&pdev.dev, "%s: registering cpufreq failed, err: %d\n",
    __func__, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn scpi_cpufreq_remove(pdev: *mut platform_device) {
    static void scpi_cpufreq_remove(struct platform_device *pdev)
    {
    cpufreq_unregister_driver(&scpi_cpufreq_driver);
    scpi_ops = core::ptr::null_mut();
    }
    static struct platform_driver scpi_cpufreq_platdrv = {
    .driver = {
    .name	= "scpi-cpufreq",
    },
    .probe		= scpi_cpufreq_probe,
    .remove		= scpi_cpufreq_remove,
    };
    module_platform_driver(scpi_cpufreq_platdrv);
    MODULE_ALIAS("platform:scpi-cpufreq");
    MODULE_AUTHOR("Sudeep Holla <sudeep.holla@arm.com>");
    MODULE_DESCRIPTION("ARM SCPI CPUFreq interface driver");
    MODULE_LICENSE("GPL v2");
