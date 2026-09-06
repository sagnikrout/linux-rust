//! Automatically rewritten from C to Rust
//! Source: drivers/opp/cpu.c
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
// Generic OPP helper interface for CPU device
//
// Copyright (C) 2009-2014 Texas Instruments Incorporated.
// Nishanth Menon
// Romit Dasgupta
// Kevin Hilman
//

//
// dev_pm_opp_init_cpufreq_table() - create a cpufreq table for a device
// @dev:	device for which we do this operation
// @opp_table:	Cpufreq table returned back to caller
//
// Generate a cpufreq table for a provided device- this assumes that the
// opp table is already initialized and ready for usage.
//
// This function allocates required memory for the cpufreq table. It is
// expected that the caller does the required maintenance such as freeing
// the table as required.
//
// Returns -EINVAL for bad pointers, -ENODEV if the device is not found, -ENOMEM
// if no memory available for the operation (table is not populated), returns 0
// if successful and table is populated.
//
// WARNING: It is  important for the callers to ensure refreshing their copy of
// the table if any of the mentioned functions have been invoked in the interim.
//
    int dev_pm_opp_init_cpufreq_table(struct device *dev,
    struct cpufreq_frequency_table **opp_table)
    {
    struct cpufreq_frequency_table *freq_table = core::ptr::null_mut();
    int i, max_opps, ret = 0;
    unsigned long rate;
    max_opps = dev_pm_opp_get_opp_count(dev);
    if (max_opps <= 0)
    return max_opps ? max_opps : -ENODATA;
    freq_table = kzalloc_objs(*freq_table, (max_opps + 1));
    if (!freq_table)
    return -ENOMEM;
    for (i = 0, rate = 0; i < max_opps; i++, rate++) {
// find next rate
    struct dev_pm_opp *opp __free(put_opp) =
    dev_pm_opp_find_freq_ceil(dev, &rate);
    if (IS_ERR(opp)) {
    ret = PTR_ERR(opp);
    goto out;
    }
    freq_table[i].driver_data = i;
    freq_table[i].frequency = rate / 1000;
// Is Boost/turbo opp ?
    if (dev_pm_opp_is_turbo(opp))
    freq_table[i].flags = CPUFREQ_BOOST_FREQ;
    }
    freq_table[i].driver_data = i;
    freq_table[i].frequency = CPUFREQ_TABLE_END;
// opp_table = &freq_table[0];
    out:
    if (ret)
    kfree(freq_table);
    return ret;
    }
    EXPORT_SYMBOL_GPL(dev_pm_opp_init_cpufreq_table);
//
// dev_pm_opp_free_cpufreq_table() - free the cpufreq table
// @dev:	device for which we do this operation
// @opp_table:	table to free
//
// Free up the table allocated by dev_pm_opp_init_cpufreq_table
//
    void dev_pm_opp_free_cpufreq_table(struct device *dev,
    struct cpufreq_frequency_table **opp_table)
    {
    if (!opp_table)
    return;
    kfree(*opp_table);
// opp_table = NULL;
    }
    EXPORT_SYMBOL_GPL(dev_pm_opp_free_cpufreq_table);

    void _dev_pm_opp_cpumask_remove_table(const struct cpumask *cpumask,
    int last_cpu)
    {
    struct device *cpu_dev;
    int cpu;
    WARN_ON(cpumask_empty(cpumask));
    for_each_cpu(cpu, cpumask) {
    if (cpu == last_cpu)
    break;
    cpu_dev = get_cpu_device(cpu);
    if (!cpu_dev) {
    pr_err("%s: failed to get cpu%d device\n", __func__,
    cpu);
    continue;
    }
    dev_pm_opp_remove_table(cpu_dev);
    }
    }
//
// dev_pm_opp_cpumask_remove_table() - Removes OPP table for @cpumask
// @cpumask:	cpumask for which OPP table needs to be removed
//
// This removes the OPP tables for CPUs present in the @cpumask.
// This should be used to remove all the OPPs entries associated with
// the cpus in @cpumask.
//
#[no_mangle]
pub unsafe extern "C" fn dev_pm_opp_cpumask_remove_table(cpumask: *const cpumask) {
    void dev_pm_opp_cpumask_remove_table(const struct cpumask *cpumask)
    {
    _dev_pm_opp_cpumask_remove_table(cpumask, -1);
    }
    EXPORT_SYMBOL_GPL(dev_pm_opp_cpumask_remove_table);
//
// dev_pm_opp_set_sharing_cpus() - Mark OPP table as shared by few CPUs
// @cpu_dev:	CPU device for which we do this operation
// @cpumask:	cpumask of the CPUs which share the OPP table with @cpu_dev
//
// This marks OPP table of the @cpu_dev as shared by the CPUs present in
// @cpumask.
//
// Returns -ENODEV if OPP table isn't already present.
//
    int dev_pm_opp_set_sharing_cpus(struct device *cpu_dev,
    const struct cpumask *cpumask)
    {
    struct opp_device *opp_dev;
    struct device *dev;
    int cpu;
    struct opp_table *opp_table __free(put_opp_table) =
    _find_opp_table(cpu_dev);
    if (IS_ERR(opp_table))
    return PTR_ERR(opp_table);
    for_each_cpu(cpu, cpumask) {
    if (cpu == cpu_dev.id)
    continue;
    dev = get_cpu_device(cpu);
    if (!dev) {
    dev_err(cpu_dev, "%s: failed to get cpu%d device\n",
    __func__, cpu);
    continue;
    }
    opp_dev = _add_opp_dev(dev, opp_table);
    if (!opp_dev) {
    dev_err(dev, "%s: failed to add opp-dev for cpu%d device\n",
    __func__, cpu);
    continue;
    }
// Mark opp-table as multiple CPUs are sharing it now
    opp_table.shared_opp = OPP_TABLE_ACCESS_SHARED;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(dev_pm_opp_set_sharing_cpus);
//
// dev_pm_opp_get_sharing_cpus() - Get cpumask of CPUs sharing OPPs with @cpu_dev
// @cpu_dev:	CPU device for which we do this operation
// @cpumask:	cpumask to update with information of sharing CPUs
//
// This updates the @cpumask with CPUs that are sharing OPPs with @cpu_dev.
//
// Returns -ENODEV if OPP table isn't already present and -EINVAL if the OPP
// table's status is access-unknown.
//
#[no_mangle]
pub unsafe extern "C" fn dev_pm_opp_get_sharing_cpus(cpu_dev: *mut device, cpumask: *mut cpumask) -> c_int {
    int dev_pm_opp_get_sharing_cpus(struct device *cpu_dev, struct cpumask *cpumask)
    {
    struct opp_device *opp_dev;
    struct opp_table *opp_table __free(put_opp_table) =
    _find_opp_table(cpu_dev);
    if (IS_ERR(opp_table))
    return PTR_ERR(opp_table);
    if (opp_table.shared_opp == OPP_TABLE_ACCESS_UNKNOWN)
    return -EINVAL;
    cpumask_clear(cpumask);
    if (opp_table.shared_opp == OPP_TABLE_ACCESS_SHARED) {
    guard(mutex)(&opp_table.lock);
    list_for_each_entry(opp_dev, &opp_table.dev_list, node)
    cpumask_set_cpu(opp_dev.dev.id, cpumask);
    } else {
    cpumask_set_cpu(cpu_dev.id, cpumask);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(dev_pm_opp_get_sharing_cpus);
