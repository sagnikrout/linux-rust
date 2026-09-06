//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/book3s32/mmu_context.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// This file contains the routines for handling the MMU on those
// PowerPC implementations where the MMU substantially follows the
// architecture specification.  This includes the 6xx, 7xx, 7xxx,
// and 8260 implementations but excludes the 8xx and 4xx.
// -- paulus
//
// Derived from arch/ppc/mm/init.c:
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
// and Cort Dougan (PReP) (cort@cs.nmt.edu)
// Copyright (C) 1996 Paul Mackerras
//
// Derived from "arch/i386/mm/init.c"
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//

//
// Room for two PTE pointers, usually the kernel and current user pointers
// to their respective root page table.
//
    void *abatron_pteptrs[2];
//
// On 32-bit PowerPC 6xx/7xx/7xxx CPUs, we use a set of 16 VSIDs
// (virtual segment identifiers) for each context.  Although the
// hardware supports 24-bit VSIDs, and thus >1 million contexts,
// we only use 32,768 of them.  That is ample, since there can be
// at most around 30,000 tasks in the system anyway, and it means
// that we can use a bitmap to indicate which contexts are in use.
// Using a bitmap means that we entirely avoid all of the problems
// that we used to have when the context number overflowed,
// particularly on SMP systems.
// -- paulus.
//

pub const LAST_CONTEXT: c_int = 32767;
pub const FIRST_CONTEXT: c_int = 1;
    static unsigned long next_mmu_context;
    static unsigned long context_map[LAST_CONTEXT / BITS_PER_LONG + 1];
#[no_mangle]
pub unsafe extern "C" fn __init_new_context() -> c_ulong {
    unsigned long __init_new_context(void)
    {
    let mut ctx: c_ulong = next_mmu_context;
    while (test_and_set_bit(ctx, context_map)) {
    ctx = find_next_zero_bit(context_map, LAST_CONTEXT+1, ctx);
    if (ctx > LAST_CONTEXT)
    ctx = 0;
    }
    next_mmu_context = (ctx + 1) & LAST_CONTEXT;
    return ctx;
    }
    EXPORT_SYMBOL_GPL(__init_new_context);
//
// Set up the context for a new address space.
//
#[no_mangle]
pub unsafe extern "C" fn init_new_context(t: *mut task_struct, mm: *mut mm_struct) -> c_int {
    int init_new_context(struct task_struct *t, struct mm_struct *mm)
    {
    mm.context.id = __init_new_context();
    mm.context.sr0 = CTX_TO_VSID(mm.context.id, 0);
    if (IS_ENABLED(CONFIG_PPC_KUEP))
    mm.context.sr0 |= SR_NX;
    if (!kuap_is_disabled())
    mm.context.sr0 |= SR_KS;
    return 0;
    }
//
// Free a context ID. Make sure to call this with preempt disabled!
//
#[no_mangle]
pub unsafe extern "C" fn __destroy_context(ctx: c_ulong) {
    void __destroy_context(unsigned long ctx)
    {
    clear_bit(ctx, context_map);
    }
    EXPORT_SYMBOL_GPL(__destroy_context);
//
// We're finished using the context for an address space.
//
#[no_mangle]
pub unsafe extern "C" fn destroy_context(mm: *mut mm_struct) {
    void destroy_context(struct mm_struct *mm)
    {
    preempt_disable();
    if (mm.context.id != NO_CONTEXT) {
    __destroy_context(mm.context.id);
    mm.context.id = NO_CONTEXT;
    }
    preempt_enable();
    }
//
// Initialize the context management stuff.
//
#[no_mangle]
pub unsafe extern "C" fn mmu_context_init() -> void __init {
    void __init mmu_context_init(void)
    {
// Reserve context 0 for kernel use
    context_map[0] = (1 << FIRST_CONTEXT) - 1;
    next_mmu_context = FIRST_CONTEXT;
    }
#[no_mangle]
pub unsafe extern "C" fn switch_mmu_context(prev: *mut mm_struct, next: *mut mm_struct, tsk: *mut task_struct) {
    void switch_mmu_context(struct mm_struct *prev, struct mm_struct *next, struct task_struct *tsk)
    {
    let mut id: c_long = next.context.id;
    if (id < 0)
    panic("mm_struct %p has no context ID", next);
    isync();
    update_user_segments(next.context.sr0);
    if (IS_ENABLED(CONFIG_BDI_SWITCH))
    abatron_pteptrs[1] = next.pgd;
    if (!mmu_has_feature(MMU_FTR_HPTE_TABLE))
    mtspr(SPRN_SDR1, rol32(__pa(next.pgd), 4) & 0xffff01ff);
    mb();	/* sync */
    isync();
    }
    EXPORT_SYMBOL(switch_mmu_context);
