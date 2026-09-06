//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/pgprot.c
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

    static pgprot_t protection_map[16] __ro_after_init = {
    [VM_NONE]					= PAGE_NONE,
    [VM_READ]					= PAGE_READONLY,
    [VM_WRITE]					= PAGE_COPY,
    [VM_WRITE | VM_READ]				= PAGE_COPY,
    [VM_EXEC]					= PAGE_READONLY_EXEC,
    [VM_EXEC | VM_READ]				= PAGE_READONLY_EXEC,
    [VM_EXEC | VM_WRITE]				= PAGE_COPY_EXEC,
    [VM_EXEC | VM_WRITE | VM_READ]			= PAGE_COPY_EXEC,
    [VM_SHARED]					= PAGE_NONE,
    [VM_SHARED | VM_READ]				= PAGE_READONLY,
    [VM_SHARED | VM_WRITE]				= PAGE_SHARED,
    [VM_SHARED | VM_WRITE | VM_READ]		= PAGE_SHARED,
    [VM_SHARED | VM_EXEC]				= PAGE_READONLY_EXEC,
    [VM_SHARED | VM_EXEC | VM_READ]			= PAGE_READONLY_EXEC,
    [VM_SHARED | VM_EXEC | VM_WRITE]		= PAGE_SHARED_EXEC,
    [VM_SHARED | VM_EXEC | VM_WRITE | VM_READ]	= PAGE_SHARED_EXEC
    };
#[no_mangle]
pub unsafe extern "C" fn add_encrypt_protection_map() {
    void add_encrypt_protection_map(void)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(protection_map); i++)
    protection_map[i] = pgprot_encrypted(protection_map[i]);
    }
#[no_mangle]
pub unsafe extern "C" fn vm_get_page_prot(vm_flags: vm_flags_t) -> pgprot_t {
    pgprot_t vm_get_page_prot(vm_flags_t vm_flags)
    {
    unsigned long val = pgprot_val(protection_map[vm_flags &
    (VM_READ|VM_WRITE|VM_EXEC|VM_SHARED)]);

//
// Take the 4 protection key bits out of the vma->vm_flags value and
// turn them in to the bits that we can put in to a pte.
//
// Only override these if Protection Keys are available (which is only
// on 64-bit).
//
    if (vm_flags & VM_PKEY_BIT0)
    val |= _PAGE_PKEY_BIT0;
    if (vm_flags & VM_PKEY_BIT1)
    val |= _PAGE_PKEY_BIT1;
    if (vm_flags & VM_PKEY_BIT2)
    val |= _PAGE_PKEY_BIT2;
    if (vm_flags & VM_PKEY_BIT3)
    val |= _PAGE_PKEY_BIT3;

    val = __sme_set(val);
    if (val & _PAGE_PRESENT)
    val &= __supported_pte_mask;
    return __pgprot(val);
    }
    EXPORT_SYMBOL(vm_get_page_prot);
