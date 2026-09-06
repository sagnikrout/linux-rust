//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/watchdog_hld.c
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
// Safe maximum CPU frequency in case a particular platform doesn't implement
// cpufreq driver. Although, architecture doesn't put any restrictions on
// maximum frequency but 5 GHz seems to be safe maximum given the available
// Arm CPUs in the market which are clocked much less than 5 GHz. On the other
// hand, we can't make it much higher as it would lead to a large hard-lockup
// detection timeout on parts which are running slower (eg. 1GHz on
// Developerbox) and doesn't possess a cpufreq driver.
//

#[no_mangle]
pub unsafe extern "C" fn hw_nmi_get_sample_period(watchdog_thresh: c_int) -> u64 {
    u64 hw_nmi_get_sample_period(int watchdog_thresh)
    {
    let mut cpu: c_uint = smp_processor_id();
    unsigned long max_cpu_freq;
    max_cpu_freq = cpufreq_get_hw_max_freq(cpu) * 1000UL;
    if (!max_cpu_freq)
    max_cpu_freq = SAFE_MAX_CPU_FREQ;
    return (u64)max_cpu_freq * watchdog_thresh;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_perf_nmi_is_available() -> bool __init {
    bool __init arch_perf_nmi_is_available(void)
    {
//
// hardlockup_detector_perf_init() will success even if Pseudo-NMI turns off,
// however, the pmu interrupts will act like a normal interrupt instead of
// NMI and the hardlockup detector would be broken.
//
    return arm_pmu_irq_is_nmi();
    }
#[no_mangle]
unsafe extern "C" fn watchdog_perf_update_period(data: *mut c_void) -> c_int {
    static int watchdog_perf_update_period(void *data)
    {
    let mut cpu: c_int = smp_processor_id();
    u64 max_cpu_freq, new_period;
    max_cpu_freq = cpufreq_get_hw_max_freq(cpu) * 1000UL;
    if (!max_cpu_freq)
    return 0;
    new_period = watchdog_thresh * max_cpu_freq;
    hardlockup_detector_perf_adjust_period(new_period);
    return 0;
    }
    static int watchdog_freq_notifier_callback(struct notifier_block *nb,
    unsigned long val, void *data)
    {
    struct cpufreq_policy *policy = data;
    int cpu;
    if (val != CPUFREQ_CREATE_POLICY)
    return NOTIFY_DONE;
//
// Let each online CPU related to the policy update the period by their
// own. This will serialize with the framework on start/stop the lockup
// detector (softlockup_{start,stop}_all) and avoid potential race
// condition. Otherwise we may have below theoretical race condition:
// (core 0/1 share the same policy)
// [core 0]                      [core 1]
// hardlockup_detector_event_create()
// hw_nmi_get_sample_period()
// (cpufreq registered, notifier callback invoked)
// watchdog_freq_notifier_callback()
// watchdog_perf_update_period()
// (since core 1's event's not yet created,
// the period is not set)
// perf_event_create_kernel_counter()
// (event's period is SAFE_MAX_CPU_FREQ)
//
    for_each_cpu(cpu, policy.cpus)
    smp_call_on_cpu(cpu, watchdog_perf_update_period, core::ptr::null_mut(), false);
    return NOTIFY_DONE;
    }
    static struct notifier_block watchdog_freq_notifier = {
    .notifier_call = watchdog_freq_notifier_callback,
    };
#[no_mangle]
unsafe extern "C" fn init_watchdog_freq_notifier() -> int __init {
    static int __init init_watchdog_freq_notifier(void)
    {
    return cpufreq_register_notifier(&watchdog_freq_notifier,
    CPUFREQ_POLICY_NOTIFIER);
    }
    core_initcall(init_watchdog_freq_notifier);
