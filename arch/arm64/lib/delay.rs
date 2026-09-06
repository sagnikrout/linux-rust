//! Automatically rewritten from C to Rust
//! Source: arch/arm64/lib/delay.c
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
// Delay loops based on the OpenRISC implementation.
//
// Copyright (C) 2012 ARM Limited
//
// Author: Will Deacon <will.deacon@arm.com>
//

    xloops_to_cycles((time_usecs) * 0x10C7UL)
#[no_mangle]
pub unsafe extern "C" fn xloops_to_cycles(xloops: c_ulong) -> c_ulong {
    static inline unsigned long xloops_to_cycles(unsigned long xloops)
    {
    return (xloops * loops_per_jiffy * HZ) >> 32;
    }
//
// Force the use of CNTVCT_EL0 in order to have the same base as WFxT.
// This avoids some annoying issues when CNTVOFF_EL2 is not reset 0 on a
// KVM host running at EL1 until we do a vcpu_put() on the vcpu. When
// running at EL2, the effective offset is always 0.
//
// Note that userspace cannot change the offset behind our back either,
// as the vcpu mutex is held as long as KVM_RUN is in progress.
//
#[no_mangle]
unsafe extern "C" fn __delay_cycles() -> cycles_t notrace {
    static cycles_t notrace __delay_cycles(void)
    {
    guard(preempt_notrace)();
    return __arch_counter_get_cntvct_stable();
    }
#[no_mangle]
pub unsafe extern "C" fn __delay(cycles: c_ulong) {
    void __delay(unsigned long cycles)
    {
    let mut start: cycles_t = __delay_cycles();
    if (alternative_has_cap_unlikely(ARM64_HAS_WFXT)) {
    let mut end: u64 = start + cycles;
//
// Start with WFIT. If an interrupt makes us resume
// early, use a WFET loop to complete the delay.
//
    wfit(end);
    while ((__delay_cycles() - start) < cycles)
    wfet(end);
    } else 	if (arch_timer_evtstrm_available()) {
    const cycles_t timer_evt_period =
    USECS_TO_CYCLES(ARCH_TIMER_EVT_STREAM_PERIOD_US);
    while ((__delay_cycles() - start + timer_evt_period) < cycles)
    wfe();
    }
    while ((__delay_cycles() - start) < cycles)
    cpu_relax();
    }
    EXPORT_SYMBOL(__delay);
#[no_mangle]
pub unsafe extern "C" fn __const_udelay(xloops: c_ulong) {
    inline void __const_udelay(unsigned long xloops)
    {
    __delay(xloops_to_cycles(xloops));
    }
    EXPORT_SYMBOL(__const_udelay);
#[no_mangle]
pub unsafe extern "C" fn __udelay(usecs: c_ulong) {
    void __udelay(unsigned long usecs)
    {
    __const_udelay(usecs * 0x10C7UL); /* 2**32 / 1000000 (rounded up) */
    }
    EXPORT_SYMBOL(__udelay);
#[no_mangle]
pub unsafe extern "C" fn __ndelay(nsecs: c_ulong) {
    void __ndelay(unsigned long nsecs)
    {
    __const_udelay(nsecs * 0x5UL); /* 2**32 / 1000000000 (rounded up) */
    }
    EXPORT_SYMBOL(__ndelay);
