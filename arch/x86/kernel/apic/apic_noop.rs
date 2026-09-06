//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/apic/apic_noop.c
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
// NOOP APIC driver.
//
// Does almost nothing and should be substituted by a real apic driver via
// probe routine.
//
// Though in case if apic is disabled (for some reason) we try
// to not uglify the caller's code and allow to call (some) apic routines
// like self-ipi, etc...
//
// FIXME: Remove this gunk. The above argument which was intentionally left
// in place is silly to begin with because none of the callbacks except for
// APIC::read/write() have a WARN_ON_ONCE() in them. Sigh...
//

    static void noop_send_IPI(int cpu, int vector) { }
    static void noop_send_IPI_mask(const struct cpumask *cpumask, int vector) { }
    static void noop_send_IPI_mask_allbutself(const struct cpumask *cpumask, int vector) { }
    static void noop_send_IPI_allbutself(int vector) { }
    static void noop_send_IPI_all(int vector) { }
    static void noop_send_IPI_self(int vector) { }
    static void noop_apic_icr_write(u32 low, u32 id) { }
    static int noop_wakeup_secondary_cpu(u32 apicid, unsigned long start_eip,
    unsigned int cpu)
    {
    return -1;
    }
    static u64 noop_apic_icr_read(void) { return 0; }
    static u32 noop_get_apic_id(u32 apicid) { return 0; }
    static void noop_apic_eoi(void) { }
#[no_mangle]
unsafe extern "C" fn noop_apic_read(reg: u32) -> u32 {
    static u32 noop_apic_read(u32 reg)
    {
    WARN_ON_ONCE(boot_cpu_has(X86_FEATURE_APIC) && !apic_is_disabled);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn noop_apic_write(reg: u32, val: u32) {
    static void noop_apic_write(u32 reg, u32 val)
    {
    WARN_ON_ONCE(boot_cpu_has(X86_FEATURE_APIC) && !apic_is_disabled);
    }
    struct apic apic_noop __ro_after_init = {
    .name				= "noop",
    .dest_mode_logical		= true,
    .disable_esr			= 0,
    .cpu_present_to_apicid		= default_cpu_present_to_apicid,
    .max_apic_id			= 0xFE,
    .get_apic_id			= noop_get_apic_id,
    .calc_dest_apicid		= apic_flat_calc_apicid,
    .send_IPI			= noop_send_IPI,
    .send_IPI_mask			= noop_send_IPI_mask,
    .send_IPI_mask_allbutself	= noop_send_IPI_mask_allbutself,
    .send_IPI_allbutself		= noop_send_IPI_allbutself,
    .send_IPI_all			= noop_send_IPI_all,
    .send_IPI_self			= noop_send_IPI_self,
    .wakeup_secondary_cpu		= noop_wakeup_secondary_cpu,
    .read				= noop_apic_read,
    .write				= noop_apic_write,
    .eoi				= noop_apic_eoi,
    .icr_read			= noop_apic_icr_read,
    .icr_write			= noop_apic_icr_write,
    };
