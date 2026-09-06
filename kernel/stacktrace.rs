//! Automatically rewritten from C to Rust
//! Source: kernel/stacktrace.c
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

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }



// SPDX-License-Identifier: GPL-2.0-only
//
// kernel/stacktrace.c
//
// Stack trace management functions
//
// Copyright (C) 2006 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
//

//
// stack_trace_print - Print the entries in the stack trace
// @entries:	Pointer to storage array
// @nr_entries:	Number of entries in the storage array
// @spaces:	Number of leading spaces to print
//
    void stack_trace_print(const unsigned long *entries, unsigned int nr_entries,
    int spaces)
    {
    unsigned int i;
    if (WARN_ON(!entries))
    return;
    for (i = 0; i < nr_entries; i++)
    printk("%*c%pS\n", 1 + spaces, ' ', (void *)entries[i]);
    }
    EXPORT_SYMBOL_GPL(stack_trace_print);
//
// stack_trace_snprint - Print the entries in the stack trace into a buffer
// @buf:	Pointer to the print buffer
// @size:	Size of the print buffer
// @entries:	Pointer to storage array
// @nr_entries:	Number of entries in the storage array
// @spaces:	Number of leading spaces to print
//
// Return: Number of bytes printed.
//
    int stack_trace_snprint(char *buf, size_t size, const unsigned long *entries,
    unsigned int nr_entries, int spaces)
    {
    unsigned int generated, i, total = 0;
    if (WARN_ON(!entries))
    return 0;
    for (i = 0; i < nr_entries && size; i++) {
    generated = snprintf(buf, size, "%*c%pS\n", 1 + spaces, ' ',
    (void *)entries[i]);
    total += generated;
    if (generated >= size) {
    buf += size;
    size = 0;
    } else {
    buf += generated;
    size -= generated;
    }
    }
    return total;
    }
    EXPORT_SYMBOL_GPL(stack_trace_snprint);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stacktrace_cookie {
    pub store: *mut c_ulong,
    pub size: c_uint,
    pub skip: c_uint,
    pub len: c_uint,
}

#[no_mangle]
unsafe extern "C" fn stack_trace_consume_entry(cookie: *mut c_void, addr: c_ulong) -> bool {
    struct stacktrace_cookie *c = cookie;
    if (c.len >= c.size)
    return false;
    if (c.skip > 0) {
    c.skip--;
    return true;
    }
    c.store[c.len++] = addr;
    return c.len < c.size;
    }
#[no_mangle]
unsafe extern "C" fn stack_trace_consume_entry_nosched(cookie: *mut c_void, addr: c_ulong) -> bool {
    if (in_sched_functions(addr))
    return true;
    return stack_trace_consume_entry(cookie, addr);
    }
//
// stack_trace_save - Save a stack trace into a storage array
// @store:	Pointer to storage array
// @size:	Size of the storage array
// @skipnr:	Number of entries to skip at the start of the stack trace
//
// Return: Number of trace entries stored.
//
    unsigned int stack_trace_save(unsigned long *store, unsigned int size,
    unsigned int skipnr)
    {
    let mut consume_entry: stack_trace_consume_fn = stack_trace_consume_entry;
    struct stacktrace_cookie c = {
    .store	= store,
    .size	= size,
    .skip	= skipnr + 1,
    };
    arch_stack_walk(consume_entry, &c, current, core::ptr::null_mut());
    return c.len;
    }
    EXPORT_SYMBOL_GPL(stack_trace_save);
//
// stack_trace_save_tsk - Save a task stack trace into a storage array
// @tsk:	The task to examine
// @store:	Pointer to storage array
// @size:	Size of the storage array
// @skipnr:	Number of entries to skip at the start of the stack trace
//
// Return: Number of trace entries stored.
//
    unsigned int stack_trace_save_tsk(struct task_struct *tsk, unsigned long *store,
    unsigned int size, unsigned int skipnr)
    {
    let mut consume_entry: stack_trace_consume_fn = stack_trace_consume_entry_nosched;
    struct stacktrace_cookie c = {
    .store	= store,
    .size	= size,
// skip this function if they are tracing us
    .skip	= skipnr + (current == tsk),
    };
    if (!try_get_task_stack(tsk))
    return 0;
    arch_stack_walk(consume_entry, &c, tsk, core::ptr::null_mut());
    put_task_stack(tsk);
    return c.len;
    }
    EXPORT_SYMBOL_GPL(stack_trace_save_tsk);
