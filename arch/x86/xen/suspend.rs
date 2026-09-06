//! Automatically rewritten from C to Rust
//! Source: arch/x86/xen/suspend.c
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

    static DEFINE_PER_CPU(u64, spec_ctrl);
#[no_mangle]
pub unsafe extern "C" fn xen_arch_pre_suspend() {
    void xen_arch_pre_suspend(void)
    {
    xen_save_time_memory_area();
    if (xen_pv_domain())
    xen_pv_pre_suspend();
    }
#[no_mangle]
pub unsafe extern "C" fn xen_arch_post_suspend(cancelled: c_int) {
    void xen_arch_post_suspend(int cancelled)
    {
    if (xen_pv_domain())
    xen_pv_post_suspend(cancelled);
    else
    xen_hvm_post_suspend(cancelled);
    xen_restore_time_memory_area();
    }
#[no_mangle]
unsafe extern "C" fn xen_vcpu_notify_restore(data: *mut c_void) {
    static void xen_vcpu_notify_restore(void *data)
    {
    if (xen_pv_domain() && boot_cpu_has(X86_FEATURE_SPEC_CTRL))
    wrmsrq(MSR_IA32_SPEC_CTRL, this_cpu_read(spec_ctrl));
// Boot processor notified via generic timekeeping_resume()
    if (smp_processor_id() == 0)
    return;
    tick_resume_local();
    }
#[no_mangle]
unsafe extern "C" fn xen_vcpu_notify_suspend(data: *mut c_void) {
    static void xen_vcpu_notify_suspend(void *data)
    {
    u64 tmp;
    tick_suspend_local();
    if (xen_pv_domain() && boot_cpu_has(X86_FEATURE_SPEC_CTRL)) {
    rdmsrq(MSR_IA32_SPEC_CTRL, tmp);
    this_cpu_write(spec_ctrl, tmp);
    wrmsrq(MSR_IA32_SPEC_CTRL, 0);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn xen_arch_resume() {
    void xen_arch_resume(void)
    {
    int cpu;
    on_each_cpu(xen_vcpu_notify_restore, core::ptr::null_mut(), 1);
    for_each_online_cpu(cpu)
    xen_pmu_init(cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn xen_arch_suspend() {
    void xen_arch_suspend(void)
    {
    int cpu;
    for_each_online_cpu(cpu)
    xen_pmu_finish(cpu);
    on_each_cpu(xen_vcpu_notify_suspend, core::ptr::null_mut(), 1);
    }
