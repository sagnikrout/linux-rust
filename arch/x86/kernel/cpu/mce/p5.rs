//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/mce/p5.c
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
// P5 specific Machine Check Exception Reporting
// (C) Copyright 2002 Alan Cox <alan@lxorguk.ukuu.org.uk>
//

// By default disabled
    int mce_p5_enabled __read_mostly;
// Machine check handler for Pentium class Intel CPUs:
#[no_mangle]
pub unsafe extern "C" fn pentium_machine_check(regs: *mut pt_regs) -> noinstr void {
    noinstr void pentium_machine_check(struct pt_regs *regs)
    {
    u64 addr, type;
    instrumentation_begin();
    rdmsrq(MSR_IA32_P5_MC_ADDR, addr);
    rdmsrq(MSR_IA32_P5_MC_TYPE, type);
    pr_emerg("CPU#%d: Machine Check Exception:  0x%8X (type 0x%8X).\n",
    smp_processor_id(), (u32)addr, (u32)type);
    if (type & (1<<5)) {
    pr_emerg("CPU#%d: Possible thermal failure (CPU on fire ?).\n",
    smp_processor_id());
    }
    add_taint(TAINT_MACHINE_CHECK, LOCKDEP_NOW_UNRELIABLE);
    instrumentation_end();
    }
// Set up machine check reporting for processors with Intel style MCE:
#[no_mangle]
pub unsafe extern "C" fn intel_p5_mcheck_init(c: *mut cpuinfo_x86) {
    void intel_p5_mcheck_init(struct cpuinfo_x86 *c)
    {
    u64 __maybe_unused q;
// Default P5 to off as its often misconnected:
    if (!mce_p5_enabled)
    return;
// Check for MCE support:
    if (!cpu_has(c, X86_FEATURE_MCE))
    return;
// Read registers before enabling:
    rdmsrq(MSR_IA32_P5_MC_ADDR, q);
    rdmsrq(MSR_IA32_P5_MC_TYPE, q);
    pr_info("Intel old style machine check architecture supported.\n");
// Enable MCE:
    cr4_set_bits(X86_CR4_MCE);
    pr_info("Intel old style machine check reporting enabled on CPU#%d.\n",
    smp_processor_id());
    }
