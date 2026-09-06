//! Automatically rewritten from C to Rust
//! Source: arch/x86/xen/smp_hvm.c
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

#[no_mangle]
unsafe extern "C" fn xen_hvm_smp_prepare_boot_cpu() -> void __init {
    static void __init xen_hvm_smp_prepare_boot_cpu(void)
    {
    BUG_ON(smp_processor_id() != 0);
    native_smp_prepare_boot_cpu();
//
// Setup vcpu_info for boot CPU. Secondary CPUs get their vcpu_info
// in xen_cpu_up_prepare_hvm().
//
    xen_vcpu_setup(0);
//
// Called again in case the kernel boots on vcpu >= MAX_VIRT_CPUS.
// Refer to comments in xen_hvm_init_time_ops().
//
    xen_hvm_init_time_ops();
//
// The alternative logic (which patches the unlock/lock) runs before
// the smp bootup up code is activated. Hence we need to set this up
// the core kernel is being patched. Otherwise we will have only
// modules patched but not core code.
//
    xen_init_spinlocks();
    }
#[no_mangle]
unsafe extern "C" fn xen_hvm_smp_prepare_cpus(max_cpus: c_uint) -> void __init {
    static void __init xen_hvm_smp_prepare_cpus(unsigned int max_cpus)
    {
    int cpu;
    native_smp_prepare_cpus(max_cpus);
    if (xen_have_vector_callback) {
    WARN_ON(xen_smp_intr_init(0));
    xen_init_lock_cpu(0);
    }
    for_each_possible_cpu(cpu) {
    if (cpu == 0)
    continue;
// Set default vcpu_id to make sure that we don't use cpu-0's
    per_cpu(xen_vcpu_id, cpu) = XEN_VCPU_ID_INVALID;
    }
    }

#[no_mangle]
unsafe extern "C" fn xen_hvm_cleanup_dead_cpu(cpu: c_uint) {
    static void xen_hvm_cleanup_dead_cpu(unsigned int cpu)
    {
    if (xen_have_vector_callback) {
    xen_smp_intr_free(cpu);
    xen_uninit_lock_cpu(cpu);
    xen_teardown_timer(cpu);
    }
    }

#[no_mangle]
unsafe extern "C" fn xen_hvm_cleanup_dead_cpu(cpu: c_uint) {
    static void xen_hvm_cleanup_dead_cpu(unsigned int cpu)
    {
    BUG();
    }

#[no_mangle]
pub unsafe extern "C" fn xen_hvm_smp_init() -> void __init {
    void __init xen_hvm_smp_init(void)
    {
    smp_ops.smp_prepare_boot_cpu = xen_hvm_smp_prepare_boot_cpu;
    smp_ops.smp_prepare_cpus = xen_hvm_smp_prepare_cpus;
    smp_ops.smp_cpus_done = xen_smp_cpus_done;
    smp_ops.cleanup_dead_cpu = xen_hvm_cleanup_dead_cpu;
    if (!xen_have_vector_callback) {

    nopvspin = true;

    return;
    }
    smp_ops.smp_send_reschedule = xen_smp_send_reschedule;
    smp_ops.send_call_func_ipi = xen_smp_send_call_function_ipi;
    smp_ops.send_call_func_single_ipi = xen_smp_send_call_function_single_ipi;
    }
