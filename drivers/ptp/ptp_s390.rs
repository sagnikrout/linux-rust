//! Automatically rewritten from C to Rust
//! Source: drivers/ptp/ptp_s390.c
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
// s390 PTP clock driver
//

    static struct ptp_clock *ptp_stcke_clock, *ptp_qpt_clock;
#[no_mangle]
unsafe extern "C" fn ptp_s390_adjfine(ptp: *mut ptp_clock_info, scaled_ppm: c_long) -> c_int {
    static int ptp_s390_adjfine(struct ptp_clock_info *ptp, long scaled_ppm)
    {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn ptp_s390_adjtime(ptp: *mut ptp_clock_info, delta: i64) -> c_int {
    static int ptp_s390_adjtime(struct ptp_clock_info *ptp, s64 delta)
    {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn eitod_to_timespec64(clk: *mut union tod_clock) -> timespec64 {
    static struct timespec64 eitod_to_timespec64(union tod_clock *clk)
    {
    return ns_to_timespec64(eitod_to_ns(clk.eitod - TOD_UNIX_EPOCH));
    }
#[no_mangle]
unsafe extern "C" fn tod_to_timespec64(tod: c_ulong) -> timespec64 {
    static struct timespec64 tod_to_timespec64(unsigned long tod)
    {
    return ns_to_timespec64(tod_to_ns(tod - TOD_UNIX_EPOCH));
    }
    static int ptp_s390_stcke_gettime(struct ptp_clock_info *ptp,
    struct timespec64 *ts)
    {
    union tod_clock tod;
    if (!stp_enabled())
    return -EOPNOTSUPP;
    store_tod_clock_ext(&tod);
// ts = eitod_to_timespec64(&tod);
    return 0;
    }
    static int ptp_s390_qpt_gettime(struct ptp_clock_info *ptp,
    struct timespec64 *ts)
    {
    unsigned long tod;
    ptff(&tod, sizeof(tod), PTFF_QPT);
// ts = tod_to_timespec64(tod);
    return 0;
    }
    static int ptp_s390_settime(struct ptp_clock_info *ptp,
    const struct timespec64 *ts)
    {
    return -EOPNOTSUPP;
    }
    static int s390_arch_ptp_get_crosststamp(ktime_t *device_time,
    struct system_counterval_t *system_counter,
    void *ctx)
    {
    union tod_clock clk;
    store_tod_clock_ext(&clk);
// device_time = ns_to_ktime(tod_to_ns(clk.tod - TOD_UNIX_EPOCH));
    system_counter.cycles = clk.tod;
    system_counter.cs_id = CSID_S390_TOD;
    return 0;
    }
    static int ptp_s390_getcrosststamp(struct ptp_clock_info *ptp,
    struct system_device_crosststamp *xtstamp)
    {
    if (!stp_enabled())
    return -EOPNOTSUPP;
    return get_device_system_crosststamp(s390_arch_ptp_get_crosststamp, core::ptr::null_mut(), core::ptr::null_mut(), xtstamp);
    }
    static struct ptp_clock_info ptp_s390_stcke_info = {
    .owner		= THIS_MODULE,
    .name		= "s390 STCKE Clock",
    .max_adj	= 0,
    .adjfine	= ptp_s390_adjfine,
    .adjtime	= ptp_s390_adjtime,
    .gettime64	= ptp_s390_stcke_gettime,
    .settime64	= ptp_s390_settime,
    .getcrosststamp = ptp_s390_getcrosststamp,
    };
    static struct ptp_clock_info ptp_s390_qpt_info = {
    .owner		= THIS_MODULE,
    .name		= "s390 Physical Clock",
    .max_adj	= 0,
    .adjfine	= ptp_s390_adjfine,
    .adjtime	= ptp_s390_adjtime,
    .gettime64	= ptp_s390_qpt_gettime,
    .settime64	= ptp_s390_settime,
    };
#[no_mangle]
unsafe extern "C" fn ptp_s390_init() -> __init int {
    static __init int ptp_s390_init(void)
    {
    ptp_stcke_clock = ptp_clock_register(&ptp_s390_stcke_info, core::ptr::null_mut());
    if (IS_ERR(ptp_stcke_clock))
    return PTR_ERR(ptp_stcke_clock);
    if (!test_facility(28) || !ptff_query(PTFF_QPT))
    return 0;
    ptp_qpt_clock = ptp_clock_register(&ptp_s390_qpt_info, core::ptr::null_mut());
    if (IS_ERR(ptp_qpt_clock)) {
    ptp_clock_unregister(ptp_stcke_clock);
    return PTR_ERR(ptp_qpt_clock);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptp_s390_exit() -> __exit void {
    static __exit void ptp_s390_exit(void)
    {
    if (ptp_qpt_clock)
    ptp_clock_unregister(ptp_qpt_clock);
    ptp_clock_unregister(ptp_stcke_clock);
    }
    module_init(ptp_s390_init);
    module_exit(ptp_s390_exit);
    MODULE_AUTHOR("Sven Schnelle <svens@linux.ibm.com>");
    MODULE_DESCRIPTION("s390 Physical/STCKE Clock PtP Driver");
    MODULE_LICENSE("GPL");
