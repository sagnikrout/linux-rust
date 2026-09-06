//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/irq_64.c
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
// This file contains the lowest level x86_64-specific interrupt
// entry and irq statistics code. All the remaining irq logic is
// done by the generic kernel/irq/ code and in the
// x86_64-specific irq controller code. (e.g. i8259.c and
// io_apic.c.)
//

    DEFINE_PER_CPU_CACHE_HOT(bool, hardirq_stack_inuse);
    DEFINE_PER_CPU_PAGE_ALIGNED(struct irq_stack, irq_stack_backing_store) __visible;

//
// VMAP the backing store with guard pages
//
#[no_mangle]
unsafe extern "C" fn map_irq_stack(cpu: c_uint) -> c_int {
    static int map_irq_stack(unsigned int cpu)
    {
    char *stack = (char *)per_cpu_ptr(&irq_stack_backing_store, cpu);
    struct page *pages[IRQ_STACK_SIZE / PAGE_SIZE];
    void *va;
    int i;
    for (i = 0; i < IRQ_STACK_SIZE / PAGE_SIZE; i++) {
    let mut pa: phys_addr_t = per_cpu_ptr_to_phys(stack + (i << PAGE_SHIFT));
    pages[i] = pfn_to_page(pa >> PAGE_SHIFT);
    }
    va = vmap(pages, IRQ_STACK_SIZE / PAGE_SIZE, VM_MAP, PAGE_KERNEL);
    if (!va)
    return -ENOMEM;
// Store actual TOS to avoid adjustment in the hotpath
    per_cpu(hardirq_stack_ptr, cpu) = va + IRQ_STACK_SIZE - 8;
    return 0;
    }

//
// If VMAP stacks are disabled due to KASAN, just use the per cpu
// backing store without guard pages.
//
#[no_mangle]
unsafe extern "C" fn map_irq_stack(cpu: c_uint) -> c_int {
    static int map_irq_stack(unsigned int cpu)
    {
    void *va = per_cpu_ptr(&irq_stack_backing_store, cpu);
// Store actual TOS to avoid adjustment in the hotpath
    per_cpu(hardirq_stack_ptr, cpu) = va + IRQ_STACK_SIZE - 8;
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn irq_init_percpu_irqstack(cpu: c_uint) -> c_int {
    int irq_init_percpu_irqstack(unsigned int cpu)
    {
    if (per_cpu(hardirq_stack_ptr, cpu))
    return 0;
    return map_irq_stack(cpu);
    }
