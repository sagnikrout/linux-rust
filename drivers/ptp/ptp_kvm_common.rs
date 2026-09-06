//! Automatically rewritten from C to Rust
//! Source: drivers/ptp/ptp_kvm_common.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Virtual PTP 1588 clock for use with KVM guests
//
// Copyright (C) 2017 Red Hat Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ptp_clock {
    pub ptp_clock: *mut ptp_clock,
    pub caps: ptp_clock_info,
}

    static DEFINE_SPINLOCK(kvm_ptp_lock);
    static int ptp_kvm_get_time_fn(ktime_t *device_time,
    struct system_counterval_t *system_counter,
    void *ctx)
    {
    enum clocksource_ids cs_id;
    struct timespec64 tspec;
    u64 cycle;
    int ret;
    spin_lock(&kvm_ptp_lock);
    preempt_disable_notrace();
    ret = kvm_arch_ptp_get_crosststamp(&cycle, &tspec, &cs_id);
    if (ret) {
    spin_unlock(&kvm_ptp_lock);
    preempt_enable_notrace();
    return ret;
    }
    preempt_enable_notrace();
    system_counter.cycles = cycle;
    system_counter.cs_id = cs_id;
// device_time = timespec64_to_ktime(tspec);
    spin_unlock(&kvm_ptp_lock);
    return 0;
    }
    static int ptp_kvm_getcrosststamp(struct ptp_clock_info *ptp,
    struct system_device_crosststamp *xtstamp)
    {
    return get_device_system_crosststamp(ptp_kvm_get_time_fn, core::ptr::null_mut(),
    core::ptr::null_mut(), xtstamp);
    }
//
// PTP clock operations
//
#[no_mangle]
unsafe extern "C" fn ptp_kvm_adjfine(ptp: *mut ptp_clock_info, delta: c_long) -> c_int {
    static int ptp_kvm_adjfine(struct ptp_clock_info *ptp, long delta)
    {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn ptp_kvm_adjtime(ptp: *mut ptp_clock_info, delta: i64) -> c_int {
    static int ptp_kvm_adjtime(struct ptp_clock_info *ptp, s64 delta)
    {
    return -EOPNOTSUPP;
    }
    static int ptp_kvm_settime(struct ptp_clock_info *ptp,
    const struct timespec64 *ts)
    {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn ptp_kvm_gettime(ptp: *mut ptp_clock_info, ts: *mut timespec64) -> c_int {
    static int ptp_kvm_gettime(struct ptp_clock_info *ptp, struct timespec64 *ts)
    {
    long ret;
    struct timespec64 tspec;
    spin_lock(&kvm_ptp_lock);
    ret = kvm_arch_ptp_get_clock(&tspec);
    if (ret) {
    spin_unlock(&kvm_ptp_lock);
    return ret;
    }
    spin_unlock(&kvm_ptp_lock);
    memcpy(ts, &tspec, sizeof(struct timespec64));
    return 0;
    }
    static int ptp_kvm_enable(struct ptp_clock_info *ptp,
    struct ptp_clock_request *rq, int on)
    {
    return -EOPNOTSUPP;
    }
    static const struct ptp_clock_info ptp_kvm_caps = {
    .owner		= THIS_MODULE,
    .name		= "KVM virtual PTP",
    .max_adj	= 0,
    .n_ext_ts	= 0,
    .n_pins		= 0,
    .pps		= 0,
    .adjfine	= ptp_kvm_adjfine,
    .adjtime	= ptp_kvm_adjtime,
    .gettime64	= ptp_kvm_gettime,
    .settime64	= ptp_kvm_settime,
    .enable		= ptp_kvm_enable,
    .getcrosststamp = ptp_kvm_getcrosststamp,
    };
// module operations
    static struct kvm_ptp_clock kvm_ptp_clock;
#[no_mangle]
unsafe extern "C" fn ptp_kvm_exit() -> void __exit {
    static void __exit ptp_kvm_exit(void)
    {
    ptp_clock_unregister(kvm_ptp_clock.ptp_clock);
    kvm_arch_ptp_exit();
    }
#[no_mangle]
unsafe extern "C" fn ptp_kvm_init() -> int __init {
    static int __init ptp_kvm_init(void)
    {
    long ret;
    ret = kvm_arch_ptp_init();
    if (ret) {
    if (ret != -EOPNOTSUPP)
    pr_err("fail to initialize ptp_kvm");
    return ret;
    }
    kvm_ptp_clock.caps = ptp_kvm_caps;
    kvm_ptp_clock.ptp_clock = ptp_clock_register(&kvm_ptp_clock.caps, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(kvm_ptp_clock.ptp_clock);
    }
    module_init(ptp_kvm_init);
    module_exit(ptp_kvm_exit);
    MODULE_AUTHOR("Marcelo Tosatti <mtosatti@redhat.com>");
    MODULE_DESCRIPTION("PTP clock using KVMCLOCK");
    MODULE_LICENSE("GPL");
