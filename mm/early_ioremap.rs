//! Automatically rewritten from C to Rust
//! Source: mm/early_ioremap.c
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
// Provide common bits of early_ioremap() support for architectures needing
// temporary mappings during boot before ioremap() is available.
//
// This is mostly a direct copy of the x86 early_ioremap implementation.
//
// (C) Copyright 1995 1996, 2014 Linus Torvalds
//

    static int early_ioremap_debug __initdata;
#[no_mangle]
unsafe extern "C" fn early_ioremap_debug_setup(str: *mut c_char) -> int __init {
    static int __init early_ioremap_debug_setup(char *str)
    {
    early_ioremap_debug = 1;
    return 0;
    }
    early_param("early_ioremap_debug", early_ioremap_debug_setup);

    do {						\
    if (unlikely(early_ioremap_debug)) {	\
    pr_warn(fmt, ##args);		\
    dump_stack();			\
    }					\
    } while (0)
    static int after_paging_init __initdata;
    pgprot_t __init __weak early_memremap_pgprot_adjust(resource_size_t phys_addr,
    unsigned long size,
    pgprot_t prot)
    {
    return prot;
    }
//
// Only architectures whose early_ioremap() must stop using __early_set_fixmap()
// after paging_init() need to call this.
//
#[no_mangle]
pub unsafe extern "C" fn early_ioremap_reset() -> void __init {
    void __init early_ioremap_reset(void)
    {
    after_paging_init = 1;
    }
//
// Only architectures that call early_ioremap_reset() need to define
// __late_set_fixmap() and __late_clear_fixmap(), which early_ioremap() uses
// instead of __early_set_fixmap() after the reset.
//

    static inline void __init __late_set_fixmap(enum fixed_addresses idx,
    phys_addr_t phys, pgprot_t prot)
    {
    BUG();
    }

#[no_mangle]
pub unsafe extern "C" fn __late_clear_fixmap(idx: enum fixed_addresses) -> void __init {
    static inline void __init __late_clear_fixmap(enum fixed_addresses idx)
    {
    BUG();
    }

    static void __iomem *prev_map[FIX_BTMAPS_SLOTS] __initdata;
    static unsigned long prev_size[FIX_BTMAPS_SLOTS] __initdata;
    static unsigned long slot_virt[FIX_BTMAPS_SLOTS] __initdata;
#[no_mangle]
pub unsafe extern "C" fn early_ioremap_setup() -> void __init {
    void __init early_ioremap_setup(void)
    {
    int i;
    for (i = 0; i < FIX_BTMAPS_SLOTS; i++) {
    WARN_ON_ONCE(prev_map[i]);
    slot_virt[i] = __fix_to_virt(FIX_BTMAP_BEGIN - NR_FIX_BTMAPS*i);
    }
    }
#[no_mangle]
unsafe extern "C" fn check_early_ioremap_leak() -> int __init {
    static int __init check_early_ioremap_leak(void)
    {
    let mut count: c_int = 0;
    int i;
    for (i = 0; i < FIX_BTMAPS_SLOTS; i++)
    if (prev_map[i])
    count++;
    if (WARN(count, KERN_WARNING
    "Debug warning: early ioremap leak of %d areas detected.\n"
    "please boot with early_ioremap_debug and report the dmesg.\n",
    count))
    return 1;
    return 0;
    }
    late_initcall(check_early_ioremap_leak);
    static void __init __iomem *
    __early_ioremap(resource_size_t phys_addr, unsigned long size, pgprot_t prot)
    {
    unsigned long offset;
    resource_size_t last_addr;
    unsigned int nrpages;
    enum fixed_addresses idx;
    int i, slot;
    WARN_ON(system_state >= SYSTEM_RUNNING);
    slot = -1;
    for (i = 0; i < FIX_BTMAPS_SLOTS; i++) {
    if (!prev_map[i]) {
    slot = i;
    break;
    }
    }
    if (WARN(slot < 0, "%s(%pa, %08lx) not found slot\n",
    __func__, &phys_addr, size))
    return core::ptr::null_mut();
// Don't allow wraparound or zero size
    last_addr = phys_addr + size - 1;
    if (WARN_ON(!size || last_addr < phys_addr))
    return core::ptr::null_mut();
    prev_size[slot] = size;
//
// Mappings have to be page-aligned
//
    offset = offset_in_page(phys_addr);
    phys_addr &= PAGE_MASK;
    size = PAGE_ALIGN(last_addr + 1) - phys_addr;
//
// Mappings have to fit in the FIX_BTMAP area.
//
    nrpages = size >> PAGE_SHIFT;
    if (WARN_ON(nrpages > NR_FIX_BTMAPS))
    return core::ptr::null_mut();
    early_ioremap_dbg("%s(%pa, %08lx) [%d] => %08lx + %08lx\n",
    __func__, &phys_addr, size, slot, slot_virt[slot], offset);
//
// Ok, go for it..
//
    idx = FIX_BTMAP_BEGIN - NR_FIX_BTMAPS*slot;
    while (nrpages > 0) {
    if (after_paging_init)
    __late_set_fixmap(idx, phys_addr, prot);
    else
    __early_set_fixmap(idx, phys_addr, prot);
    phys_addr += PAGE_SIZE;
    --idx;
    --nrpages;
    }
    prev_map[slot] = (void __iomem *)(offset + slot_virt[slot]);
    return prev_map[slot];
    }
#[no_mangle]
pub unsafe extern "C" fn early_iounmap(addr: *mut void __iomem, size: c_ulong) -> void __init {
    void __init early_iounmap(void __iomem *addr, unsigned long size)
    {
    unsigned long virt_addr;
    unsigned long offset;
    unsigned int nrpages;
    enum fixed_addresses idx;
    int i, slot;
    slot = -1;
    for (i = 0; i < FIX_BTMAPS_SLOTS; i++) {
    if (prev_map[i] == addr) {
    slot = i;
    break;
    }
    }
    if (WARN(slot < 0, "%s(%p, %08lx) not found slot\n",
    __func__, addr, size))
    return;
    if (WARN(prev_size[slot] != size,
    "%s(%p, %08lx) [%d] size not consistent %08lx\n",
    __func__, addr, size, slot, prev_size[slot]))
    return;
    early_ioremap_dbg("%s(%p, %08lx) [%d]\n", __func__, addr, size, slot);
    virt_addr = (unsigned long)addr;
    if (WARN_ON(virt_addr < fix_to_virt(FIX_BTMAP_BEGIN)))
    return;
    offset = offset_in_page(virt_addr);
    nrpages = PAGE_ALIGN(offset + size) >> PAGE_SHIFT;
    idx = FIX_BTMAP_BEGIN - NR_FIX_BTMAPS*slot;
    while (nrpages > 0) {
    if (after_paging_init)
    __late_clear_fixmap(idx);
    else
    __early_set_fixmap(idx, 0, FIXMAP_PAGE_CLEAR);
    --idx;
    --nrpages;
    }
    prev_map[slot] = core::ptr::null_mut();
    }
// Remap an IO device
    void __init __iomem *
    early_ioremap(resource_size_t phys_addr, unsigned long size)
    {
    return __early_ioremap(phys_addr, size, FIXMAP_PAGE_IO);
    }
// Remap memory
    void __init *
    early_memremap(resource_size_t phys_addr, unsigned long size)
    {
    pgprot_t prot = early_memremap_pgprot_adjust(phys_addr, size,
    FIXMAP_PAGE_NORMAL);
    return ( void *)__early_ioremap(phys_addr, size, prot);
    }

    void __init *
    early_memremap_ro(resource_size_t phys_addr, unsigned long size)
    {
    pgprot_t prot = early_memremap_pgprot_adjust(phys_addr, size,
    FIXMAP_PAGE_RO);
    return ( void *)__early_ioremap(phys_addr, size, prot);
    }

    void __init *
    early_memremap_prot(resource_size_t phys_addr, unsigned long size,
    unsigned long prot_val)
    {
    return ( void *)__early_ioremap(phys_addr, size,
    __pgprot(prot_val));
    }

//
// If no empty slot, handle that and return -ENOMEM.
//
#[no_mangle]
pub unsafe extern "C" fn copy_from_early_mem(dest: *mut c_void, src: phys_addr_t, size: c_ulong) -> int __init {
    int __init copy_from_early_mem(void *dest, phys_addr_t src, unsigned long size)
    {
    unsigned long slop, clen;
    char *p;
    while (size) {
    slop = offset_in_page(src);
    clen = size;
    if (clen > MAX_MAP_CHUNK - slop)
    clen = MAX_MAP_CHUNK - slop;
    p = early_memremap(src & PAGE_MASK, clen + slop);
    if (!p)
    return -ENOMEM;
    memcpy(dest, p + slop, clen);
    early_memunmap(p, clen + slop);
    dest += clen;
    src += clen;
    size -= clen;
    }
    return 0;
    }

    void __init __iomem *
    early_ioremap(resource_size_t phys_addr, unsigned long size)
    {
    return ( void __iomem *)phys_addr;
    }
// Remap memory
    void __init *
    early_memremap(resource_size_t phys_addr, unsigned long size)
    {
    return (void *)phys_addr;
    }
    void __init *
    early_memremap_ro(resource_size_t phys_addr, unsigned long size)
    {
    return (void *)phys_addr;
    }
#[no_mangle]
pub unsafe extern "C" fn early_iounmap(addr: *mut void __iomem, size: c_ulong) -> void __init {
    void __init early_iounmap(void __iomem *addr, unsigned long size)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn early_memunmap(addr: *mut c_void, size: c_ulong) -> void __init {
    void __init early_memunmap(void *addr, unsigned long size)
    {
    early_iounmap(( void __iomem *)addr, size);
    }
