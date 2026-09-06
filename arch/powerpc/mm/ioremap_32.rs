//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/ioremap_32.c
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

    void __iomem *ioremap_wt(phys_addr_t addr, unsigned long size)
    {
    let mut prot: pgprot_t = pgprot_cached_wthru(PAGE_KERNEL);
    return __ioremap_caller(addr, size, prot, __builtin_return_address(0));
    }
    EXPORT_SYMBOL(ioremap_wt);
    void __iomem *
    __ioremap_caller(phys_addr_t addr, unsigned long size, pgprot_t prot, void *caller)
    {
    unsigned long v;
    phys_addr_t p, offset;
    int err;
//
// If the address lies within the first 16 MB, assume it's in ISA
// memory space
//
    if (addr < SZ_16M)
    addr += _ISA_MEM_BASE;
//
// Choose an address to map it to.
// Once the vmalloc system is running, we use it.
// Before then, we use space going down from IOREMAP_TOP
// (ioremap_bot records where we're up to).
//
    p = addr & PAGE_MASK;
    offset = addr & ~PAGE_MASK;
    size = PAGE_ALIGN(addr + size) - p;

//
// Don't allow anybody to remap normal RAM that we're using.
// mem_init() sets high_memory so only do the check after that.
//
    if (slab_is_available() && p <= virt_to_phys(high_memory - 1) &&
    page_is_ram(__phys_to_pfn(p))) {
    pr_warn("%s(): phys addr 0x%llx is RAM lr %ps\n", __func__,
    (unsigned long long)p, __builtin_return_address(0));
    return core::ptr::null_mut();
    }

    if (size == 0)
    return core::ptr::null_mut();
//
// Is it already mapped?  Perhaps overlapped by a previous
// mapping.
//
    v = p_block_mapped(p);
    if (v)
    return (void __iomem *)v + offset;
    if (slab_is_available())
    return generic_ioremap_prot(addr, size, prot);
//
// Should check if it is a candidate for a BAT mapping
//
    pr_warn("ioremap() called early from %pS. Use early_ioremap() instead\n", caller);
    err = early_ioremap_range(ioremap_bot - size - PAGE_SIZE, p, size, prot);
    if (err)
    return core::ptr::null_mut();
    ioremap_bot -= size + PAGE_SIZE;
    return (void __iomem *)ioremap_bot + offset;
    }
#[no_mangle]
pub unsafe extern "C" fn iounmap(addr: *mut volatile void __iomem) {
    void iounmap(volatile void __iomem *addr)
    {
//
// If mapped by BATs then there is nothing to do.
// Calling vfree() generates a benign warning.
//
    if (v_block_mapped((unsigned long)addr))
    return;
    generic_iounmap(addr);
    }
    EXPORT_SYMBOL(iounmap);
