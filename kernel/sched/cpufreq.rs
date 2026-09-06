//! Automatically rewritten from C to Rust
//! Source: kernel/sched/cpufreq.c
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
// Scheduler code and data structures related to cpufreq.
//
// Copyright (C) 2016, Intel Corporation
// Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
//

    DEFINE_PER_CPU(struct update_util_data __rcu *, cpufreq_update_util_data);
//
// cpufreq_add_update_util_hook - Populate the CPU's update_util_data pointer.
// @cpu: The CPU to set the pointer for.
// @data: New pointer value.
// @func: Callback function to set for the CPU.
//
// Set and publish the update_util_data pointer for the given CPU.
//
// The update_util_data pointer of @cpu is set to @data and the callback
// function pointer in the target struct update_util_data is set to @func.
// That function will be called by cpufreq_update_util() from RCU-sched
// read-side critical sections, so it must not sleep.  @data will always be
// passed to it as the first argument which allows the function to get to the
// target update_util_data structure and its container.
//
// The update_util_data pointer of @cpu must be NULL when this function is
// called or it will WARN() and return with no effect.
//
    void cpufreq_add_update_util_hook(int cpu, struct update_util_data *data,
    void (*func)(struct update_util_data *data, u64 time,
    unsigned int flags))
    {
    if (WARN_ON(!data || !func))
    return;
    if (WARN_ON(per_cpu(cpufreq_update_util_data, cpu)))
    return;
    data.func = func;
    rcu_assign_pointer(per_cpu(cpufreq_update_util_data, cpu), data);
    }
    EXPORT_SYMBOL_GPL(cpufreq_add_update_util_hook);
//
// cpufreq_remove_update_util_hook - Clear the CPU's update_util_data pointer.
// @cpu: The CPU to clear the pointer for.
//
// Clear the update_util_data pointer for the given CPU.
//
// Callers must use RCU callbacks to free any memory that might be
// accessed via the old update_util_data pointer or invoke synchronize_rcu()
// right after this function to avoid use-after-free.
//
#[no_mangle]
pub unsafe extern "C" fn cpufreq_remove_update_util_hook(cpu: c_int) {
    void cpufreq_remove_update_util_hook(int cpu)
    {
    rcu_assign_pointer(per_cpu(cpufreq_update_util_data, cpu), core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(cpufreq_remove_update_util_hook);
//
// cpufreq_this_cpu_can_update - Check if cpufreq policy can be updated.
// @policy: cpufreq policy to check.
//
// Return 'true' if:
// - the local and remote CPUs share @policy,
// - dvfs_possible_from_any_cpu is set in @policy and the local CPU is not going
// offline (in which case it is not expected to run cpufreq updates any more).
//
#[no_mangle]
pub unsafe extern "C" fn cpufreq_this_cpu_can_update(policy: *mut cpufreq_policy) -> bool {
    bool cpufreq_this_cpu_can_update(struct cpufreq_policy *policy)
    {
    return cpumask_test_cpu(smp_processor_id(), policy.cpus) ||
    (policy.dvfs_possible_from_any_cpu &&
    rcu_dereference_sched(*this_cpu_ptr(&cpufreq_update_util_data)));
    }
