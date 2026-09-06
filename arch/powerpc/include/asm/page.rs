//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/page.h
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
// Copyright (C) 2001,2005 IBM Corporation.
//

//
// On regular PPC32 page size is 4K (but we support 4K/16K/64K/256K pages
// on PPC44x and 4K/16K on 8xx). For PPC64 we support either 4K or 64K software
// page size. When using 64K pages however, whether we are really supporting
// 64K pages in HW or not is irrelevant to those definitions.
//

//
// KERNELBASE is the virtual address of the start of the kernel, it's often
// the same as PAGE_OFFSET, but _might not be_.
//
// The kdump dump kernel is one example where KERNELBASE != PAGE_OFFSET.
//
// PAGE_OFFSET is the virtual address of the start of lowmem.
//
// PHYSICAL_START is the physical address of the start of the kernel.
//
// MEMORY_START is the physical address of the start of lowmem.
//
// KERNELBASE, PAGE_OFFSET, and PHYSICAL_START are all configurable on
// ppc32 and based on how they are set we determine MEMORY_START.
//
// For the linear mapping the following equation should be true:
// KERNELBASE - PAGE_OFFSET = PHYSICAL_START - MEMORY_START
//
// Also, KERNELBASE >= PAGE_OFFSET and PHYSICAL_START >= MEMORY_START
//
// There are two ways to determine a physical address from a virtual one:
// va = pa + PAGE_OFFSET - MEMORY_START
// va = pa + KERNELBASE - PHYSICAL_START
//
// If you want to know something's offset from the start of the kernel you
// should subtract KERNELBASE.
//
// If you want to test if something's a kernel address, use is_kernel_addr().
//

// See Description below for VIRT_PHYS_OFFSET

//
// On Book-E parts we need __va to parse the device tree and we can't
// determine MEMORY_START until then.  However we can determine PHYSICAL_START
// from information at hand (program counter, TLB lookup).
//
// On BookE with RELOCATABLE && PPC32
//
// With RELOCATABLE && PPC32,  we support loading the kernel at any physical
// address without any restriction on the page alignment.
//
// We find the runtime address of _stext and relocate ourselves based on
// the following calculation:
//
// virtual_base = ALIGN_DOWN(KERNELBASE,256M) +
// MODULO(_stext.run,256M)
// and create the following mapping:
//
// ALIGN_DOWN(_stext.run,256M) => ALIGN_DOWN(KERNELBASE,256M)
//
// When we process relocations, we cannot depend on the
// existing equation for the __va()/__pa() translations:
//
// __va(x) = (x)  - PHYSICAL_START + KERNELBASE
//
// Where:
// PHYSICAL_START = kernstart_addr = Physical address of _stext
// KERNELBASE = Compiled virtual address of _stext.
//
// This formula holds true iff, kernel load address is TLB page aligned.
//
// In our case, we need to also account for the shift in the kernel Virtual
// address.
//
// E.g.,
//
// Let the kernel be loaded at 64MB and KERNELBASE be 0xc0000000 (same as PAGE_OFFSET).
// In this case, we would be mapping 0 to 0xc0000000, and kernstart_addr = 64M
//
// Now __va(1MB) = (0x100000) - (0x4000000) + 0xc0000000
// = 0xbc100000 , which is wrong.
//
// Rather, it should be : 0xc0000000 + 0x100000 = 0xc0100000
// according to our mapping.
//
// Hence we use the following formula to get the translations right:
//
// __va(x) = (x) - [ PHYSICAL_START - Effective KERNELBASE ]
//
// Where :
// PHYSICAL_START = dynamic load address.(kernstart_addr variable)
// Effective KERNELBASE = virtual_base =
// = ALIGN_DOWN(KERNELBASE,256M) +
// MODULO(PHYSICAL_START,256M)
//
// To make the cost of __va() / __pa() more light weight, we introduce
// a new variable virt_phys_offset, which will hold :
//
// virt_phys_offset = Effective KERNELBASE - PHYSICAL_START
// = ALIGN_DOWN(KERNELBASE,256M) -
// ALIGN_DOWN(PHYSICALSTART,256M)
//
// Hence :
//
// __va(x) = x - PHYSICAL_START + Effective KERNELBASE
// = x + virt_phys_offset
//
// and
// __pa(x) = x + PHYSICAL_START - Effective KERNELBASE
// = x - virt_phys_offset
//
// On non-Book-E PPC64 PAGE_OFFSET and MEMORY_START are constants so use
// the other definitions for __va & __pa.
//

//
// gcc miscompiles (unsigned long)(&static_var) - PAGE_OFFSET
// with -mcmodel=medium, so we use & and | instead of - and + on 64-bit.
// This also results in better code generation.
//

extern "C" {
    pub fn __va(PAGE_SHIFT: pfn <<) -> return;
}

//
// Unfortunately the PLT is in the BSS in the PPC32 ELF ABI,
// and needs to be executable.  This means the whole heap ends
// up being executable.
//

//
// Don't compare things with KERNELBASE or PAGE_OFFSET to test for
// "kernelness", use is_kernel_addr() - it should do what you want.
//

extern "C" {
    pub fn clear_user_page(page: *mut c_void, vaddr: c_ulong, pg: *mut page);
}

extern "C" {
    pub fn devmem_is_allowed(pfn: c_ulong) -> c_int;
}

extern "C" {
    pub fn arch_free_page(page: *mut page, order: c_int);
}
// Macro flag: #define HAVE_ARCH_FREE_PAGE

