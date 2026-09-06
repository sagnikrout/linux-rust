//! Automatically rewritten from C to Rust
//! Source: drivers/ptp/ptp_mock.c
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
// Copyright 2023 NXP
//
// Mock-up PTP Hardware Clock driver for virtual network devices
//
// Create a PTP clock which offers PTP time manipulation operations
// using a timecounter/cyclecounter on top of CLOCK_MONOTONIC_RAW.
//

// Clamp scaled_ppm between -2,097,152,000 and 2,097,152,000,
// and thus "adj" between -68,719,476 and 68,719,476
//
pub const MOCK_PHC_MAX_ADJ_PPB: c_int = 32000000;
// Timestamps from ktime_get_raw() have 1 ns resolution, so the scale factor
// (MULT >> SHIFT) needs to be 1. Pick SHIFT as 31 bits, which translates
// MULT(freq 0) into 0x80000000.
//
pub const MOCK_PHC_CC_SHIFT: c_int = 31;

pub const MOCK_PHC_FADJ_SHIFT: c_int = 9;

// The largest cycle_delta that timecounter_read_delta() can handle without a
// 64-bit overflow during the multiplication with cc->mult, given the max "adj"
// we permit, is ~8.3 seconds. Make sure readouts are more frequent than that.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mock_phc {
    pub info: ptp_clock_info,
    pub clock: *mut ptp_clock,
    pub tc: timecounter,
    pub cc: cyclecounter,
    pub lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn mock_phc_cc_read(cc: *mut cyclecounter) -> u64 {
    static u64 mock_phc_cc_read(struct cyclecounter *cc)
    {
    return ktime_get_raw_ns();
    }
#[no_mangle]
unsafe extern "C" fn mock_phc_adjfine(info: *mut ptp_clock_info, scaled_ppm: c_long) -> c_int {
    static int mock_phc_adjfine(struct ptp_clock_info *info, long scaled_ppm)
    {
    struct mock_phc *phc = info_to_phc(info);
    s64 adj;
    adj = (s64)scaled_ppm << MOCK_PHC_FADJ_SHIFT;
    adj = div_s64(adj, MOCK_PHC_FADJ_DENOMINATOR);
    spin_lock(&phc.lock);
    timecounter_read(&phc.tc);
    phc.cc.mult = MOCK_PHC_CC_MULT + adj;
    spin_unlock(&phc.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mock_phc_adjtime(info: *mut ptp_clock_info, delta: i64) -> c_int {
    static int mock_phc_adjtime(struct ptp_clock_info *info, s64 delta)
    {
    struct mock_phc *phc = info_to_phc(info);
    spin_lock(&phc.lock);
    timecounter_adjtime(&phc.tc, delta);
    spin_unlock(&phc.lock);
    return 0;
    }
    static int mock_phc_settime64(struct ptp_clock_info *info,
    const struct timespec64 *ts)
    {
    struct mock_phc *phc = info_to_phc(info);
    let mut ns: u64 = timespec64_to_ns(ts);
    spin_lock(&phc.lock);
    timecounter_init(&phc.tc, &phc.cc, ns);
    spin_unlock(&phc.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mock_phc_gettime64(info: *mut ptp_clock_info, ts: *mut timespec64) -> c_int {
    static int mock_phc_gettime64(struct ptp_clock_info *info, struct timespec64 *ts)
    {
    struct mock_phc *phc = info_to_phc(info);
    u64 ns;
    spin_lock(&phc.lock);
    ns = timecounter_read(&phc.tc);
    spin_unlock(&phc.lock);
// ts = ns_to_timespec64(ns);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mock_phc_refresh(info: *mut ptp_clock_info) -> c_long {
    static long mock_phc_refresh(struct ptp_clock_info *info)
    {
    struct timespec64 ts;
    mock_phc_gettime64(info, &ts);
    return MOCK_PHC_REFRESH_INTERVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn mock_phc_index(phc: *mut mock_phc) -> c_int {
    int mock_phc_index(struct mock_phc *phc)
    {
    return ptp_clock_index(phc.clock);
    }
    EXPORT_SYMBOL_GPL(mock_phc_index);
    struct mock_phc *mock_phc_create(struct device *dev)
    {
    struct mock_phc *phc;
    int err;
    phc = kzalloc_obj(*phc);
    if (!phc) {
    err = -ENOMEM;
    goto out;
    }
    phc.info = (struct ptp_clock_info) {
    .owner		= THIS_MODULE,
    .name		= "Mock-up PTP clock",
    .max_adj	= MOCK_PHC_MAX_ADJ_PPB,
    .adjfine	= mock_phc_adjfine,
    .adjtime	= mock_phc_adjtime,
    .gettime64	= mock_phc_gettime64,
    .settime64	= mock_phc_settime64,
    .do_aux_work	= mock_phc_refresh,
    };
    phc.cc = (struct cyclecounter) {
    .read	= mock_phc_cc_read,
    .mask	= CYCLECOUNTER_MASK(64),
    .mult	= MOCK_PHC_CC_MULT,
    .shift	= MOCK_PHC_CC_SHIFT,
    };
    spin_lock_init(&phc.lock);
    timecounter_init(&phc.tc, &phc.cc, 0);
    phc.clock = ptp_clock_register(&phc.info, dev);
    if (IS_ERR(phc.clock)) {
    err = PTR_ERR(phc.clock);
    goto out_free_phc;
    }
    ptp_schedule_worker(phc.clock, MOCK_PHC_REFRESH_INTERVAL);
    return phc;
    out_free_phc:
    kfree(phc);
    out:
    return ERR_PTR(err);
    }
    EXPORT_SYMBOL_GPL(mock_phc_create);
#[no_mangle]
pub unsafe extern "C" fn mock_phc_destroy(phc: *mut mock_phc) {
    void mock_phc_destroy(struct mock_phc *phc)
    {
    ptp_clock_unregister(phc.clock);
    kfree(phc);
    }
    EXPORT_SYMBOL_GPL(mock_phc_destroy);
    MODULE_DESCRIPTION("Mock-up PTP Hardware Clock driver");
    MODULE_LICENSE("GPL");