//
// stack_trace_save_regs - Save a stack trace based on pt_regs into a storage array
// @regs:	Pointer to pt_regs to examine
// @store:	Pointer to storage array
// @size:	Size of the storage array
// @skipnr:	Number of entries to skip at the start of the stack trace
//
// Return: Number of trace entries stored.
//
    unsigned int stack_trace_save_regs(struct pt_regs *regs, unsigned long *store,
    unsigned int size, unsigned int skipnr)
    {
    let mut consume_entry: stack_trace_consume_fn = stack_trace_consume_entry;
    struct stacktrace_cookie c = {
    .store	= store,
    .size	= size,
    .skip	= skipnr,
    };
    arch_stack_walk(consume_entry, &c, current, regs);
    return c.len;
    }

//
// stack_trace_save_tsk_reliable - Save task stack with verification
// @tsk:	Pointer to the task to examine
// @store:	Pointer to storage array
// @size:	Size of the storage array
//
// Return:	An error if it detects any unreliable features of the
// stack. Otherwise it guarantees that the stack trace is
// reliable and returns the number of entries stored.
//
// If the task is not 'current', the caller *must* ensure the task is inactive.
//
    int stack_trace_save_tsk_reliable(struct task_struct *tsk, unsigned long *store,
    unsigned int size)
    {
    let mut consume_entry: stack_trace_consume_fn = stack_trace_consume_entry;
    struct stacktrace_cookie c = {
    .store	= store,
    .size	= size,
    };
    int ret;
//
// If the task doesn't have a stack (e.g., a zombie), the stack is
// "reliably" empty.
//
    if (!try_get_task_stack(tsk))
    return 0;
    ret = arch_stack_walk_reliable(consume_entry, &c, tsk);
    put_task_stack(tsk);
    return ret ? ret : c.len;
    }

//
// stack_trace_save_user - Save a user space stack trace into a storage array
// @store:	Pointer to storage array
// @size:	Size of the storage array
//
// Return: Number of trace entries stored.
//
#[no_mangle]
pub unsafe extern "C" fn stack_trace_save_user(store: *mut c_ulong, size: c_uint) -> c_uint {
    let mut consume_entry: stack_trace_consume_fn = stack_trace_consume_entry;
    struct stacktrace_cookie c = {
    .store	= store,
    .size	= size,
    };
// Trace user stack if not a kernel thread
    if (current.flags & PF_KTHREAD)
    return 0;
    arch_stack_walk_user(consume_entry, &c, task_pt_regs(current));
    return c.len;
    }

//
// Architectures that do not implement save_stack_trace_*()
// get these weak aliases and once-per-bootup warnings
// (whenever this facility is utilized - for example by procfs):
//
    __weak void
    save_stack_trace_tsk(struct task_struct *tsk, struct stack_trace *trace)
    {
    WARN_ONCE(1, KERN_INFO "save_stack_trace_tsk() not implemented yet.\n");
    }
    __weak void
    save_stack_trace_regs(struct pt_regs *regs, struct stack_trace *trace)
    {
    WARN_ONCE(1, KERN_INFO "save_stack_trace_regs() not implemented yet.\n");
    }
//
// stack_trace_save - Save a stack trace into a storage array
// @store:	Pointer to storage array
// @size:	Size of the storage array
// @skipnr:	Number of entries to skip at the start of the stack trace
//
// Return: Number of trace entries stored
//
    unsigned int stack_trace_save(unsigned long *store, unsigned int size,
    unsigned int skipnr)
    {
    struct stack_trace trace = {
    .entries	= store,
    .max_entries	= size,
    .skip		= skipnr + 1,
    };
    save_stack_trace(&trace);
    return trace.nr_entries;
    }
    EXPORT_SYMBOL_GPL(stack_trace_save);
