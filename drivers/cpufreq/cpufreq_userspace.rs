//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/cpufreq_userspace.c
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
// linux/drivers/cpufreq/cpufreq_userspace.c
//
// Copyright (C)  2001 Russell King
// (C)  2002 - 2004 Dominik Brodowski <linux@brodo.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct userspace_policy {
    pub is_managed: c_uint,
    pub setspeed: c_uint,
    pub mutex: mutex,
}

//
// cpufreq_set - set the CPU frequency
// @policy: pointer to policy struct where freq is being set
// @freq: target frequency in kHz
//
// Sets the CPU frequency to freq.
//
#[no_mangle]
unsafe extern "C" fn cpufreq_set(policy: *mut cpufreq_policy, freq: c_uint) -> c_int {
    static int cpufreq_set(struct cpufreq_policy *policy, unsigned int freq)
    {
    let mut ret: c_int = -EINVAL;
    struct userspace_policy *userspace = policy.governor_data;
    pr_debug("cpufreq_set for cpu %u, freq %u kHz\n", policy.cpu, freq);
    mutex_lock(&userspace.mutex);
    if (!userspace.is_managed)
    goto err;
    userspace.setspeed = freq;
    ret = __cpufreq_driver_target(policy, freq, CPUFREQ_RELATION_L);
    err:
    mutex_unlock(&userspace.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn show_speed(policy: *mut cpufreq_policy, buf: *mut c_char) -> isize {
    static ssize_t show_speed(struct cpufreq_policy *policy, char *buf)
    {
    struct userspace_policy *userspace = policy.governor_data;
    return sprintf(buf, "%u\n", userspace.setspeed);
    }
#[no_mangle]
unsafe extern "C" fn cpufreq_userspace_policy_init(policy: *mut cpufreq_policy) -> c_int {
    static int cpufreq_userspace_policy_init(struct cpufreq_policy *policy)
    {
    struct userspace_policy *userspace;
    userspace = kzalloc_obj(*userspace);
    if (!userspace)
    return -ENOMEM;
    mutex_init(&userspace.mutex);
    policy.governor_data = userspace;
    return 0;
    }
//
// Any routine that writes to the policy struct will hold the "rwsem" of
// policy struct that means it is free to free "governor_data" here.
//
#[no_mangle]
unsafe extern "C" fn cpufreq_userspace_policy_exit(policy: *mut cpufreq_policy) {
    static void cpufreq_userspace_policy_exit(struct cpufreq_policy *policy)
    {
    kfree(policy.governor_data);
    policy.governor_data = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn cpufreq_userspace_policy_start(policy: *mut cpufreq_policy) -> c_int {
    static int cpufreq_userspace_policy_start(struct cpufreq_policy *policy)
    {
    struct userspace_policy *userspace = policy.governor_data;
    BUG_ON(!policy.cur);
    pr_debug("started managing cpu %u\n", policy.cpu);
    mutex_lock(&userspace.mutex);
    userspace.is_managed = 1;
    userspace.setspeed = policy.cur;
    mutex_unlock(&userspace.mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpufreq_userspace_policy_stop(policy: *mut cpufreq_policy) {
    static void cpufreq_userspace_policy_stop(struct cpufreq_policy *policy)
    {
    struct userspace_policy *userspace = policy.governor_data;
    pr_debug("managing cpu %u stopped\n", policy.cpu);
    mutex_lock(&userspace.mutex);
    userspace.is_managed = 0;
    userspace.setspeed = 0;
    mutex_unlock(&userspace.mutex);
    }
#[no_mangle]
unsafe extern "C" fn cpufreq_userspace_policy_limits(policy: *mut cpufreq_policy) {
    static void cpufreq_userspace_policy_limits(struct cpufreq_policy *policy)
    {
    struct userspace_policy *userspace = policy.governor_data;
    mutex_lock(&userspace.mutex);
    pr_debug("limit event for cpu %u: %u - %u kHz, currently %u kHz, last set to %u kHz\n",
    policy.cpu, policy.min, policy.max, policy.cur, userspace.setspeed);
    if (policy.max < userspace.setspeed)
    __cpufreq_driver_target(policy, policy.max,
    CPUFREQ_RELATION_H);
#[no_mangle]
pub unsafe extern "C" fn if(userspace->setspeed: policy->min >) -> else {
    else if (policy.min > userspace.setspeed)
    __cpufreq_driver_target(policy, policy.min,
    CPUFREQ_RELATION_L);
    else
    __cpufreq_driver_target(policy, userspace.setspeed,
    CPUFREQ_RELATION_L);
    mutex_unlock(&userspace.mutex);
    }
    static struct cpufreq_governor cpufreq_gov_userspace = {
    .name		= "userspace",
    .init		= cpufreq_userspace_policy_init,
    .exit		= cpufreq_userspace_policy_exit,
    .start		= cpufreq_userspace_policy_start,
    .stop		= cpufreq_userspace_policy_stop,
    .limits		= cpufreq_userspace_policy_limits,
    .store_setspeed	= cpufreq_set,
    .show_setspeed	= show_speed,
    .owner		= THIS_MODULE,
    .flags		= CPUFREQ_GOV_STRICT_TARGET,
    };
    MODULE_AUTHOR("Dominik Brodowski <linux@brodo.de>, "
    "Russell King <rmk@arm.linux.org.uk>");
    MODULE_DESCRIPTION("CPUfreq policy governor 'userspace'");
    MODULE_LICENSE("GPL");

    struct cpufreq_governor *cpufreq_default_governor(void)
    {
    return &cpufreq_gov_userspace;
    }

    cpufreq_governor_init(cpufreq_gov_userspace);
    cpufreq_governor_exit(cpufreq_gov_userspace);
