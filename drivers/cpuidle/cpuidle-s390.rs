//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/cpuidle-s390.c
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
// s390 generic CPU idle driver.
//
// Copyright IBM Corp. 2026
//

    static __cpuidle int s390_enter_idle(struct cpuidle_device *dev,
    struct cpuidle_driver *drv,
    int index)
    {
    arch_cpu_idle();
    return index;
    }
    static struct cpuidle_driver s390_cpuidle_driver = {
    .cpumask = (struct cpumask *)cpu_present_mask,
    .name = "s390-idle",
    .states = {
    { /* entry 0 is for polling */},
    {
    .enter			= s390_enter_idle,
    .name			= "IDLE",
    .desc			= "ENABLED WAIT",
    },
    },
    .safe_state_index = 0,
    .state_count = 2,
    };
#[no_mangle]
unsafe extern "C" fn s390_cpuidle_cpu_online(cpu: c_uint) -> c_int {
    static int s390_cpuidle_cpu_online(unsigned int cpu)
    {
    struct cpuidle_device *dev = &per_cpu(cpuidle_dev, cpu);
    int rc;
    if (dev.registered) {
    cpuidle_pause_and_lock();
    rc = cpuidle_enable_device(dev);
    cpuidle_resume_and_unlock();
    if (rc)
    pr_err("Failed to enable cpuidle device on cpu %u\n", cpu);
    } else {
    dev.cpu = cpu;
    rc = cpuidle_register_device(dev);
    if (rc)
    pr_err("Failed to register cpuidle driver on cpu %u\n", cpu);
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn s390_cpuidle_cpu_dead(cpu: c_uint) -> c_int {
    static int s390_cpuidle_cpu_dead(unsigned int cpu)
    {
    struct cpuidle_device *dev = &per_cpu(cpuidle_dev, cpu);
    if (!dev.registered)
    return 0;
    cpuidle_pause_and_lock();
    cpuidle_disable_device(dev);
    cpuidle_resume_and_unlock();
    return 0;
    }
//
// The target_residency and exit_latency values are benchmark-derived estimates
// that remain non-deterministic due to s390's virtualized architecture.
//
// Configuration strategy:
// - Poll idle state: Values derived from the next enabled idle state (EW)
// - Enabled Wait state: Values selected based on idle behavior and empirical
// measurement data
//
// Goal is to improve responsiveness for workloads with frequent sleep/wakeup
// cycles while minimizing any side effects.
//
#[no_mangle]
unsafe extern "C" fn s390_cpuidle_ew_tune() -> void __init {
    static void __init s390_cpuidle_ew_tune(void)
    {
    struct cpuidle_state *state = &s390_cpuidle_driver.states[1];
    if (machine_is_lpar()) {
    state.target_residency = 5;
    state.exit_latency = 5;
    } else {
    state.target_residency = 1;
    state.exit_latency = 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn s390_cpuidle_init() -> int __init {
    static int __init s390_cpuidle_init(void)
    {
    int rc;
    s390_cpuidle_ew_tune();
    cpuidle_poll_state_init(&s390_cpuidle_driver);
    rc = cpuidle_register(&s390_cpuidle_driver, core::ptr::null_mut());
    if (rc)
    return rc;
    rc = cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN,
    "cpuidle/s390:online",
    s390_cpuidle_cpu_online,
    s390_cpuidle_cpu_dead);
    if (rc < 0) {
    cpuidle_unregister(&s390_cpuidle_driver);
    pr_err("Failed to allocate hotplug state: cpuidle/s390:online\n");
    return rc;
    }
    return 0;
    }
    device_initcall(s390_cpuidle_init);
