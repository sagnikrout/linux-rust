//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/stacktrace.c
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
// Copyright (C) 2008 ARM Limited
// Copyright (C) 2014 Regents of the University of California
//

//
// This disables KASAN checking when reading a value from another task's stack,
// since the other task could be running on another CPU and could have poisoned
// the stack in the meantime.
//

    ({							\
    unsigned long val;				\
    unsigned long addr = x;				\
    if ((task) == current)				\
    val = READ_ONCE(addr);			\
    else						\
    val = READ_ONCE_NOCHECK(addr);		\
    val;						\
    })
    extern asmlinkage void handle_exception(void);
    extern unsigned long ret_from_exception_end;
#[no_mangle]
pub unsafe extern "C" fn fp_is_valid(fp: c_ulong, sp: c_ulong) -> c_int {
    static inline int fp_is_valid(unsigned long fp, unsigned long sp)
    {
    unsigned long low, high;
    low = sp + sizeof(struct stackframe);
    high = ALIGN(sp, THREAD_SIZE);
    return !(fp < low || fp > high || fp & 0x07);
    }
    void notrace walk_stackframe(struct task_struct *task, struct pt_regs *regs,
    bool (*fn)(void *, unsigned long), void *arg)
    {
    unsigned long fp, sp, pc;
    let mut graph_idx: c_int = 0;
    let mut level: c_int = 0;
    if (regs) {
    fp = frame_pointer(regs);
    sp = user_stack_pointer(regs);
    pc = instruction_pointer(regs);
    } else if (task == core::ptr::null_mut() || task == current) {
    fp = (unsigned long)__builtin_frame_address(0);
    sp = current_stack_pointer;
    pc = (unsigned long)walk_stackframe;
    level = -1;
    } else {
// task blocked in __switch_to
    fp = task.thread.s[0];
    sp = task.thread.sp;
    pc = task.thread.ra;
    }
    for (;;) {
    struct stackframe *frame;
    if (unlikely(!__kernel_text_address(pc) || (level++ >= 0 && !fn(arg, pc))))
    break;
    if (unlikely(!fp_is_valid(fp, sp)))
    break;
// Unwind stack frame
    frame = (struct stackframe *)fp - 1;
    sp = fp;
    if (regs && (regs.epc == pc) && fp_is_valid(frame.ra, sp)) {
// We hit function where ra is not saved on the stack
    fp = frame.ra;
    pc = regs.ra;
    } else {
    fp = READ_ONCE_TASK_STACK(task, frame.fp);
    pc = READ_ONCE_TASK_STACK(task, frame.ra);
    pc = ftrace_graph_ret_addr(task, &graph_idx, pc,
    &frame.ra);
    if (pc >= (unsigned long)handle_exception &&
    pc < (unsigned long)&ret_from_exception_end) {
    if (unlikely(!fn(arg, pc)))
    break;
    pc = ((struct pt_regs *)sp).epc;
    fp = ((struct pt_regs *)sp).s0;
    }
    }
    }
    }

    void notrace walk_stackframe(struct task_struct *task,
    struct pt_regs *regs, bool (*fn)(void *, unsigned long), void *arg)
    {
    unsigned long sp, pc;
    unsigned long *ksp;
    if (regs) {
    sp = user_stack_pointer(regs);
    pc = instruction_pointer(regs);
    } else if (task == core::ptr::null_mut() || task == current) {
    sp = current_stack_pointer;
    pc = (unsigned long)walk_stackframe;
    } else {
// task blocked in __switch_to
    sp = task.thread.sp;
    pc = task.thread.ra;
    }
    if (unlikely(sp & 0x7))
    return;
    ksp = (unsigned long *)sp;
    while (!kstack_end(ksp)) {
    if (__kernel_text_address(pc) && unlikely(!fn(arg, pc)))
    break;
    pc = READ_ONCE_NOCHECK(*ksp++);
    }
    }

#[no_mangle]
unsafe extern "C" fn print_trace_address(arg: *mut c_void, pc: c_ulong) -> bool {
    static bool print_trace_address(void *arg, unsigned long pc)
    {
    const char *loglvl = arg;
    print_ip_sym(loglvl, pc);
    return true;
    }
    noinline void dump_backtrace(struct pt_regs *regs, struct task_struct *task,
    const char *loglvl)
    {
    walk_stackframe(task, regs, print_trace_address, (void *)loglvl);
    }
#[no_mangle]
pub unsafe extern "C" fn show_stack(task: *mut task_struct, sp: *mut c_ulong, loglvl: *const c_char) {
    void show_stack(struct task_struct *task, unsigned long *sp, const char *loglvl)
    {
    pr_cont("%sCall Trace:\n", loglvl);
    dump_backtrace(core::ptr::null_mut(), task, loglvl);
    }
#[no_mangle]
unsafe extern "C" fn save_wchan(arg: *mut c_void, pc: c_ulong) -> bool {
    static bool save_wchan(void *arg, unsigned long pc)
    {
    if (!in_sched_functions(pc)) {
    unsigned long *p = arg;
// p = pc;
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn __get_wchan(task: *mut task_struct) -> c_ulong {
    unsigned long __get_wchan(struct task_struct *task)
    {
    let mut pc: c_ulong = 0;
    if (!try_get_task_stack(task))
    return 0;
    walk_stackframe(task, core::ptr::null_mut(), save_wchan, &pc);
    put_task_stack(task);
    return pc;
    }
    noinline noinstr void arch_stack_walk(stack_trace_consume_fn consume_entry, void *cookie,
    struct task_struct *task, struct pt_regs *regs)
    {
    walk_stackframe(task, regs, consume_entry, cookie);
    }
//
// Get the return address for a single stackframe and return a pointer to the
// next frame tail.
//
    static unsigned long unwind_user_frame(stack_trace_consume_fn consume_entry,
    void *cookie, unsigned long fp,
    unsigned long reg_ra)
    {
    struct stackframe buftail;
    let mut ra: c_ulong = 0;
    unsigned long __user *user_frame_tail =
    (unsigned long __user *)(fp - sizeof(struct stackframe));
// Check accessibility of one struct frame_tail beyond
    if (!access_ok(user_frame_tail, sizeof(buftail)))
    return 0;
    if (__copy_from_user_inatomic(&buftail, user_frame_tail,
    sizeof(buftail)))
    return 0;
    ra = reg_ra ? : buftail.ra;
    fp = buftail.fp;
    if (!ra || !consume_entry(cookie, ra))
    return 0;
    return fp;
    }
    void arch_stack_walk_user(stack_trace_consume_fn consume_entry, void *cookie,
    const struct pt_regs *regs)
    {
    let mut fp: c_ulong = 0;
    fp = regs.s0;
    if (!consume_entry(cookie, regs.epc))
    return;
    fp = unwind_user_frame(consume_entry, cookie, fp, regs.ra);
    while (fp && !(fp & 0x7))
    fp = unwind_user_frame(consume_entry, cookie, fp, 0);
    }
