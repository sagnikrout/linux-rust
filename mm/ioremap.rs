//! Automatically rewritten from C to Rust
//! Source: mm/ioremap.c
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
// Re-map IO memory to kernel address space so that we can access it.
// This is needed for high PCI addresses that aren't mapped in the
// 640k-1MB IO memory area on PC's
//
// (C) Copyright 1995 1996 Linus Torvalds
//

    void __iomem *generic_ioremap_prot(phys_addr_t phys_addr, size_t size,
    pgprot_t prot)
    {
    unsigned long offset, vaddr;
    phys_addr_t last_addr;
    struct vm_struct *area;
// An early platform driver might end up here
    if (WARN_ON_ONCE(!slab_is_available()))
    return core::ptr::null_mut();
// Disallow wrap-around or zero size
    last_addr = phys_addr + size - 1;
    if (!size || last_addr < phys_addr)
    return core::ptr::null_mut();
// Page-align mappings
    offset = phys_addr & (~PAGE_MASK);
    phys_addr -= offset;
    size = PAGE_ALIGN(size + offset);
    area = __get_vm_area_caller(size, VM_IOREMAP, IOREMAP_START,
    IOREMAP_END, __builtin_return_address(0));
    if (!area)
    return core::ptr::null_mut();
    vaddr = (unsigned long)area.addr;
    area.phys_addr = phys_addr;
    if (ioremap_page_range(vaddr, vaddr + size, phys_addr, prot)) {
    free_vm_area(area);
    return core::ptr::null_mut();
    }
    return (void __iomem *)(vaddr + offset);
    }

    void __iomem *ioremap_prot(phys_addr_t phys_addr, size_t size,
    pgprot_t prot)
    {
    return generic_ioremap_prot(phys_addr, size, prot);
    }
    EXPORT_SYMBOL(ioremap_prot);

#[no_mangle]
pub unsafe extern "C" fn generic_iounmap(addr: *mut volatile void __iomem) {
    void generic_iounmap(volatile void __iomem *addr)
    {
    void *vaddr = (void *)((unsigned long)addr & PAGE_MASK);
    if (is_ioremap_addr(vaddr))
    vunmap(vaddr);
    }

#[no_mangle]
pub unsafe extern "C" fn iounmap(addr: *mut volatile void __iomem) {
    void iounmap(volatile void __iomem *addr)
    {
    generic_iounmap(addr);
    }
    EXPORT_SYMBOL(iounmap);
