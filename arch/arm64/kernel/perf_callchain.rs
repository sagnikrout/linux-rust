//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/perf_callchain.c
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
// arm64 callchain support
//
// Copyright (C) 2015 ARM Limited
//

#[no_mangle]
unsafe extern "C" fn callchain_trace(data: *mut c_void, pc: c_ulong) -> bool {
    static bool callchain_trace(void *data, unsigned long pc)
    {
    struct perf_callchain_entry_ctx *entry = data;
    return perf_callchain_store(entry, pc) == 0;
    }
    void perf_callchain_user(struct perf_callchain_entry_ctx *entry,
    struct pt_regs *regs)
    {
    if (perf_guest_state()) {
// We don't support guest os callchain now
    return;
    }
    arch_stack_walk_user(callchain_trace, entry, regs);
    }
    void perf_callchain_kernel(struct perf_callchain_entry_ctx *entry,
    struct pt_regs *regs)
    {
    if (perf_guest_state()) {
// We don't support guest os callchain now
    return;
    }
    arch_stack_walk(callchain_trace, entry, current, regs);
    }
