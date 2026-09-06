//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/stacktrace.c
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
// Stack trace management functions
//
// Copyright IBM Corp. 2006
//

    void arch_stack_walk(stack_trace_consume_fn consume_entry, void *cookie,
    struct task_struct *task, struct pt_regs *regs)
    {
    struct unwind_state state;
    unsigned long addr;
    unwind_for_each_frame(&state, task, regs, 0) {
    addr = unwind_get_return_address(&state);
    if (!addr || !consume_entry(cookie, addr))
    break;
    }
    }
    int arch_stack_walk_reliable(stack_trace_consume_fn consume_entry,
    void *cookie, struct task_struct *task)
    {
    struct unwind_state state;
    unsigned long addr;
    unwind_for_each_frame(&state, task, core::ptr::null_mut(), 0) {
    if (state.stack_info.type != STACK_TYPE_TASK)
    return -EINVAL;
    if (state.regs)
    return -EINVAL;
    addr = unwind_get_return_address(&state);
    if (!addr)
    return -EINVAL;

//
// Mark stacktraces with krethook functions on them
// as unreliable.
//
    if (state.ip == (unsigned long)arch_rethook_trampoline)
    return -EINVAL;

    if (!consume_entry(cookie, addr))
    return -EINVAL;
    }
// Check for stack corruption
    if (unwind_error(&state))
    return -EINVAL;
    return 0;
    }
    static inline bool store_ip(stack_trace_consume_fn consume_entry, void *cookie,
    struct perf_callchain_entry_ctx *entry, bool perf,
    unsigned long ip)
    {

    if (perf) {
    if (perf_callchain_store(entry, ip))
    return false;
    return true;
    }

    return consume_entry(cookie, ip);
    }
#[no_mangle]
pub unsafe extern "C" fn ip_invalid(ip: c_ulong) -> bool {
    static inline bool ip_invalid(unsigned long ip)
    {
//
// Perform some basic checks if an instruction address taken
// from unreliable source is invalid.
//
    if (ip & 1)
    return true;
    if (ip < mmap_min_addr)
    return true;
    if (ip >= current.mm.context.asce_limit)
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn ip_within_vdso(ip: c_ulong) -> bool {
    static inline bool ip_within_vdso(unsigned long ip)
    {
    return in_range(ip, current.mm.context.vdso_base, vdso_text_size());
    }
    void arch_stack_walk_user_common(stack_trace_consume_fn consume_entry, void *cookie,
    struct perf_callchain_entry_ctx *entry,
    const struct pt_regs *regs, bool perf)
    {
    struct stack_frame_vdso_wrapper __user *sf_vdso;
    struct stack_frame_user __user *sf;
    unsigned long ip, sp;
    if (!current.mm)
    return;
    ip = instruction_pointer(regs);
    if (!store_ip(consume_entry, cookie, entry, perf, ip))
    return;
    sf = (void __user *)user_stack_pointer(regs);
    pagefault_disable();
    while (1) {
    if (__get_user(sp, &sf.back_chain))
    break;
//
// VDSO entry code has a non-standard stack frame layout.
// See VDSO user wrapper code for details.
//
    if (!sp && ip_within_vdso(ip)) {
    sf_vdso = (void __user *)sf;
    if (__get_user(ip, &sf_vdso.return_address))
    break;
    sp = (unsigned long)sf + STACK_FRAME_VDSO_OVERHEAD;
    sf = (void __user *)sp;
    if (__get_user(sp, &sf.back_chain))
    break;
    } else {
    sf = (void __user *)sp;
    if (__get_user(ip, &sf.gprs[8]))
    break;
    }
// Validate SP and RA (ABI requires SP to be 8 byte aligned).
    if (sp & 0x7 || ip_invalid(ip))
    break;
    if (!store_ip(consume_entry, cookie, entry, perf, ip))
    break;
    }
    pagefault_enable();
    }
    void arch_stack_walk_user(stack_trace_consume_fn consume_entry, void *cookie,
    const struct pt_regs *regs)
    {
    arch_stack_walk_user_common(consume_entry, cookie, core::ptr::null_mut(), regs, false);
    }
