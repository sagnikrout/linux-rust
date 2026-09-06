//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/head32.c
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
// linux/arch/i386/kernel/head32.c -- prepare to run common code
//
// Copyright (C) 2000 Andrea Arcangeli <andrea@suse.de> SuSE
// Copyright (C) 2007 Eric Biederman <ebiederm@xmission.com>
//

#[no_mangle]
unsafe extern "C" fn i386_default_early_setup() -> void __init {
    static void __init i386_default_early_setup(void)
    {
// Initialize 32bit specific setup functions
    x86_init.resources.reserve_resources = i386_reserve_resources;
    x86_init.mpparse.setup_ioapic_ids = setup_ioapic_ids_from_mpc;
    }

    unsigned long __initdata initrd_start_early;
    static pte_t __initdata *initrd_pl2p_start, *initrd_pl2p_end;
#[no_mangle]
unsafe extern "C" fn zap_early_initrd_mapping() {
    static void zap_early_initrd_mapping(void)
    {
    pte_t *pl2p = initrd_pl2p_start;
    for (; pl2p < initrd_pl2p_end; pl2p++) {
// pl2p = (pte_t){ .pte = 0 };
    if (!IS_ENABLED(CONFIG_X86_PAE))
// (pl2p + ((PAGE_OFFSET >> PGDIR_SHIFT))) = (pte_t) {.pte = 0};
    }
    }

    static inline void zap_early_initrd_mapping(void) { }

#[no_mangle]
pub unsafe extern "C" fn i386_start_kernel() -> asmlinkage __visible void __init __noreturn {
    asmlinkage __visible void __init __noreturn i386_start_kernel(void)
    {
// Make sure IDT is set up before any exception happens
    idt_setup_early_handler();
    load_ucode_bsp();
    zap_early_initrd_mapping();
    cr4_init_shadow();
    sanitize_boot_params(&boot_params);
    x86_early_init_platform_quirks();
// Call the subarch specific early setup function
    switch (boot_params.hdr.hardware_subarch) {
    case X86_SUBARCH_INTEL_MID:
    x86_intel_mid_early_setup();
    break;
    case X86_SUBARCH_CE4100:
    x86_ce4100_early_setup();
    break;
    default:
    i386_default_early_setup();
    break;
    }
    start_kernel();
    }
//
// Initialize page tables.  This creates a PDE and a set of page
// tables, which are located immediately beyond __brk_base.  The variable
// _brk_end is set up to point to the first "safe" location.
// Mappings are created both at virtual address 0 (identity mapping)
// and PAGE_OFFSET for up to _end.
//
// In PAE mode initial_page_table is statically defined to contain
// enough entries to cover the VMSPLIT option (that is the top 1, 2 or 3
// entries). The identity mapping is handled by pointing two PGD entries
// to the first kernel PMD. Note the upper half of each PMD or PTE are
// always zero at this stage.
//

    typedef pmd_t			pl2_t;

    typedef pgd_t			pl2_t;

    static __init __no_stack_protector pte_t init_map(pte_t pte, pte_t **ptep, pl2_t **pl2p,
    const unsigned long limit)
    {
    while ((pte.pte & PTE_PFN_MASK) < limit) {
    let mut pl2: pl2_t = SET_PL2((unsigned long)*ptep | PDE_IDENT_ATTR);
    int i;
// pl2p = pl2;
    if (!IS_ENABLED(CONFIG_X86_PAE)) {
// Kernel PDE entry
// (*pl2p + ((PAGE_OFFSET >> PGDIR_SHIFT))) = pl2;
    }
    for (i = 0; i < PTRS_PER_PTE; i++) {
// ptep = pte;
    pte.pte += PAGE_SIZE;
    (*ptep)++;
    }
    (*pl2p)++;
    }
    return pte;
    }
#[no_mangle]
pub unsafe extern "C" fn mk_early_pgtbl_32() -> void __init __no_stack_protector {
    void __init __no_stack_protector mk_early_pgtbl_32(void)
    {
// Enough space to fit pagetables for the low memory linear map
    let mut limit: c_ulong = __pa_nodebug(_end) + (PAGE_TABLE_SIZE(LOWMEM_PAGES) << PAGE_SHIFT);
    pte_t pte, *ptep = (pte_t *)__pa_nodebug(__brk_base);
    struct boot_params __maybe_unused *params;
    pl2_t *pl2p = (pl2_t *)__pa_nodebug(pl2_base);
    unsigned long *ptr;
    pte.pte = PTE_IDENT_ATTR;
    pte = init_map(pte, &ptep, &pl2p, limit);
    ptr = (unsigned long *)__pa_nodebug(&max_pfn_mapped);
// Can't use pte_pfn() since it's a call with CONFIG_PARAVIRT
// ptr = (pte.pte & PTE_PFN_MASK) >> PAGE_SHIFT;
    ptr = (unsigned long *)__pa_nodebug(&_brk_end);
// ptr = (unsigned long)ptep + PAGE_OFFSET;

    params = (struct boot_params *)__pa_nodebug(&boot_params);
    if (!params.hdr.ramdisk_size || !params.hdr.ramdisk_image)
    return;
// Save the virtual start address
    ptr = (unsigned long *)__pa_nodebug(&initrd_start_early);
// ptr = (pte.pte & PTE_PFN_MASK) + PAGE_OFFSET;
// ptr += ((unsigned long)params->hdr.ramdisk_image) & ~PAGE_MASK;
// Save PLP2 for cleanup
    ptr = (unsigned long *)__pa_nodebug(&initrd_pl2p_start);
// ptr = (unsigned long)pl2p + PAGE_OFFSET;
    limit = (unsigned long)params.hdr.ramdisk_image;
    pte.pte = PTE_IDENT_ATTR | PFN_ALIGN(limit);
    limit = (unsigned long)params.hdr.ramdisk_image + params.hdr.ramdisk_size;
    init_map(pte, &ptep, &pl2p, limit);
    ptr = (unsigned long *)__pa_nodebug(&initrd_pl2p_end);
// ptr = (unsigned long)pl2p + PAGE_OFFSET;

    }
