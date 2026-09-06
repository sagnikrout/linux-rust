//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/irq_32.c
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
// Copyright (C) 1992, 1998 Linus Torvalds, Ingo Molnar
//
// This file contains the lowest level x86-specific interrupt
// entry, irq-stacks and irq statistics code. All the remaining
// irq logic is done by the generic kernel/irq/ code and
// by the x86-specific irq controller code. (e.g. i8259.c and
// io_apic.c.)
//

    int sysctl_panic_on_stackoverflow __read_mostly;
// Debugging check for stack overflow: is there less than 1KB free?
#[no_mangle]
unsafe extern "C" fn check_stack_overflow() -> bool {
    static bool check_stack_overflow(void)
    {
    let mut sp: c_ulong = current_stack_pointer & (THREAD_SIZE - 1);
    return sp < (sizeof(struct thread_info) + STACK_WARN);
    }
#[no_mangle]
unsafe extern "C" fn print_stack_overflow() {
    static void print_stack_overflow(void)
    {
    printk(KERN_WARNING "low stack detected by irq handler\n");
    dump_stack();
    if (sysctl_panic_on_stackoverflow)
    panic("low stack detected by irq handler - check messages\n");
    }

    static inline bool check_stack_overflow(void) { return false; }
    static inline void print_stack_overflow(void) { }

    DEFINE_PER_CPU_CACHE_HOT(struct irq_stack *, softirq_stack_ptr);
#[no_mangle]
unsafe extern "C" fn call_on_stack(func: *mut c_void, stack: *mut c_void) {
    static void call_on_stack(void *func, void *stack)
    {
    asm volatile("xchgl %[sp], %%esp\n"
    CALL_NOSPEC
    "movl %[sp], %%esp"
    : [sp] "+b" (stack)
    : [thunk_target] "D" (func)
    : "memory", "cc", "edx", "ecx", "eax");
    }
    static inline void *current_stack(void)
    {
    return (void *)(current_stack_pointer & ~(THREAD_SIZE - 1));
    }
#[no_mangle]
pub unsafe extern "C" fn execute_on_irq_stack(overflow: bool, desc: *mut irq_desc) -> bool {
    static inline bool execute_on_irq_stack(bool overflow, struct irq_desc *desc)
    {
    struct irq_stack *curstk, *irqstk;
    u32 *isp, *prev_esp;
    curstk = (struct irq_stack *) current_stack();
    irqstk = __this_cpu_read(hardirq_stack_ptr);
//
// this is where we switch to the IRQ stack. However, if we are
// already using the IRQ stack (because we interrupted a hardirq
// handler) we can't do that and just have to keep using the
// current stack (which is the irq stack already after all)
//
    if (unlikely(curstk == irqstk))
    return false;
    isp = (u32 *) ((char *)irqstk + sizeof(*irqstk));
// Save the next esp at the bottom of the stack
    prev_esp = (u32 *)irqstk;
// prev_esp = current_stack_pointer;
    if (unlikely(overflow))
    call_on_stack(print_stack_overflow, isp);
    asm volatile("xchgl %[sp], %%esp\n"
    CALL_NOSPEC
    "movl %[sp], %%esp"
    : "+a" (desc), [sp] "+b" (isp)
    : [thunk_target] "D" (desc.handle_irq)
    : "memory", "cc", "edx", "ecx");
    return true;
    }
//
// Allocate per-cpu stacks for hardirq and softirq processing
//
#[no_mangle]
pub unsafe extern "C" fn irq_init_percpu_irqstack(cpu: c_uint) -> c_int {
    int irq_init_percpu_irqstack(unsigned int cpu)
    {
    let mut node: c_int = cpu_to_node(cpu);
    struct page *ph, *ps;
    if (per_cpu(hardirq_stack_ptr, cpu))
    return 0;
    ph = alloc_pages_node(node, THREADINFO_GFP, THREAD_SIZE_ORDER);
    if (!ph)
    return -ENOMEM;
    ps = alloc_pages_node(node, THREADINFO_GFP, THREAD_SIZE_ORDER);
    if (!ps) {
    __free_pages(ph, THREAD_SIZE_ORDER);
    return -ENOMEM;
    }
    per_cpu(hardirq_stack_ptr, cpu) = page_address(ph);
    per_cpu(softirq_stack_ptr, cpu) = page_address(ps);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn do_softirq_own_stack() {
    void do_softirq_own_stack(void)
    {
    struct irq_stack *irqstk;
    u32 *isp, *prev_esp;
    irqstk = __this_cpu_read(softirq_stack_ptr);
// build the stack frame on the softirq stack
    isp = (u32 *) ((char *)irqstk + sizeof(*irqstk));
// Push the previous esp onto the stack
    prev_esp = (u32 *)irqstk;
// prev_esp = current_stack_pointer;
    call_on_stack(__do_softirq, isp);
    }

#[no_mangle]
pub unsafe extern "C" fn __handle_irq(desc: *mut irq_desc, regs: *mut pt_regs) {
    void __handle_irq(struct irq_desc *desc, struct pt_regs *regs)
    {
    let mut overflow: bool = check_stack_overflow();
    if (user_mode(regs) || !execute_on_irq_stack(overflow, desc)) {
    if (unlikely(overflow))
    print_stack_overflow();
    generic_handle_irq_desc(desc);
    }
    }
