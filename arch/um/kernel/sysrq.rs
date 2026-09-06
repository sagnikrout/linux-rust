//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/sysrq.c
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
// Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
// Copyright (C) 2013 Richard Weinberger <richrd@nod.at>
//

#[no_mangle]
unsafe extern "C" fn _print_addr(data: *mut c_void, address: c_ulong, reliable: c_int) {
    static void _print_addr(void *data, unsigned long address, int reliable)
    {
    const char *loglvl = data;
    printk("%s [<%08lx>] %s%pS\n", loglvl, address, reliable ? "" : "? ",
    (void *)address);
    }
    static const struct stacktrace_ops stackops = {
    .address = _print_addr
    };
    void show_stack(struct task_struct *task, unsigned long *stack,
    const char *loglvl)
    {
    struct pt_regs *segv_regs = current.thread.segv_regs;
    int i;
    if (!stack)
    stack = get_stack_pointer(task, segv_regs);
    printk("%sStack:\n", loglvl);
    for (i = 0; i < 3 * STACKSLOTS_PER_LINE; i++) {
    if (kstack_end(stack))
    break;
    if (i && ((i % STACKSLOTS_PER_LINE) == 0))
    pr_cont("\n");
    pr_cont(" %08lx", READ_ONCE_NOCHECK(*stack));
    stack++;
    }
    printk("%sCall Trace:\n", loglvl);
    dump_trace(task ?: current, &stackops, (void *)loglvl);
    }
