//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/itmt.c
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
// itmt.c: Support Intel Turbo Boost Max Technology 3.0
//
// (C) Copyright 2016 Intel Corporation
// Author: Tim Chen <tim.c.chen@linux.intel.com>
//
// On platforms supporting Intel Turbo Boost Max Technology 3.0, (ITMT),
// the maximum turbo frequencies of some cores in a CPU package may be
// higher than for the other cores in the same package.  In that case,
// better performance can be achieved by making the scheduler prefer
// to run tasks on the CPUs with higher max turbo frequencies.
//
// This file provides functions and data structures for enabling the
// scheduler to favor scheduling on cores can be boosted to a higher
// frequency under ITMT.
//

    static DEFINE_MUTEX(itmt_update_mutex);
    DEFINE_PER_CPU_READ_MOSTLY(int, sched_core_priority);
// Boolean to track if system has ITMT capabilities
    static bool __read_mostly sched_itmt_capable;
//
// Boolean to control whether we want to move processes to cpu capable
// of higher turbo frequency for cpus supporting Intel Turbo Boost Max
// Technology 3.0.
//
// It can be set via /sys/kernel/debug/x86/sched_itmt_enabled
//
    bool __read_mostly sysctl_sched_itmt_enabled;
    static ssize_t sched_itmt_enabled_write(struct file *filp,
    const char __user *ubuf,
    size_t cnt, loff_t *ppos)
    {
    ssize_t result;
    bool orig;
    guard(mutex)(&itmt_update_mutex);
    orig = sysctl_sched_itmt_enabled;
    result = debugfs_write_file_bool(filp, ubuf, cnt, ppos);
    if (sysctl_sched_itmt_enabled != orig) {
    x86_topology_update = true;
    rebuild_sched_domains();
    }
    return result;
    }
#[no_mangle]
unsafe extern "C" fn sched_core_priority_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int sched_core_priority_show(struct seq_file *s, void *unused)
    {
    int cpu;
    seq_puts(s, "CPU #\tPriority\n");
    for_each_possible_cpu(cpu)
    seq_printf(s, "%d\t%d\n", cpu, arch_asym_cpu_priority(cpu));
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(sched_core_priority);
    static const struct file_operations dfs_sched_itmt_fops = {
    .read =         debugfs_read_file_bool,
    .write =        sched_itmt_enabled_write,
    .open =         simple_open,
    .llseek =       default_llseek,
    };
    static struct dentry *dfs_sched_itmt;
    static struct dentry *dfs_sched_core_prio;
//
// sched_set_itmt_support() - Indicate platform supports ITMT
//
// This function is used by the OS to indicate to scheduler that the platform
// is capable of supporting the ITMT feature.
//
// The current scheme has the pstate driver detects if the system
// is ITMT capable and call sched_set_itmt_support.
//
// This must be done only after sched_set_itmt_core_prio
// has been called to set the cpus' priorities.
// It must not be called with cpu hot plug lock
// held as we need to acquire the lock to rebuild sched domains
// later.
//
// Return: 0 on success
//
#[no_mangle]
pub unsafe extern "C" fn sched_set_itmt_support() -> c_int {
    int sched_set_itmt_support(void)
    {
    guard(mutex)(&itmt_update_mutex);
    if (sched_itmt_capable)
    return 0;
    dfs_sched_itmt = debugfs_create_file_unsafe("sched_itmt_enabled",
    0644,
    arch_debugfs_dir,
    &sysctl_sched_itmt_enabled,
    &dfs_sched_itmt_fops);
    if (IS_ERR_OR_NULL(dfs_sched_itmt)) {
    dfs_sched_itmt = core::ptr::null_mut();
    return -ENOMEM;
    }
    dfs_sched_core_prio = debugfs_create_file("sched_core_priority", 0644,
    arch_debugfs_dir, core::ptr::null_mut(),
    &sched_core_priority_fops);
    if (IS_ERR_OR_NULL(dfs_sched_core_prio)) {
    dfs_sched_core_prio = core::ptr::null_mut();
    return -ENOMEM;
    }
    sched_itmt_capable = true;
    sysctl_sched_itmt_enabled = 1;
    x86_topology_update = true;
    rebuild_sched_domains();
    return 0;
    }
//
// sched_clear_itmt_support() - Revoke platform's support of ITMT
//
// This function is used by the OS to indicate that it has
// revoked the platform's support of ITMT feature.
//
// It must not be called with cpu hot plug lock
// held as we need to acquire the lock to rebuild sched domains
// later.
//
#[no_mangle]
pub unsafe extern "C" fn sched_clear_itmt_support() {
    void sched_clear_itmt_support(void)
    {
    guard(mutex)(&itmt_update_mutex);
    if (!sched_itmt_capable)
    return;
    sched_itmt_capable = false;
    debugfs_remove(dfs_sched_itmt);
    dfs_sched_itmt = core::ptr::null_mut();
    debugfs_remove(dfs_sched_core_prio);
    dfs_sched_core_prio = core::ptr::null_mut();
    if (sysctl_sched_itmt_enabled) {
// disable sched_itmt if we are no longer ITMT capable
    sysctl_sched_itmt_enabled = 0;
    x86_topology_update = true;
    rebuild_sched_domains();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_asym_cpu_priority(cpu: c_int) -> c_int {
    int arch_asym_cpu_priority(int cpu)
    {
    return per_cpu(sched_core_priority, cpu);
    }
//
// sched_set_itmt_core_prio() - Set CPU priority based on ITMT
// @prio:	Priority of @cpu
// @cpu:	The CPU number
//
// The pstate driver will find out the max boost frequency
// and call this function to set a priority proportional
// to the max boost frequency. CPUs with higher boost
// frequency will receive higher priority.
//
// No need to rebuild sched domain after updating
// the CPU priorities. The sched domains have no
// dependency on CPU priorities.
//
#[no_mangle]
pub unsafe extern "C" fn sched_set_itmt_core_prio(prio: c_int, cpu: c_int) {
    void sched_set_itmt_core_prio(int prio, int cpu)
    {
    per_cpu(sched_core_priority, cpu) = prio;
    }
