//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/vexpress-spc-cpufreq.c
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
// Versatile Express SPC CPUFreq Interface driver
//
// Copyright (C) 2013 - 2019 ARM Ltd.
// Sudeep Holla <sudeep.holla@arm.com>
//
// Copyright (C) 2013 Linaro.
// Viresh Kumar <viresh.kumar@linaro.org>
//

// Currently we support only two clusters
pub const A15_CLUSTER: c_int = 0;
pub const A7_CLUSTER: c_int = 1;
pub const MAX_CLUSTERS: c_int = 2;

    static bool bL_switching_enabled;

    static struct clk *clk[MAX_CLUSTERS];
    static struct cpufreq_frequency_table *freq_table[MAX_CLUSTERS + 1];
    static atomic_t cluster_usage[MAX_CLUSTERS + 1];
    static unsigned int clk_big_min;	/* (Big) clock frequencies */
    static unsigned int clk_little_max;	/* Maximum clock frequency (Little) */
    static DEFINE_PER_CPU(unsigned int, physical_cluster);
    static DEFINE_PER_CPU(unsigned int, cpu_last_req_freq);
    static struct mutex cluster_lock[MAX_CLUSTERS];
#[no_mangle]
pub unsafe extern "C" fn raw_cpu_to_cluster(cpu: c_int) -> c_int {
    static inline int raw_cpu_to_cluster(int cpu)
    {
    return topology_physical_package_id(cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_to_cluster(cpu: c_int) -> c_int {
    static inline int cpu_to_cluster(int cpu)
    {
    return is_bL_switching_enabled() ?
    MAX_CLUSTERS : raw_cpu_to_cluster(cpu);
    }
#[no_mangle]
unsafe extern "C" fn find_cluster_maxfreq(cluster: c_int) -> c_uint {
    static unsigned int find_cluster_maxfreq(int cluster)
    {
    int j;
    let mut max_freq: u32 = 0, cpu_freq;
    for_each_online_cpu(j) {
    cpu_freq = per_cpu(cpu_last_req_freq, j);
    if (cluster == per_cpu(physical_cluster, j) &&
    max_freq < cpu_freq)
    max_freq = cpu_freq;
    }
    return max_freq;
    }
#[no_mangle]
unsafe extern "C" fn clk_get_cpu_rate(cpu: c_uint) -> c_uint {
    static unsigned int clk_get_cpu_rate(unsigned int cpu)
    {
    let mut cur_cluster: u32 = per_cpu(physical_cluster, cpu);
    let mut rate: u32 = clk_get_rate(clk[cur_cluster]) / 1000;
// For switcher we use virtual A7 clock rates
    if (is_bL_switching_enabled())
    rate = VIRT_FREQ(cur_cluster, rate);
    return rate;
    }
#[no_mangle]
unsafe extern "C" fn ve_spc_cpufreq_get_rate(cpu: c_uint) -> c_uint {
    static unsigned int ve_spc_cpufreq_get_rate(unsigned int cpu)
    {
    if (is_bL_switching_enabled())
    return per_cpu(cpu_last_req_freq, cpu);
    else
    return clk_get_cpu_rate(cpu);
    }
    static unsigned int
    ve_spc_cpufreq_set_rate(u32 cpu, u32 old_cluster, u32 new_cluster, u32 rate)
    {
    u32 new_rate, prev_rate;
    int ret;
    let mut bLs: bool = is_bL_switching_enabled();
    mutex_lock(&cluster_lock[new_cluster]);
    if (bLs) {
    prev_rate = per_cpu(cpu_last_req_freq, cpu);
    per_cpu(cpu_last_req_freq, cpu) = rate;
    per_cpu(physical_cluster, cpu) = new_cluster;
    new_rate = find_cluster_maxfreq(new_cluster);
    new_rate = ACTUAL_FREQ(new_cluster, new_rate);
    } else {
    new_rate = rate;
    }
    ret = clk_set_rate(clk[new_cluster], new_rate * 1000);
    if (!ret) {
//
// FIXME: clk_set_rate hasn't returned an error here however it
// may be that clk_change_rate failed due to hardware or
// firmware issues and wasn't able to report that due to the
// current design of the clk core layer. To work around this
// problem we will read back the clock rate and check it is
// correct. This needs to be removed once clk core is fixed.
//
    if (clk_get_rate(clk[new_cluster]) != new_rate * 1000)
    ret = -EIO;
    }
    if (WARN_ON(ret)) {
    if (bLs) {
    per_cpu(cpu_last_req_freq, cpu) = prev_rate;
    per_cpu(physical_cluster, cpu) = old_cluster;
    }
    mutex_unlock(&cluster_lock[new_cluster]);
    return ret;
    }
    mutex_unlock(&cluster_lock[new_cluster]);
// Recalc freq for old cluster when switching clusters
    if (old_cluster != new_cluster) {
// Switch cluster
    bL_switch_request(cpu, new_cluster);
    mutex_lock(&cluster_lock[old_cluster]);
// Set freq of old cluster if there are cpus left on it
    new_rate = find_cluster_maxfreq(old_cluster);
    new_rate = ACTUAL_FREQ(old_cluster, new_rate);
    if (new_rate &&
    clk_set_rate(clk[old_cluster], new_rate * 1000)) {
    pr_err("%s: clk_set_rate failed: %d, old cluster: %d\n",
    __func__, ret, old_cluster);
    }
    mutex_unlock(&cluster_lock[old_cluster]);
    }
    return 0;
    }
// Set clock frequency
    static int ve_spc_cpufreq_set_target(struct cpufreq_policy *policy,
    unsigned int index)
    {
    let mut cpu: u32 = policy.cpu, cur_cluster, new_cluster, actual_cluster;
    unsigned int freqs_new;
    cur_cluster = cpu_to_cluster(cpu);
    new_cluster = actual_cluster = per_cpu(physical_cluster, cpu);
    freqs_new = freq_table[cur_cluster][index].frequency;
    if (is_bL_switching_enabled()) {
    if (actual_cluster == A15_CLUSTER && freqs_new < clk_big_min)
    new_cluster = A7_CLUSTER;
    else if (actual_cluster == A7_CLUSTER &&
    freqs_new > clk_little_max)
    new_cluster = A15_CLUSTER;
    }
    return ve_spc_cpufreq_set_rate(cpu, actual_cluster, new_cluster,
    freqs_new);
    }
#[no_mangle]
pub unsafe extern "C" fn get_table_count(table: *mut cpufreq_frequency_table) -> u32 {
    static inline u32 get_table_count(struct cpufreq_frequency_table *table)
    {
    int count;
    for (count = 0; table[count].frequency != CPUFREQ_TABLE_END; count++)
    ;
    return count;
    }
// get the minimum frequency in the cpufreq_frequency_table
#[no_mangle]
pub unsafe extern "C" fn get_table_min(table: *mut cpufreq_frequency_table) -> u32 {
    static inline u32 get_table_min(struct cpufreq_frequency_table *table)
    {
    struct cpufreq_frequency_table *pos;
    let mut min_freq: u32 = ~0;
    cpufreq_for_each_entry(pos, table)
    if (pos.frequency < min_freq)
    min_freq = pos.frequency;
    return min_freq;
    }
// get the maximum frequency in the cpufreq_frequency_table
#[no_mangle]
pub unsafe extern "C" fn get_table_max(table: *mut cpufreq_frequency_table) -> u32 {
    static inline u32 get_table_max(struct cpufreq_frequency_table *table)
    {
    struct cpufreq_frequency_table *pos;
    let mut max_freq: u32 = 0;
    cpufreq_for_each_entry(pos, table)
    if (pos.frequency > max_freq)
    max_freq = pos.frequency;
    return max_freq;
    }
    static bool search_frequency(struct cpufreq_frequency_table *table, int size,
    unsigned int freq)
    {
    int count;
    for (count = 0; count < size; count++) {
    if (table[count].frequency == freq)
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn merge_cluster_tables() -> c_int {
    static int merge_cluster_tables(void)
    {
    int i, j, k = 0, count = 1;
    struct cpufreq_frequency_table *table;
    for (i = 0; i < MAX_CLUSTERS; i++)
    count += get_table_count(freq_table[i]);
    table = kzalloc_objs(*table, count);
    if (!table)
    return -ENOMEM;
    freq_table[MAX_CLUSTERS] = table;
// Add in reverse order to get freqs in increasing order
    for (i = MAX_CLUSTERS - 1; i >= 0; i--, count = k) {
    for (j = 0; freq_table[i][j].frequency != CPUFREQ_TABLE_END;
    j++) {
    if (i == A15_CLUSTER &&
    search_frequency(table, count, freq_table[i][j].frequency))
    continue; /* skip duplicates */
    table[k++].frequency =
    VIRT_FREQ(i, freq_table[i][j].frequency);
    }
    }
    table[k].driver_data = k;
    table[k].frequency = CPUFREQ_TABLE_END;
    return 0;
    }
    static void _put_cluster_clk_and_freq_table(struct device *cpu_dev,
    const struct cpumask *cpumask)
    {
    let mut cluster: u32 = raw_cpu_to_cluster(cpu_dev.id);
    if (!freq_table[cluster])
    return;
    clk_put(clk[cluster]);
    dev_pm_opp_free_cpufreq_table(cpu_dev, &freq_table[cluster]);
    }
    static void put_cluster_clk_and_freq_table(struct device *cpu_dev,
    const struct cpumask *cpumask)
    {
    let mut cluster: u32 = cpu_to_cluster(cpu_dev.id);
    int i;
    if (atomic_dec_return(&cluster_usage[cluster]))
    return;
    if (cluster < MAX_CLUSTERS)
    return _put_cluster_clk_and_freq_table(cpu_dev, cpumask);
    for_each_present_cpu(i) {
    struct device *cdev = get_cpu_device(i);
    if (!cdev)
    return;
    _put_cluster_clk_and_freq_table(cdev, cpumask);
    }
// free virtual table
    kfree(freq_table[cluster]);
    }
    static int _get_cluster_clk_and_freq_table(struct device *cpu_dev,
    const struct cpumask *cpumask)
    {
    let mut cluster: u32 = raw_cpu_to_cluster(cpu_dev.id);
    int ret;
    if (freq_table[cluster])
    return 0;
//
// platform specific SPC code must initialise the opp table
// so just check if the OPP count is non-zero
//
    ret = dev_pm_opp_get_opp_count(cpu_dev) <= 0;
    if (ret)
    goto out;
    ret = dev_pm_opp_init_cpufreq_table(cpu_dev, &freq_table[cluster]);
    if (ret)
    goto out;
    clk[cluster] = clk_get(cpu_dev, core::ptr::null_mut());
    if (!IS_ERR(clk[cluster]))
    return 0;
    dev_err(cpu_dev, "%s: Failed to get clk for cpu: %d, cluster: %d\n",
    __func__, cpu_dev.id, cluster);
    ret = PTR_ERR(clk[cluster]);
    dev_pm_opp_free_cpufreq_table(cpu_dev, &freq_table[cluster]);
    out:
    dev_err(cpu_dev, "%s: Failed to get data for cluster: %d\n", __func__,
    cluster);
    return ret;
    }
    static int get_cluster_clk_and_freq_table(struct device *cpu_dev,
    const struct cpumask *cpumask)
    {
    let mut cluster: u32 = cpu_to_cluster(cpu_dev.id);
    int i, ret;
    if (atomic_inc_return(&cluster_usage[cluster]) != 1)
    return 0;
    if (cluster < MAX_CLUSTERS) {
    ret = _get_cluster_clk_and_freq_table(cpu_dev, cpumask);
    if (ret)
    atomic_dec(&cluster_usage[cluster]);
    return ret;
    }
//
// Get data for all clusters and fill virtual cluster with a merge of
// both
//
    for_each_present_cpu(i) {
    struct device *cdev = get_cpu_device(i);
    if (!cdev)
    return -ENODEV;
    ret = _get_cluster_clk_and_freq_table(cdev, cpumask);
    if (ret)
    goto put_clusters;
    }
    ret = merge_cluster_tables();
    if (ret)
    goto put_clusters;
// Assuming 2 cluster, set clk_big_min and clk_little_max
    clk_big_min = get_table_min(freq_table[A15_CLUSTER]);
    clk_little_max = VIRT_FREQ(A7_CLUSTER,
    get_table_max(freq_table[A7_CLUSTER]));
    return 0;
    put_clusters:
    for_each_present_cpu(i) {
    struct device *cdev = get_cpu_device(i);
    if (!cdev)
    return -ENODEV;
    _put_cluster_clk_and_freq_table(cdev, cpumask);
    }
    atomic_dec(&cluster_usage[cluster]);
    return ret;
    }
// Per-CPU initialization
#[no_mangle]
unsafe extern "C" fn ve_spc_cpufreq_init(policy: *mut cpufreq_policy) -> c_int {
    static int ve_spc_cpufreq_init(struct cpufreq_policy *policy)
    {
    let mut cur_cluster: u32 = cpu_to_cluster(policy.cpu);
    struct device *cpu_dev;
    int ret;
    cpu_dev = get_cpu_device(policy.cpu);
    if (!cpu_dev) {
    pr_err("%s: failed to get cpu%d device\n", __func__,
    policy.cpu);
    return -ENODEV;
    }
    if (cur_cluster < MAX_CLUSTERS) {
    int cpu;
    dev_pm_opp_get_sharing_cpus(cpu_dev, policy.cpus);
    for_each_cpu(cpu, policy.cpus)
    per_cpu(physical_cluster, cpu) = cur_cluster;
    } else {
// Assumption: during init, we are always running on A15
    per_cpu(physical_cluster, policy.cpu) = A15_CLUSTER;
    }
    ret = get_cluster_clk_and_freq_table(cpu_dev, policy.cpus);
    if (ret)
    return ret;
    policy.freq_table = freq_table[cur_cluster];
    policy.cpuinfo.transition_latency = 1000000; /* 1 ms */
    if (is_bL_switching_enabled())
    per_cpu(cpu_last_req_freq, policy.cpu) =
    clk_get_cpu_rate(policy.cpu);
    dev_info(cpu_dev, "%s: CPU %d initialized\n", __func__, policy.cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ve_spc_cpufreq_exit(policy: *mut cpufreq_policy) {
    static void ve_spc_cpufreq_exit(struct cpufreq_policy *policy)
    {
    struct device *cpu_dev;
    cpu_dev = get_cpu_device(policy.cpu);
    if (!cpu_dev) {
    pr_err("%s: failed to get cpu%d device\n", __func__,
    policy.cpu);
    return;
    }
    put_cluster_clk_and_freq_table(cpu_dev, policy.related_cpus);
    }
    static struct cpufreq_driver ve_spc_cpufreq_driver = {
    .name			= "vexpress-spc",
    .flags			= CPUFREQ_HAVE_GOVERNOR_PER_POLICY |
    CPUFREQ_NEED_INITIAL_FREQ_CHECK,
    .verify			= cpufreq_generic_frequency_table_verify,
    .target_index		= ve_spc_cpufreq_set_target,
    .get			= ve_spc_cpufreq_get_rate,
    .init			= ve_spc_cpufreq_init,
    .exit			= ve_spc_cpufreq_exit,
    .register_em		= cpufreq_register_em_with_opp,
    };

    static int bL_cpufreq_switcher_notifier(struct notifier_block *nfb,
    unsigned long action, void *_arg)
    {
    pr_debug("%s: action: %ld\n", __func__, action);
    switch (action) {
    case BL_NOTIFY_PRE_ENABLE:
    case BL_NOTIFY_PRE_DISABLE:
    cpufreq_unregister_driver(&ve_spc_cpufreq_driver);
    break;
    case BL_NOTIFY_POST_ENABLE:
    set_switching_enabled(true);
    cpufreq_register_driver(&ve_spc_cpufreq_driver);
    break;
    case BL_NOTIFY_POST_DISABLE:
    set_switching_enabled(false);
    cpufreq_register_driver(&ve_spc_cpufreq_driver);
    break;
    default:
    return NOTIFY_DONE;
    }
    return NOTIFY_OK;
    }
    static struct notifier_block bL_switcher_notifier = {
    .notifier_call = bL_cpufreq_switcher_notifier,
    };
#[no_mangle]
unsafe extern "C" fn __bLs_register_notifier() -> c_int {
    static int __bLs_register_notifier(void)
    {
    return bL_switcher_register_notifier(&bL_switcher_notifier);
    }
#[no_mangle]
unsafe extern "C" fn __bLs_unregister_notifier() -> c_int {
    static int __bLs_unregister_notifier(void)
    {
    return bL_switcher_unregister_notifier(&bL_switcher_notifier);
    }

    static int __bLs_register_notifier(void) { return 0; }
    static int __bLs_unregister_notifier(void) { return 0; }

#[no_mangle]
unsafe extern "C" fn ve_spc_cpufreq_probe(pdev: *mut platform_device) -> c_int {
    static int ve_spc_cpufreq_probe(struct platform_device *pdev)
    {
    int ret, i;
    set_switching_enabled(bL_switcher_get_enabled());
    for (i = 0; i < MAX_CLUSTERS; i++)
    mutex_init(&cluster_lock[i]);
    if (!is_bL_switching_enabled())
    ve_spc_cpufreq_driver.flags |= CPUFREQ_IS_COOLING_DEV;
    ret = cpufreq_register_driver(&ve_spc_cpufreq_driver);
    if (ret) {
    pr_info("%s: Failed registering platform driver: %s, err: %d\n",
    __func__, ve_spc_cpufreq_driver.name, ret);
    } else {
    ret = __bLs_register_notifier();
    if (ret)
    cpufreq_unregister_driver(&ve_spc_cpufreq_driver);
    else
    pr_info("%s: Registered platform driver: %s\n",
    __func__, ve_spc_cpufreq_driver.name);
    }
    bL_switcher_put_enabled();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ve_spc_cpufreq_remove(pdev: *mut platform_device) {
    static void ve_spc_cpufreq_remove(struct platform_device *pdev)
    {
    bL_switcher_get_enabled();
    __bLs_unregister_notifier();
    cpufreq_unregister_driver(&ve_spc_cpufreq_driver);
    bL_switcher_put_enabled();
    pr_info("%s: Un-registered platform driver: %s\n", __func__,
    ve_spc_cpufreq_driver.name);
    }
    static struct platform_driver ve_spc_cpufreq_platdrv = {
    .driver = {
    .name	= "vexpress-spc-cpufreq",
    },
    .probe		= ve_spc_cpufreq_probe,
    .remove		= ve_spc_cpufreq_remove,
    };
    module_platform_driver(ve_spc_cpufreq_platdrv);
    MODULE_ALIAS("platform:vexpress-spc-cpufreq");
    MODULE_AUTHOR("Viresh Kumar <viresh.kumar@linaro.org>");
    MODULE_AUTHOR("Sudeep Holla <sudeep.holla@arm.com>");
    MODULE_DESCRIPTION("Vexpress SPC ARM big LITTLE cpufreq driver");
    MODULE_LICENSE("GPL v2");