//
// stack_trace_save_tsk - Save a task stack trace into a storage array
// @task:	The task to examine
// @store:	Pointer to storage array
// @size:	Size of the storage array
// @skipnr:	Number of entries to skip at the start of the stack trace
//
// Return: Number of trace entries stored
//
    unsigned int stack_trace_save_tsk(struct task_struct *task,
    unsigned long *store, unsigned int size,
    unsigned int skipnr)
    {
    struct stack_trace trace = {
    .entries	= store,
    .max_entries	= size,
// skip this function if they are tracing us
    .skip	= skipnr + (current == task),
    };
    save_stack_trace_tsk(task, &trace);
    return trace.nr_entries;
    }
    EXPORT_SYMBOL_GPL(stack_trace_save_tsk);
//
// stack_trace_save_regs - Save a stack trace based on pt_regs into a storage array
// @regs:	Pointer to pt_regs to examine
// @store:	Pointer to storage array
// @size:	Size of the storage array
// @skipnr:	Number of entries to skip at the start of the stack trace
//
// Return: Number of trace entries stored
//
    unsigned int stack_trace_save_regs(struct pt_regs *regs, unsigned long *store,
    unsigned int size, unsigned int skipnr)
    {
    struct stack_trace trace = {
    .entries	= store,
    .max_entries	= size,
    .skip		= skipnr,
    };
    save_stack_trace_regs(regs, &trace);
    return trace.nr_entries;
    }

//
// stack_trace_save_tsk_reliable - Save task stack with verification
// @tsk:	Pointer to the task to examine
// @store:	Pointer to storage array
// @size:	Size of the storage array
//
// Return:	An error if it detects any unreliable features of the
// stack. Otherwise it guarantees that the stack trace is
// reliable and returns the number of entries stored.
//
// If the task is not 'current', the caller *must* ensure the task is inactive.
//
    int stack_trace_save_tsk_reliable(struct task_struct *tsk, unsigned long *store,
    unsigned int size)
    {
    struct stack_trace trace = {
    .entries	= store,
    .max_entries	= size,
    };
    let mut ret: c_int = save_stack_trace_tsk_reliable(tsk, &trace);
    return ret ? ret : trace.nr_entries;
    }

//
// stack_trace_save_user - Save a user space stack trace into a storage array
// @store:	Pointer to storage array
// @size:	Size of the storage array
//
// Return: Number of trace entries stored
//
#[no_mangle]
pub unsafe extern "C" fn stack_trace_save_user(store: *mut c_ulong, size: c_uint) -> c_uint {
    struct stack_trace trace = {
    .entries	= store,
    .max_entries	= size,
    };
    save_stack_trace_user(&trace);
    return trace.nr_entries;
    }

#[no_mangle]
pub unsafe extern "C" fn in_irqentry_text(ptr: c_ulong) -> bool {
    return (ptr >= (unsigned long)&__irqentry_text_start &&
    ptr < (unsigned long)&__irqentry_text_end) ||
    (ptr >= (unsigned long)&__softirqentry_text_start &&
    ptr < (unsigned long)&__softirqentry_text_end);
    }
//
// filter_irq_stacks - Find first IRQ stack entry in trace
// @entries:	Pointer to stack trace array
// @nr_entries:	Number of entries in the storage array
//
// Return: Number of trace entries until IRQ stack starts.
//
#[no_mangle]
pub unsafe extern "C" fn filter_irq_stacks(entries: *mut c_ulong, nr_entries: c_uint) -> c_uint {
    unsigned int i;
    for (i = 0; i < nr_entries; i++) {
    if (in_irqentry_text(entries[i])) {
// Include the irqentry function into the stack.
    return i + 1;
    }
    }
    return nr_entries;
    }
    EXPORT_SYMBOL_GPL(filter_irq_stacks);
