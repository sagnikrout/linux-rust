//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/uncore-frequency/uncore-frequency.c
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
// Intel Uncore Frequency Setting
// Copyright (c) 2022, Intel Corporation.
// All rights reserved.
//
// Provide interface to set MSR 620 at a granularity of per die. On CPU online,
// one control CPU is identified per die to read/write limit. This control CPU
// is changed, if the CPU state is changed to offline. When the last CPU is
// offline in a die then remove the sysfs object for that die.
// The majority of actual code is related to sysfs create and read/write
// attributes.
//
// Author: Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>
//

// Max instances for uncore data, one for each die
    static int uncore_max_entries __read_mostly;
// Storage for uncore data for all instances
    static struct uncore_data *uncore_instances;
// Stores the CPU mask of the target CPUs to use during uncore read/write
    static cpumask_t uncore_cpu_mask;
// CPU online callback register instance
    static enum cpuhp_state uncore_hp_state __read_mostly;
pub const MSR_UNCORE_RATIO_LIMIT: c_uint = 0x620;
pub const MSR_UNCORE_PERF_STATUS: c_uint = 0x621;
pub const UNCORE_FREQ_KHZ_MULTIPLIER: c_int = 100000;

    static int uncore_read_control_freq(struct uncore_data *data, unsigned int *value,
    enum uncore_index index)
    {
    u64 cap;
    int ret;
    if (data.control_cpu < 0)
    return -ENXIO;
    ret = rdmsrq_on_cpu(data.control_cpu, MSR_UNCORE_RATIO_LIMIT, &cap);
    if (ret)
    return ret;
    if (index == UNCORE_INDEX_MAX_FREQ)
// value = FIELD_GET(UNCORE_MAX_RATIO_MASK, cap) * UNCORE_FREQ_KHZ_MULTIPLIER;
    else
// value = FIELD_GET(UNCORE_MIN_RATIO_MASK, cap) * UNCORE_FREQ_KHZ_MULTIPLIER;
    return 0;
    }
    static int uncore_write_control_freq(struct uncore_data *data, unsigned int input,
    enum uncore_index index)
    {
    int ret;
    u64 cap;
    input /= UNCORE_FREQ_KHZ_MULTIPLIER;
    if (!input || input > FIELD_MAX(UNCORE_MAX_RATIO_MASK))
    return -EINVAL;
    if (data.control_cpu < 0)
    return -ENXIO;
    ret = rdmsrq_on_cpu(data.control_cpu, MSR_UNCORE_RATIO_LIMIT, &cap);
    if (ret)
    return ret;
    if (index == UNCORE_INDEX_MAX_FREQ) {
    cap &= ~UNCORE_MAX_RATIO_MASK;
    cap |= FIELD_PREP(UNCORE_MAX_RATIO_MASK, input);
    } else  {
    cap &= ~UNCORE_MIN_RATIO_MASK;
    cap |= FIELD_PREP(UNCORE_MIN_RATIO_MASK, input);
    }
    ret = wrmsrq_on_cpu(data.control_cpu, MSR_UNCORE_RATIO_LIMIT, cap);
    if (ret)
    return ret;
    data.stored_uncore_data = cap;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uncore_read_freq(data: *mut uncore_data, freq: *mut c_uint) -> c_int {
    static int uncore_read_freq(struct uncore_data *data, unsigned int *freq)
    {
    u64 ratio;
    int ret;
    if (data.control_cpu < 0)
    return -ENXIO;
    ret = rdmsrq_on_cpu(data.control_cpu, MSR_UNCORE_PERF_STATUS, &ratio);
    if (ret)
    return ret;
// freq = FIELD_GET(UNCORE_CURRENT_RATIO_MASK, ratio) * UNCORE_FREQ_KHZ_MULTIPLIER;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uncore_read(data: *mut uncore_data, value: *mut c_uint, index: enum uncore_index) -> c_int {
    static int uncore_read(struct uncore_data *data, unsigned int *value, enum uncore_index index)
    {
    switch (index) {
    case UNCORE_INDEX_MIN_FREQ:
    case UNCORE_INDEX_MAX_FREQ:
    return uncore_read_control_freq(data, value, index);
    case UNCORE_INDEX_CURRENT_FREQ:
    return uncore_read_freq(data, value);
    default:
    break;
    }
    return -EOPNOTSUPP;
    }
// Caller provides protection
    static struct uncore_data *uncore_get_instance(unsigned int cpu)
    {
    let mut id: c_int = topology_logical_die_id(cpu);
    if (id >= 0 && id < uncore_max_entries)
    return &uncore_instances[id];
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn uncore_event_cpu_online(cpu: c_uint) -> c_int {
    static int uncore_event_cpu_online(unsigned int cpu)
    {
    struct uncore_data *data;
    int target;
    int ret;
// Check if there is an online cpu in the package for uncore MSR
    target = cpumask_any_and(&uncore_cpu_mask, topology_die_cpumask(cpu));
    if (target < nr_cpu_ids)
    return 0;
    data = uncore_get_instance(cpu);
    if (!data)
    return 0;
    data.package_id = topology_physical_package_id(cpu);
    data.die_id = topology_die_id(cpu);
    data.domain_id = UNCORE_DOMAIN_ID_INVALID;
    ret = uncore_freq_add_entry(data, cpu);
    if (ret)
    return ret;
// Use this CPU on this die as a control CPU
    cpumask_set_cpu(cpu, &uncore_cpu_mask);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uncore_event_cpu_offline(cpu: c_uint) -> c_int {
    static int uncore_event_cpu_offline(unsigned int cpu)
    {
    struct uncore_data *data;
    int target;
    data = uncore_get_instance(cpu);
    if (!data)
    return 0;
// Check if existing cpu is used for uncore MSRs
    if (!cpumask_test_and_clear_cpu(cpu, &uncore_cpu_mask))
    return 0;
// Find a new cpu to set uncore MSR
    target = cpumask_any_but(topology_die_cpumask(cpu), cpu);
    if (target < nr_cpu_ids) {
    cpumask_set_cpu(target, &uncore_cpu_mask);
    uncore_freq_add_entry(data, target);
    } else {
    uncore_freq_remove_die_entry(data);
    }
    return 0;
    }
    static int uncore_pm_notify(struct notifier_block *nb, unsigned long mode,
    void *_unused)
    {
    int i;
    switch (mode) {
    case PM_POST_HIBERNATION:
    case PM_POST_RESTORE:
    case PM_POST_SUSPEND:
    for (i = 0; i < uncore_max_entries; ++i) {
    struct uncore_data *data = &uncore_instances[i];
    if (!data || !data.valid || !data.stored_uncore_data)
    return 0;
    wrmsrq_on_cpu(data.control_cpu, MSR_UNCORE_RATIO_LIMIT,
    data.stored_uncore_data);
    }
    break;
    default:
    break;
    }
    return 0;
    }
    static struct notifier_block uncore_pm_nb = {
    .notifier_call = uncore_pm_notify,
    };
    static const struct x86_cpu_id intel_uncore_cpu_ids[] = {
    X86_MATCH_VFM(INTEL_BROADWELL_G,	core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_BROADWELL_X,	core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_BROADWELL_D,	core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_SKYLAKE_X,	core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ICELAKE_X,	core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ICELAKE_D,	core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_SAPPHIRERAPIDS_X, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_EMERALDRAPIDS_X, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_KABYLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_KABYLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_COMETLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_COMETLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_CANNONLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ICELAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ICELAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ROCKETLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_TIGERLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_TIGERLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ALDERLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ALDERLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_RAPTORLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_RAPTORLAKE_P, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_RAPTORLAKE_S, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_METEORLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_METEORLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ARROWLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ARROWLAKE_H, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_LUNARLAKE_M, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_PANTHERLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_WILDCATLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_NOVALAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_NOVALAKE_L, core::ptr::null_mut()),
    {}
    };
    MODULE_DEVICE_TABLE(x86cpu, intel_uncore_cpu_ids);
#[no_mangle]
unsafe extern "C" fn intel_uncore_init() -> int __init {
    static int __init intel_uncore_init(void)
    {
    const struct x86_cpu_id *id;
    int ret;
    if (cpu_feature_enabled(X86_FEATURE_HYPERVISOR))
    return -ENODEV;
    id = x86_match_cpu(intel_uncore_cpu_ids);
    if (!id)
    return -ENODEV;
    uncore_max_entries = topology_max_packages() *
    topology_max_dies_per_package();
    uncore_instances = kzalloc_objs(*uncore_instances, uncore_max_entries);
    if (!uncore_instances)
    return -ENOMEM;
    ret = uncore_freq_common_init(uncore_read, uncore_write_control_freq);
    if (ret)
    goto err_free;
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN,
    "platform/x86/uncore-freq:online",
    uncore_event_cpu_online,
    uncore_event_cpu_offline);
    if (ret < 0)
    goto err_rem_kobj;
    uncore_hp_state = ret;
    ret = register_pm_notifier(&uncore_pm_nb);
    if (ret)
    goto err_rem_state;
    return 0;
    err_rem_state:
    cpuhp_remove_state(uncore_hp_state);
    err_rem_kobj:
    uncore_freq_common_exit();
    err_free:
    kfree(uncore_instances);
    return ret;
    }
    module_init(intel_uncore_init)
#[no_mangle]
unsafe extern "C" fn intel_uncore_exit() -> void __exit {
    static void __exit intel_uncore_exit(void)
    {
    int i;
    unregister_pm_notifier(&uncore_pm_nb);
    cpuhp_remove_state(uncore_hp_state);
    for (i = 0; i < uncore_max_entries; ++i)
    uncore_freq_remove_die_entry(&uncore_instances[i]);
    uncore_freq_common_exit();
    kfree(uncore_instances);
    }
    module_exit(intel_uncore_exit)
    MODULE_IMPORT_NS("INTEL_UNCORE_FREQUENCY");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Intel Uncore Frequency Limits Driver");
