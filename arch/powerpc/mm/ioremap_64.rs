//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/ioremap_64.c
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

    void __iomem *__ioremap_caller(phys_addr_t addr, unsigned long size,
    pgprot_t prot, void *caller)
    {
    phys_addr_t paligned, offset;
    void __iomem *ret;
    int err;
// We don't support the 4K PFN hack with ioremap
    if (pgprot_val(prot) & H_PAGE_4K_PFN)
    return core::ptr::null_mut();
//
// Choose an address to map it to. Once the vmalloc system is running,
// we use it. Before that, we map using addresses going up from
// ioremap_bot.  vmalloc will use the addresses from IOREMAP_BASE
// through ioremap_bot.
//
    paligned = addr & PAGE_MASK;
    offset = addr & ~PAGE_MASK;
    size = PAGE_ALIGN(addr + size) - paligned;
    if (size == 0 || paligned == 0)
    return core::ptr::null_mut();
    if (slab_is_available())
    return generic_ioremap_prot(addr, size, prot);
    pr_warn("ioremap() called early from %pS. Use early_ioremap() instead\n", caller);
    err = early_ioremap_range(ioremap_bot, paligned, size, prot);
    if (err)
    return core::ptr::null_mut();
    ret = (void __iomem *)ioremap_bot + offset;
    ioremap_bot += size + PAGE_SIZE;
    return ret;
    }
//
// Unmap an IO region and remove it from vmalloc'd list.
// Access to IO memory should be serialized by driver.
//
#[no_mangle]
pub unsafe extern "C" fn iounmap(token: *mut volatile void __iomem) {
    void iounmap(volatile void __iomem *token)
    {
    if (!slab_is_available())
    return;
    generic_iounmap(token);
    }
    EXPORT_SYMBOL(iounmap);
