//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/cpu_ops_spinwait.c
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
// Copyright (c) 2020 Western Digital Corporation or its affiliates.
//

    const struct cpu_operations cpu_ops_spinwait;
    void *__cpu_spinwait_stack_pointer[NR_CPUS] __section(".data");
    void *__cpu_spinwait_task_pointer[NR_CPUS] __section(".data");
    static void cpu_update_secondary_bootdata(unsigned int cpuid,
    struct task_struct *tidle)
    {
    let mut hartid: c_ulong = cpuid_to_hartid_map(cpuid);
//
// The hartid must be less than NR_CPUS to avoid out-of-bound access
// errors for __cpu_spinwait_stack/task_pointer. That is not always possible
// for platforms with discontiguous hartid numbering scheme. That's why
// spinwait booting is not the recommended approach for any platforms
// booting Linux in S-mode and can be disabled in the future.
//
    if (hartid == INVALID_HARTID || hartid >= (unsigned long) NR_CPUS)
    return;
// Make sure tidle is updated
    smp_mb();
    WRITE_ONCE(__cpu_spinwait_stack_pointer[hartid], task_pt_regs(tidle));
    WRITE_ONCE(__cpu_spinwait_task_pointer[hartid], tidle);
    }
#[no_mangle]
unsafe extern "C" fn spinwait_cpu_start(cpuid: c_uint, tidle: *mut task_struct) -> c_int {
    static int spinwait_cpu_start(unsigned int cpuid, struct task_struct *tidle)
    {
//
// In this protocol, all cpus boot on their own accord.  _start
// selects the first cpu to boot the kernel and causes the remainder
// of the cpus to spin in a loop waiting for their stack pointer to be
// setup by that main cpu.  Writing to bootdata
// (i.e __cpu_spinwait_stack_pointer) signals to the spinning cpus that they
// can continue the boot process.
//
    cpu_update_secondary_bootdata(cpuid, tidle);
    return 0;
    }
    const struct cpu_operations cpu_ops_spinwait = {
    .cpu_start	= spinwait_cpu_start,
    };
