//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/hardirq.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irq_stat_counts {
    IRQ_COUNT_NMI,

    IRQ_COUNT_APIC_TIMER,
    IRQ_COUNT_SPURIOUS,
    IRQ_COUNT_APIC_PERF,
    IRQ_COUNT_IRQ_WORK,
    IRQ_COUNT_ICR_READ_RETRY,
    IRQ_COUNT_X86_PLATFORM_IPI,

    IRQ_COUNT_RESCHEDULE,
    IRQ_COUNT_CALL_FUNCTION,

    IRQ_COUNT_TLB,

    IRQ_COUNT_THERMAL_APIC,

    IRQ_COUNT_THRESHOLD_APIC,

    IRQ_COUNT_DEFERRED_ERROR,

    IRQ_COUNT_MCE_EXCEPTION,
    IRQ_COUNT_MCE_POLL,

    IRQ_COUNT_HYPERVISOR_CALLBACK,

    IRQ_COUNT_HYPERV_REENLIGHTENMENT,
    IRQ_COUNT_HYPERV_STIMER0,

    IRQ_COUNT_POSTED_INTR,
    IRQ_COUNT_POSTED_INTR_NESTED,
    IRQ_COUNT_POSTED_INTR_WAKEUP,

    IRQ_COUNT_PERF_GUEST_MEDIATED_PMI,

    IRQ_COUNT_POSTED_MSI_NOTIFICATION,

    IRQ_COUNT_PIC_APIC_ERROR,

    IRQ_COUNT_IOAPIC_MISROUTED,

    IRQ_COUNT_MAX,
}

extern "C" {
    pub fn irq_stat_inc_and_enable(which: irq_stat_counts);
}

extern "C" {
    pub fn ack_bad_irq(irq: c_uint);
}

extern "C" {
    pub fn arch_irq_stat_cpu(cpu: c_uint) -> u64;
}

//
// This function is called from noinstr interrupt contexts
// and must be inlined to not get instrumentation.
//
extern "C" {
    pub fn __this_cpu_read(_arg: irq_stat.kvm_cpu_l1tf_flush_l1d) -> return;
}

