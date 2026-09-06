//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/machine_kexec_32.c
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
// handle transition of Linux booting another kernel
// Copyright (C) 2002-2005 Eric Biederman  <ebiederm@xmission.com>
//

#[no_mangle]
unsafe extern "C" fn load_segments() {
    static void load_segments(void)
    {

    __asm__ __volatile__ (
    "\tljmp $"STR(__KERNEL_CS)",$1f\n"
    "\t1:\n"
    "\tmovl $"STR(__KERNEL_DS)",%%eax\n"
    "\tmovl %%eax,%%ds\n"
    "\tmovl %%eax,%%es\n"
    "\tmovl %%eax,%%ss\n"
    : : : "eax", "memory");

    }
#[no_mangle]
unsafe extern "C" fn machine_kexec_free_page_tables(image: *mut kimage) {
    static void machine_kexec_free_page_tables(struct kimage *image)
    {
    free_pages((unsigned long)image.arch.pgd, pgd_allocation_order());
    image.arch.pgd = core::ptr::null_mut();

    free_page((unsigned long)image.arch.pmd0);
    image.arch.pmd0 = core::ptr::null_mut();
    free_page((unsigned long)image.arch.pmd1);
    image.arch.pmd1 = core::ptr::null_mut();

    free_page((unsigned long)image.arch.pte0);
    image.arch.pte0 = core::ptr::null_mut();
    free_page((unsigned long)image.arch.pte1);
    image.arch.pte1 = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn machine_kexec_alloc_page_tables(image: *mut kimage) -> c_int {
    static int machine_kexec_alloc_page_tables(struct kimage *image)
    {
    image.arch.pgd = (pgd_t *)__get_free_pages(GFP_KERNEL | __GFP_ZERO,
    pgd_allocation_order());

    image.arch.pmd0 = (pmd_t *)get_zeroed_page(GFP_KERNEL);
    image.arch.pmd1 = (pmd_t *)get_zeroed_page(GFP_KERNEL);

    image.arch.pte0 = (pte_t *)get_zeroed_page(GFP_KERNEL);
    image.arch.pte1 = (pte_t *)get_zeroed_page(GFP_KERNEL);
    if (!image.arch.pgd ||

    !image.arch.pmd0 || !image.arch.pmd1 ||

    !image.arch.pte0 || !image.arch.pte1) {
    return -ENOMEM;
    }
    return 0;
    }
    static void machine_kexec_page_table_set_one(
    pgd_t *pgd, pmd_t *pmd, pte_t *pte,
    unsigned long vaddr, unsigned long paddr)
    {
    p4d_t *p4d;
    pud_t *pud;
    pgd += pgd_index(vaddr);

    if (!(pgd_val(*pgd) & _PAGE_PRESENT))
    set_pgd(pgd, __pgd(__pa(pmd) | _PAGE_PRESENT));

    p4d = p4d_offset(pgd, vaddr);
    pud = pud_offset(p4d, vaddr);
    pmd = pmd_offset(pud, vaddr);
    if (!(pmd_val(*pmd) & _PAGE_PRESENT))
    set_pmd(pmd, __pmd(__pa(pte) | _PAGE_TABLE));
    pte = pte_offset_kernel(pmd, vaddr);
    set_pte(pte, pfn_pte(paddr >> PAGE_SHIFT, PAGE_KERNEL_EXEC));
    }
#[no_mangle]
unsafe extern "C" fn machine_kexec_prepare_page_tables(image: *mut kimage) {
    static void machine_kexec_prepare_page_tables(struct kimage *image)
    {
    void *control_page;
    pmd_t *pmd = core::ptr::null_mut();
    control_page = page_address(image.control_code_page);

    pmd = image.arch.pmd0;

    machine_kexec_page_table_set_one(
    image.arch.pgd, pmd, image.arch.pte0,
    (unsigned long)control_page, __pa(control_page));

    pmd = image.arch.pmd1;

    machine_kexec_page_table_set_one(
    image.arch.pgd, pmd, image.arch.pte1,
    __pa(control_page), __pa(control_page));
    }
//
// A architecture hook called to validate the
// proposed image and prepare the control pages
// as needed.  The pages for KEXEC_CONTROL_PAGE_SIZE
// have been allocated, but the segments have yet
// been copied into the kernel.
//
// Do what every setup is needed on image and the
// reboot code buffer to allow us to avoid allocations
// later.
//
// - Make control page executable.
// - Allocate page tables
// - Setup page tables
//
#[no_mangle]
pub unsafe extern "C" fn machine_kexec_prepare(image: *mut kimage) -> c_int {
    int machine_kexec_prepare(struct kimage *image)
    {
    int error;
    set_memory_x((unsigned long)page_address(image.control_code_page), 1);
    error = machine_kexec_alloc_page_tables(image);
    if (error)
    return error;
    machine_kexec_prepare_page_tables(image);
    return 0;
    }
//
// Undo anything leftover by machine_kexec_prepare
// when an image is freed.
//
#[no_mangle]
pub unsafe extern "C" fn machine_kexec_cleanup(image: *mut kimage) {
    void machine_kexec_cleanup(struct kimage *image)
    {
    set_memory_nx((unsigned long)page_address(image.control_code_page), 1);
    machine_kexec_free_page_tables(image);
    }
//
// Do not allocate memory (or fail in any way) in machine_kexec().
// We are past the point of no return, committed to rebooting now.
//
#[no_mangle]
pub unsafe extern "C" fn machine_kexec(image: *mut kimage) {
    void machine_kexec(struct kimage *image)
    {
    relocate_kernel_fn *relocate_kernel_ptr;
    unsigned long page_list[PAGES_NR];
    void *control_page;
    int save_ftrace_enabled;

    if (image.preserve_context)
    save_processor_state();

    save_ftrace_enabled = __ftrace_enabled_save();
// Interrupts aren't acceptable while we reboot
    local_irq_disable();
    hw_breakpoint_disable();
    if (image.preserve_context) {

//
// We need to put APICs in legacy mode so that we can
// get timer interrupts in second kernel. kexec/kdump
// paths already have calls to restore_boot_irq_mode()
// in one form or other. kexec jump path also need one.
//
    clear_IO_APIC();
    restore_boot_irq_mode();

    }
    control_page = page_address(image.control_code_page);
    memcpy(control_page, relocate_kernel, KEXEC_CONTROL_CODE_MAX_SIZE);
    relocate_kernel_ptr = control_page;
    page_list[PA_CONTROL_PAGE] = __pa(control_page);
    page_list[VA_CONTROL_PAGE] = (unsigned long)control_page;
    page_list[PA_PGD] = __pa(image.arch.pgd);
    if (image.type == KEXEC_TYPE_DEFAULT)
    page_list[PA_SWAP_PAGE] = (page_to_pfn(image.swap_page)
    << PAGE_SHIFT);
//
// The segment registers are funny things, they have both a
// visible and an invisible part.  Whenever the visible part is
// set to a specific selector, the invisible part is loaded
// with from a table in memory.  At no other time is the
// descriptor table in memory accessed.
//
// I take advantage of this here by force loading the
// segments, before I zap the gdt with an invalid value.
//
    load_segments();
//
// The gdt & idt are now invalid.
// If you want to load them you must set up your own idt & gdt.
//
    native_idt_invalidate();
    native_gdt_invalidate();
// now call it
    image.start = relocate_kernel_ptr((unsigned long)image.head,
    (unsigned long)page_list,
    image.start,
    boot_cpu_has(X86_FEATURE_PAE),
    image.preserve_context);

    if (image.preserve_context)
    restore_processor_state();

    __ftrace_enabled_restore(save_ftrace_enabled);
    }
