//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/unwind_bc.c
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

#[no_mangle]
pub unsafe extern "C" fn unwind_get_return_address(state: *mut unwind_state) -> c_ulong {
    unsigned long unwind_get_return_address(struct unwind_state *state)
    {
    if (unwind_done(state))
    return 0;
    return __kernel_text_address(state.ip) ? state.ip : 0;
    }
    EXPORT_SYMBOL_GPL(unwind_get_return_address);
#[no_mangle]
unsafe extern "C" fn outside_of_stack(state: *mut unwind_state, sp: c_ulong) -> bool {
    static bool outside_of_stack(struct unwind_state *state, unsigned long sp)
    {
    return (sp <= state.sp) ||
    (sp > state.stack_info.end - sizeof(struct stack_frame));
    }
#[no_mangle]
unsafe extern "C" fn update_stack_info(state: *mut unwind_state, sp: c_ulong) -> bool {
    static bool update_stack_info(struct unwind_state *state, unsigned long sp)
    {
    struct stack_info *info = &state.stack_info;
    unsigned long *mask = &state.stack_mask;
// New stack pointer leaves the current stack
    if (get_stack_info(sp, state.task, info, mask) != 0 ||
    !on_stack(info, sp, sizeof(struct stack_frame)))
// 'sp' does not point to a valid stack
    return false;
    return true;
    }
    static inline bool is_final_pt_regs(struct unwind_state *state,
    struct pt_regs *regs)
    {
// user mode or kernel thread pt_regs at the bottom of task stack
    if (task_pt_regs(state.task) == regs)
    return true;
// user mode pt_regs at the bottom of irq stack
    return state.stack_info.type == STACK_TYPE_IRQ &&
    state.stack_info.end - sizeof(struct pt_regs) == (unsigned long)regs &&
    READ_ONCE_NOCHECK(regs.psw.mask) & PSW_MASK_PSTATE;
    }
// Avoid KMSAN false positives from touching uninitialized frames.
    __no_kmsan_checks
#[no_mangle]
pub unsafe extern "C" fn unwind_next_frame(state: *mut unwind_state) -> bool {
    bool unwind_next_frame(struct unwind_state *state)
    {
    struct stack_info *info = &state.stack_info;
    struct stack_frame *sf;
    struct pt_regs *regs;
    unsigned long sp, ip;
    bool reliable;
    regs = state.regs;
    if (unlikely(regs)) {
    sp = state.sp;
    sf = (struct stack_frame *) sp;
    ip = READ_ONCE_NOCHECK(sf.gprs[8]);
    reliable = false;
    regs = core::ptr::null_mut();
// skip bogus %r14 or if is the same as regs->psw.addr
    if (!__kernel_text_address(ip) || state.ip == unwind_recover_ret_addr(state, ip)) {
    state.regs = core::ptr::null_mut();
    return unwind_next_frame(state);
    }
    } else {
    sf = (struct stack_frame *) state.sp;
    sp = READ_ONCE_NOCHECK(sf.back_chain);
    if (likely(sp)) {
// Non-zero back-chain points to the previous frame
    if (unlikely(outside_of_stack(state, sp))) {
    if (!update_stack_info(state, sp))
    goto out_err;
    }
    sf = (struct stack_frame *) sp;
    ip = READ_ONCE_NOCHECK(sf.gprs[8]);
    reliable = true;
    } else {
// No back-chain, look for a pt_regs structure
    sp = state.sp + STACK_FRAME_OVERHEAD;
    if (!on_stack(info, sp, sizeof(struct pt_regs)))
    goto out_err;
    regs = (struct pt_regs *) sp;
    if (is_final_pt_regs(state, regs))
    goto out_stop;
    ip = READ_ONCE_NOCHECK(regs.psw.addr);
    sp = READ_ONCE_NOCHECK(regs.gprs[15]);
    if (unlikely(outside_of_stack(state, sp))) {
    if (!update_stack_info(state, sp))
    goto out_err;
    }
    reliable = true;
    }
    }
// Sanity check: ABI requires SP to be aligned 8 bytes.
    if (sp & 0x7)
    goto out_err;
// Update unwind state
    state.sp = sp;
    state.regs = regs;
    state.reliable = reliable;
    state.ip = unwind_recover_ret_addr(state, ip);
    return true;
    out_err:
    state.error = true;
    out_stop:
    state.stack_info.type = STACK_TYPE_UNKNOWN;
    return false;
    }
    EXPORT_SYMBOL_GPL(unwind_next_frame);
// Avoid KMSAN false positives from touching uninitialized frames.
    __no_kmsan_checks
    void __unwind_start(struct unwind_state *state, struct task_struct *task,
    struct pt_regs *regs, unsigned long first_frame)
    {
    struct stack_info *info = &state.stack_info;
    struct stack_frame *sf;
    unsigned long ip, sp;
    memset(state, 0, sizeof(*state));
    state.task = task;
    state.regs = regs;
// Don't even attempt to start from user mode regs:
    if (regs && user_mode(regs)) {
    info.type = STACK_TYPE_UNKNOWN;
    return;
    }
// Get the instruction pointer from pt_regs or the stack frame
    if (regs) {
    ip = regs.psw.addr;
    sp = regs.gprs[15];
    } else if (task == current) {
    sp = current_frame_address();
    } else {
    sp = task.thread.ksp;
    }
// Get current stack pointer and initialize stack info
    if (!update_stack_info(state, sp)) {
// Something is wrong with the stack pointer
    info.type = STACK_TYPE_UNKNOWN;
    state.error = true;
    return;
    }
    if (!regs) {
// Stack frame is within valid stack
    sf = (struct stack_frame *)sp;
    ip = READ_ONCE_NOCHECK(sf.gprs[8]);
    }
// Update unwind state
    state.sp = sp;
    state.reliable = true;
    state.ip = unwind_recover_ret_addr(state, ip);
    if (!first_frame)
    return;
// Skip through the call chain to the specified starting frame
    while (!unwind_done(state)) {
    if (on_stack(&state.stack_info, first_frame, sizeof(struct stack_frame))) {
    if (state.sp >= first_frame)
    break;
    }
    unwind_next_frame(state);
    }
    }
    EXPORT_SYMBOL_GPL(__unwind_start);
