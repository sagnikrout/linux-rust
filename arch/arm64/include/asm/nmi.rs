//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/nmi.h
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
// Cross-CPU NMI provider hooks, consulted by the arm64 arch code before
// its regular-IRQ / pseudo-NMI IPI paths. The SDEI provider in
// drivers/firmware/arm_sdei_nmi.c implements them when active; a future
// FEAT_NMI provider could slot in here too. The stubs let callers stay
// unconditional when ARM_SDEI_NMI is off.
//
// sdei_nmi_active() lets a caller test for the service before committing
// to (and waiting on) the SDEI stop rung; sdei_nmi_stop_cpus() then signals
// the targets, which ack by going offline.
//

extern "C" {
    pub fn sdei_nmi_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: c_int) -> bool;
}
extern "C" {
    pub fn sdei_nmi_active() -> bool;
}
extern "C" {
    pub fn sdei_nmi_stop_cpus(mask: *const cpumask_t);
}

//
// The common "stop this CPU" entry every arm64 stop path funnels through:
// the regular/pseudo-NMI stop IPI handlers, panic_smp_self_stop(), and the
// SDEI cross-CPU NMI handler. @die_on_crash powers the CPU off on the kdump
// crash path (IPI handlers) instead of parking it (SDEI / self-stop).
// Defined in arch/arm64/kernel/smp.c.
//
extern "C" {
    pub fn arm64_nmi_cpu_stop(regs: *mut pt_regs, die_on_crash: bool) -> void __noreturn;
}
