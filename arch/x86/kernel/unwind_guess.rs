//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/unwind_guess.c
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

#[no_mangle]
pub unsafe extern "C" fn unwind_get_return_address(state: *mut unwind_state) -> c_ulong {
    unsigned long unwind_get_return_address(struct unwind_state *state)
    {
    unsigned long addr;
    if (unwind_done(state))
    return 0;
    addr = READ_ONCE_NOCHECK(*state.sp);
    return unwind_recover_ret_addr(state, addr, state.sp);
    }
    EXPORT_SYMBOL_GPL(unwind_get_return_address);
    unsigned long *unwind_get_return_address_ptr(struct unwind_state *state)
    {
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn unwind_next_frame(state: *mut unwind_state) -> bool {
    bool unwind_next_frame(struct unwind_state *state)
    {
    struct stack_info *info = &state.stack_info;
    if (unwind_done(state))
    return false;
    do {
    for (state.sp++; state.sp < info.end; state.sp++) {
    let mut addr: c_ulong = READ_ONCE_NOCHECK(*state.sp);
    if (__kernel_text_address(addr))
    return true;
    }
    state.sp = PTR_ALIGN(info.next_sp, sizeof(long));
    } while (!get_stack_info(state.sp, state.task, info,
    &state.stack_mask));
    return false;
    }
    EXPORT_SYMBOL_GPL(unwind_next_frame);
    void __unwind_start(struct unwind_state *state, struct task_struct *task,
    struct pt_regs *regs, unsigned long *first_frame)
    {
    memset(state, 0, sizeof(*state));
    state.task = task;
    state.sp   = PTR_ALIGN(first_frame, sizeof(long));
    get_stack_info(first_frame, state.task, &state.stack_info,
    &state.stack_mask);
//
// The caller can provide the address of the first frame directly
// (first_frame) or indirectly (regs->sp) to indicate which stack frame
// to start unwinding at.  Skip ahead until we reach it.
//
    if (!unwind_done(state) &&
    (!on_stack(&state.stack_info, first_frame, sizeof(long)) ||
    !__kernel_text_address(*first_frame)))
    unwind_next_frame(state);
    }
    EXPORT_SYMBOL_GPL(__unwind_start);
