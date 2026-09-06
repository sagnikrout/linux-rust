//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/cpufreq_performance.c
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
// linux/drivers/cpufreq/cpufreq_performance.c
//
// Copyright (C) 2002 - 2003 Dominik Brodowski <linux@brodo.de>
//

#[no_mangle]
unsafe extern "C" fn cpufreq_gov_performance_limits(policy: *mut cpufreq_policy) {
    static void cpufreq_gov_performance_limits(struct cpufreq_policy *policy)
    {
    pr_debug("setting to %u kHz\n", policy.max);
    __cpufreq_driver_target(policy, policy.max, CPUFREQ_RELATION_H);
    }
    static struct cpufreq_governor cpufreq_gov_performance = {
    .name		= "performance",
    .owner		= THIS_MODULE,
    .flags		= CPUFREQ_GOV_STRICT_TARGET,
    .limits		= cpufreq_gov_performance_limits,
    };

    struct cpufreq_governor *cpufreq_default_governor(void)
    {
    return &cpufreq_gov_performance;
    }

    struct cpufreq_governor *cpufreq_fallback_governor(void)
    {
    return &cpufreq_gov_performance;
    }

    MODULE_AUTHOR("Dominik Brodowski <linux@brodo.de>");
    MODULE_DESCRIPTION("CPUfreq policy governor 'performance'");
    MODULE_LICENSE("GPL");
    cpufreq_governor_init(cpufreq_gov_performance);
    cpufreq_governor_exit(cpufreq_gov_performance);
