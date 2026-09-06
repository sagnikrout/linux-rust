//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-tracepoints.c
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

    let mut opal_tracepoint_key: static_key = STATIC_KEY_INIT;
#[no_mangle]
pub unsafe extern "C" fn opal_tracepoint_regfunc() -> c_int {
    int opal_tracepoint_regfunc(void)
    {
    static_key_slow_inc(&opal_tracepoint_key);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn opal_tracepoint_unregfunc() {
    void opal_tracepoint_unregfunc(void)
    {
    static_key_slow_dec(&opal_tracepoint_key);
    }

//
// We optimise OPAL calls by placing opal_tracepoint_refcount
// directly in the TOC so we can check if the opal tracepoints are
// enabled via a single load.
//
// NB: reg/unreg are called while guarded with the tracepoints_mutex
    extern long opal_tracepoint_refcount;
#[no_mangle]
pub unsafe extern "C" fn opal_tracepoint_regfunc() -> c_int {
    int opal_tracepoint_regfunc(void)
    {
    opal_tracepoint_refcount++;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn opal_tracepoint_unregfunc() {
    void opal_tracepoint_unregfunc(void)
    {
    opal_tracepoint_refcount--;
    }

//
// Since the tracing code might execute OPAL calls we need to guard against
// recursion.
//
    static DEFINE_PER_CPU(unsigned int, opal_trace_depth);
#[no_mangle]
pub unsafe extern "C" fn __trace_opal_entry(opcode: c_ulong, args: *mut c_ulong) {
    void __trace_opal_entry(unsigned long opcode, unsigned long *args)
    {
    unsigned long flags;
    unsigned int *depth;
    local_irq_save(flags);
    depth = this_cpu_ptr(&opal_trace_depth);
    if (*depth)
    goto out;
    (*depth)++;
    preempt_disable();
    trace_opal_entry(opcode, args);
    (*depth)--;
    out:
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn __trace_opal_exit(opcode: c_long, retval: c_ulong) {
    void __trace_opal_exit(long opcode, unsigned long retval)
    {
    unsigned long flags;
    unsigned int *depth;
    local_irq_save(flags);
    depth = this_cpu_ptr(&opal_trace_depth);
    if (*depth)
    goto out;
    (*depth)++;
    trace_opal_exit(opcode, retval);
    preempt_enable();
    (*depth)--;
    out:
    local_irq_restore(flags);
    }
