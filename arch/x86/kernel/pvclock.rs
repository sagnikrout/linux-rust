//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/pvclock.c
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
// paravirtual clock -- common code used by kvm/xen
//

    let mut __read_mostly: static u8 valid_flags = 0;
    static struct pvclock_vsyscall_time_info *pvti_cpu0_va __read_mostly;
#[no_mangle]
pub unsafe extern "C" fn pvclock_set_flags(flags: u8) {
    void pvclock_set_flags(u8 flags)
    {
    valid_flags = flags;
    }
#[no_mangle]
pub unsafe extern "C" fn pvclock_tsc_khz(src: *mut pvclock_vcpu_time_info) -> c_ulong {
    unsigned long pvclock_tsc_khz(struct pvclock_vcpu_time_info *src)
    {
    let mut pv_tsc_khz: u64 = 1000000ULL << 32;
    do_div(pv_tsc_khz, src.tsc_to_system_mul);
    if (src.tsc_shift < 0)
    pv_tsc_khz <<= -src.tsc_shift;
    else
    pv_tsc_khz >>= src.tsc_shift;
    return pv_tsc_khz;
    }
#[no_mangle]
pub unsafe extern "C" fn pvclock_touch_watchdogs() {
    void pvclock_touch_watchdogs(void)
    {
    touch_softlockup_watchdog_sync();
    clocksource_touch_watchdog();
    rcu_cpu_stall_reset();
    reset_hung_task_detector();
    }
    let mut last_value: static atomic64_t = ATOMIC64_INIT(0);
#[no_mangle]
pub unsafe extern "C" fn pvclock_resume() {
    void pvclock_resume(void)
    {
    atomic64_set(&last_value, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn pvclock_read_flags(src: *mut pvclock_vcpu_time_info) -> u8 {
    u8 pvclock_read_flags(struct pvclock_vcpu_time_info *src)
    {
    unsigned version;
    u8 flags;
    do {
    version = pvclock_read_begin(src);
    flags = src.flags;
    } while (pvclock_read_retry(src, version));
    return flags & valid_flags;
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn __pvclock_clocksource_read(src: *mut pvclock_vcpu_time_info, dowd: bool) -> u64 {
    u64 __pvclock_clocksource_read(struct pvclock_vcpu_time_info *src, bool dowd)
    {
    unsigned version;
    u64 ret;
    u64 last;
    u8 flags;
    do {
    version = pvclock_read_begin(src);
    ret = __pvclock_read_cycles(src, rdtsc_ordered());
    flags = src.flags;
    } while (pvclock_read_retry(src, version));
    if (dowd && unlikely((flags & PVCLOCK_GUEST_STOPPED) != 0)) {
    src.flags &= ~PVCLOCK_GUEST_STOPPED;
    pvclock_touch_watchdogs();
    }
    if ((valid_flags & PVCLOCK_TSC_STABLE_BIT) &&
    (flags & PVCLOCK_TSC_STABLE_BIT))
    return ret;
//
// Assumption here is that last_value, a global accumulator, always goes
// forward. If we are less than that, we should not be much smaller.
// We assume there is an error margin we're inside, and then the correction
// does not sacrifice accuracy.
//
// For reads: global may have changed between test and return,
// but this means someone else updated poked the clock at a later time.
// We just need to make sure we are not seeing a backwards event.
//
// For updates: last_value = ret is not enough, since two vcpus could be
// updating at the same time, and one of them could be slightly behind,
// making the assumption that last_value always go forward fail to hold.
//
    last = raw_atomic64_read(&last_value);
    do {
    if (ret <= last)
    return last;
    } while (!raw_atomic64_try_cmpxchg(&last_value, &last, ret));
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn pvclock_clocksource_read(src: *mut pvclock_vcpu_time_info) -> u64 {
    u64 pvclock_clocksource_read(struct pvclock_vcpu_time_info *src)
    {
    return __pvclock_clocksource_read(src, true);
    }
#[no_mangle]
pub unsafe extern "C" fn pvclock_clocksource_read_nowd(src: *mut pvclock_vcpu_time_info) -> noinstr u64 {
    noinstr u64 pvclock_clocksource_read_nowd(struct pvclock_vcpu_time_info *src)
    {
    return __pvclock_clocksource_read(src, false);
    }
    void pvclock_read_wallclock(struct pvclock_wall_clock *wall_clock,
    struct pvclock_vcpu_time_info *vcpu_time,
    struct timespec64 *ts)
    {
    u32 version;
    u64 delta;
    struct timespec64 now;
// get wallclock at system boot
    do {
    version = wall_clock.version;
    rmb();		/* fetch version before time */
//
// Note: wall_clock->sec is a u32 value, so it can
// only store dates between 1970 and 2106. To allow
// times beyond that, we need to create a new hypercall
// interface with an extended pvclock_wall_clock structure
// like ARM has.
//
    now.tv_sec  = wall_clock.sec;
    now.tv_nsec = wall_clock.nsec;
    rmb();		/* fetch time before checking version */
    } while ((wall_clock.version & 1) || (version != wall_clock.version));
    delta = pvclock_clocksource_read(vcpu_time);	/* time since system boot */
    delta += now.tv_sec * NSEC_PER_SEC + now.tv_nsec;
    now.tv_nsec = do_div(delta, NSEC_PER_SEC);
    now.tv_sec = delta;
    set_normalized_timespec64(ts, now.tv_sec, now.tv_nsec);
    }
#[no_mangle]
pub unsafe extern "C" fn pvclock_set_pvti_cpu0_va(pvti: *mut pvclock_vsyscall_time_info) {
    void pvclock_set_pvti_cpu0_va(struct pvclock_vsyscall_time_info *pvti)
    {
    WARN_ON(vclock_was_used(VDSO_CLOCKMODE_PVCLOCK));
    pvti_cpu0_va = pvti;
    }
    struct pvclock_vsyscall_time_info *pvclock_get_pvti_cpu0_va(void)
    {
    return pvti_cpu0_va;
    }
    EXPORT_SYMBOL_GPL(pvclock_get_pvti_cpu0_va);
