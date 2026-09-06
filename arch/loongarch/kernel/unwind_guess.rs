//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/unwind_guess.c
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

#[no_mangle]
pub unsafe extern "C" fn unwind_get_return_address(state: *mut unwind_state) -> c_ulong {
    unsigned long unwind_get_return_address(struct unwind_state *state)
    {
    return __unwind_get_return_address(state);
    }
    EXPORT_SYMBOL_GPL(unwind_get_return_address);
    void unwind_start(struct unwind_state *state, struct task_struct *task,
    struct pt_regs *regs)
    {
    __unwind_start(state, task, regs);
    if (!unwind_done(state) && !__kernel_text_address(state.pc))
    unwind_next_frame(state);
    }
    EXPORT_SYMBOL_GPL(unwind_start);
#[no_mangle]
pub unsafe extern "C" fn unwind_next_frame(state: *mut unwind_state) -> bool {
    bool unwind_next_frame(struct unwind_state *state)
    {
    return default_next_frame(state);
    }
    EXPORT_SYMBOL_GPL(unwind_next_frame);
