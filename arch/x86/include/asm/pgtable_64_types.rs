//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pgtable_64_types.h
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
// These are used to make use of C type-checking..
//
pub type pteval_t = c_ulong;
pub type pmdval_t = c_ulong;
pub type pudval_t = c_ulong;
pub type p4dval_t = c_ulong;
pub type pgdval_t = c_ulong;
pub type pgprotval_t = c_ulong;

//
// cpu_feature_enabled() is not available in early boot code.
// Use variable instead.
//

//
// PGDIR_SHIFT determines what a top-level page table entry can map
//

pub const PTRS_PER_PGD: c_int = 512;
//
// 4th level page in 5-level paging case
//
pub const P4D_SHIFT: c_int = 39;
pub const MAX_PTRS_PER_P4D: c_int = 512;

pub const MAX_POSSIBLE_PHYSMEM_BITS: c_int = 52;
//
// 3rd level page
//
pub const PUD_SHIFT: c_int = 30;
pub const PTRS_PER_PUD: c_int = 512;
//
// PMD_SHIFT determines the size of the area a middle-level
// page table can map
//
pub const PMD_SHIFT: c_int = 21;
pub const PTRS_PER_PMD: c_int = 512;
//
// entries per page directory level
//
pub const PTRS_PER_PTE: c_int = 512;

//
// See Documentation/arch/x86/x86_64/mm.rst for a description of the memory map.
//
// Be very careful vs. KASLR when changing anything here. The KASLR address
// range must not overlap with anything except the KASAN shadow area, which
// is correct as KASAN disables KASLR.
//

pub const __VMALLOC_BASE_L4: c_uint = 0xffffc90000000000UL;
pub const __VMALLOC_BASE_L5: c_uint = 0xffa0000000000000UL;

pub const __VMEMMAP_BASE_L4: c_uint = 0xffffea0000000000UL;
pub const __VMEMMAP_BASE_L5: c_uint = 0xffd4000000000000UL;

//
// End of the region for which vmalloc page tables are pre-allocated.
// For non-KMSAN builds, this is the same as VMALLOC_END.
// For KMSAN builds, VMALLOC_START..VMEMORY_END is 4 times bigger than
// VMALLOC_START..VMALLOC_END (see below).
//

//
// In KMSAN builds vmalloc area is four times smaller, and the remaining 3/4
// are used to keep the metadata for virtual pages. The memory formerly
// belonging to vmalloc area is now laid out as follows:
//
// 1st quarter: VMALLOC_START to VMALLOC_END - new vmalloc area
// 2nd quarter: KMSAN_VMALLOC_SHADOW_START to
// VMALLOC_END+KMSAN_VMALLOC_SHADOW_OFFSET - vmalloc area shadow
// 3rd quarter: KMSAN_VMALLOC_ORIGIN_START to
// VMALLOC_END+KMSAN_VMALLOC_ORIGIN_OFFSET - vmalloc area origins
// 4th quarter: KMSAN_MODULES_SHADOW_START to KMSAN_MODULES_ORIGIN_START
// - shadow for modules,
// KMSAN_MODULES_ORIGIN_START to
// KMSAN_MODULES_ORIGIN_START + MODULES_LEN - origins for modules.
//

//
// vmalloc metadata addresses are calculated by adding shadow/origin offsets
// to vmalloc address.
//

//
// The shadow/origin for modules are placed one by one in the last 1/4 of
// vmalloc space.
//

// The module sections ends with the start of the fixmap

pub const EARLY_DYNAMIC_PAGE_TABLES: c_int = 64;

//
// We borrow bit 3 to remember PG_anon_exclusive.
//

