//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/head64.c
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
// prepare to run common code
//
// Copyright (C) 2000 Andrea Arcangeli <andrea@suse.de> SuSE
//
// cpu_feature_enabled() cannot be used this early
// Macro flag: #define USE_EARLY_PGTABLE_L5

//
// Manage page tables very early on.
//
    extern pmd_t early_dynamic_pgts[EARLY_DYNAMIC_PAGE_TABLES][PTRS_PER_PMD];
    unsigned int __initdata next_early_pgt;
    SYM_PIC_ALIAS(next_early_pgt);
    let mut early_pmd_flags: pmdval_t = __PAGE_KERNEL_LARGE & ~(_PAGE_GLOBAL | _PAGE_NX);
    unsigned int __pgtable_l5_enabled __ro_after_init;
    SYM_PIC_ALIAS(__pgtable_l5_enabled);
    let mut __ro_after_init: unsigned int pgdir_shift = 39;
    EXPORT_SYMBOL(pgdir_shift);
    SYM_PIC_ALIAS(pgdir_shift);
    let mut __ro_after_init: unsigned int ptrs_per_p4d = 1;
    EXPORT_SYMBOL(ptrs_per_p4d);
    SYM_PIC_ALIAS(ptrs_per_p4d);
    let mut __ro_after_init: unsigned long page_offset_base = __PAGE_OFFSET_BASE_L4;
    EXPORT_SYMBOL(page_offset_base);
    let mut __ro_after_init: unsigned long vmalloc_base = __VMALLOC_BASE_L4;
    EXPORT_SYMBOL(vmalloc_base);
    let mut __ro_after_init: unsigned long vmemmap_base = __VMEMMAP_BASE_L4;
    EXPORT_SYMBOL(vmemmap_base);
