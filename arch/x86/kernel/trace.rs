//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/trace.c
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


//
// trace_intel_irq_entry - record intel specific IRQ entry
//
#[no_mangle]
unsafe extern "C" fn trace_intel_irq_entry(data: *mut c_void, vector: c_int) {
    static void trace_intel_irq_entry(void *data, int vector)
    {
    osnoise_trace_irq_entry(vector);
    }
//
// trace_intel_irq_exit - record intel specific IRQ exit
//
#[no_mangle]
unsafe extern "C" fn trace_intel_irq_exit(data: *mut c_void, vector: c_int) {
    static void trace_intel_irq_exit(void *data, int vector)
    {
    char *vector_desc = (char *) data;
    osnoise_trace_irq_exit(vector, vector_desc);
    }
//
// register_intel_irq_tp - Register intel specific IRQ entry tracepoints
//
#[no_mangle]
pub unsafe extern "C" fn osnoise_arch_register() -> c_int {
    int osnoise_arch_register(void)
    {
    int ret;
    ret = register_trace_local_timer_entry(trace_intel_irq_entry, core::ptr::null_mut());
    if (ret)
    goto out_err;
    ret = register_trace_local_timer_exit(trace_intel_irq_exit, "local_timer");
    if (ret)
    goto out_timer_entry;

    ret = register_trace_thermal_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());
    if (ret)
    goto out_timer_exit;
    ret = register_trace_thermal_apic_exit(trace_intel_irq_exit, "thermal_apic");
    if (ret)
    goto out_thermal_entry;

    ret = register_trace_deferred_error_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());
    if (ret)
    goto out_thermal_exit;
    ret = register_trace_deferred_error_apic_exit(trace_intel_irq_exit, "deferred_error");
    if (ret)
    goto out_deferred_entry;

    ret = register_trace_threshold_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());
    if (ret)
    goto out_deferred_exit;
    ret = register_trace_threshold_apic_exit(trace_intel_irq_exit, "threshold_apic");
    if (ret)
    goto out_threshold_entry;

    ret = register_trace_call_function_single_entry(trace_intel_irq_entry, core::ptr::null_mut());
    if (ret)
    goto out_threshold_exit;
    ret = register_trace_call_function_single_exit(trace_intel_irq_exit,
    "call_function_single");
    if (ret)
    goto out_call_function_single_entry;
    ret = register_trace_call_function_entry(trace_intel_irq_entry, core::ptr::null_mut());
    if (ret)
    goto out_call_function_single_exit;
    ret = register_trace_call_function_exit(trace_intel_irq_exit, "call_function");
    if (ret)
    goto out_call_function_entry;
    ret = register_trace_reschedule_entry(trace_intel_irq_entry, core::ptr::null_mut());
    if (ret)
    goto out_call_function_exit;
    ret = register_trace_reschedule_exit(trace_intel_irq_exit, "reschedule");
    if (ret)
    goto out_reschedule_entry;

    ret = register_trace_irq_work_entry(trace_intel_irq_entry, core::ptr::null_mut());
    if (ret)
    goto out_reschedule_exit;
    ret = register_trace_irq_work_exit(trace_intel_irq_exit, "irq_work");
    if (ret)
    goto out_irq_work_entry;

    ret = register_trace_x86_platform_ipi_entry(trace_intel_irq_entry, core::ptr::null_mut());
    if (ret)
    goto out_irq_work_exit;
    ret = register_trace_x86_platform_ipi_exit(trace_intel_irq_exit, "x86_platform_ipi");
    if (ret)
    goto out_x86_ipi_entry;
    ret = register_trace_error_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());
    if (ret)
    goto out_x86_ipi_exit;
    ret = register_trace_error_apic_exit(trace_intel_irq_exit, "error_apic");
    if (ret)
    goto out_error_apic_entry;
    ret = register_trace_spurious_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());
    if (ret)
    goto out_error_apic_exit;
    ret = register_trace_spurious_apic_exit(trace_intel_irq_exit, "spurious_apic");
    if (ret)
    goto out_spurious_apic_entry;
    return 0;
    out_spurious_apic_entry:
    unregister_trace_spurious_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());
    out_error_apic_exit:
    unregister_trace_error_apic_exit(trace_intel_irq_exit, "error_apic");
    out_error_apic_entry:
    unregister_trace_error_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());
    out_x86_ipi_exit:
    unregister_trace_x86_platform_ipi_exit(trace_intel_irq_exit, "x86_platform_ipi");
    out_x86_ipi_entry:
    unregister_trace_x86_platform_ipi_entry(trace_intel_irq_entry, core::ptr::null_mut());
    out_irq_work_exit:

    unregister_trace_irq_work_exit(trace_intel_irq_exit, "irq_work");
    out_irq_work_entry:
    unregister_trace_irq_work_entry(trace_intel_irq_entry, core::ptr::null_mut());
    out_reschedule_exit:

    unregister_trace_reschedule_exit(trace_intel_irq_exit, "reschedule");
    out_reschedule_entry:
    unregister_trace_reschedule_entry(trace_intel_irq_entry, core::ptr::null_mut());
    out_call_function_exit:
    unregister_trace_call_function_exit(trace_intel_irq_exit, "call_function");
    out_call_function_entry:
    unregister_trace_call_function_entry(trace_intel_irq_entry, core::ptr::null_mut());
    out_call_function_single_exit:
    unregister_trace_call_function_single_exit(trace_intel_irq_exit, "call_function_single");
    out_call_function_single_entry:
    unregister_trace_call_function_single_entry(trace_intel_irq_entry, core::ptr::null_mut());
    out_threshold_exit:

    unregister_trace_threshold_apic_exit(trace_intel_irq_exit, "threshold_apic");
    out_threshold_entry:
    unregister_trace_threshold_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());
    out_deferred_exit:

    unregister_trace_deferred_error_apic_exit(trace_intel_irq_exit, "deferred_error");
    out_deferred_entry:
    unregister_trace_deferred_error_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());
    out_thermal_exit:

    unregister_trace_thermal_apic_exit(trace_intel_irq_exit, "thermal_apic");
    out_thermal_entry:
    unregister_trace_thermal_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());
    out_timer_exit:

    unregister_trace_local_timer_exit(trace_intel_irq_exit, "local_timer");
    out_timer_entry:
    unregister_trace_local_timer_entry(trace_intel_irq_entry, core::ptr::null_mut());
    out_err:
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn osnoise_arch_unregister() {
    void osnoise_arch_unregister(void)
    {
    unregister_trace_spurious_apic_exit(trace_intel_irq_exit, "spurious_apic");
    unregister_trace_spurious_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());
    unregister_trace_error_apic_exit(trace_intel_irq_exit, "error_apic");
    unregister_trace_error_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());
    unregister_trace_x86_platform_ipi_exit(trace_intel_irq_exit, "x86_platform_ipi");
    unregister_trace_x86_platform_ipi_entry(trace_intel_irq_entry, core::ptr::null_mut());

    unregister_trace_irq_work_exit(trace_intel_irq_exit, "irq_work");
    unregister_trace_irq_work_entry(trace_intel_irq_entry, core::ptr::null_mut());

    unregister_trace_reschedule_exit(trace_intel_irq_exit, "reschedule");
    unregister_trace_reschedule_entry(trace_intel_irq_entry, core::ptr::null_mut());
    unregister_trace_call_function_exit(trace_intel_irq_exit, "call_function");
    unregister_trace_call_function_entry(trace_intel_irq_entry, core::ptr::null_mut());
    unregister_trace_call_function_single_exit(trace_intel_irq_exit, "call_function_single");
    unregister_trace_call_function_single_entry(trace_intel_irq_entry, core::ptr::null_mut());

    unregister_trace_threshold_apic_exit(trace_intel_irq_exit, "threshold_apic");
    unregister_trace_threshold_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());

    unregister_trace_deferred_error_apic_exit(trace_intel_irq_exit, "deferred_error");
    unregister_trace_deferred_error_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());

    unregister_trace_thermal_apic_exit(trace_intel_irq_exit, "thermal_apic");
    unregister_trace_thermal_apic_entry(trace_intel_irq_entry, core::ptr::null_mut());

    unregister_trace_local_timer_exit(trace_intel_irq_exit, "local_timer");
    unregister_trace_local_timer_entry(trace_intel_irq_entry, core::ptr::null_mut());
    }
