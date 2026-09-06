//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/stacktrace.c
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
// Copyright (C) 2013 Richard Weinberger <richard@nod.at>
// Copyright (C) 2014 Google Inc., Author: Daniel Walter <dwalter@google.com>
//

    void dump_trace(struct task_struct *tsk,
    const struct stacktrace_ops *ops,
    void *data)
    {
    let mut reliable: c_int = 0;
    unsigned long *sp, bp, addr;
    struct pt_regs *segv_regs = tsk.thread.segv_regs;
    struct stack_frame *frame;
    bp = get_frame_pointer(tsk, segv_regs);
    sp = get_stack_pointer(tsk, segv_regs);
    frame = (struct stack_frame *)bp;
    while (((long) sp & (THREAD_SIZE-1)) != 0) {
    addr = READ_ONCE_NOCHECK(*sp);
    if (__kernel_text_address(addr)) {
    reliable = 0;
    if ((unsigned long) sp == bp + sizeof(long)) {
    frame = frame ? frame.next_frame : core::ptr::null_mut();
    bp = (unsigned long)frame;
    reliable = 1;
    }
    ops.address(data, addr, reliable);
    }
    sp++;
    }
    }
#[no_mangle]
unsafe extern "C" fn save_addr(data: *mut c_void, address: c_ulong, reliable: c_int) {
    static void save_addr(void *data, unsigned long address, int reliable)
    {
    struct stack_trace *trace = data;
    if (!reliable)
    return;
    if (trace.nr_entries >= trace.max_entries)
    return;
    trace.entries[trace.nr_entries++] = address;
    }
    static const struct stacktrace_ops dump_ops = {
    .address = save_addr
    };
#[no_mangle]
unsafe extern "C" fn __save_stack_trace(tsk: *mut task_struct, trace: *mut stack_trace) {
    static void __save_stack_trace(struct task_struct *tsk, struct stack_trace *trace)
    {
    dump_trace(tsk, &dump_ops, trace);
    }
#[no_mangle]
pub unsafe extern "C" fn save_stack_trace(trace: *mut stack_trace) {
    void save_stack_trace(struct stack_trace *trace)
    {
    __save_stack_trace(current, trace);
    }
    EXPORT_SYMBOL_GPL(save_stack_trace);
#[no_mangle]
pub unsafe extern "C" fn save_stack_trace_tsk(tsk: *mut task_struct, trace: *mut stack_trace) {
    void save_stack_trace_tsk(struct task_struct *tsk, struct stack_trace *trace)
    {
    __save_stack_trace(tsk, trace);
    }
    EXPORT_SYMBOL_GPL(save_stack_trace_tsk);
