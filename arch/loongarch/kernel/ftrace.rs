//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/ftrace.c
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
// Copyright (C) 2022 Loongson Technology Corporation Limited
//

//
// As `call _mcount` follows LoongArch psABI, ra-saved operation and
// stack operation can be found before this insn.
//
#[no_mangle]
unsafe extern "C" fn ftrace_get_parent_ra_addr(insn_addr: c_ulong, ra_off: *mut c_int) -> c_int {
    static int ftrace_get_parent_ra_addr(unsigned long insn_addr, int *ra_off)
    {
    let mut limit: c_int = 32;
    union loongarch_instruction *insn;
    insn = (union loongarch_instruction *)insn_addr;
    do {
    insn--;
    limit--;
    if (is_ra_save_ins(insn))
// ra_off = -((1 << 12) - insn->reg2i12_format.immediate);
    } while (!is_stack_alloc_ins(insn) && limit);
    if (!limit)
    return -EINVAL;
    return 0;
    }
    void prepare_ftrace_return(unsigned long self_addr,
    unsigned long callsite_sp, unsigned long old)
    {
    int ra_off;
    let mut return_hooker: c_ulong = (unsigned long)&return_to_handler;
    if (unlikely(ftrace_graph_is_dead()))
    return;
    if (unlikely(atomic_read(&current.tracing_graph_pause)))
    return;
    if (ftrace_get_parent_ra_addr(self_addr, &ra_off))
    goto out;
    if (!function_graph_enter(old, self_addr, 0, core::ptr::null_mut()))
// (unsigned long *)(callsite_sp + ra_off) = return_hooker;
    return;
    out:
    ftrace_graph_stop();
    WARN_ON(1);
    }
