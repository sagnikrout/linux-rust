//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/mce/winchip.c
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
// IDT Winchip specific Machine Check Exception Reporting
// (C) Copyright 2002 Alan Cox <alan@lxorguk.ukuu.org.uk>
//

// Machine check handler for WinChip C6:
#[no_mangle]
pub unsafe extern "C" fn winchip_machine_check(regs: *mut pt_regs) -> noinstr void {
    noinstr void winchip_machine_check(struct pt_regs *regs)
    {
    instrumentation_begin();
    pr_emerg("CPU0: Machine Check Exception.\n");
    add_taint(TAINT_MACHINE_CHECK, LOCKDEP_NOW_UNRELIABLE);
    instrumentation_end();
    }
// Set up machine check reporting on the Winchip C6 series
#[no_mangle]
pub unsafe extern "C" fn winchip_mcheck_init(c: *mut cpuinfo_x86) {
    void winchip_mcheck_init(struct cpuinfo_x86 *c)
    {
    struct msr val;
    rdmsrq(MSR_IDT_FCR1, val.q);
    val.l |= (1<<2);	/* Enable EIERRINT (int 18 MCE) */
    val.l &= ~(1<<4);	/* Enable MCE */
    wrmsrq(MSR_IDT_FCR1, val.q);
    cr4_set_bits(X86_CR4_MCE);
    pr_info("Winchip machine check reporting enabled on CPU#0.\n");
    }
