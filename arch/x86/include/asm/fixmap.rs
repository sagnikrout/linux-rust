//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/fixmap.h
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
//
// Support of BIGMEM added by Gerhard Wichert, Siemens AG, July 1999
// x86_32 and x86_64 integration by Gustavo F. Padovan, February 2009
//

//
// Exposed to assembly code for setting up initial page tables. Cannot be
// calculated in assembly code (fixmap entries are an enum), but is sanity
// checked in the actual fixmap C code to make sure that the fixmap is
// covered fully.
//

// fixmap starts downwards from the 507th entry in level2_fixmap_pgt
pub const FIXMAP_PMD_TOP: c_int = 507;

//
// We can't declare FIXADDR_TOP as variable for x86_64 because vsyscall
// uses fixmaps that relies on FIXADDR_TOP for proper address calculation.
// Because of this, FIXADDR_TOP x86 integration was left as later work.
//

//
// Leave one empty page between vmalloc'ed areas and
// the start of the fixmap.
//

//
// Here we define all the compile-time 'special' virtual
// addresses. The point is to have a constant address at
// compile time, but to set the physical address only
// in the boot process.
// for x86_32: We allocate these special addresses
// from the end of virtual memory (0xfffff000) backwards.
// Also this lets us do fail-safe vmalloc(), we
// can guarantee that these special addresses and
// vmalloc()-ed addresses never overlap.
//
// These 'compile-time allocated' memory buffers are
// fixed-size 4k pages (or larger if used with an increment
// higher than 1). Use set_fixmap(idx,phys) to associate
// physical memory with fixmap indices.
//
// TLB entries of such buffers will not be flushed across
// task switches.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fixed_addresses {

    FIX_HOLE,

    VSYSCALL_PAGE = (FIXADDR_TOP - VSYSCALL_ADDR) >> PAGE_SHIFT,

    FIX_DBGP_BASE,
    FIX_EARLYCON_MEM_BASE,

    FIX_OHCI1394_BASE,

    FIX_APIC_BASE,	/* local (CPU) APIC) -- required for SMP or not */

    FIX_IO_APIC_BASE_0,
    FIX_IO_APIC_BASE_END = FIX_IO_APIC_BASE_0 + MAX_IO_APICS - 1,

    FIX_KMAP_BEGIN,	/* reserved pte's for temporary kernel mappings */
    FIX_KMAP_END = FIX_KMAP_BEGIN + (KM_MAX_IDX * NR_CPUS) - 1,

    FIX_PCIE_MCFG,

    FIX_PARAVIRT_BOOTMAP,

// Used for GHES mapping from assorted contexts
    FIX_APEI_GHES_IRQ,
    FIX_APEI_GHES_NMI,

    __end_of_permanent_fixed_addresses,

//
// 512 temporary boot-time mappings, used by early_ioremap(),
// before ioremap() is functional.
//
// If necessary we round it up to the next 512 pages boundary so
// that we can have a single pmd entry and a single pte table:
//
pub const NR_FIX_BTMAPS: c_int = 64;
pub const FIX_BTMAPS_SLOTS: c_int = 8;

    FIX_BTMAP_END =
    (__end_of_permanent_fixed_addresses ^
    (__end_of_permanent_fixed_addresses + TOTAL_FIX_BTMAPS - 1)) &
    -PTRS_PER_PTE
    ? __end_of_permanent_fixed_addresses + TOTAL_FIX_BTMAPS -
    (__end_of_permanent_fixed_addresses & (TOTAL_FIX_BTMAPS - 1))
    : __end_of_permanent_fixed_addresses,
    FIX_BTMAP_BEGIN = FIX_BTMAP_END + TOTAL_FIX_BTMAPS - 1,

    FIX_WP_TEST,

    FIX_TBOOT_BASE,

    __end_of_fixed_addresses
}

extern "C" {
    pub fn reserve_top_address(reserve: c_ulong);
}

extern "C" {
    pub fn __native_set_fixmap(idx: fixed_addresses, pte: pte_t);
}

//
// FIXMAP_PAGE_NOCACHE is used for MMIO. Memory encryption is not
// supported for MMIO addresses, so make sure that the memory encryption
// mask is not part of the page attributes.
//

//
// Early memremap routines used for in-place encryption. The mappings created
// by these routines are intended to be used as temporary mappings.
//

