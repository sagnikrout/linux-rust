//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/memory.h
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
// Based on arch/arm/include/asm/memory.h
//
// Copyright (C) 2000-2002 Russell King
// Copyright (C) 2012 ARM Ltd.
//
// Note: this file should not be included by non-asm/.h files
//

//
// Size of the PCI I/O space. This must remain a power of two so that
// IO_SPACE_LIMIT acts as a mask for the low bits of I/O addresses.
//

//
// VMEMMAP_SIZE - allows the whole linear region to be covered by
// a struct page array
//
// If we are configured with a 52-bit kernel VA then our VMEMMAP_SIZE
// needs to cover the memory region from the beginning of the 52-bit
// PAGE_OFFSET all the way to PAGE_END for 48-bit. This allows us to
// keep a constant PAGE_OFFSET and "fallback" to using the higher end
// of the VMEMMAP where 52-bit support is not available in hardware.
//

//
// PAGE_OFFSET - the virtual address of the start of the linear map, at the
// start of the TTBR1 address space.
// PAGE_END - the end of the linear map, where all other kernel mappings begin.
// KIMAGE_VADDR - the virtual address of the start of the kernel image.
// VA_BITS - the maximum number of bits for virtual addresses.
//

//
// Generic and Software Tag-Based KASAN modes require 1/8th and 1/16th of the
// kernel virtual address space for storing the shadow memory respectively.
//
// The mapping between a virtual memory address and its corresponding shadow
// memory address is defined based on the formula:
//
// shadow_addr = (addr >> KASAN_SHADOW_SCALE_SHIFT) + KASAN_SHADOW_OFFSET
//
// where KASAN_SHADOW_SCALE_SHIFT is the order of the number of bits that map
// to a single shadow byte and KASAN_SHADOW_OFFSET is a constant that offsets
// the mapping. Note that KASAN_SHADOW_OFFSET does not point to the start of
// the shadow memory region.
//
// Based on this mapping, we define two constants:
//
// KASAN_SHADOW_START: the start of the shadow memory region;
// KASAN_SHADOW_END: the end of the shadow memory region.
//
// KASAN_SHADOW_END is defined first as the shadow address that corresponds to
// the upper bound of possible virtual kernel memory addresses UL(1) << 64
// according to the mapping formula.
//
// KASAN_SHADOW_START is defined second based on KASAN_SHADOW_END. The shadow
// memory start must map to the lowest possible kernel virtual memory address
// and thus it depends on the actual bitness of the address space.
//
// As KASAN inserts redzones between stack variables, this increases the stack
// memory usage significantly. Thus, we double the (minimum) stack size.
//

pub const KASAN_THREAD_SHIFT: c_int = 1;

pub const KASAN_THREAD_SHIFT: c_int = 0;

//
// VMAP'd stacks are allocated at page granularity, so we must ensure that such
// stacks are a multiple of page size.
//

//
// By aligning VMAP'd stacks to 2 * THREAD_SIZE, we can detect overflow by
// checking sp & (1 << THREAD_SHIFT), which we can do cheaply in the entry
// assembly.
//

//
// With the minimum frame size of [x29, x30], exactly half the combined
// sizes of the hyp and overflow stacks is the maximum size needed to
// save the unwinded stacktrace; plus an additional entry to delimit the
// end.
//

//
// Alignment of kernel segments (e.g. .text, .data).
//
// 4 KB granule:  16 level 3 entries, with contiguous bit
// 16 KB granule:   4 level 3 entries, without contiguous bit
// 64 KB granule:   1 level 3 entry
//

//
// Memory types available.
//
// IMPORTANT: MT_NORMAL must be index 0 since vm_get_page_prot() may 'or' in
// the MT_NORMAL_TAGGED memory type for PROT_MTE mappings. Note
// that protection_map[] only contains MT_NORMAL attributes.
//
pub const MT_NORMAL: c_int = 0;
pub const MT_NORMAL_TAGGED: c_int = 1;
pub const MT_NORMAL_NC: c_int = 2;
pub const MT_DEVICE_nGnRnE: c_int = 3;
pub const MT_DEVICE_nGnRE: c_int = 4;
//
// Memory types for Stage-2 translation when HCR_EL2.FWB=0. See R_HMNDG,
// R_TNHFM, R_GQFSF and I_MCQKW for the details on how these attributes get
// combined with Stage-1.
//
pub const MT_S2_NORMAL: c_uint = 0xf;
pub const MT_S2_NORMAL_NC: c_uint = 0x5;
pub const MT_S2_DEVICE_nGnRE: c_uint = 0x1;

