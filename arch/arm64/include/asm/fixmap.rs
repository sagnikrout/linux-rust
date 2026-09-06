//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/fixmap.h
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


//
// fixmap.h: compile-time virtual memory allocation
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 1998 Ingo Molnar
// Copyright (C) 2013 Mark Salter <msalter@redhat.com>
//
// Adapted from arch/x86 version.
//

//
// Here we define all the compile-time 'special' virtual
// addresses. The point is to have a constant address at
// compile time, but to set the physical address only
// in the boot process.
//
// Each enum increment in these 'compile-time allocated'
// memory buffers is page-sized. Use set_fixmap(idx,phys)
// to associate physical memory with a fixmap index.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fixed_addresses {
    FIX_HOLE,

//
// Reserve a virtual window for the FDT that is a page bigger than the
// maximum supported size. The additional space ensures that any FDT
// that does not exceed MAX_FDT_SIZE can be mapped regardless of
// whether it crosses any page boundary.
//
    FIX_FDT_END,
    FIX_FDT = FIX_FDT_END + DIV_ROUND_UP(MAX_FDT_SIZE, PAGE_SIZE) + 1,

    FIX_EARLYCON_MEM_BASE,
    FIX_TEXT_POKE0,

// One slot per CPU, mapping the guest's VNCR page at EL2.
    FIX_VNCR_END,
    FIX_VNCR = FIX_VNCR_END + NR_CPUS,

// Used for GHES mapping from assorted contexts
    FIX_APEI_GHES_IRQ,
    FIX_APEI_GHES_SEA,

    FIX_APEI_GHES_SDEI_NORMAL,
    FIX_APEI_GHES_SDEI_CRITICAL,

    FIX_ENTRY_TRAMP_TEXT4,	/* one extra slot for the data page */

    FIX_ENTRY_TRAMP_TEXT3,
    FIX_ENTRY_TRAMP_TEXT2,
    FIX_ENTRY_TRAMP_TEXT1,

    __end_of_permanent_fixed_addresses,

//
// Temporary boot-time mappings, used by early_ioremap(),
// before ioremap() is functional.
//
// Reserve one extra page so a 256K mapping may start at any
// offset within a page. early_ioremap() maps the page-aligned
// physical range, so the initial offset can consume an extra page.
//

pub const FIX_BTMAPS_SLOTS: c_int = 7;

    FIX_BTMAP_END = __end_of_permanent_fixed_addresses,
    FIX_BTMAP_BEGIN = FIX_BTMAP_END + TOTAL_FIX_BTMAPS - 1,

//
// Used for kernel page table creation, so unmapped memory may be used
// for tables.
//
    FIX_PTE,
    FIX_PMD,
    FIX_PUD,
    FIX_P4D,
    FIX_PGD,

    __end_of_fixed_addresses
}

extern "C" {
    pub fn early_fixmap_init() -> void __init;
}

extern "C" {
    pub fn __set_fixmap(idx: fixed_addresses, phys: phys_addr_t, prot: pgprot_t);
}

