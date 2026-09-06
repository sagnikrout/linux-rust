//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/trace.c
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
// Tracepoint definitions for s390
//
// Copyright IBM Corp. 2015
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
//

// Macro flag: #define CREATE_TRACE_POINTS

    EXPORT_TRACEPOINT_SYMBOL(s390_diagnose);
    static DEFINE_PER_CPU(unsigned int, diagnose_trace_depth);
#[no_mangle]
pub unsafe extern "C" fn trace_s390_diagnose_norecursion(diag_nr: c_int) -> void notrace {
    void notrace trace_s390_diagnose_norecursion(int diag_nr)
    {
    unsigned long flags;
    unsigned int *depth;
// Avoid lockdep recursion.
    if (IS_ENABLED(CONFIG_LOCKDEP))
    return;
    local_irq_save(flags);
    depth = this_cpu_ptr(&diagnose_trace_depth);
    if (*depth == 0) {
    (*depth)++;
    trace_s390_diagnose(diag_nr);
    (*depth)--;
    }
    local_irq_restore(flags);
    }
