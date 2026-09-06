//! Automatically rewritten from C to Rust
//! Source: arch/x86/power/hibernate_64.c
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
// Hibernation support for x86-64
//
// Copyright (c) 2007 Rafael J. Wysocki <rjw@sisk.pl>
// Copyright (c) 2002 Pavel Machek <pavel@ucw.cz>
// Copyright (c) 2001 Patrick Mochel <mochel@osdl.org>
//

#[no_mangle]
unsafe extern "C" fn set_up_temporary_text_mapping(pgd: *mut pgd_t) -> c_int {
    static int set_up_temporary_text_mapping(pgd_t *pgd)
    {
    pmd_t *pmd;
    pud_t *pud;
    p4d_t *p4d = core::ptr::null_mut();
    let mut pgtable_prot: pgprot_t = __pgprot(_KERNPG_TABLE);
    let mut pmd_text_prot: pgprot_t = __pgprot(__PAGE_KERNEL_LARGE_EXEC);
// Filter out unsupported __PAGE_KERNEL* bits:
    pgprot_val(pmd_text_prot) &= __default_kernel_pte_mask;
    pgprot_val(pgtable_prot)  &= __default_kernel_pte_mask;
//
// The new mapping only has to cover the page containing the image
// kernel's entry point (jump_address_phys), because the switch over to
// it is carried out by relocated code running from a page allocated
// specifically for this purpose and covered by the identity mapping, so
// the temporary kernel text mapping is only needed for the final jump.
// Moreover, in that mapping the virtual address of the image kernel's
// entry point must be the same as its virtual address in the image
// kernel (restore_jump_address), so the image kernel's
// restore_registers() code doesn't find itself in a different area of
// the virtual address space after switching over to the original page
// tables used by the image kernel.
//
    if (pgtable_l5_enabled()) {
    p4d = (p4d_t *)get_safe_page(GFP_ATOMIC);
    if (!p4d)
    return -ENOMEM;
    }
    pud = (pud_t *)get_safe_page(GFP_ATOMIC);
    if (!pud)
    return -ENOMEM;
    pmd = (pmd_t *)get_safe_page(GFP_ATOMIC);
    if (!pmd)
    return -ENOMEM;
    set_pmd(pmd + pmd_index(restore_jump_address),
    __pmd((jump_address_phys & PMD_MASK) | pgprot_val(pmd_text_prot)));
    set_pud(pud + pud_index(restore_jump_address),
    __pud(__pa(pmd) | pgprot_val(pgtable_prot)));
    if (p4d) {
    let mut new_p4d: p4d_t = __p4d(__pa(pud) | pgprot_val(pgtable_prot));
    let mut new_pgd: pgd_t = __pgd(__pa(p4d) | pgprot_val(pgtable_prot));
    set_p4d(p4d + p4d_index(restore_jump_address), new_p4d);
    set_pgd(pgd + pgd_index(restore_jump_address), new_pgd);
    } else {
// No p4d for 4-level paging: point the pgd to the pud page table
    let mut new_pgd: pgd_t = __pgd(__pa(pud) | pgprot_val(pgtable_prot));
    set_pgd(pgd + pgd_index(restore_jump_address), new_pgd);
    }
    return 0;
    }
    static void *alloc_pgt_page(void *context)
    {
    return (void *)get_safe_page(GFP_ATOMIC);
    }
#[no_mangle]
unsafe extern "C" fn set_up_temporary_mappings() -> c_int {
    static int set_up_temporary_mappings(void)
    {
    struct x86_mapping_info info = {
    .alloc_pgt_page	= alloc_pgt_page,
    .page_flag	= __PAGE_KERNEL_LARGE_EXEC,
    .offset		= __PAGE_OFFSET,
    };
    unsigned long mstart, mend;
    pgd_t *pgd;
    int result;
    int i;
    pgd = (pgd_t *)get_safe_page(GFP_ATOMIC);
    if (!pgd)
    return -ENOMEM;
// Prepare a temporary mapping for the kernel text
    result = set_up_temporary_text_mapping(pgd);
    if (result)
    return result;
// Set up the direct mapping from scratch
    for (i = 0; i < nr_pfn_mapped; i++) {
    mstart = pfn_mapped[i].start << PAGE_SHIFT;
    mend   = pfn_mapped[i].end << PAGE_SHIFT;
    result = kernel_ident_mapping_init(&info, pgd, mstart, mend);
    if (result)
    return result;
    }
    temp_pgt = __pa(pgd);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn swsusp_arch_resume() -> asmlinkage int {
    asmlinkage int swsusp_arch_resume(void)
    {
    int error;
// We have got enough memory and from now on we cannot recover
    error = set_up_temporary_mappings();
    if (error)
    return error;
    error = relocate_restore_code();
    if (error)
    return error;
    restore_image();
    return 0;
    }
