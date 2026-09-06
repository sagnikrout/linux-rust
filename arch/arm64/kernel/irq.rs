//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/irq.c
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
// Based on arch/arm/kernel/irq.c
//
// Copyright (C) 1992 Linus Torvalds
// Modifications for ARM processor Copyright (C) 1995-2000 Russell King.
// Support for Dynamic Tick Timer Copyright (C) 2004-2005 Nokia Corporation.
// Dynamic Tick Timer written by Tony Lindgren <tony@atomide.com> and
// Tuukka Tikkanen <tuukka.tikkanen@elektrobit.com>.
// Copyright (C) 2012 ARM Ltd.
//

// Only access this in an NMI enter/exit
    DEFINE_PER_CPU(struct nmi_ctx, nmi_contexts);
    DEFINE_PER_CPU(unsigned long *, irq_stack_ptr);
    DECLARE_PER_CPU(unsigned long *, irq_shadow_call_stack_ptr);

    DEFINE_PER_CPU(unsigned long *, irq_shadow_call_stack_ptr);

#[no_mangle]
unsafe extern "C" fn init_irq_scs() -> int __init {
    static int __init init_irq_scs(void)
    {
    int cpu;
    void *s;
    if (!scs_is_enabled())
    return 0;
    for_each_possible_cpu(cpu) {
    s = scs_alloc(early_cpu_to_node(cpu));
    if (!s)
    return -ENOMEM;
    per_cpu(irq_shadow_call_stack_ptr, cpu) = s;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_irq_stacks() -> int __init {
    static int __init init_irq_stacks(void)
    {
    int cpu;
    unsigned long *p;
    for_each_possible_cpu(cpu) {
    p = arch_alloc_vmap_stack(IRQ_STACK_SIZE, early_cpu_to_node(cpu));
    if (!p)
    return -ENOMEM;
    per_cpu(irq_stack_ptr, cpu) = p;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ____do_softirq(regs: *mut pt_regs) {
    static void ____do_softirq(struct pt_regs *regs)
    {
    __do_softirq();
    }
#[no_mangle]
pub unsafe extern "C" fn do_softirq_own_stack() {
    void do_softirq_own_stack(void)
    {
    call_on_irq_stack(core::ptr::null_mut(), ____do_softirq);
    }

#[no_mangle]
unsafe extern "C" fn default_handle_irq(regs: *mut pt_regs) {
    static void default_handle_irq(struct pt_regs *regs)
    {
    panic("IRQ taken without a root IRQ handler\n");
    }
#[no_mangle]
unsafe extern "C" fn default_handle_fiq(regs: *mut pt_regs) {
    static void default_handle_fiq(struct pt_regs *regs)
    {
    panic("FIQ taken without a root FIQ handler\n");
    }
    void (*handle_arch_irq)(struct pt_regs *) __ro_after_init = default_handle_irq;
    void (*handle_arch_fiq)(struct pt_regs *) __ro_after_init = default_handle_fiq;
#[no_mangle]
pub unsafe extern "C" fn set_handle_irq(): *mut *mut void (handle_irq)(struct pt_regs) -> int __init {
    int __init set_handle_irq(void (*handle_irq)(struct pt_regs *))
    {
    if (handle_arch_irq != default_handle_irq)
    return -EBUSY;
    handle_arch_irq = handle_irq;
    pr_info("Root IRQ handler: %ps\n", handle_irq);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn set_handle_fiq(): *mut *mut void (handle_fiq)(struct pt_regs) -> int __init {
    int __init set_handle_fiq(void (*handle_fiq)(struct pt_regs *))
    {
    if (handle_arch_fiq != default_handle_fiq)
    return -EBUSY;
    handle_arch_fiq = handle_fiq;
    pr_info("Root FIQ handler: %ps\n", handle_fiq);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn init_IRQ() -> void __init {
    void __init init_IRQ(void)
    {
    if (init_irq_stacks() || init_irq_scs())
    panic("Failed to allocate IRQ stack resources\n");
    irqchip_init();
    if (system_uses_irq_prio_masking()) {
//
// Now that we have a stack for our IRQ handler, set
// the PMR/PSR pair to a consistent state.
//
    WARN_ON(read_sysreg(daif) & PSR_A_BIT);
    local_daif_restore(DAIF_PROCCTX_NOIRQ);
    }
    }
