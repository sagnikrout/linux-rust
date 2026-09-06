//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/cpufreq_conservative.c
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
// drivers/cpufreq/cpufreq_conservative.c
//
// Copyright (C)  2001 Russell King
// (C)  2003 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>.
// Jun Nakajima <jun.nakajima@intel.com>
// (C)  2009 Alexander Clouter <alex@digriz.org.uk>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_policy_dbs_info {
    pub policy_dbs: policy_dbs_info,
    pub down_skip: c_uint,
    pub requested_freq: c_uint,
}

    static inline struct cs_policy_dbs_info *to_dbs_info(struct policy_dbs_info *policy_dbs)
    {
    return container_of(policy_dbs, struct cs_policy_dbs_info, policy_dbs);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dbs_tuners {
    pub down_threshold: c_uint,
    pub freq_step: c_uint,
}

// Conservative governor macros

    static inline unsigned int get_freq_step(struct cs_dbs_tuners *cs_tuners,
    struct cpufreq_policy *policy)
    {
    let mut freq_step: c_uint = (cs_tuners.freq_step * policy.max) / 100;
// max freq cannot be less than 100. But who knows...
    if (unlikely(freq_step == 0))
    freq_step = DEF_FREQUENCY_STEP;
    return freq_step;
    }
//
// Every sampling_rate, we check, if current idle time is less than 20%
// (default), then we try to increase frequency. Every sampling_rate
// sampling_down_factor, we check, if current idle time is more than 80%
// (default), then we try to decrease frequency
//
// Frequency updates happen at minimum steps of 5% (default) of maximum
// frequency
//
#[no_mangle]
unsafe extern "C" fn cs_dbs_update(policy: *mut cpufreq_policy) -> c_uint {
    static unsigned int cs_dbs_update(struct cpufreq_policy *policy)
    {
    struct policy_dbs_info *policy_dbs = policy.governor_data;
    struct cs_policy_dbs_info *dbs_info = to_dbs_info(policy_dbs);
    let mut requested_freq: c_uint = dbs_info.requested_freq;
    struct dbs_data *dbs_data = policy_dbs.dbs_data;
    struct cs_dbs_tuners *cs_tuners = dbs_data.tuners;
    let mut load: c_uint = dbs_update(policy);
    unsigned int freq_step;
//
// break out if we 'cannot' reduce the speed as the user might
// want freq_step to be zero
//
    if (cs_tuners.freq_step == 0)
    goto out;
//
// If requested_freq is out of range, it is likely that the limits
// changed in the meantime, so fall back to current frequency in that
// case.
//
    if (requested_freq > policy.max || requested_freq < policy.min) {
    requested_freq = policy.cur;
    dbs_info.requested_freq = requested_freq;
    }
    freq_step = get_freq_step(cs_tuners, policy);
//
// Decrease requested_freq one freq_step for each idle period that
// we didn't update the frequency.
//
    if (policy_dbs.idle_periods < UINT_MAX) {
    let mut freq_steps: c_uint = policy_dbs.idle_periods * freq_step;
    if (requested_freq > policy.min + freq_steps)
    requested_freq -= freq_steps;
    else
    requested_freq = policy.min;
    policy_dbs.idle_periods = UINT_MAX;
    }
// Check for frequency increase
    if (load > dbs_data.up_threshold) {
    dbs_info.down_skip = 0;
    requested_freq += freq_step;
    if (requested_freq > policy.max)
    requested_freq = policy.max;
    __cpufreq_driver_target(policy, requested_freq,
    CPUFREQ_RELATION_HE);
    dbs_info.requested_freq = requested_freq;
    goto out;
    }
// if sampling_down_factor is active break out early
    if (++dbs_info.down_skip < dbs_data.sampling_down_factor)
    goto out;
    dbs_info.down_skip = 0;
// Check for frequency decrease
    if (load < cs_tuners.down_threshold) {
    if (requested_freq > policy.min + freq_step)
    requested_freq -= freq_step;
    else
    requested_freq = policy.min;
    __cpufreq_driver_target(policy, requested_freq,
    CPUFREQ_RELATION_LE);
    dbs_info.requested_freq = requested_freq;
    }
    out:
    return dbs_data.sampling_rate;
    }
// sysfs interface
    static ssize_t sampling_down_factor_store(struct gov_attr_set *attr_set,
    const char *buf, size_t count)
    {
    struct dbs_data *dbs_data = to_dbs_data(attr_set);
    unsigned int input;
    int ret;
    ret = kstrtouint(buf, 0, &input);
    if (ret || input > MAX_SAMPLING_DOWN_FACTOR || input < 1)
    return -EINVAL;
    dbs_data.sampling_down_factor = input;
    return count;
    }
    static ssize_t up_threshold_store(struct gov_attr_set *attr_set,
    const char *buf, size_t count)
    {
    struct dbs_data *dbs_data = to_dbs_data(attr_set);
    struct cs_dbs_tuners *cs_tuners = dbs_data.tuners;
    unsigned int input;
    int ret;
    ret = kstrtouint(buf, 0, &input);
    if (ret || input > 100 || input <= cs_tuners.down_threshold)
    return -EINVAL;
    dbs_data.up_threshold = input;
    return count;
    }
    static ssize_t down_threshold_store(struct gov_attr_set *attr_set,
    const char *buf, size_t count)
    {
    struct dbs_data *dbs_data = to_dbs_data(attr_set);
    struct cs_dbs_tuners *cs_tuners = dbs_data.tuners;
    unsigned int input;
    int ret;
    ret = kstrtouint(buf, 0, &input);
// cannot be lower than 1 otherwise freq will not fall
    if (ret || input < 1 || input >= dbs_data.up_threshold)
    return -EINVAL;
    cs_tuners.down_threshold = input;
    return count;
    }
    static ssize_t ignore_nice_load_store(struct gov_attr_set *attr_set,
    const char *buf, size_t count)
    {
    struct dbs_data *dbs_data = to_dbs_data(attr_set);
    unsigned int input;
    int ret;
    ret = kstrtouint(buf, 0, &input);
    if (ret)
    return ret;
    if (input > 1)
    input = 1;
    if (input == dbs_data.ignore_nice_load) /* nothing to do */
    return count;
    dbs_data.ignore_nice_load = input;
// we need to re-evaluate prev_cpu_idle
    gov_update_cpu_data(dbs_data);
    return count;
    }
    static ssize_t freq_step_store(struct gov_attr_set *attr_set, const char *buf,
    size_t count)
    {
    struct dbs_data *dbs_data = to_dbs_data(attr_set);
    struct cs_dbs_tuners *cs_tuners = dbs_data.tuners;
    unsigned int input;
    int ret;
    ret = kstrtouint(buf, 0, &input);
    if (ret)
    return ret;
    if (input > 100)
    input = 100;
//
// no need to test here if freq_step is zero as the user might actually
// want this, they would be crazy though :)
//
    cs_tuners.freq_step = input;
    return count;
    }
    gov_show_one_common(sampling_rate);
    gov_show_one_common(sampling_down_factor);
    gov_show_one_common(up_threshold);
    gov_show_one_common(ignore_nice_load);
    gov_show_one(cs, down_threshold);
    gov_show_one(cs, freq_step);
    gov_attr_rw(sampling_rate);
    gov_attr_rw(sampling_down_factor);
    gov_attr_rw(up_threshold);
    gov_attr_rw(ignore_nice_load);
    gov_attr_rw(down_threshold);
    gov_attr_rw(freq_step);
    static struct attribute *cs_attrs[] = {
    &sampling_rate.attr,
    &sampling_down_factor.attr,
    &up_threshold.attr,
    &down_threshold.attr,
    &ignore_nice_load.attr,
    &freq_step.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(cs);
// sysfs end
    static struct policy_dbs_info *cs_alloc(void)
    {
    struct cs_policy_dbs_info *dbs_info;
    dbs_info = kzalloc_obj(*dbs_info);
    return dbs_info ? &dbs_info.policy_dbs : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn cs_free(policy_dbs: *mut policy_dbs_info) {
    static void cs_free(struct policy_dbs_info *policy_dbs)
    {
    kfree(to_dbs_info(policy_dbs));
    }
#[no_mangle]
unsafe extern "C" fn cs_init(dbs_data: *mut dbs_data) -> c_int {
    static int cs_init(struct dbs_data *dbs_data)
    {
    struct cs_dbs_tuners *tuners;
    tuners = kzalloc_obj(*tuners);
    if (!tuners)
    return -ENOMEM;
    tuners.down_threshold = DEF_FREQUENCY_DOWN_THRESHOLD;
    tuners.freq_step = DEF_FREQUENCY_STEP;
    dbs_data.up_threshold = DEF_FREQUENCY_UP_THRESHOLD;
    dbs_data.sampling_down_factor = DEF_SAMPLING_DOWN_FACTOR;
    dbs_data.ignore_nice_load = 0;
    dbs_data.tuners = tuners;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cs_exit(dbs_data: *mut dbs_data) {
    static void cs_exit(struct dbs_data *dbs_data)
    {
    kfree(dbs_data.tuners);
    }
#[no_mangle]
unsafe extern "C" fn cs_start(policy: *mut cpufreq_policy) {
    static void cs_start(struct cpufreq_policy *policy)
    {
    struct cs_policy_dbs_info *dbs_info = to_dbs_info(policy.governor_data);
    dbs_info.down_skip = 0;
    dbs_info.requested_freq = policy.cur;
    }
#[no_mangle]
unsafe extern "C" fn cs_limits(policy: *mut cpufreq_policy) {
    static void cs_limits(struct cpufreq_policy *policy)
    {
    struct cs_policy_dbs_info *dbs_info = to_dbs_info(policy.governor_data);
//
// The limits have changed, so may have the current frequency. Reset
// requested_freq to avoid any unintended outcomes due to the mismatch.
//
    dbs_info.requested_freq = policy.cur;
    }
    static struct dbs_governor cs_governor = {
    .gov = CPUFREQ_DBS_GOVERNOR_INITIALIZER("conservative"),
    .kobj_type = { .default_groups = cs_groups },
    .gov_dbs_update = cs_dbs_update,
    .alloc = cs_alloc,
    .free = cs_free,
    .init = cs_init,
    .exit = cs_exit,
    .start = cs_start,
    .limits = cs_limits,
    };

    MODULE_AUTHOR("Alexander Clouter <alex@digriz.org.uk>");
    MODULE_DESCRIPTION("'cpufreq_conservative' - A dynamic cpufreq governor for "
    "Low Latency Frequency Transition capable processors "
    "optimised for use in a battery environment");
    MODULE_LICENSE("GPL");

    struct cpufreq_governor *cpufreq_default_governor(void)
    {
    return &CPU_FREQ_GOV_CONSERVATIVE;
    }

    cpufreq_governor_init(CPU_FREQ_GOV_CONSERVATIVE);
    cpufreq_governor_exit(CPU_FREQ_GOV_CONSERVATIVE);