// Wipe all early page tables except for the kernel symbol map
#[no_mangle]
unsafe extern "C" fn reset_early_page_tables() -> void __init {
    static void __init reset_early_page_tables(void)
    {
    memset(early_top_pgt, 0, sizeof(pgd_t)*(PTRS_PER_PGD-1));
    next_early_pgt = 0;
    write_cr3(__sme_pa_nodebug(early_top_pgt));
    }
// Create a new PMD entry
#[no_mangle]
pub unsafe extern "C" fn __early_make_pgtable(address: c_ulong, pmd: pmdval_t) -> bool __init {
    bool __init __early_make_pgtable(unsigned long address, pmdval_t pmd)
    {
    let mut physaddr: c_ulong = address - __PAGE_OFFSET;
    pgdval_t pgd, *pgd_p;
    p4dval_t p4d, *p4d_p;
    pudval_t pud, *pud_p;
    pmdval_t *pmd_p;
// Invalid address or early pgt is done ?
    if (physaddr >= MAXMEM || read_cr3_pa() != __pa_nodebug(early_top_pgt))
    return false;
    again:
    pgd_p = &early_top_pgt[pgd_index(address)].pgd;
    pgd = *pgd_p;
//
// The use of __START_KERNEL_map rather than __PAGE_OFFSET here is
// critical -- __PAGE_OFFSET would point us back into the dynamic
// range and we might end up looping forever...
//
    if (!pgtable_l5_enabled())
    p4d_p = pgd_p;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: pgd) -> else {
    else if (pgd)
    p4d_p = (p4dval_t *)((pgd & PTE_PFN_MASK) + __START_KERNEL_map - phys_base);
    else {
    if (next_early_pgt >= EARLY_DYNAMIC_PAGE_TABLES) {
    reset_early_page_tables();
    goto again;
    }
    p4d_p = (p4dval_t *)early_dynamic_pgts[next_early_pgt++];
    memset(p4d_p, 0, sizeof(*p4d_p) * PTRS_PER_P4D);
// pgd_p = (pgdval_t)p4d_p - __START_KERNEL_map + phys_base + _KERNPG_TABLE;
    }
    p4d_p += p4d_index(address);
    p4d = *p4d_p;
    if (p4d)
    pud_p = (pudval_t *)((p4d & PTE_PFN_MASK) + __START_KERNEL_map - phys_base);
    else {
    if (next_early_pgt >= EARLY_DYNAMIC_PAGE_TABLES) {
    reset_early_page_tables();
    goto again;
    }
    pud_p = (pudval_t *)early_dynamic_pgts[next_early_pgt++];
    memset(pud_p, 0, sizeof(*pud_p) * PTRS_PER_PUD);
// p4d_p = (p4dval_t)pud_p - __START_KERNEL_map + phys_base + _KERNPG_TABLE;
    }
    pud_p += pud_index(address);
    pud = *pud_p;
    if (pud)
    pmd_p = (pmdval_t *)((pud & PTE_PFN_MASK) + __START_KERNEL_map - phys_base);
    else {
    if (next_early_pgt >= EARLY_DYNAMIC_PAGE_TABLES) {
    reset_early_page_tables();
    goto again;
    }
    pmd_p = (pmdval_t *)early_dynamic_pgts[next_early_pgt++];
    memset(pmd_p, 0, sizeof(*pmd_p) * PTRS_PER_PMD);
// pud_p = (pudval_t)pmd_p - __START_KERNEL_map + phys_base + _KERNPG_TABLE;
    }
    pmd_p[pmd_index(address)] = pmd;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn early_make_pgtable(address: c_ulong) -> bool __init {
    static bool __init early_make_pgtable(unsigned long address)
    {
    let mut physaddr: c_ulong = address - __PAGE_OFFSET;
    pmdval_t pmd;
    pmd = (physaddr & PMD_MASK) + early_pmd_flags;
    return __early_make_pgtable(address, pmd);
    }
#[no_mangle]
pub unsafe extern "C" fn do_early_exception(regs: *mut pt_regs, trapnr: c_int) -> void __init {
    void __init do_early_exception(struct pt_regs *regs, int trapnr)
    {
    if (trapnr == X86_TRAP_PF &&
    early_make_pgtable(native_read_cr2()))
    return;
    if (IS_ENABLED(CONFIG_AMD_MEM_ENCRYPT) &&
    trapnr == X86_TRAP_VC && handle_vc_boot_ghcb(regs))
    return;
    if (trapnr == X86_TRAP_VE && tdx_early_handle_ve(regs))
    return;
    early_fixup_exception(regs, trapnr);
    }
// Don't add a printk in there. printk relies on the PDA which is not initialized
    yet. */
#[no_mangle]
pub unsafe extern "C" fn clear_bss() -> void __init {
    void __init clear_bss(void)
    {
    memset(__bss_start, 0,
    (unsigned long) __bss_stop - (unsigned long) __bss_start);
    memset(__brk_base, 0,
    (unsigned long) __brk_limit - (unsigned long) __brk_base);
    }
#[no_mangle]
unsafe extern "C" fn get_cmd_line_ptr() -> c_ulong {
    static unsigned long get_cmd_line_ptr(void)
    {
    let mut cmd_line_ptr: c_ulong = boot_params.hdr.cmd_line_ptr;
    cmd_line_ptr |= (u64)boot_params.ext_cmd_line_ptr << 32;
    return cmd_line_ptr;
    }
#[no_mangle]
unsafe extern "C" fn copy_bootdata(real_mode_data: *mut c_char) -> void __init {
    static void __init copy_bootdata(char *real_mode_data)
    {
    char * command_line;
    unsigned long cmd_line_ptr;
//
// If SME is active, this will create decrypted mappings of the
// boot data in advance of the copy operations.
//
    sme_map_bootdata(real_mode_data);
    memcpy(&boot_params, real_mode_data, sizeof(boot_params));
    sanitize_boot_params(&boot_params);
    cmd_line_ptr = get_cmd_line_ptr();
    if (cmd_line_ptr) {
    command_line = __va(cmd_line_ptr);
    memcpy(boot_command_line, command_line, COMMAND_LINE_SIZE);
    }
//
// The old boot data is no longer needed and won't be reserved,
// freeing up that memory for use by the system. If SME is active,
// we need to remove the mappings that were created so that the
// memory doesn't remain mapped as decrypted.
//
    sme_unmap_bootdata(real_mode_data);
    }
#[no_mangle]
pub unsafe extern "C" fn x86_64_start_kernel(real_mode_data: *mut *mut c_char) -> asmlinkage __visible void __init __noreturn {
    asmlinkage __visible void __init __noreturn x86_64_start_kernel(char * real_mode_data)
    {
//
// Build-time sanity checks on the kernel image and module
// area mappings. (these are purely build-time and produce no code)
//
    BUILD_BUG_ON(MODULES_VADDR < __START_KERNEL_map);
    BUILD_BUG_ON(MODULES_VADDR - __START_KERNEL_map < KERNEL_IMAGE_SIZE);
    BUILD_BUG_ON(MODULES_LEN + KERNEL_IMAGE_SIZE > 2*PUD_SIZE);
    BUILD_BUG_ON((__START_KERNEL_map & ~PMD_MASK) != 0);
    BUILD_BUG_ON((MODULES_VADDR & ~PMD_MASK) != 0);
    BUILD_BUG_ON(!(MODULES_VADDR > __START_KERNEL));
    MAYBE_BUILD_BUG_ON(!(((MODULES_END - 1) & PGDIR_MASK) ==
    (__START_KERNEL & PGDIR_MASK)));
    BUILD_BUG_ON(__fix_to_virt(__end_of_fixed_addresses) <= MODULES_END);
    cr4_init_shadow();
// Kill off the identity-map trampoline
    reset_early_page_tables();
    if (pgtable_l5_enabled()) {
    page_offset_base	= __PAGE_OFFSET_BASE_L5;
    vmalloc_base		= __VMALLOC_BASE_L5;
    vmemmap_base		= __VMEMMAP_BASE_L5;
    }
    clear_bss();
//
// This needs to happen *before* kasan_early_init() because latter maps stuff
// into that page.
//
    clear_page(init_top_pgt);
//
// SME support may update early_pmd_flags to include the memory
// encryption mask, so it needs to be called before anything
// that may generate a page fault.
//
    sme_early_init();
    kasan_early_init();
//
// Flush global TLB entries which could be left over from the trampoline page
// table.
//
// This needs to happen *after* kasan_early_init() as KASAN-enabled .configs
// instrument native_write_cr4() so KASAN must be initialized for that
// instrumentation to work.
//
    __native_tlb_flush_global(this_cpu_read(cpu_tlbstate.cr4));
    idt_setup_early_handler();
// Needed before cc_platform_has() can be used for TDX
    tdx_early_init();
    copy_bootdata(__va(real_mode_data));
//
// Load microcode early on BSP.
//
    load_ucode_bsp();
// set init_top_pgt kernel high mapping
    init_top_pgt[511] = early_top_pgt[511];
    x86_64_start_reservations(real_mode_data);
    }
#[no_mangle]
pub unsafe extern "C" fn x86_64_start_reservations(real_mode_data: *mut c_char) -> void __init __noreturn {
    void __init __noreturn x86_64_start_reservations(char *real_mode_data)
    {
// version is always not zero if it is copied
    if (!boot_params.hdr.version)
    copy_bootdata(__va(real_mode_data));
    x86_early_init_platform_quirks();
    switch (boot_params.hdr.hardware_subarch) {
    case X86_SUBARCH_INTEL_MID:
    x86_intel_mid_early_setup();
    break;
    default:
    break;
    }
    start_kernel();
    }
#[no_mangle]
pub unsafe extern "C" fn early_setup_idt() {
    void early_setup_idt(void)
    {
    void *handler = core::ptr::null_mut();
    if (IS_ENABLED(CONFIG_AMD_MEM_ENCRYPT)) {
    setup_ghcb();
    handler = vc_boot_ghcb;
    }
    __pi_startup_64_load_idt(handler);
    }
