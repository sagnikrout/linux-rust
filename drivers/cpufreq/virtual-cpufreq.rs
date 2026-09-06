//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/virtual-cpufreq.c
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
// Copyright (C) 2024 Google LLC
//

//
// CPU0..CPUn
// +-------------+-------------------------------+--------+-------+
// | Register    | Description                   | Offset |   Len |
// +-------------+-------------------------------+--------+-------+
// | cur_perf    | read this register to get     |    0x0 |   0x4 |
// |             | the current perf (integer val |        |       |
// |             | representing perf relative to |        |       |
// |             | max performance)              |        |       |
// |             | that vCPU is running at       |        |       |
// +-------------+-------------------------------+--------+-------+
// | set_perf    | write to this register to set |    0x4 |   0x4 |
// |             | perf value of the vCPU        |        |       |
// +-------------+-------------------------------+--------+-------+
// | perftbl_len | number of entries in perf     |    0x8 |   0x4 |
// |             | table. A single entry in the  |        |       |
// |             | perf table denotes no table   |        |       |
// |             | and the entry contains        |        |       |
// |             | the maximum perf value        |        |       |
// |             | that this vCPU supports.      |        |       |
// |             | The guest can request any     |        |       |
// |             | value between 1 and max perf  |        |       |
// |             | when perftbls are not used.   |        |       |
// +---------------------------------------------+--------+-------+
// | perftbl_sel | write to this register to     |    0xc |   0x4 |
// |             | select perf table entry to    |        |       |
// |             | read from                     |        |       |
// +---------------------------------------------+--------+-------+
// | perftbl_rd  | read this register to get     |   0x10 |   0x4 |
// |             | perf value of the selected    |        |       |
// |             | entry based on perftbl_sel    |        |       |
// +---------------------------------------------+--------+-------+
// | perf_domain | performance domain number     |   0x14 |   0x4 |
// |             | that this vCPU belongs to.    |        |       |
// |             | vCPUs sharing the same perf   |        |       |
// |             | domain number are part of the |        |       |
// |             | same performance domain.      |        |       |
// +-------------+-------------------------------+--------+-------+
//
pub const REG_CUR_PERF_STATE_OFFSET: c_uint = 0x0;
pub const REG_SET_PERF_STATE_OFFSET: c_uint = 0x4;
pub const REG_PERFTBL_LEN_OFFSET: c_uint = 0x8;
pub const REG_PERFTBL_SEL_OFFSET: c_uint = 0xc;
pub const REG_PERFTBL_RD_OFFSET: c_uint = 0x10;
pub const REG_PERF_DOMAIN_OFFSET: c_uint = 0x14;
pub const PER_CPU_OFFSET: c_uint = 0x1000;

    static void __iomem *base;
    static DEFINE_PER_CPU(u32, perftbl_num_entries);
