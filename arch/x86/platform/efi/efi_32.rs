//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/efi/efi_32.c
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
// Extensible Firmware Interface
//
// Based on Extensible Firmware Interface Specification version 1.0
//
// Copyright (C) 1999 VA Linux Systems
// Copyright (C) 1999 Walt Drummond <drummond@valinux.com>
// Copyright (C) 1999-2002 Hewlett-Packard Co.
// David Mosberger-Tang <davidm@hpl.hp.com>
// Stephane Eranian <eranian@hpl.hp.com>
//
// All EFI Runtime Services are not implemented yet as EFI only
// supports physical mode addressing on SoftSDV. This is to be fixed
// in a future version.  --drummond 1999-07-20
//
// Implemented EFI runtime services and virtual mode calls.  --davidm
//
// Goutham Rao: <goutham.rao@intel.com>
// Skip non-WB memory and ignore empty memory ranges.
//

#[no_mangle]
pub unsafe extern "C" fn efi_map_region(md: *mut efi_memory_desc_t) -> void __init {
    void __init efi_map_region(efi_memory_desc_t *md)
    {
    u64 start_pfn, end_pfn, end;
    unsigned long size;
    void *va;
    start_pfn	= PFN_DOWN(md.phys_addr);
    size		= md.num_pages << PAGE_SHIFT;
    end		= md.phys_addr + size;
    end_pfn 	= PFN_UP(end);
    if (pfn_range_is_mapped(start_pfn, end_pfn)) {
    va = __va(md.phys_addr);
    if (!(md.attribute & EFI_MEMORY_WB))
    set_memory_uc((unsigned long)va, md.num_pages);
    } else {
    va = ioremap_cache(md.phys_addr, size);
    }
    md.virt_addr = (unsigned long)va;
    if (!va)
    pr_err("ioremap of 0x%llX failed!\n", md.phys_addr);
    }
//
// To make EFI call EFI runtime service in physical addressing mode we need
// prolog/epilog before/after the invocation to claim the EFI runtime service
// handler exclusively and to duplicate a memory mapping in low memory space,
// say 0 - 3G.
//
#[no_mangle]
pub unsafe extern "C" fn efi_alloc_page_tables() -> int __init {
    int __init efi_alloc_page_tables(void)
    {
    return 0;
    }
    void efi_sync_low_kernel_mappings(void) {}
#[no_mangle]
pub unsafe extern "C" fn efi_dump_pagetable() -> void __init {
    void __init efi_dump_pagetable(void)
    {

    ptdump_walk_pgd_level(core::ptr::null_mut(), &init_mm);

    }
#[no_mangle]
pub unsafe extern "C" fn efi_setup_page_tables(pa_memmap: c_ulong, num_pages: unsigned) -> int __init {
    int __init efi_setup_page_tables(unsigned long pa_memmap, unsigned num_pages)
    {
    return 0;
    }
    void __init efi_map_region_fixed(efi_memory_desc_t *md) {}
    void __init parse_efi_setup(u64 phys_addr, u32 data_len) {}
    efi_status_t efi_call_svam(efi_runtime_services_t * const *,
    u32, u32, u32, void *, u32);
    efi_status_t __init efi_set_virtual_address_map(unsigned long memory_map_size,
    unsigned long descriptor_size,
    u32 descriptor_version,
    efi_memory_desc_t *virtual_map,
    unsigned long systab_phys)
    {
    const efi_system_table_t *systab = (efi_system_table_t *)systab_phys;
    struct desc_ptr gdt_descr;
    efi_status_t status;
    unsigned long flags;
    pgd_t *save_pgd;
// Current pgd is swapper_pg_dir, we'll restore it later:
    save_pgd = swapper_pg_dir;
    load_cr3(initial_page_table);
    __flush_tlb_all();
    gdt_descr.address = get_cpu_gdt_paddr(0);
    gdt_descr.size = GDT_SIZE - 1;
    load_gdt(&gdt_descr);
// Disable interrupts around EFI calls:
    local_irq_save(flags);
    status = efi_call_svam(&systab.runtime,
    memory_map_size, descriptor_size,
    descriptor_version, virtual_map,
    __pa(&efi.runtime));
    local_irq_restore(flags);
    load_fixmap_gdt(0);
    load_cr3(save_pgd);
    __flush_tlb_all();
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_runtime_update_mappings() -> void __init {
    void __init efi_runtime_update_mappings(void)
    {
    if (__supported_pte_mask & _PAGE_NX) {
    efi_memory_desc_t *md;
// Make EFI runtime service code area executable
    for_each_efi_memory_desc(md) {
    if (md.type != EFI_RUNTIME_SERVICES_CODE)
    continue;
    set_memory_x(md.virt_addr, md.num_pages);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_efi_call_virt_setup() {
    void arch_efi_call_virt_setup(void)
    {
    efi_fpu_begin();
    firmware_restrict_branch_speculation_start();
    }
#[no_mangle]
pub unsafe extern "C" fn arch_efi_call_virt_teardown() {
    void arch_efi_call_virt_teardown(void)
    {
    firmware_restrict_branch_speculation_end();
    efi_fpu_end();
    }
