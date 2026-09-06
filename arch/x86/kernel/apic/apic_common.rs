//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/apic/apic_common.c
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


//
// Common functions shared between the various APIC flavours
//
// SPDX-License-Identifier: GPL-2.0
//

#[no_mangle]
pub unsafe extern "C" fn apic_default_calc_apicid(cpu: c_uint) -> u32 {
    u32 apic_default_calc_apicid(unsigned int cpu)
    {
    return per_cpu(x86_cpu_to_apicid, cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn apic_flat_calc_apicid(cpu: c_uint) -> u32 {
    u32 apic_flat_calc_apicid(unsigned int cpu)
    {
    return 1U << cpu;
    }
#[no_mangle]
pub unsafe extern "C" fn default_cpu_present_to_apicid(mps_cpu: c_int) -> u32 {
    u32 default_cpu_present_to_apicid(int mps_cpu)
    {
    if (mps_cpu < nr_cpu_ids && cpu_present(mps_cpu))
    return (int)per_cpu(x86_cpu_to_apicid, mps_cpu);
    else
    return BAD_APICID;
    }
//
// Set up the logical destination ID when the APIC operates in logical
// destination mode.
//
#[no_mangle]
pub unsafe extern "C" fn default_init_apic_ldr() {
    void default_init_apic_ldr(void)
    {
    unsigned long val;
    apic_write(APIC_DFR, APIC_DFR_FLAT);
    val = apic_read(APIC_LDR) & ~APIC_LDR_MASK;
    val |= SET_APIC_LOGICAL_ID(1UL << smp_processor_id());
    apic_write(APIC_LDR, val);
    }
