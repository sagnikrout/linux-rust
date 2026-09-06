//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/trace.h
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
// Tracepoint for guest mode entry.
//
// Tracepoint for hypercall.
//
// Tracepoint for hypercall.
//
// Tracepoint for Xen hypercall.
//
// Tracepoint for PIO.
//
pub const KVM_PIO_IN: c_int = 0;
pub const KVM_PIO_OUT: c_int = 1;
//
// Tracepoint for fast mmio.
//
// Tracepoint for cpuid.
//

//
// Tracepoint for apic access.
//

pub const KVM_ISA_VMX: c_int = 1;
pub const KVM_ISA_SVM: c_int = 2;

//
// Tracepoint for kvm guest exit:
//
// Tracepoint for kvm interrupt injection:
//

//
// Tracepoint for kvm interrupt injection:
//
// Tracepoint for page fault.
//
// Tracepoint for guest MSR access.
//

//
// Tracepoint for guest CR access.
//

//
// Tracepoint for nested VMRUN
//
// Tracepoint for #VMEXIT while nested
//
// Tracepoint for #VMEXIT reinjected to the guest
//
// Tracepoint for nested #vmexit because of interrupt pending
//
// Tracepoint for nested #vmexit because of interrupt pending
//
// Tracepoint for nested #vmexit because of interrupt pending
//

//
// Tracepoint for PML full VMEXIT.
//
// Tracepoint for VT-d posted-interrupts and AMD-Vi Guest Virtual APIC.
//
// Tracepoint for kvm_hv_notify_acked_sint.
//
// Tracepoint for synic_set_irq.
//
// Tracepoint for kvm_hv_synic_send_eoi.
//
// Tracepoint for synic_set_msr.
//
// Tracepoint for stimer_set_config.
//
// Tracepoint for stimer_set_count.
//
// Tracepoint for stimer_start(periodic timer case).
//
// Tracepoint for stimer_start(one-shot timer case).
//
// Tracepoint for stimer_timer_callback.
//
// Tracepoint for stimer_expiration.
//
// Tracepoint for stimer_cleanup.
//

//
// Tracepoint for AMD AVIC
//
// Tracepoint for kvm_hv_flush_tlb.
//
// Tracepoint for kvm_hv_flush_tlb_ex.
//
// Tracepoints for kvm_hv_send_ipi.
//
// Tracepoint for failed nested VMX VM-Enter.
//
// Tracepoint for syndbg_set_msr.
//
// Tracepoint for syndbg_get_msr.
//
// Tracepoint for the start of VMGEXIT processing
//
// Tracepoint for the end of VMGEXIT processing
//
// Tracepoint for the start of VMGEXIT MSR procotol processing
//
// Tracepoint for the end of VMGEXIT MSR procotol processing
//
// Tracepoint for #NPFs due to RMP faults.
//

// This part must be outside protection
