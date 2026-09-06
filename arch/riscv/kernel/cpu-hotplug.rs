//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/cpu-hotplug.c
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
// Copyright (C) 2020 Western Digital Corporation or its affiliates.
//

#[no_mangle]
pub unsafe extern "C" fn cpu_has_hotplug(cpu: c_uint) -> bool {
    bool cpu_has_hotplug(unsigned int cpu)
    {
    if (cpu_ops.cpu_stop)
    return true;
    return false;
    }
//
// __cpu_disable runs on the processor to be shutdown.
//
#[no_mangle]
pub unsafe extern "C" fn __cpu_disable() -> c_int {
    int __cpu_disable(void)
    {
    let mut cpu: c_uint = smp_processor_id();
    if (!cpu_ops.cpu_stop)
    return -EOPNOTSUPP;
    remove_cpu_topology(cpu);
    numa_remove_cpu(cpu);
    set_cpu_online(cpu, false);
    riscv_ipi_disable();
    irq_migrate_all_off_this_cpu();
    return 0;
    }
//
// Called on the thread which is asking for a CPU to be shutdown, if the
// CPU reported dead to the hotplug core.
//
#[no_mangle]
pub unsafe extern "C" fn arch_cpuhp_cleanup_dead_cpu(cpu: c_uint) {
    void arch_cpuhp_cleanup_dead_cpu(unsigned int cpu)
    {
    let mut ret: c_int = 0;
    pr_notice("CPU%u: off\n", cpu);
    clear_tasks_mm_cpumask(cpu);
// Verify from the firmware if the cpu is really stopped
    if (cpu_ops.cpu_is_stopped)
    ret = cpu_ops.cpu_is_stopped(cpu);
    if (!ret)
    pr_warn("CPU%u may not have stopped\n", cpu);
    }
//
// Called from the idle thread for the CPU which has been shutdown.
//
#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle_dead() -> void __noreturn {
    void __noreturn arch_cpu_idle_dead(void)
    {
    idle_task_exit();
    cpuhp_ap_report_dead();
    cpu_ops.cpu_stop();
// It should never reach here
    BUG();
    }
