//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/irq_work.c
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
// x86 specific code for irq_work
//
// Copyright (C) 2010 Red Hat, Inc., Peter Zijlstra
//

    DEFINE_IDTENTRY_SYSVEC(sysvec_irq_work)
    {
    apic_eoi();
    trace_irq_work_entry(IRQ_WORK_VECTOR);
    inc_irq_stat(IRQ_WORK);
    irq_work_run();
    trace_irq_work_exit(IRQ_WORK_VECTOR);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_irq_work_raise() {
    void arch_irq_work_raise(void)
    {
    if (!arch_irq_work_has_interrupt())
    return;
    __apic_send_IPI_self(IRQ_WORK_VECTOR);
    apic_wait_icr_idle();
    }
