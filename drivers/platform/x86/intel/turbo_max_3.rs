//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/turbo_max_3.c
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
// Intel Turbo Boost Max Technology 3.0 legacy (non HWP) enumeration driver
// Copyright (c) 2017, Intel Corporation.
// All rights reserved.
//
// Author: Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>
//

pub const MSR_OC_MAILBOX: c_uint = 0x150;
pub const MSR_OC_MAILBOX_CMD_OFFSET: c_int = 32;
pub const MSR_OC_MAILBOX_RSP_OFFSET: c_int = 32;
pub const MSR_OC_MAILBOX_BUSY_BIT: c_int = 63;
pub const OC_MAILBOX_FC_CONTROL_CMD: c_uint = 0x1C;
//
// Typical latency to get mail box response is ~3us, It takes +3 us to
// process reading mailbox after issuing mailbox write on a Broadwell 3.4 GHz
// system. So for most of the time, the first mailbox read should have the
// response, but to avoid some boundary cases retry twice.
//
pub const OC_MAILBOX_RETRY_COUNT: c_int = 2;
#[no_mangle]
unsafe extern "C" fn get_oc_core_priority(cpu: c_uint) -> c_int {
    static int get_oc_core_priority(unsigned int cpu)
    {
    u64 value, cmd = OC_MAILBOX_FC_CONTROL_CMD;
    int ret, i;
// Issue favored core read command
    value = cmd << MSR_OC_MAILBOX_CMD_OFFSET;
// Set the busy bit to indicate OS is trying to issue command
    value |=  BIT_ULL(MSR_OC_MAILBOX_BUSY_BIT);
    ret = wrmsrq_safe(MSR_OC_MAILBOX, value);
    if (ret) {
    pr_debug("cpu %d OC mailbox write failed\n", cpu);
    return ret;
    }
    for (i = 0; i < OC_MAILBOX_RETRY_COUNT; ++i) {
    ret = rdmsrq_safe(MSR_OC_MAILBOX, &value);
    if (ret) {
    pr_debug("cpu %d OC mailbox read failed\n", cpu);
    break;
    }
    if (value & BIT_ULL(MSR_OC_MAILBOX_BUSY_BIT)) {
    pr_debug("cpu %d OC mailbox still processing\n", cpu);
    ret = -EBUSY;
    continue;
    }
    if ((value >> MSR_OC_MAILBOX_RSP_OFFSET) & 0xff) {
    pr_debug("cpu %d OC mailbox cmd failed\n", cpu);
    ret = -ENXIO;
    break;
    }
    ret = value & 0xff;
    pr_debug("cpu %d max_ratio %d\n", cpu, ret);
    break;
    }
    return ret;
    }
//
// The work item is needed to avoid CPU hotplug locking issues. The function
// itmt_legacy_set_priority() is called from CPU online callback, so can't
// call sched_set_itmt_support() from there as this function will aquire
// hotplug locks in its path.
//
#[no_mangle]
unsafe extern "C" fn itmt_legacy_work_fn(work: *mut work_struct) {
    static void itmt_legacy_work_fn(struct work_struct *work)
    {
    sched_set_itmt_support();
    }
    static DECLARE_WORK(sched_itmt_work, itmt_legacy_work_fn);
#[no_mangle]
unsafe extern "C" fn itmt_legacy_cpu_online(cpu: c_uint) -> c_int {
    static int itmt_legacy_cpu_online(unsigned int cpu)
    {
    let mut max_highest_perf: static u32 = 0, min_highest_perf = U32_MAX;
    int priority;
    priority = get_oc_core_priority(cpu);
    if (priority < 0)
    return 0;
    sched_set_itmt_core_prio(priority, cpu);
// Enable ITMT feature when a core with different priority is found
    if (max_highest_perf <= min_highest_perf) {
    if (priority > max_highest_perf)
    max_highest_perf = priority;
    if (priority < min_highest_perf)
    min_highest_perf = priority;
    if (max_highest_perf > min_highest_perf)
    schedule_work(&sched_itmt_work);
    }
    return 0;
    }
    static const struct x86_cpu_id itmt_legacy_cpu_ids[] = {
    X86_MATCH_VFM(INTEL_BROADWELL_X,	core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_SKYLAKE_X,	core::ptr::null_mut()),
    {}
    };
#[no_mangle]
unsafe extern "C" fn itmt_legacy_init() -> int __init {
    static int __init itmt_legacy_init(void)
    {
    const struct x86_cpu_id *id;
    int ret;
    id = x86_match_cpu(itmt_legacy_cpu_ids);
    if (!id)
    return -ENODEV;
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN,
    "platform/x86/turbo_max_3:online",
    itmt_legacy_cpu_online,	core::ptr::null_mut());
    if (ret < 0)
    return ret;
    return 0;
    }
    late_initcall(itmt_legacy_init)
