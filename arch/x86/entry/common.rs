//! Automatically rewritten from C to Rust
//! Source: arch/x86/entry/common.c
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
// On VMX, NMIs and IRQs (as configured by KVM) are acknowledged by hardware as
// part of the VM-Exit, i.e. the event itself is consumed as part the VM-Exit.
// x86_entry_from_kvm() is invoked by KVM to effectively forward NMIs and IRQs
// to the kernel for servicing.  On SVM, a.k.a. AMD, the NMI/IRQ VM-Exit is
// purely a signal that an NMI/IRQ is pending, i.e. the event that triggered
// the VM-Exit is held pending until it's unblocked in the host.
//
#[no_mangle]
pub unsafe extern "C" fn x86_entry_from_kvm(event_type: c_uint, vector: c_uint) -> noinstr void {
    noinstr void x86_entry_from_kvm(unsigned int event_type, unsigned int vector)
    {
    if (event_type == EVENT_TYPE_EXTINT) {

//
// Use FRED dispatch, even when running IDT. The dispatch
// tables are kept in sync between FRED and IDT, and the FRED
// dispatch works well with CFI.
//
    fred_entry_from_kvm(event_type, vector);

    idt_entry_from_kvm(vector);

//
// Strictly speaking, only the NMI path requires noinstr.
//
    instrumentation_begin();
//
// KVM/VMX will dispatch from IRQ-disabled but for a context
// that will have IRQs-enabled. This confuses the entry code
// and it will not have reprogrammed the timer. Do so now.
//
    hrtimer_rearm_deferred();
    instrumentation_end();
    return;
    }
    WARN_ON_ONCE(event_type != EVENT_TYPE_NMI);

    if (cpu_feature_enabled(X86_FEATURE_FRED))
    return fred_entry_from_kvm(event_type, vector);

//
// Notably, we must use IDT dispatch for NMI when running in IDT mode.
// The FRED NMI context is significantly different and will not work
// right (specifically FRED fixed the NMI recursion issue).
//
    idt_do_nmi_irqoff();
    }
    EXPORT_SYMBOL_FOR_KVM(x86_entry_from_kvm);
