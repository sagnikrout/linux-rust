//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/smp.h
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
// Copyright (C) 2012 ARM Ltd.
//

// Values for secondary_data.status

// The cpu invoked ops->cpu_die, synchronise it with cpu_kill

// The cpu couldn't die gracefully and is looping in the kernel

// Fatal system error detected by secondary CPU, crash the system

//
// Logical CPU mapping.
//
extern "C" {
    pub fn cpu_logical_map(cpu: c_uint) -> u64;
}
//
// Discover the set of possible CPUs and determine their
// SMP operations.
//
extern "C" {
    pub fn smp_init_cpus();
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipi_msg_type {
    IPI_RESCHEDULE,
    IPI_CALL_FUNC,
    IPI_CPU_STOP,
    IPI_CPU_STOP_NMI,
    IPI_TIMER,
    IPI_IRQ_WORK,
    NR_IPI,
//
// Any enum >= NR_IPI and < MAX_IPI is special and not tracable
// with trace_ipi_
//
    IPI_CPU_BACKTRACE = NR_IPI,
    IPI_KGDB_ROUNDUP,
    MAX_IPI
}

//
// Register IPI interrupts with the arch SMP code
//
extern "C" {
    pub fn set_smp_ipi_range_percpu(ipi_base: c_int, nr_ipi: c_int, ncpus: c_int);
}
//
// Called from the secondary holding pen, this is the secondary CPU entry point.
//
extern "C" {
    pub fn secondary_start_kernel() -> asmlinkage void;
}
//
// Initial data for bringing up a secondary CPU.
// @status - Result passed back from the secondary CPU to
// indicate failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct secondary_data {
    pub task: *mut task_struct,
    pub status: c_long,
}

extern "C" {
    pub fn secondary_entry();
}
extern "C" {
    pub fn arch_send_call_function_single_ipi(cpu: c_int);
}
extern "C" {
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);
}

extern "C" {
    pub fn arch_send_wakeup_ipi(cpu: c_uint);
}

extern "C" {
    pub fn __cpu_disable() -> c_int;
}
extern "C" {
    pub fn cpu_die() -> void __noreturn;
}
extern "C" {
    pub fn cpu_die_early() -> void __noreturn;
}
// Ensure the visibility of the status update
//
// The calling secondary CPU has detected serious configuration mismatch,
// which calls for a kernel panic. Update the boot status and park the calling
// CPU.
//
// If a secondary CPU enters the kernel but fails to come online,
// (e.g. due to mismatched features), and cannot exit the kernel,
// we increment cpus_stuck_in_kernel and leave the CPU in a
// quiesecent loop within the kernel text. The memory containing
// this loop must not be re-used for anything else as the 'stuck'
// core is executing it.
//
// This function is used to inhibit features like kexec and hibernate.
//
extern "C" {
    pub fn cpus_are_stuck_in_kernel() -> bool;
}
extern "C" {
    pub fn crash_smp_send_stop();
}
extern "C" {
    pub fn smp_crash_stop_failed() -> bool;
}

