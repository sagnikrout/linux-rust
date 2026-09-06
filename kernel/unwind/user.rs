//! Automatically rewritten from C to Rust
//! Source: kernel/unwind/user.c
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
// Generic interfaces for unwinding user space
//

    for (unwind_user_start(state); !(state).done; unwind_user_next(state))
    static inline int
    get_user_word(unsigned long *word, unsigned long base, int off, unsigned int ws)
    {
    unsigned long __user *addr = (void __user *)base + off;

    if (ws == sizeof(int)) {
    unsigned int data;
    let mut ret: c_int = get_user(data, (unsigned int __user *)addr);
// word = data;
    return ret;
    }

    return get_user(*word, addr);
    }
    static int unwind_user_next_common(struct unwind_user_state *state,
    const struct unwind_user_frame *frame)
    {
    unsigned long cfa, fp, ra;
// Get the Canonical Frame Address (CFA)
    if (frame.use_fp) {
    if (state.fp < state.sp)
    return -EINVAL;
    cfa = state.fp;
    } else {
    cfa = state.sp;
    }
    cfa += frame.cfa_off;
// Make sure that stack is not going in wrong direction
    if (cfa <= state.sp)
    return -EINVAL;
// Make sure that the address is word aligned
    if (cfa & (state.ws - 1))
    return -EINVAL;
// Get the Return Address (RA)
    if (get_user_word(&ra, cfa, frame.ra_off, state.ws))
    return -EINVAL;
// Get the Frame Pointer (FP)
    if (frame.fp_off && get_user_word(&fp, cfa, frame.fp_off, state.ws))
    return -EINVAL;
    state.ip = ra;
    state.sp = cfa;
    if (frame.fp_off)
    state.fp = fp;
    state.topmost = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn unwind_user_next_fp(state: *mut unwind_user_state) -> c_int {
    static int unwind_user_next_fp(struct unwind_user_state *state)
    {
    struct pt_regs *regs = task_pt_regs(current);
    if (state.topmost && unwind_user_at_function_start(regs)) {
    const struct unwind_user_frame fp_entry_frame = {
    ARCH_INIT_USER_FP_ENTRY_FRAME(state.ws)
    };
    return unwind_user_next_common(state, &fp_entry_frame);
    }
    const struct unwind_user_frame fp_frame = {
    ARCH_INIT_USER_FP_FRAME(state.ws)
    };
    return unwind_user_next_common(state, &fp_frame);
    }
#[no_mangle]
unsafe extern "C" fn unwind_user_next(state: *mut unwind_user_state) -> c_int {
    static int unwind_user_next(struct unwind_user_state *state)
    {
    let mut iter_mask: c_ulong = state.available_types;
    unsigned int bit;
    if (state.done)
    return -EINVAL;
    for_each_set_bit(bit, &iter_mask, NR_UNWIND_USER_TYPE_BITS) {
    let mut type: enum unwind_user_type = BIT(bit);
    state.current_type = type;
    switch (type) {
    case UNWIND_USER_TYPE_FP:
    if (!unwind_user_next_fp(state))
    return 0;
    continue;
    default:
    WARN_ONCE(1, "Undefined unwind bit %d", bit);
    break;
    }
    break;
    }
// No successful unwind method.
    state.current_type = UNWIND_USER_TYPE_NONE;
    state.done = true;
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn unwind_user_start(state: *mut unwind_user_state) -> c_int {
    static int unwind_user_start(struct unwind_user_state *state)
    {
    struct pt_regs *regs = task_pt_regs(current);
    memset(state, 0, sizeof(*state));
    if ((current.flags & PF_KTHREAD) || !user_mode(regs)) {
    state.done = true;
    return -EINVAL;
    }
    if (IS_ENABLED(CONFIG_HAVE_UNWIND_USER_FP))
    state.available_types |= UNWIND_USER_TYPE_FP;
    state.ip = instruction_pointer(regs);
    state.sp = user_stack_pointer(regs);
    state.fp = frame_pointer(regs);
    state.ws = unwind_user_word_size(regs);
    if (!state.ws) {
    state.done = true;
    return -EINVAL;
    }
    state.topmost = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn unwind_user(trace: *mut unwind_stacktrace, max_entries: c_uint) -> c_int {
    int unwind_user(struct unwind_stacktrace *trace, unsigned int max_entries)
    {
    struct unwind_user_state state;
    trace.nr = 0;
    if (!max_entries)
    return -EINVAL;
    if (current.flags & PF_KTHREAD)
    return 0;
    for_each_user_frame(&state) {
    trace.entries[trace.nr++] = state.ip;
    if (trace.nr >= max_entries)
    break;
    }
    return 0;
    }
