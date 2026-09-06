//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/unwind.c
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
// Copyright (C) 2022-2023 Loongson Technology Corporation Limited
//

#[no_mangle]
pub unsafe extern "C" fn default_next_frame(state: *mut unwind_state) -> bool {
    bool default_next_frame(struct unwind_state *state)
    {
    struct stack_info *info = &state.stack_info;
    unsigned long addr;
    if (unwind_done(state))
    return false;
    do {
    for (state.sp += sizeof(unsigned long);
    state.sp < info.end; state.sp += sizeof(unsigned long)) {
    addr = *(unsigned long *)(state.sp);
    state.pc = unwind_graph_addr(state, addr, state.sp + 8);
    if (__kernel_text_address(state.pc))
    return true;
    }
    state.sp = info.next_sp;
    } while (!get_stack_info(state.sp, state.task, info));
    return false;
    }
