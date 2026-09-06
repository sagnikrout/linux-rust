//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/crash_dump.c
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
// Routines for doing kexec-based kdump.
//
// Copyright (C) 2005, IBM Corp.
//
// Created by: Michael Ellerman
//

// Macro flag: #define DBG(fmt...)

#[no_mangle]
pub unsafe extern "C" fn reserve_kdump_trampoline() -> void __init {
    void __init reserve_kdump_trampoline(void)
    {
    memblock_reserve(0, KDUMP_RESERVE_LIMIT);
    }
#[no_mangle]
unsafe extern "C" fn create_trampoline(addr: c_ulong) -> void __init {
    static void __init create_trampoline(unsigned long addr)
    {
    u32 *p = (u32 *)addr;
// The maximum range of a single instruction branch, is the current
// instruction's address + (32 MB - 4) bytes. For the trampoline we
// need to branch to current address + 32 MB. So we insert a nop at
// the trampoline address, then the next instruction (+ 4 bytes)
// does a branch to (32 MB - 4). The net effect is that when we
// branch to "addr" we jump to ("addr" + 32 MB). Although it requires
// two instructions it doesn't require any registers.
//
    patch_instruction(p, ppc_inst(PPC_RAW_NOP()));
    patch_branch(p + 1, addr + PHYSICAL_START, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn setup_kdump_trampoline() -> void __init {
    void __init setup_kdump_trampoline(void)
    {
    unsigned long i;
    DBG(" . setup_kdump_trampoline()\n");
    for (i = KDUMP_TRAMPOLINE_START; i < KDUMP_TRAMPOLINE_END; i += 8) {
    create_trampoline(i);
    }

    create_trampoline(__pa(system_reset_fwnmi) - PHYSICAL_START);
    create_trampoline(__pa(machine_check_fwnmi) - PHYSICAL_START);

    DBG(" <- setup_kdump_trampoline()\n");
    }

    ssize_t copy_oldmem_page(struct iov_iter *iter, unsigned long pfn,
    size_t csize, unsigned long offset)
    {
    void  *vaddr;
    phys_addr_t paddr;
    if (!csize)
    return 0;
    csize = min_t(size_t, csize, PAGE_SIZE);
    paddr = pfn << PAGE_SHIFT;
    if (memblock_is_region_memory(paddr, csize)) {
    vaddr = __va(paddr);
    csize = copy_to_iter(vaddr + offset, csize, iter);
    } else {
    vaddr = ioremap_cache(paddr, PAGE_SIZE);
    csize = copy_to_iter(vaddr + offset, csize, iter);
    iounmap(vaddr);
    }
    return csize;
    }
//
// Return true only when kexec based kernel dump capturing method is used.
// This ensures all restritions applied for kdump case are not automatically
// applied for fadump case.
//
#[no_mangle]
pub unsafe extern "C" fn is_kdump_kernel() -> bool {
    bool is_kdump_kernel(void)
    {
    return !is_fadump_active() && elfcorehdr_addr != ELFCORE_ADDR_MAX;
    }
    EXPORT_SYMBOL_GPL(is_kdump_kernel);

//
// The crashkernel region will almost always overlap the RTAS region, so
// we have to be careful when shrinking the crashkernel region.
//
#[no_mangle]
pub unsafe extern "C" fn crash_free_reserved_phys_range(begin: c_ulong, end: c_ulong) {
    void crash_free_reserved_phys_range(unsigned long begin, unsigned long end)
    {
    unsigned long addr;
    const __be32 *basep, *sizep;
    let mut rtas_start: c_uint = 0, rtas_end = 0;
    basep = of_get_property(rtas.dev, "linux,rtas-base", core::ptr::null_mut());
    sizep = of_get_property(rtas.dev, "rtas-size", core::ptr::null_mut());
    if (basep && sizep) {
    rtas_start = be32_to_cpup(basep);
    rtas_end = rtas_start + be32_to_cpup(sizep);
    }
    for (addr = begin; addr < end; addr += PAGE_SIZE) {
// Does this page overlap with the RTAS region?
    if (addr <= rtas_end && ((addr + PAGE_SIZE) > rtas_start))
    continue;
    free_reserved_page(pfn_to_page(addr >> PAGE_SHIFT));
    }
    }
