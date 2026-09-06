//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/ioremap.c
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

    unsigned long ioremap_bot;
    EXPORT_SYMBOL(ioremap_bot);
    void __iomem *ioremap(phys_addr_t addr, unsigned long size)
    {
    let mut prot: pgprot_t = pgprot_noncached(PAGE_KERNEL);
    void *caller = __builtin_return_address(0);
    return __ioremap_caller(addr, size, prot, caller);
    }
    EXPORT_SYMBOL(ioremap);
    void __iomem *ioremap_wc(phys_addr_t addr, unsigned long size)
    {
    let mut prot: pgprot_t = pgprot_noncached_wc(PAGE_KERNEL);
    void *caller = __builtin_return_address(0);
    return __ioremap_caller(addr, size, prot, caller);
    }
    EXPORT_SYMBOL(ioremap_wc);
    void __iomem *ioremap_coherent(phys_addr_t addr, unsigned long size)
    {
    let mut prot: pgprot_t = pgprot_cached(PAGE_KERNEL);
    void *caller = __builtin_return_address(0);
    return __ioremap_caller(addr, size, prot, caller);
    }
    void __iomem *ioremap_prot(phys_addr_t addr, size_t size, pgprot_t prot)
    {
    let mut pte: pte_t = __pte(pgprot_val(prot));
    void *caller = __builtin_return_address(0);
// writeable implies dirty for kernel addresses
    if (pte_write(pte))
    pte = pte_mkdirty(pte);
    return __ioremap_caller(addr, size, pte_pgprot(pte), caller);
    }
    EXPORT_SYMBOL(ioremap_prot);
    int early_ioremap_range(unsigned long ea, phys_addr_t pa,
    unsigned long size, pgprot_t prot)
    {
    unsigned long i;
    for (i = 0; i < size; i += PAGE_SIZE) {
    let mut err: c_int = map_kernel_page(ea + i, pa + i, pgprot_nx(prot));
    if (WARN_ON_ONCE(err))  /* Should clean up */
    return err;
    }
    return 0;
    }
