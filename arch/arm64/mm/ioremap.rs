//! Automatically rewritten from C to Rust
//! Source: arch/arm64/mm/ioremap.c
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

    static ioremap_prot_hook_t ioremap_prot_hook;
#[no_mangle]
pub unsafe extern "C" fn arm64_ioremap_prot_hook_register(hook: ioremap_prot_hook_t) -> c_int {
    int arm64_ioremap_prot_hook_register(ioremap_prot_hook_t hook)
    {
    if (WARN_ON(ioremap_prot_hook))
    return -EBUSY;
    ioremap_prot_hook = hook;
    return 0;
    }
    void __iomem *__ioremap_prot(phys_addr_t phys_addr, size_t size,
    pgprot_t pgprot)
    {
    let mut last_addr: c_ulong = phys_addr + size - 1;
// Don't allow outside PHYS_MASK
    if (last_addr & ~PHYS_MASK)
    return core::ptr::null_mut();
// Don't allow RAM to be mapped.
    if (WARN_ONCE(pfn_is_map_memory(__phys_to_pfn(phys_addr)),
    "ioremap attempted on RAM pfn\n"))
    return core::ptr::null_mut();
//
// If a hook is registered (e.g. for confidential computing
// purposes), call that now and barf if it fails.
//
    if (unlikely(ioremap_prot_hook) &&
    WARN_ON(ioremap_prot_hook(phys_addr, size, &pgprot))) {
    return core::ptr::null_mut();
    }
    return generic_ioremap_prot(phys_addr, size, pgprot);
    }
    EXPORT_SYMBOL(__ioremap_prot);
//
// Must be called after early_fixmap_init
//
#[no_mangle]
pub unsafe extern "C" fn early_ioremap_init() -> void __init {
    void __init early_ioremap_init(void)
    {
    early_ioremap_setup();
    }
    bool arch_memremap_can_ram_remap(resource_size_t offset, size_t size,
    unsigned long flags)
    {
    let mut pfn: c_ulong = PHYS_PFN(offset);
    return pfn_is_map_memory(pfn);
    }
