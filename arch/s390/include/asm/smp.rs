//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/smp.h
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
// Copyright IBM Corp. 1999, 2012
// Author(s): Denis Joseph Barrow,
// Martin Schwidefsky <schwidefsky@de.ibm.com>,
//

extern "C" {
    pub fn __cpu_up(cpu: c_uint, tidle: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn arch_send_call_function_single_ipi(cpu: c_int);
}
extern "C" {
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);
}
extern "C" {
    pub fn smp_call_ipl_cpu(): *mut *mut void (func)(void, data: *mut c_void) -> void __noreturn;
}
extern "C" {
    pub fn smp_emergency_stop();
}
extern "C" {
    pub fn smp_find_processor_id(address: u16) -> c_int;
}
extern "C" {
    pub fn smp_store_status(cpu: c_int) -> c_int;
}
extern "C" {
    pub fn smp_save_dump_ipl_cpu();
}
extern "C" {
    pub fn smp_save_dump_secondary_cpus();
}
extern "C" {
    pub fn smp_yield_cpu(cpu: c_int);
}
extern "C" {
    pub fn smp_cpu_set_polarization(cpu: c_int, val: c_int);
}
extern "C" {
    pub fn smp_cpu_get_polarization(cpu: c_int) -> c_int;
}
extern "C" {
    pub fn smp_set_core_capacity(cpu: c_int, val: c_ulong);
}
extern "C" {
    pub fn smp_cpu_get_cpu_address(cpu: c_int) -> c_int;
}
extern "C" {
    pub fn smp_fill_possible_mask();
}
extern "C" {
    pub fn smp_detect_cpus();
}
// Return thread 0 CPU number as base CPU
extern "C" {
    pub fn smp_rescan_cpus(early: bool) -> c_int;
}
extern "C" {
    pub fn cpu_die() -> void __noreturn;
}
extern "C" {
    pub fn __cpu_die(cpu: c_uint);
}
extern "C" {
    pub fn __cpu_disable() -> c_int;
}
extern "C" {
    pub fn schedule_mcck_handler();
}
extern "C" {
    pub fn smp_yield_cpu(cpu: c_int) -> void notrace;
}
