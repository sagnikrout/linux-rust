//! Automatically rewritten from C to Rust
//! Source: lib/smp_processor_id.c
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
// lib/smp_processor_id.c
//
// DEBUG_PREEMPT variant of smp_processor_id().
//

    noinstr static
#[no_mangle]
pub unsafe extern "C" fn check_preemption_disabled(what1: *const c_char, what2: *const c_char) -> c_uint {
    unsigned int check_preemption_disabled(const char *what1, const char *what2)
    {
    let mut this_cpu: c_int = raw_smp_processor_id();
    if (likely(preempt_count()))
    goto out;
    if (irqs_disabled())
    goto out;
    if (is_percpu_thread())
    goto out;
    if (current.migration_disabled)
    goto out;
//
// It is valid to assume CPU-locality during early bootup:
//
    if (system_state < SYSTEM_SCHEDULING)
    goto out;
//
// Avoid recursion:
//
    preempt_disable_notrace();
    instrumentation_begin();
    if (!printk_ratelimit())
    goto out_enable;
    printk(KERN_ERR "BUG: using %s%s() in preemptible [%08x] code: %s/%d\n",
    what1, what2, preempt_count() - 1, current.comm, current.pid);
    printk("caller is %pS\n", __builtin_return_address(0));
    dump_stack();
    out_enable:
    instrumentation_end();
    preempt_enable_no_resched_notrace();
    out:
    return this_cpu;
    }
#[no_mangle]
pub unsafe extern "C" fn debug_smp_processor_id() -> noinstr unsigned int {
    noinstr unsigned int debug_smp_processor_id(void)
    {
    return check_preemption_disabled("smp_processor_id", "");
    }
    EXPORT_SYMBOL(debug_smp_processor_id);
#[no_mangle]
pub unsafe extern "C" fn __this_cpu_preempt_check(op: *const c_char) -> noinstr void {
    noinstr void __this_cpu_preempt_check(const char *op)
    {
    check_preemption_disabled("__this_cpu_", op);
    }
    EXPORT_SYMBOL(__this_cpu_preempt_check);
