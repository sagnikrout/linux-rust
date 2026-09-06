//! Automatically rewritten from C to Rust
//! Source: drivers/powercap/dtpm_cpu.c
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
// Copyright 2020 Linaro Limited
//
// Author: Daniel Lezcano <daniel.lezcano@linaro.org>
//
// The DTPM CPU is based on the energy model. It hooks the CPU in the
// DTPM tree which in turns update the power number by propagating the
// power number from the CPU energy model information to the parents.
//
// The association between the power and the performance state, allows
// to set the power of the CPU at the OPP granularity.
//
// The CPU hotplug is supported and the power numbers will be updated
// if a CPU is hot plugged / unplugged.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtpm_cpu {
    pub dtpm: dtpm,
    pub qos_req: freq_qos_request,
    pub cpu: c_int,
}

    static DEFINE_PER_CPU(struct dtpm_cpu *, dtpm_per_cpu);
    static struct dtpm_cpu *to_dtpm_cpu(struct dtpm *dtpm)
    {
    return container_of(dtpm, struct dtpm_cpu, dtpm);
    }
#[no_mangle]
unsafe extern "C" fn set_pd_power_limit(dtpm: *mut dtpm, power_limit: u64) -> u64 {
    static u64 set_pd_power_limit(struct dtpm *dtpm, u64 power_limit)
    {
    struct dtpm_cpu *dtpm_cpu = to_dtpm_cpu(dtpm);
    struct em_perf_domain *pd = em_cpu_get(dtpm_cpu.cpu);
    struct em_perf_state *table;
    unsigned long freq;
    u64 power;
    int i, nr_cpus;
    nr_cpus = cpumask_weight_and(cpu_online_mask, to_cpumask(pd.cpus));
    rcu_read_lock();
    table = em_perf_state_from_pd(pd);
    for (i = 0; i < pd.nr_perf_states; i++) {
    power = table[i].power * nr_cpus;
    if (power > power_limit)
    break;
    }
    freq = table[i - 1].frequency;
    power_limit = table[i - 1].power * nr_cpus;
    rcu_read_unlock();
    freq_qos_update_request(&dtpm_cpu.qos_req, freq);
    return power_limit;
    }
#[no_mangle]
unsafe extern "C" fn scale_pd_power_uw(pd_mask: *mut cpumask, power: u64) -> u64 {
    static u64 scale_pd_power_uw(struct cpumask *pd_mask, u64 power)
    {
    unsigned long max, sum_util = 0;
    int cpu;
//
// The capacity is the same for all CPUs belonging to
// the same perf domain.
//
    max = arch_scale_cpu_capacity(cpumask_first(pd_mask));
    for_each_cpu_and(cpu, pd_mask, cpu_online_mask)
    sum_util += sched_cpu_util(cpu);
    return (power * ((sum_util << 10) / max)) >> 10;
    }
#[no_mangle]
unsafe extern "C" fn get_pd_power_uw(dtpm: *mut dtpm) -> u64 {
    static u64 get_pd_power_uw(struct dtpm *dtpm)
    {
    struct dtpm_cpu *dtpm_cpu = to_dtpm_cpu(dtpm);
    struct em_perf_state *table;
    struct em_perf_domain *pd;
    struct cpumask *pd_mask;
    unsigned long freq;
    let mut power: u64 = 0;
    int i;
    pd = em_cpu_get(dtpm_cpu.cpu);
    if (!pd)
    return 0;
    pd_mask = em_span_cpus(pd);
    freq = cpufreq_quick_get(dtpm_cpu.cpu);
    rcu_read_lock();
    table = em_perf_state_from_pd(pd);
    for (i = 0; i < pd.nr_perf_states; i++) {
    if (table[i].frequency < freq)
    continue;
    power = scale_pd_power_uw(pd_mask, table[i].power);
    break;
    }
    rcu_read_unlock();
    return power;
    }
#[no_mangle]
unsafe extern "C" fn update_pd_power_uw(dtpm: *mut dtpm) -> c_int {
    static int update_pd_power_uw(struct dtpm *dtpm)
    {
    struct dtpm_cpu *dtpm_cpu = to_dtpm_cpu(dtpm);
    struct em_perf_domain *em = em_cpu_get(dtpm_cpu.cpu);
    struct em_perf_state *table;
    int nr_cpus;
    nr_cpus = cpumask_weight_and(cpu_online_mask, to_cpumask(em.cpus));
    rcu_read_lock();
    table = em_perf_state_from_pd(em);
    dtpm.power_min = table[0].power;
    dtpm.power_min *= nr_cpus;
    dtpm.power_max = table[em.nr_perf_states - 1].power;
    dtpm.power_max *= nr_cpus;
    rcu_read_unlock();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pd_release(dtpm: *mut dtpm) {
    static void pd_release(struct dtpm *dtpm)
    {
    struct dtpm_cpu *dtpm_cpu = to_dtpm_cpu(dtpm);
    struct cpufreq_policy *policy;
    if (freq_qos_request_active(&dtpm_cpu.qos_req))
    freq_qos_remove_request(&dtpm_cpu.qos_req);
    policy = cpufreq_cpu_get(dtpm_cpu.cpu);
    if (policy) {
    for_each_cpu(dtpm_cpu.cpu, policy.related_cpus)
    per_cpu(dtpm_per_cpu, dtpm_cpu.cpu) = core::ptr::null_mut();
    cpufreq_cpu_put(policy);
    }
    kfree(dtpm_cpu);
    }
    static struct dtpm_ops dtpm_ops = {
    .set_power_uw	 = set_pd_power_limit,
    .get_power_uw	 = get_pd_power_uw,
    .update_power_uw = update_pd_power_uw,
    .release	 = pd_release,
    };
#[no_mangle]
unsafe extern "C" fn cpuhp_dtpm_cpu_offline(cpu: c_uint) -> c_int {
    static int cpuhp_dtpm_cpu_offline(unsigned int cpu)
    {
    struct dtpm_cpu *dtpm_cpu;
    dtpm_cpu = per_cpu(dtpm_per_cpu, cpu);
    if (dtpm_cpu)
    dtpm_update_power(&dtpm_cpu.dtpm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpuhp_dtpm_cpu_online(cpu: c_uint) -> c_int {
    static int cpuhp_dtpm_cpu_online(unsigned int cpu)
    {
    struct dtpm_cpu *dtpm_cpu;
    dtpm_cpu = per_cpu(dtpm_per_cpu, cpu);
    if (dtpm_cpu)
    return dtpm_update_power(&dtpm_cpu.dtpm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __dtpm_cpu_setup(cpu: c_int, parent: *mut dtpm) -> c_int {
    static int __dtpm_cpu_setup(int cpu, struct dtpm *parent)
    {
    struct dtpm_cpu *dtpm_cpu;
    struct cpufreq_policy *policy;
    struct em_perf_state *table;
    struct em_perf_domain *pd;
    char name[CPUFREQ_NAME_LEN];
    let mut ret: c_int = -ENOMEM;
    dtpm_cpu = per_cpu(dtpm_per_cpu, cpu);
    if (dtpm_cpu)
    return 0;
    policy = cpufreq_cpu_get(cpu);
    if (!policy)
    return 0;
    pd = em_cpu_get(cpu);
    if (!pd || em_is_artificial(pd)) {
    ret = -EINVAL;
    goto release_policy;
    }
    dtpm_cpu = kzalloc_obj(*dtpm_cpu);
    if (!dtpm_cpu) {
    ret = -ENOMEM;
    goto release_policy;
    }
    dtpm_init(&dtpm_cpu.dtpm, &dtpm_ops);
    dtpm_cpu.cpu = cpu;
    for_each_cpu(cpu, policy.related_cpus)
    per_cpu(dtpm_per_cpu, cpu) = dtpm_cpu;
    snprintf(name, sizeof(name), "cpu%d-cpufreq", dtpm_cpu.cpu);
    ret = dtpm_register(name, &dtpm_cpu.dtpm, parent);
    if (ret)
    goto out_kfree_dtpm_cpu;
    rcu_read_lock();
    table = em_perf_state_from_pd(pd);
    ret = freq_qos_add_request(&policy.constraints,
    &dtpm_cpu.qos_req, FREQ_QOS_MAX,
    table[pd.nr_perf_states - 1].frequency);
    rcu_read_unlock();
    if (ret < 0)
    goto out_dtpm_unregister;
    cpufreq_cpu_put(policy);
    return 0;
    out_dtpm_unregister:
    dtpm_unregister(&dtpm_cpu.dtpm);
    dtpm_cpu = core::ptr::null_mut();
    out_kfree_dtpm_cpu:
    for_each_cpu(cpu, policy.related_cpus)
    per_cpu(dtpm_per_cpu, cpu) = core::ptr::null_mut();
    kfree(dtpm_cpu);
    release_policy:
    cpufreq_cpu_put(policy);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dtpm_cpu_setup(dtpm: *mut dtpm, np: *mut device_node) -> c_int {
    static int dtpm_cpu_setup(struct dtpm *dtpm, struct device_node *np)
    {
    int cpu;
    cpu = of_cpu_node_to_id(np);
    if (cpu < 0)
    return 0;
    return __dtpm_cpu_setup(cpu, dtpm);
    }
#[no_mangle]
unsafe extern "C" fn dtpm_cpu_init() -> c_int {
    static int dtpm_cpu_init(void)
    {
    int ret;
//
// The callbacks at CPU hotplug time are calling
// dtpm_update_power() which in turns calls update_pd_power().
//
// The function update_pd_power() uses the online mask to
// figure out the power consumption limits.
//
// At CPUHP_AP_ONLINE_DYN, the CPU is present in the CPU
// online mask when the cpuhp_dtpm_cpu_online function is
// called, but the CPU is still in the online mask for the
// tear down callback. So the power can not be updated when
// the CPU is unplugged.
//
// At CPUHP_AP_DTPM_CPU_DEAD, the situation is the opposite as
// above. The CPU online mask is not up to date when the CPU
// is plugged in.
//
// For this reason, we need to call the online and offline
// callbacks at different moments when the CPU online mask is
// consistent with the power numbers we want to update.
//
    ret = cpuhp_setup_state(CPUHP_AP_DTPM_CPU_DEAD, "dtpm_cpu:offline",
    core::ptr::null_mut(), cpuhp_dtpm_cpu_offline);
    if (ret < 0)
    return ret;
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "dtpm_cpu:online",
    cpuhp_dtpm_cpu_online, core::ptr::null_mut());
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dtpm_cpu_exit() {
    static void dtpm_cpu_exit(void)
    {
    cpuhp_remove_state_nocalls(CPUHP_AP_ONLINE_DYN);
    cpuhp_remove_state_nocalls(CPUHP_AP_DTPM_CPU_DEAD);
    }
    struct dtpm_subsys_ops dtpm_cpu_ops = {
    .name = KBUILD_MODNAME,
    .init = dtpm_cpu_init,
    .exit = dtpm_cpu_exit,
    .setup = dtpm_cpu_setup,
    };
