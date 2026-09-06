//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/init_32.c
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
// PowerPC version
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
// and Cort Dougan (PReP) (cort@cs.nmt.edu)
// Copyright (C) 1996 Paul Mackerras
// PPC44x/36-bit changes by Matt Porter (mporter@mvista.com)
//
// Derived from "arch/i386/mm/init.c"
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//

// The amount of lowmem must be within 0xF0000000 - KERNELBASE.

    phys_addr_t total_memory;
    phys_addr_t total_lowmem;

// Used in __va()/__pa()
    long long virt_phys_offset;
    EXPORT_SYMBOL(virt_phys_offset);

    phys_addr_t lowmem_end_addr;
    int boot_mapsize;

    unsigned long agp_special_page;
    EXPORT_SYMBOL(agp_special_page);

    void MMU_init(void);
// max amount of low RAM to map in
    let mut __max_low_memory: c_ulong = MAX_LOW_MEM;
//
// MMU_init sets up the basic memory mappings for the kernel,
// including both RAM and possibly some I/O regions,
// and sets up the page tables and the MMU hardware ready to go.
//
#[no_mangle]
pub unsafe extern "C" fn MMU_init() -> void __init {
    void __init MMU_init(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("MMU:enter", 0x111);
    total_lowmem = total_memory = memblock_end_of_DRAM() - memstart_addr;
    lowmem_end_addr = memstart_addr + total_lowmem;

// Freescale Book-E parts expect lowmem to be mapped by fixed TLB
// entries, so we need to adjust lowmem to match the amount we can map
// in the fixed entries
    adjust_total_lowmem();

    if (total_lowmem > __max_low_memory) {
    total_lowmem = __max_low_memory;
    lowmem_end_addr = memstart_addr + total_lowmem;

    total_memory = total_lowmem;
    memblock_enforce_memory_limit(total_lowmem);

    }
// Initialize the MMU hardware
    if (ppc_md.progress)
    ppc_md.progress("MMU:hw init", 0x300);
    MMU_init_hw();
// Map in all of RAM starting at KERNELBASE
    if (ppc_md.progress)
    ppc_md.progress("MMU:mapin", 0x301);
    mapin_ram();
// Initialize early top-down ioremap allocator
    ioremap_bot = IOREMAP_TOP;
    if (ppc_md.progress)
    ppc_md.progress("MMU:exit", 0x211);
// From now on, btext is no longer BAT mapped if it was at all

    btext_unmap();

    kasan_mmu_init();
    setup_kup();
    update_mmu_feature_fixups(MMU_FTR_KUAP);
// Shortly after that, the entire linear mapping will be available
    memblock_set_current_limit(lowmem_end_addr);
    }