//
// Memory types for Stage-2 translation when HCR_EL2.FWB=1. Stage-2 enforces
// Normal-WB and Device-nGnRE, unless we actively say that S1 wins. See
// R_VRJSW and R_RHWZM for details.
//
pub const MT_S2_FWB_NORMAL: c_int = 6;
pub const MT_S2_FWB_NORMAL_NC: c_int = 5;
pub const MT_S2_FWB_DEVICE_nGnRE: c_int = 1;
pub const MT_S2_FWB_AS_S1: c_int = 7;

//
// Open-coded (swapper_pg_dir - reserved_pg_dir) as this cannot be calculated
// until link time.
//

//
// Open-coded (swapper_pg_dir - tramp_pg_dir) as this cannot be calculated
// until link time.
//

// read_sysreg() uses asm volatile, so avoid it here

// For reasons of #include hell, we can't use TCR_T1SZ_OFFSET/TCR_T1SZ_MASK here

// PHYS_OFFSET - the physical address of the start of memory.

// the offset between the kernel virtual and physical mappings

extern "C" {
    pub fn kaslr_init();
}

//
// Allow all memory at the discovery stage. We will clip it later.
//
pub const MIN_MEMBLOCK_ADDR: c_int = 0;

//
// PFNs are used to describe any physical page; this means
// PFN 0 == physical address 0.
//
// This is the PFN of the first RAM page in the kernel
// direct-mapped view.  We assume this is the first page
// of RAM in the mem_map as well.
//

//
// When dealing with data aborts, watchpoints, or instruction traps we may end
// up with a tagged userland pointer. Clear the tag to get a sane pointer to
// pass on to access_ok(), for instance.
//

pub const __tag_get(addr): c_int = 0;

//
// Physical vs virtual RAM address space conversion.  These are
// private definitions which should NOT be used outside memory.h
// files.  Use virt_to_phys/phys_to_virt/__pa/__va instead.
//
// Check whether an arbitrary address is within the linear map, which
// lives in the [PAGE_OFFSET, PAGE_END) interval at the bottom of the
// kernel's TTBR1 address range.
//

extern "C" {
    pub fn __virt_to_phys(x: c_ulong) -> phys_addr_t;
}
extern "C" {
    pub fn __phys_addr_symbol(x: c_ulong) -> phys_addr_t;
}

//
// Note: Drivers should NOT use these.  They are the wrong
// translation for translating DMA addresses.  Use the driver
// DMA support - see dma-mapping.h.
//

extern "C" {
    pub fn __virt_to_phys(long)(x): (unsigned) -> return;
}

// Needed already here for resolving __phys_to_pfn() in virt_to_pfn()

extern "C" {
    pub fn __phys_to_pfn(_arg: virt_to_phys(kaddr)) -> return;
}
//
// Drivers should NOT use these either.
//

//
// virt_to_page(x)	convert a _valid_ virtual address to struct page
// virt_addr_valid(x)	indicates whether a virtual address is valid
//

extern "C" {
    pub fn dump_mem_limit();
}

//
// Given that the GIC architecture permits ITS implementations that can only be
// configured with a LPI table address once, GICv3 systems with many CPUs may
// end up reserving a lot of different regions after a kexec for their LPI
// tables (one per CPU), as we are forced to reuse the same memory after kexec
// (and thus reserve it persistently with EFI beforehand)
//

//
// memory regions which marked with flag MEMBLOCK_NOMAP(for example, the memory
// of the EFI_UNUSABLE_MEMORY type) may divide a continuous memory block into
// multiple parts. As a result, the number of memory regions is large.
//

