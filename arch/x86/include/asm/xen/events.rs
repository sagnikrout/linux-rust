//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/xen/events.h
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
pub enum ipi_vector {
    XEN_RESCHEDULE_VECTOR,
    XEN_CALL_FUNCTION_VECTOR,
    XEN_CALL_FUNCTION_SINGLE_VECTOR,
    XEN_SPIN_UNLOCK_VECTOR,
    XEN_IRQ_WORK_VECTOR,
    XEN_NMI_VECTOR,

    XEN_NR_IPIS,
}

extern "C" {
    pub fn raw_irqs_disabled_flags(_arg: regs->flags) -> return;
}
// No need for a barrier -- XCHG is a barrier on x86.

//
// Events delivered via platform PCI interrupts are always
// routed to vcpu 0 and hence cannot be rebound.
//
