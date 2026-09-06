//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/efi.c
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
// Copyright (C) 2020 Western Digital Corporation or its affiliates.
// Adapted from arch/arm64/kernel/efi.c
//

//
// Only regions of type EFI_RUNTIME_SERVICES_CODE need to be
// executable, everything else can be mapped with the XN bits
// set. Also take the new (optional) RO/XP bits into account.
//
#[no_mangle]
unsafe extern "C" fn efimem_to_pgprot_map(md: *mut efi_memory_desc_t) -> __init pgprot_t {
    static __init pgprot_t efimem_to_pgprot_map(efi_memory_desc_t *md)
    {
    let mut attr: u64 = md.attribute;
    let mut type: u32 = md.type;
    if (type == EFI_MEMORY_MAPPED_IO)
    return PAGE_KERNEL;
// R--
    if ((attr & (EFI_MEMORY_XP | EFI_MEMORY_RO)) ==
    (EFI_MEMORY_XP | EFI_MEMORY_RO))
    return PAGE_KERNEL_READ;
// R-X
    if (attr & EFI_MEMORY_RO)
    return PAGE_KERNEL_READ_EXEC;
// RW-
    if (((attr & (EFI_MEMORY_RP | EFI_MEMORY_WP | EFI_MEMORY_XP)) ==
    EFI_MEMORY_XP) ||
    type != EFI_RUNTIME_SERVICES_CODE)
    return PAGE_KERNEL;
// RWX
    return PAGE_KERNEL_EXEC;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_create_mapping(mm: *mut mm_struct, md: *mut efi_memory_desc_t) -> int __init {
    int __init efi_create_mapping(struct mm_struct *mm, efi_memory_desc_t *md)
    {
    pgprot_t prot = __pgprot(pgprot_val(efimem_to_pgprot_map(md)) &
    ~(_PAGE_GLOBAL));
    int i;
// RISC-V maps one page at a time
    for (i = 0; i < md.num_pages; i++)
    create_pgd_mapping(mm.pgd, md.virt_addr + i * PAGE_SIZE,
    md.phys_addr + i * PAGE_SIZE,
    PAGE_SIZE, prot);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_permissions(ptep: *mut pte_t, addr: c_ulong, data: *mut c_void) -> int __init {
    static int __init set_permissions(pte_t *ptep, unsigned long addr, void *data)
    {
    efi_memory_desc_t *md = data;
    let mut pte: pte_t = ptep_get(ptep);
    unsigned long val;
    if (md.attribute & EFI_MEMORY_RO) {
    val = pte_val(pte) & ~_PAGE_WRITE;
    val |= _PAGE_READ;
    pte = __pte(val);
    }
    if (md.attribute & EFI_MEMORY_XP) {
    val = pte_val(pte) & ~_PAGE_EXEC;
    pte = __pte(val);
    }
    set_pte(ptep, pte);
    return 0;
    }
    int __init efi_set_mapping_permissions(struct mm_struct *mm,
    efi_memory_desc_t *md,
    bool ignored)
    {
    BUG_ON(md.type != EFI_RUNTIME_SERVICES_CODE &&
    md.type != EFI_RUNTIME_SERVICES_DATA);
//
// Calling apply_to_page_range() is only safe on regions that are
// guaranteed to be mapped down to pages. Since we are only called
// for regions that have been mapped using efi_create_mapping() above
// (and this is checked by the generic Memory Attributes table parsing
// routines), there is no need to check that again here.
//
    return apply_to_page_range(mm, md.virt_addr,
    md.num_pages << EFI_PAGE_SHIFT,
    set_permissions, md);
    }
#[no_mangle]
pub unsafe extern "C" fn efi_poweroff_required() -> bool {
    bool efi_poweroff_required(void)
    {
    return efi_enabled(EFI_RUNTIME_SERVICES);
    }
