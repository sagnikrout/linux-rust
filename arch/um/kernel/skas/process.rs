//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/skas/process.c
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
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

    extern void start_kernel(void);
#[no_mangle]
unsafe extern "C" fn start_kernel_proc(unused: *mut c_void) -> int __init {
    static int __init start_kernel_proc(void *unused)
    {
    block_signals_trace();
    start_kernel();
    return 0;
    }
    char cpu_irqstacks[NR_CPUS][THREAD_SIZE] __aligned(THREAD_SIZE);
#[no_mangle]
pub unsafe extern "C" fn start_uml() -> int __init {
    int __init start_uml(void)
    {
    stack_protections((unsigned long) &cpu_irqstacks[0]);
    set_sigstack(cpu_irqstacks[0], THREAD_SIZE);
    init_new_thread_signals();
    init_task.thread.request.thread.proc = start_kernel_proc;
    init_task.thread.request.thread.arg = core::ptr::null_mut();
    return start_idle_thread(task_stack_page(&init_task),
    &init_task.thread.switch_buf);
    }
#[no_mangle]
pub unsafe extern "C" fn current_stub_stack() -> c_ulong {
    unsigned long current_stub_stack(void)
    {
    if (current.mm == core::ptr::null_mut())
    return 0;
    return current.mm.context.id.stack;
    }
    struct mm_id *current_mm_id(void)
    {
    if (current.mm == core::ptr::null_mut())
    return core::ptr::null_mut();
    return &current.mm.context.id;
    }
#[no_mangle]
pub unsafe extern "C" fn current_mm_sync() {
    void current_mm_sync(void)
    {
    if (current.mm == core::ptr::null_mut())
    return;
    um_tlb_sync(current.mm);
    }
    static DEFINE_SPINLOCK(initial_jmpbuf_spinlock);
#[no_mangle]
pub unsafe extern "C" fn initial_jmpbuf_lock() {
    void initial_jmpbuf_lock(void)
    {
    spin_lock_irq(&initial_jmpbuf_spinlock);
    }
#[no_mangle]
pub unsafe extern "C" fn initial_jmpbuf_unlock() {
    void initial_jmpbuf_unlock(void)
    {
    spin_unlock_irq(&initial_jmpbuf_spinlock);
    }