#[no_mangle]
unsafe extern "C" fn virt_scale_freq_tick() {
    static void virt_scale_freq_tick(void)
    {
    let mut cpu: c_int = smp_processor_id();
    let mut max_freq: u32 = (u32)cpufreq_get_hw_max_freq(cpu);
    u64 cur_freq;
    unsigned long scale;
    cur_freq = (u64)readl_relaxed(base + cpu * PER_CPU_OFFSET
    + REG_CUR_PERF_STATE_OFFSET);
    cur_freq <<= SCHED_CAPACITY_SHIFT;
    scale = (unsigned long)div_u64(cur_freq, max_freq);
    scale = min(scale, SCHED_CAPACITY_SCALE);
    this_cpu_write(arch_freq_scale, scale);
    }
    static struct scale_freq_data virt_sfd = {
    .source = SCALE_FREQ_SOURCE_VIRT,
    .set_freq_scale = virt_scale_freq_tick,
    };
    static unsigned int virt_cpufreq_set_perf(struct cpufreq_policy *policy,
    unsigned int target_freq)
    {
    writel_relaxed(target_freq,
    base + policy.cpu * PER_CPU_OFFSET + REG_SET_PERF_STATE_OFFSET);
    return 0;
    }
    static unsigned int virt_cpufreq_fast_switch(struct cpufreq_policy *policy,
    unsigned int target_freq)
    {
    virt_cpufreq_set_perf(policy, target_freq);
    return target_freq;
    }
#[no_mangle]
unsafe extern "C" fn virt_cpufreq_get_perftbl_entry(cpu: c_int, idx: u32) -> u32 {
    static u32 virt_cpufreq_get_perftbl_entry(int cpu, u32 idx)
    {
    writel_relaxed(idx, base + cpu * PER_CPU_OFFSET +
    REG_PERFTBL_SEL_OFFSET);
    return readl_relaxed(base + cpu * PER_CPU_OFFSET +
    REG_PERFTBL_RD_OFFSET);
    }
    static int virt_cpufreq_target(struct cpufreq_policy *policy,
    unsigned int target_freq,
    unsigned int relation)
    {
    struct cpufreq_freqs freqs;
    let mut ret: c_int = 0;
    freqs.old = policy.cur;
    freqs.new = target_freq;
    cpufreq_freq_transition_begin(policy, &freqs);
    ret = virt_cpufreq_set_perf(policy, target_freq);
    cpufreq_freq_transition_end(policy, &freqs, ret != 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn virt_cpufreq_get_sharing_cpus(policy: *mut cpufreq_policy) -> c_int {
    static int virt_cpufreq_get_sharing_cpus(struct cpufreq_policy *policy)
    {
    u32 cur_perf_domain, perf_domain;
    struct device *cpu_dev;
    int cpu;
    cur_perf_domain = readl_relaxed(base + policy.cpu *
    PER_CPU_OFFSET + REG_PERF_DOMAIN_OFFSET);
    for_each_present_cpu(cpu) {
    cpu_dev = get_cpu_device(cpu);
    if (!cpu_dev)
    continue;
    perf_domain = readl_relaxed(base + cpu *
    PER_CPU_OFFSET + REG_PERF_DOMAIN_OFFSET);
    if (perf_domain == cur_perf_domain)
    cpumask_set_cpu(cpu, policy.cpus);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virt_cpufreq_get_freq_info(policy: *mut cpufreq_policy) -> c_int {
    static int virt_cpufreq_get_freq_info(struct cpufreq_policy *policy)
    {
    struct cpufreq_frequency_table *table;
    u32 num_perftbl_entries, idx;
    num_perftbl_entries = per_cpu(perftbl_num_entries, policy.cpu);
    if (num_perftbl_entries == 1) {
    policy.cpuinfo.min_freq = 1;
    policy.cpuinfo.max_freq = virt_cpufreq_get_perftbl_entry(policy.cpu, 0);
    policy.cur = policy.cpuinfo.max_freq;
    return 0;
    }
    table = kzalloc_objs(*table, num_perftbl_entries + 1);
    if (!table)
    return -ENOMEM;
    for (idx = 0; idx < num_perftbl_entries; idx++)
    table[idx].frequency = virt_cpufreq_get_perftbl_entry(policy.cpu, idx);
    table[idx].frequency = CPUFREQ_TABLE_END;
    policy.freq_table = table;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virt_cpufreq_cpu_init(policy: *mut cpufreq_policy) -> c_int {
    static int virt_cpufreq_cpu_init(struct cpufreq_policy *policy)
    {
    struct device *cpu_dev;
    int ret;
    cpu_dev = get_cpu_device(policy.cpu);
    if (!cpu_dev)
    return -ENODEV;
    ret = virt_cpufreq_get_freq_info(policy);
    if (ret) {
    dev_warn(cpu_dev, "failed to get cpufreq info\n");
    return ret;
    }
    ret = virt_cpufreq_get_sharing_cpus(policy);
    if (ret) {
    dev_warn(cpu_dev, "failed to get sharing cpumask\n");
    return ret;
    }
//
// To simplify and improve latency of handling frequency requests on
// the host side, this ensures that the vCPU thread triggering the MMIO
// abort is the same thread whose performance constraints (Ex. uclamp
// settings) need to be updated. This simplifies the VMM (Virtual
// Machine Manager) having to find the correct vCPU thread and/or
// facing permission issues when configuring other threads.
//
    policy.dvfs_possible_from_any_cpu = false;
    policy.fast_switch_possible = true;
//
// Using the default SCALE_FREQ_SOURCE_CPUFREQ is insufficient since
// the actual physical CPU frequency may not match requested frequency
// from the vCPU thread due to frequency update latencies or other
// inputs to the physical CPU frequency selection. This additional FIE
// source allows for more accurate freq_scale updates and only takes
// effect if another FIE source such as AMUs have not been registered.
//
    topology_set_scale_freq_source(&virt_sfd, policy.cpus);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virt_cpufreq_cpu_exit(policy: *mut cpufreq_policy) {
    static void virt_cpufreq_cpu_exit(struct cpufreq_policy *policy)
    {
    topology_clear_scale_freq_source(SCALE_FREQ_SOURCE_VIRT, policy.related_cpus);
    kfree(policy.freq_table);
    }
#[no_mangle]
unsafe extern "C" fn virt_cpufreq_online(policy: *mut cpufreq_policy) -> c_int {
    static int virt_cpufreq_online(struct cpufreq_policy *policy)
    {
// Nothing to restore.
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virt_cpufreq_offline(policy: *mut cpufreq_policy) -> c_int {
    static int virt_cpufreq_offline(struct cpufreq_policy *policy)
    {
// Dummy offline() to avoid exit() being called and freeing resources.
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virt_cpufreq_verify_policy(policy: *mut cpufreq_policy_data) -> c_int {
    static int virt_cpufreq_verify_policy(struct cpufreq_policy_data *policy)
    {
    if (policy.freq_table)
    return cpufreq_frequency_table_verify(policy);
    cpufreq_verify_within_cpu_limits(policy);
    return 0;
    }
    static struct cpufreq_driver cpufreq_virt_driver = {
    .name		= "virt-cpufreq",
    .init		= virt_cpufreq_cpu_init,
    .exit		= virt_cpufreq_cpu_exit,
    .online         = virt_cpufreq_online,
    .offline        = virt_cpufreq_offline,
    .verify		= virt_cpufreq_verify_policy,
    .target		= virt_cpufreq_target,
    .fast_switch	= virt_cpufreq_fast_switch,
    };
#[no_mangle]
unsafe extern "C" fn virt_cpufreq_driver_probe(pdev: *mut platform_device) -> c_int {
    static int virt_cpufreq_driver_probe(struct platform_device *pdev)
    {
    u32 num_perftbl_entries;
    int ret, cpu;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    for_each_possible_cpu(cpu) {
    num_perftbl_entries = readl_relaxed(base + cpu * PER_CPU_OFFSET +
    REG_PERFTBL_LEN_OFFSET);
    if (!num_perftbl_entries || num_perftbl_entries > PERFTBL_MAX_ENTRIES)
    return -ENODEV;
    per_cpu(perftbl_num_entries, cpu) = num_perftbl_entries;
    }
    ret = cpufreq_register_driver(&cpufreq_virt_driver);
    if (ret) {
    dev_err(&pdev.dev, "Virtual CPUFreq driver failed to register: %d\n", ret);
    return ret;
    }
    dev_dbg(&pdev.dev, "Virtual CPUFreq driver initialized\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virt_cpufreq_driver_remove(pdev: *mut platform_device) {
    static void virt_cpufreq_driver_remove(struct platform_device *pdev)
    {
    cpufreq_unregister_driver(&cpufreq_virt_driver);
    }
    static const struct of_device_id virt_cpufreq_match[] = {
    { .compatible = "qemu,virtual-cpufreq", .data = core::ptr::null_mut()},
    {}
    };
    MODULE_DEVICE_TABLE(of, virt_cpufreq_match);
    static struct platform_driver virt_cpufreq_driver = {
    .probe = virt_cpufreq_driver_probe,
    .remove = virt_cpufreq_driver_remove,
    .driver = {
    .name = "virt-cpufreq",
    .of_match_table = virt_cpufreq_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn virt_cpufreq_init() -> int __init {
    static int __init virt_cpufreq_init(void)
    {
    return platform_driver_register(&virt_cpufreq_driver);
    }
    postcore_initcall(virt_cpufreq_init);
#[no_mangle]
unsafe extern "C" fn virt_cpufreq_exit() -> void __exit {
    static void __exit virt_cpufreq_exit(void)
    {
    platform_driver_unregister(&virt_cpufreq_driver);
    }
    module_exit(virt_cpufreq_exit);
    MODULE_DESCRIPTION("Virtual cpufreq driver");
    MODULE_LICENSE("GPL");
