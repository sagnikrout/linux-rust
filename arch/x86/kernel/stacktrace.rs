//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/stacktrace.c
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


//
// Stack trace management functions
//
// Copyright (C) 2006-2009 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
//

    void arch_stack_walk(stack_trace_consume_fn consume_entry, void *cookie,
    struct task_struct *task, struct pt_regs *regs)
    {
    struct unwind_state state;
    unsigned long addr;
    if (regs && !consume_entry(cookie, regs.ip))
    return;
    for (unwind_start(&state, task, regs, core::ptr::null_mut()); !unwind_done(&state);
    unwind_next_frame(&state)) {
    addr = unwind_get_return_address(&state);
    if (!addr || !consume_entry(cookie, addr))
    break;
    }
    }
    int arch_stack_walk_reliable(stack_trace_consume_fn consume_entry,
    void *cookie, struct task_struct *task)
    {
    struct unwind_state state;
    struct pt_regs *regs;
    unsigned long addr;
    for (unwind_start(&state, task, core::ptr::null_mut(), core::ptr::null_mut());
    !unwind_done(&state) && !unwind_error(&state);
    unwind_next_frame(&state)) {
    regs = unwind_get_entry_regs(&state, core::ptr::null_mut());
    if (regs) {
// Success path for user tasks
    if (user_mode(regs))
    return 0;
//
// Kernel mode registers on the stack indicate an
// in-kernel interrupt or exception (e.g., preemption
// or a page fault), which can make frame pointers
// unreliable.
//
    if (IS_ENABLED(CONFIG_FRAME_POINTER))
    return -EINVAL;
    }
    addr = unwind_get_return_address(&state);
//
// A NULL or invalid return address probably means there's some
// generated code which __kernel_text_address() doesn't know
// about.
//
    if (!addr)
    return -EINVAL;
    if (!consume_entry(cookie, addr))
    return -EINVAL;
    }
// Check for stack corruption
    if (unwind_error(&state))
    return -EINVAL;
    return 0;
    }
// Userspace stacktrace - based on kernel/trace/trace_sysprof.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_frame_user {
    pub next_fp: *const void __user,
    pub ret_addr: c_ulong,
}

    static int
    copy_stack_frame(const struct stack_frame_user __user *fp,
    struct stack_frame_user *frame)
    {
    int ret;
    if (!__access_ok(fp, sizeof(*frame)))
    return 0;
    ret = 1;
    pagefault_disable();
    if (__get_user(frame.next_fp, &fp.next_fp) ||
    __get_user(frame.ret_addr, &fp.ret_addr))
    ret = 0;
    pagefault_enable();
    return ret;
    }
    void arch_stack_walk_user(stack_trace_consume_fn consume_entry, void *cookie,
    const struct pt_regs *regs)
    {
    const void __user *fp = (const void __user *)regs.bp;
    if (!consume_entry(cookie, regs.ip))
    return;
    while (1) {
    struct stack_frame_user frame;
    frame.next_fp = core::ptr::null_mut();
    frame.ret_addr = 0;
    if (!copy_stack_frame(fp, &frame))
    break;
    if ((unsigned long)fp < regs.sp)
    break;
    if (!frame.ret_addr)
    break;
    if (!consume_entry(cookie, frame.ret_addr))
    break;
    fp = frame.next_fp;
    }
    }
