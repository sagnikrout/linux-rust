//! Automatically rewritten from C to Rust
//! Source: kernel/watchdog_perf.c
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
// Detect hard lockups on a system using perf
//
// started by Don Zickus, Copyright (C) 2010 Red Hat, Inc.
//
// Note: Most of this code is borrowed heavily from the original softlockup
// detector, so thanks to Ingo for the initial implementation.
// Some chunks also taken from the old x86-specific nmi watchdog code, thanks
// to those contributors as well.
//

    static DEFINE_PER_CPU(struct perf_event *, watchdog_ev);
    let mut watchdog_cpus: static atomic_t = ATOMIC_INIT(0);

    static DEFINE_PER_CPU(ktime_t, last_timestamp);
    static DEFINE_PER_CPU(unsigned int, nmi_rearmed);
    static ktime_t watchdog_hrtimer_sample_threshold __read_mostly;
#[no_mangle]
pub unsafe extern "C" fn watchdog_update_hrtimer_threshold(period: u64) {
    void watchdog_update_hrtimer_threshold(u64 period)
    {
//
// The hrtimer runs with a period of (watchdog_threshold * 2) / 5
//
// So it runs effectively with 2.5 times the rate of the NMI
// watchdog. That means the hrtimer should fire 2-3 times before
// the NMI watchdog expires. The NMI watchdog on x86 is based on
// unhalted CPU cycles, so if Turbo-Mode is enabled the CPU cycles
// might run way faster than expected and the NMI fires in a
// smaller period than the one deduced from the nominal CPU
// frequency. Depending on the Turbo-Mode factor this might be fast
// enough to get the NMI period smaller than the hrtimer watchdog
// period and trigger false positives.
//
// The sample threshold is used to check in the NMI handler whether
// the minimum time between two NMI samples has elapsed. That
// prevents false positives.
//
// Set this to 4/5 of the actual watchdog threshold period so the
// hrtimer is guaranteed to fire at least once within the real
// watchdog threshold.
//
    watchdog_hrtimer_sample_threshold = period * 2;
    }
#[no_mangle]
unsafe extern "C" fn watchdog_check_timestamp() -> bool {
    static bool watchdog_check_timestamp(void)
    {
    ktime_t delta, now = ktime_get_mono_fast_ns();
    delta = now - __this_cpu_read(last_timestamp);
    if (delta < watchdog_hrtimer_sample_threshold) {
//
// If ktime is jiffies based, a stalled timer would prevent
// jiffies from being incremented and the filter would look
// at a stale timestamp and never trigger.
//
    if (__this_cpu_inc_return(nmi_rearmed) < 10)
    return false;
    }
    __this_cpu_write(nmi_rearmed, 0);
    __this_cpu_write(last_timestamp, now);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn watchdog_init_timestamp() {
    static void watchdog_init_timestamp(void)
    {
    __this_cpu_write(nmi_rearmed, 0);
    __this_cpu_write(last_timestamp, ktime_get_mono_fast_ns());
    }

    static inline bool watchdog_check_timestamp(void) { return true; }
    static inline void watchdog_init_timestamp(void) { }

    static struct perf_event_attr wd_hw_attr = {
    .type		= PERF_TYPE_HARDWARE,
    .config		= PERF_COUNT_HW_CPU_CYCLES,
    .size		= sizeof(struct perf_event_attr),
    .pinned		= 1,
    .disabled	= 1,
    };
    static struct perf_event_attr fallback_wd_hw_attr = {
    .type		= PERF_TYPE_HARDWARE,
    .config		= PERF_COUNT_HW_CPU_CYCLES,
    .size		= sizeof(struct perf_event_attr),
    .pinned		= 1,
    .disabled	= 1,
    };
// Callback function for perf event subsystem
    static void watchdog_overflow_callback(struct perf_event *event,
    struct perf_sample_data *data,
    struct pt_regs *regs)
    {
// Ensure the watchdog never gets throttled
    event.hw.interrupts = 0;
    if (panic_in_progress())
    return;
    if (!watchdog_check_timestamp())
    return;
    watchdog_hardlockup_check(smp_processor_id(), regs);
    }
    static struct perf_event *hardlockup_detector_event_create(unsigned int cpu)
    {
    struct perf_event_attr *wd_attr;
    struct perf_event *evt;
    wd_attr = &wd_hw_attr;
    wd_attr.sample_period = hw_nmi_get_sample_period(watchdog_thresh);
// Try to register using hardware perf events
    evt = perf_event_create_kernel_counter(wd_attr, cpu, core::ptr::null_mut(),
    watchdog_overflow_callback, core::ptr::null_mut());
    if (IS_ERR(evt)) {
    wd_attr = &fallback_wd_hw_attr;
    wd_attr.sample_period = hw_nmi_get_sample_period(watchdog_thresh);
    evt = perf_event_create_kernel_counter(wd_attr, cpu, core::ptr::null_mut(),
    watchdog_overflow_callback, core::ptr::null_mut());
    }
    return evt;
    }
//
// watchdog_hardlockup_enable - Enable the local event
// @cpu: The CPU to enable hard lockup on.
//
#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_enable(cpu: c_uint) {
    void watchdog_hardlockup_enable(unsigned int cpu)
    {
    struct perf_event *evt;
    WARN_ON_ONCE(cpu != smp_processor_id());
    evt = hardlockup_detector_event_create(cpu);
    if (IS_ERR(evt)) {
    pr_debug("Perf event create on CPU %d failed with %ld\n", cpu,
    PTR_ERR(evt));
    return;
    }
// use original value for check
    if (!atomic_fetch_inc(&watchdog_cpus))
    pr_info("Enabled. Permanently consumes one hw-PMU counter.\n");
    WARN_ONCE(this_cpu_read(watchdog_ev), "unexpected watchdog_ev leak");
    this_cpu_write(watchdog_ev, evt);
    watchdog_init_timestamp();
    perf_event_enable(evt);
    }
//
// watchdog_hardlockup_disable - Disable the local event
// @cpu: The CPU to enable hard lockup on.
//
#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_disable(cpu: c_uint) {
    void watchdog_hardlockup_disable(unsigned int cpu)
    {
    struct perf_event *event = this_cpu_read(watchdog_ev);
    WARN_ON_ONCE(cpu != smp_processor_id());
    if (event) {
    perf_event_disable(event);
    perf_event_release_kernel(event);
    this_cpu_write(watchdog_ev, core::ptr::null_mut());
    atomic_dec(&watchdog_cpus);
    }
    }
//
// hardlockup_detector_perf_adjust_period - Adjust the event period due
// to current cpu frequency change
// @period: The target period to be set
//
#[no_mangle]
pub unsafe extern "C" fn hardlockup_detector_perf_adjust_period(period: u64) {
    void hardlockup_detector_perf_adjust_period(u64 period)
    {
    struct perf_event *event = this_cpu_read(watchdog_ev);
    if (!(watchdog_enabled & WATCHDOG_HARDLOCKUP_ENABLED))
    return;
    if (!event)
    return;
    if (event.attr.sample_period == period)
    return;
    if (perf_event_period(event, period))
    pr_err("failed to change period to %llu\n", period);
    }
//
// hardlockup_detector_perf_stop - Globally stop watchdog events
//
// Special interface for x86 to handle the perf HT bug.
//
#[no_mangle]
pub unsafe extern "C" fn hardlockup_detector_perf_stop() -> void __init {
    void __init hardlockup_detector_perf_stop(void)
    {
    int cpu;
    lockdep_assert_cpus_held();
    for_each_online_cpu(cpu) {
    struct perf_event *event = per_cpu(watchdog_ev, cpu);
    if (event)
    perf_event_disable(event);
    }
    }
//
// hardlockup_detector_perf_restart - Globally restart watchdog events
//
// Special interface for x86 to handle the perf HT bug.
//
#[no_mangle]
pub unsafe extern "C" fn hardlockup_detector_perf_restart() -> void __init {
    void __init hardlockup_detector_perf_restart(void)
    {
    int cpu;
    lockdep_assert_cpus_held();
    if (!(watchdog_enabled & WATCHDOG_HARDLOCKUP_ENABLED))
    return;
    for_each_online_cpu(cpu) {
    struct perf_event *event = per_cpu(watchdog_ev, cpu);
    if (event)
    perf_event_enable(event);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_perf_nmi_is_available() -> bool __weak __init {
    bool __weak __init arch_perf_nmi_is_available(void)
    {
    return true;
    }
//
// watchdog_hardlockup_probe - Probe whether NMI event is available at all
//
#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_probe() -> int __init {
    int __init watchdog_hardlockup_probe(void)
    {
    struct perf_event *evt;
    unsigned int cpu;
    int ret;
    if (!arch_perf_nmi_is_available())
    return -ENODEV;
    if (!hw_nmi_get_sample_period(watchdog_thresh))
    return -EINVAL;
//
// Test hardware PMU availability by creating a temporary perf event.
// The event is released immediately.
//
    cpu = raw_smp_processor_id();
    evt = hardlockup_detector_event_create(cpu);
    if (IS_ERR(evt)) {
    pr_info("Perf NMI watchdog permanently disabled\n");
    ret = PTR_ERR(evt);
    } else {
    perf_event_release_kernel(evt);
    ret = 0;
    }
    return ret;
    }
//
// hardlockup_config_perf_event - Overwrite config of wd_hw_attr.
// @str: number which identifies the raw perf event to use
//
#[no_mangle]
pub unsafe extern "C" fn hardlockup_config_perf_event(str: *const c_char) -> void __init {
    void __init hardlockup_config_perf_event(const char *str)
    {
    u64 config;
    char buf[24];
    char *comma = strchr(str, ',');
    if (!comma) {
    if (kstrtoull(str, 16, &config))
    return;
    } else {
    let mut len: c_uint = comma - str;
    if (len > sizeof(buf))
    return;
    strscpy(buf, str, len);
    if (kstrtoull(buf, 16, &config))
    return;
    }
    wd_hw_attr.type = PERF_TYPE_RAW;
    wd_hw_attr.config = config;
    }
