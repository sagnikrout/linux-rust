//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/iomap_32.c
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
//
// Copyright © 2008 Ingo Molnar
//

#[no_mangle]
unsafe extern "C" fn is_io_mapping_possible(base: resource_size_t, size: c_ulong) -> c_int {
    static int is_io_mapping_possible(resource_size_t base, unsigned long size)
    {

// There is no way to map greater than 1 << 32 address without PAE
    if (base + size > 0x100000000ULL)
    return 0;

    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn iomap_create_wc(base: resource_size_t, size: c_ulong, prot: *mut pgprot_t) -> c_int {
    int iomap_create_wc(resource_size_t base, unsigned long size, pgprot_t *prot)
    {
    let mut pcm: enum page_cache_mode = _PAGE_CACHE_MODE_WC;
    int ret;
    if (!is_io_mapping_possible(base, size))
    return -EINVAL;
    ret = memtype_reserve_io(base, base + size, &pcm);
    if (ret)
    return ret;
// prot = __pgprot(__PAGE_KERNEL | cachemode2protval(pcm));
// Filter out unsupported __PAGE_KERNEL* bits:
    pgprot_val(*prot) &= __default_kernel_pte_mask;
    return 0;
    }
    EXPORT_SYMBOL_GPL(iomap_create_wc);
#[no_mangle]
pub unsafe extern "C" fn iomap_free(base: resource_size_t, size: c_ulong) {
    void iomap_free(resource_size_t base, unsigned long size)
    {
    memtype_free_io(base, base + size);
    }
    EXPORT_SYMBOL_GPL(iomap_free);
    void __iomem *__iomap_local_pfn_prot(unsigned long pfn, pgprot_t prot)
    {
//
// For non-PAT systems, translate non-WB request to UC- just in
// case the caller set the PWT bit to prot directly without using
// pgprot_writecombine(). UC- translates to uncached if the MTRR
// is UC or WC. UC- gets the real intention, of the user, which is
// "WC if the MTRR is WC, UC if you can't do that."
//
    if (!pat_enabled() && pgprot2cachemode(prot) != _PAGE_CACHE_MODE_WB)
    prot = __pgprot(__PAGE_KERNEL |
    cachemode2protval(_PAGE_CACHE_MODE_UC_MINUS));
// Filter out unsupported __PAGE_KERNEL* bits:
    pgprot_val(prot) &= __default_kernel_pte_mask;
    return (void  __iomem *)__kmap_local_pfn_prot(pfn, prot);
    }
    EXPORT_SYMBOL_GPL(__iomap_local_pfn_prot);
