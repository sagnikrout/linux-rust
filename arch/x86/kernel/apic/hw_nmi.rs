//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/apic/hw_nmi.c
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
// HW NMI watchdog support
//
// started by Don Zickus, Copyright (C) 2010 Red Hat, Inc.
//
// Arch specific calls to support NMI watchdog
//
// Bits copied from original nmi.c file
//

#[no_mangle]
pub unsafe extern "C" fn hw_nmi_get_sample_period(watchdog_thresh: c_int) -> u64 {
    u64 hw_nmi_get_sample_period(int watchdog_thresh)
    {
    return (u64)(cpu_khz) * 1000 * watchdog_thresh;
    }

#[no_mangle]
unsafe extern "C" fn nmi_raise_cpu_backtrace(mask: *mut cpumask_t) {
    static void nmi_raise_cpu_backtrace(cpumask_t *mask)
    {
    __apic_send_IPI_mask(mask, NMI_VECTOR);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: c_int) {
    void arch_trigger_cpumask_backtrace(const cpumask_t *mask, int exclude_cpu)
    {
    nmi_trigger_cpumask_backtrace(mask, exclude_cpu,
    nmi_raise_cpu_backtrace);
    }
#[no_mangle]
unsafe extern "C" fn nmi_cpu_backtrace_handler(cmd: c_uint, regs: *mut pt_regs) -> c_int {
    static int nmi_cpu_backtrace_handler(unsigned int cmd, struct pt_regs *regs)
    {
    if (nmi_cpu_backtrace(regs))
    return NMI_HANDLED;
    return NMI_DONE;
    }
    NOKPROBE_SYMBOL(nmi_cpu_backtrace_handler);
#[no_mangle]
unsafe extern "C" fn register_nmi_cpu_backtrace_handler() -> int __init {
    static int __init register_nmi_cpu_backtrace_handler(void)
    {
    register_nmi_handler(NMI_LOCAL, nmi_cpu_backtrace_handler,
    0, "arch_bt");
    return 0;
    }
    early_initcall(register_nmi_cpu_backtrace_handler);
