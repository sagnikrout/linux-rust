//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/perfctr-watchdog.c
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
// local apic based NMI watchdog for various CPUs.
//
// This file also handles reservation of performance counters for coordination
// with other users.
//
// Note that these events normally don't tick when the CPU idles. This means
// the frequency varies with CPU load.
//
// Original code for K7/P6 written by Keith Owens
//

//
// this number is calculated from Intel's MSR_P4_CRU_ESCR5 register and it's
// offset from MSR_P4_BSU_ESCR0.
//
// It will be the max for all platforms (for now)
//
pub const NMI_MAX_COUNTER_BITS: c_int = 66;
//
// perfctr_nmi_owner tracks the ownership of the perfctr registers:
// evtsel_nmi_owner tracks the ownership of the event selection
// - different performance counters/ event selection may be reserved for
// different subsystems this reservation system just tries to coordinate
// things a little
//
    static DECLARE_BITMAP(perfctr_nmi_owner, NMI_MAX_COUNTER_BITS);
    static DECLARE_BITMAP(evntsel_nmi_owner, NMI_MAX_COUNTER_BITS);
// converts an msr to an appropriate reservation bit
#[no_mangle]
pub unsafe extern "C" fn nmi_perfctr_msr_to_bit(msr: c_uint) -> c_uint {
    static inline unsigned int nmi_perfctr_msr_to_bit(unsigned int msr)
    {
// returns the bit offset of the performance counter register
    switch (boot_cpu_data.x86_vendor) {
    case X86_VENDOR_HYGON:
    case X86_VENDOR_AMD:
    if (msr >= MSR_F15H_PERF_CTR)
    return (msr - MSR_F15H_PERF_CTR) >> 1;
    return msr - MSR_K7_PERFCTR0;
    case X86_VENDOR_INTEL:
    if (cpu_has(&boot_cpu_data, X86_FEATURE_ARCH_PERFMON))
    return msr - MSR_ARCH_PERFMON_PERFCTR0;
    switch (boot_cpu_data.x86) {
    case 6:
    return msr - MSR_P6_PERFCTR0;
    case 11:
    return msr - MSR_KNC_PERFCTR0;
    case 15:
    return msr - MSR_P4_BPU_PERFCTR0;
    }
    break;
    case X86_VENDOR_ZHAOXIN:
    case X86_VENDOR_CENTAUR:
    return msr - MSR_ARCH_PERFMON_PERFCTR0;
    }
    return 0;
    }
//
// converts an msr to an appropriate reservation bit
// returns the bit offset of the event selection register
//
#[no_mangle]
pub unsafe extern "C" fn nmi_evntsel_msr_to_bit(msr: c_uint) -> c_uint {
    static inline unsigned int nmi_evntsel_msr_to_bit(unsigned int msr)
    {
// returns the bit offset of the event selection register
    switch (boot_cpu_data.x86_vendor) {
    case X86_VENDOR_HYGON:
    case X86_VENDOR_AMD:
    if (msr >= MSR_F15H_PERF_CTL)
    return (msr - MSR_F15H_PERF_CTL) >> 1;
    return msr - MSR_K7_EVNTSEL0;
    case X86_VENDOR_INTEL:
    if (cpu_has(&boot_cpu_data, X86_FEATURE_ARCH_PERFMON))
    return msr - MSR_ARCH_PERFMON_EVENTSEL0;
    switch (boot_cpu_data.x86) {
    case 6:
    return msr - MSR_P6_EVNTSEL0;
    case 11:
    return msr - MSR_KNC_EVNTSEL0;
    case 15:
    return msr - MSR_P4_BSU_ESCR0;
    }
    break;
    case X86_VENDOR_ZHAOXIN:
    case X86_VENDOR_CENTAUR:
    return msr - MSR_ARCH_PERFMON_EVENTSEL0;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn reserve_perfctr_nmi(msr: c_uint) -> c_int {
    int reserve_perfctr_nmi(unsigned int msr)
    {
    unsigned int counter;
    counter = nmi_perfctr_msr_to_bit(msr);
// register not managed by the allocator?
    if (counter > NMI_MAX_COUNTER_BITS)
    return 1;
    if (!test_and_set_bit(counter, perfctr_nmi_owner))
    return 1;
    return 0;
    }
    EXPORT_SYMBOL(reserve_perfctr_nmi);
#[no_mangle]
pub unsafe extern "C" fn release_perfctr_nmi(msr: c_uint) {
    void release_perfctr_nmi(unsigned int msr)
    {
    unsigned int counter;
    counter = nmi_perfctr_msr_to_bit(msr);
// register not managed by the allocator?
    if (counter > NMI_MAX_COUNTER_BITS)
    return;
    clear_bit(counter, perfctr_nmi_owner);
    }
    EXPORT_SYMBOL(release_perfctr_nmi);
#[no_mangle]
pub unsafe extern "C" fn reserve_evntsel_nmi(msr: c_uint) -> c_int {
    int reserve_evntsel_nmi(unsigned int msr)
    {
    unsigned int counter;
    counter = nmi_evntsel_msr_to_bit(msr);
// register not managed by the allocator?
    if (counter > NMI_MAX_COUNTER_BITS)
    return 1;
    if (!test_and_set_bit(counter, evntsel_nmi_owner))
    return 1;
    return 0;
    }
    EXPORT_SYMBOL(reserve_evntsel_nmi);
#[no_mangle]
pub unsafe extern "C" fn release_evntsel_nmi(msr: c_uint) {
    void release_evntsel_nmi(unsigned int msr)
    {
    unsigned int counter;
    counter = nmi_evntsel_msr_to_bit(msr);
// register not managed by the allocator?
    if (counter > NMI_MAX_COUNTER_BITS)
    return;
    clear_bit(counter, evntsel_nmi_owner);
    }
    EXPORT_SYMBOL(release_evntsel_nmi);
