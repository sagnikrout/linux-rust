//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/dbell.c
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
// Author: Kumar Gala <galak@kernel.crashing.org>
//
// Copyright 2009 Freescale Semiconductor Inc.
//

    DEFINE_INTERRUPT_HANDLER_ASYNC(doorbell_exception)
    {
    struct pt_regs *old_regs = set_irq_regs(regs);
    trace_doorbell_entry(regs);
    ppc_msgsync();
    if (should_hard_irq_enable(regs))
    do_hard_irq_enable();
    kvmppc_clear_host_ipi(smp_processor_id());
    __this_cpu_inc(irq_stat.doorbell_irqs);
    smp_ipi_demux_relaxed(); /* already performed the barrier */
    trace_doorbell_exit(regs);
    set_irq_regs(old_regs);
    }

    DEFINE_INTERRUPT_HANDLER_ASYNC(doorbell_exception)
    {
    printk(KERN_WARNING "Received doorbell on non-smp system\n");
    }
