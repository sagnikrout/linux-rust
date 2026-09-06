//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/cpufreq_stats.c
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
// drivers/cpufreq/cpufreq_stats.c
//
// Copyright (C) 2003-2004 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>.
// (C) 2004 Zou Nan hai <nanhai.zou@intel.com>.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_stats {
    pub total_trans: c_uint,
    pub last_time: c_ulonglong,
    pub max_state: c_uint,
    pub state_num: c_uint,
    pub last_index: c_uint,
    pub time_in_state: *mut u64,
    pub freq_table: *mut c_uint,
    pub trans_table: *mut c_uint,
// Deferred reset
    pub reset_pending: c_uint,
    pub reset_time: c_ulonglong,
}

    static void cpufreq_stats_update(struct cpufreq_stats *stats,
    unsigned long long time)
    {
    let mut cur_time: c_ulonglong = local_clock();
    stats.time_in_state[stats.last_index] += cur_time - time;
    stats.last_time = cur_time;
    }
#[no_mangle]
unsafe extern "C" fn cpufreq_stats_reset_table(stats: *mut cpufreq_stats) {
    static void cpufreq_stats_reset_table(struct cpufreq_stats *stats)
    {
    let mut count: c_uint = stats.max_state;
    memset(stats.time_in_state, 0, count * sizeof(u64));
    memset(stats.trans_table, 0, count * count * sizeof(int));
    stats.last_time = local_clock();
    stats.total_trans = 0;
// Adjust for the time elapsed since reset was requested
    WRITE_ONCE(stats.reset_pending, 0);
//
// Prevent the reset_time read from being reordered before the
// reset_pending accesses in cpufreq_stats_record_transition().
//
    smp_rmb();
    cpufreq_stats_update(stats, READ_ONCE(stats.reset_time));
    }
#[no_mangle]
unsafe extern "C" fn show_total_trans(policy: *mut cpufreq_policy, buf: *mut c_char) -> isize {
    static ssize_t show_total_trans(struct cpufreq_policy *policy, char *buf)
    {
    struct cpufreq_stats *stats = policy.stats;
    if (READ_ONCE(stats.reset_pending))
    return sprintf(buf, "%d\n", 0);
    else
    return sprintf(buf, "%u\n", stats.total_trans);
    }
    cpufreq_freq_attr_ro(total_trans);
#[no_mangle]
unsafe extern "C" fn show_time_in_state(policy: *mut cpufreq_policy, buf: *mut c_char) -> isize {
    static ssize_t show_time_in_state(struct cpufreq_policy *policy, char *buf)
    {
    struct cpufreq_stats *stats = policy.stats;
    let mut pending: bool = READ_ONCE(stats.reset_pending);
    unsigned long long time;
    let mut len: isize = 0;
    int i;
    for (i = 0; i < stats.state_num; i++) {
    if (pending) {
    if (i == stats.last_index) {
//
// Prevent the reset_time read from occurring
// before the reset_pending read above.
//
    smp_rmb();
    time = local_clock() - READ_ONCE(stats.reset_time);
    } else {
    time = 0;
    }
    } else {
    time = stats.time_in_state[i];
    if (i == stats.last_index)
    time += local_clock() - stats.last_time;
    }
    len += sprintf(buf + len, "%u %llu\n", stats.freq_table[i],
    nsec_to_clock_t(time));
    }
    return len;
    }
    cpufreq_freq_attr_ro(time_in_state);
// We don't care what is written to the attribute
    static ssize_t store_reset(struct cpufreq_policy *policy, const char *buf,
    size_t count)
    {
    struct cpufreq_stats *stats = policy.stats;
//
// Defer resetting of stats to cpufreq_stats_record_transition() to
// avoid races.
//
    WRITE_ONCE(stats.reset_time, local_clock());
//
// The memory barrier below is to prevent the readers of reset_time from
// seeing a stale or partially updated value.
//
    smp_wmb();
    WRITE_ONCE(stats.reset_pending, 1);
    return count;
    }
    cpufreq_freq_attr_wo(reset);
#[no_mangle]
unsafe extern "C" fn show_trans_table(policy: *mut cpufreq_policy, buf: *mut c_char) -> isize {
    static ssize_t show_trans_table(struct cpufreq_policy *policy, char *buf)
    {
    struct cpufreq_stats *stats = policy.stats;
    let mut pending: bool = READ_ONCE(stats.reset_pending);
    let mut len: isize = 0;
    int i, j, count;
    len += sysfs_emit_at(buf, len, "   From  :    To\n");
    len += sysfs_emit_at(buf, len, "         : ");
    for (i = 0; i < stats.state_num; i++) {
    if (len >= PAGE_SIZE - 1)
    break;
    len += sysfs_emit_at(buf, len, "%9u ", stats.freq_table[i]);
    }
    if (len >= PAGE_SIZE - 1)
    return PAGE_SIZE - 1;
    len += sysfs_emit_at(buf, len, "\n");
    for (i = 0; i < stats.state_num; i++) {
    if (len >= PAGE_SIZE - 1)
    break;
    len += sysfs_emit_at(buf, len, "%9u: ", stats.freq_table[i]);
    for (j = 0; j < stats.state_num; j++) {
    if (len >= PAGE_SIZE - 1)
    break;
    if (pending)
    count = 0;
    else
    count = stats.trans_table[i * stats.max_state + j];
    len += sysfs_emit_at(buf, len, "%9u ", count);
    }
    if (len >= PAGE_SIZE - 1)
    break;
    len += sysfs_emit_at(buf, len, "\n");
    }
    if (len >= PAGE_SIZE - 1) {
    pr_warn_once("cpufreq transition table exceeds PAGE_SIZE. Disabling\n");
    return -EFBIG;
    }
    return len;
    }
    cpufreq_freq_attr_ro(trans_table);
    static struct attribute *default_attrs[] = {
    &total_trans.attr,
    &time_in_state.attr,
    &reset.attr,
    &trans_table.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group stats_attr_group = {
    .attrs = default_attrs,
    .name = "stats"
    };
#[no_mangle]
unsafe extern "C" fn freq_table_get_index(stats: *mut cpufreq_stats, freq: c_uint) -> c_int {
    static int freq_table_get_index(struct cpufreq_stats *stats, unsigned int freq)
    {
    int index;
    for (index = 0; index < stats.max_state; index++)
    if (stats.freq_table[index] == freq)
    return index;
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn cpufreq_stats_free_table(policy: *mut cpufreq_policy) {
    void cpufreq_stats_free_table(struct cpufreq_policy *policy)
    {
    struct cpufreq_stats *stats = policy.stats;
// Already freed
    if (!stats)
    return;
    pr_debug("%s: Free stats table\n", __func__);
    sysfs_remove_group(&policy.kobj, &stats_attr_group);
    kfree(stats.time_in_state);
    kfree(stats);
    policy.stats = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn cpufreq_stats_create_table(policy: *mut cpufreq_policy) {
    void cpufreq_stats_create_table(struct cpufreq_policy *policy)
    {
    let mut i: c_uint = 0, count;
    struct cpufreq_stats *stats;
    unsigned int alloc_size;
    struct cpufreq_frequency_table *pos;
    count = cpufreq_table_count_valid_entries(policy);
    if (!count)
    return;
// stats already initialized
    if (policy.stats)
    return;
    stats = kzalloc_obj(*stats);
    if (!stats)
    return;
    alloc_size = count * sizeof(int) + count * sizeof(u64);
    alloc_size += count * count * sizeof(int);
// Allocate memory for time_in_state/freq_table/trans_table in one go
    stats.time_in_state = kzalloc(alloc_size, GFP_KERNEL);
    if (!stats.time_in_state)
    goto free_stat;
    stats.freq_table = (unsigned int *)(stats.time_in_state + count);
    stats.trans_table = stats.freq_table + count;
    stats.max_state = count;
// Find valid-unique entries
    cpufreq_for_each_valid_entry(pos, policy.freq_table)
    if (policy.freq_table_sorted != CPUFREQ_TABLE_UNSORTED ||
    freq_table_get_index(stats, pos.frequency) == -1)
    stats.freq_table[i++] = pos.frequency;
    stats.state_num = i;
    stats.last_time = local_clock();
    stats.last_index = freq_table_get_index(stats, policy.cur);
    policy.stats = stats;
    if (!sysfs_create_group(&policy.kobj, &stats_attr_group))
    return;
// We failed, release resources
    policy.stats = core::ptr::null_mut();
    kfree(stats.time_in_state);
    free_stat:
    kfree(stats);
    }
    void cpufreq_stats_record_transition(struct cpufreq_policy *policy,
    unsigned int new_freq)
    {
    struct cpufreq_stats *stats = policy.stats;
    int old_index, new_index;
    if (unlikely(!stats))
    return;
    if (unlikely(READ_ONCE(stats.reset_pending)))
    cpufreq_stats_reset_table(stats);
    old_index = stats.last_index;
    new_index = freq_table_get_index(stats, new_freq);
// We can't do stats->time_in_state[-1]= ..
    if (unlikely(old_index == -1 || new_index == -1 || old_index == new_index))
    return;
    cpufreq_stats_update(stats, stats.last_time);
    stats.last_index = new_index;
    stats.trans_table[old_index * stats.max_state + new_index]++;
    stats.total_trans++;
    }
