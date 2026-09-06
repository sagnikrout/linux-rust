//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/smp.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2012 Regents of the University of California
//

//
// Mapping between linux logical cpu index and hartid.
//

// print IPI stats
extern "C" {
    pub fn show_ipi_stats(p: *mut seq_file, prec: c_int);
}
// SMP initialization hook for setup_arch
extern "C" {
    pub fn setup_smp() -> void __init;
}
// Hook for the generic smp_call_function_many() routine.
extern "C" {
    pub fn arch_send_call_function_ipi_mask(mask: *mut cpumask);
}
// Hook for the generic smp_call_function_single() routine.
extern "C" {
    pub fn arch_send_call_function_single_ipi(cpu: c_int);
}
extern "C" {
    pub fn riscv_hartid_to_cpuid(hartid: c_ulong) -> c_int;
}
// Enable IPI for CPU hotplug
extern "C" {
    pub fn riscv_ipi_enable();
}
// Disable IPI for CPU hotplug
extern "C" {
    pub fn riscv_ipi_disable();
}
// Check if IPI interrupt numbers are available
extern "C" {
    pub fn riscv_ipi_have_virq_range() -> bool;
}
// Set the IPI interrupt numbers for arch (called by irqchip drivers)
extern "C" {
    pub fn riscv_ipi_set_virq_range(virq: c_int, nr: c_int);
}
// Check other CPUs stop or not
extern "C" {
    pub fn smp_crash_stop_failed() -> bool;
}
// Secondary hart entry
extern "C" {
    pub fn smp_callin() -> asmlinkage void;
}
//
// Obtains the hart ID of the currently executing task.  This relies on
// THREAD_INFO_IN_TASK, but we define that unconditionally.
//

extern "C" {
    pub fn __cpu_disable() -> c_int;
}

extern "C" {
    pub fn cpu_has_hotplug(cpu: c_uint) -> bool;
}

