//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/fixmap.h
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
// Copyright 2008 Freescale Semiconductor Inc.
// Port to powerpc added by Kumar Gala
//

//
// Here we define all the compile-time 'special' virtual
// addresses. The point is to have a constant address at
// compile time, but to set the physical address only
// in the boot process. We allocate these special addresses
// from the end of virtual memory (0xfffff000) backwards.
// Also this lets us do fail-safe vmalloc(), we
// can guarantee that these special addresses and
// vmalloc()-ed addresses never overlap.
//
// these 'compile-time allocated' memory buffers are
// fixed-size 4k pages. (or larger if used with an increment
// highger than 1) use fixmap_set(idx,phys) to associate
// physical memory with fixmap indices.
//
// TLB entries of such buffers will not be flushed across
// task switches.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fixed_addresses {
    FIX_HOLE,

// reserve the top 128K for early debugging purposes
    FIX_EARLY_DEBUG_TOP = FIX_HOLE,
    FIX_EARLY_DEBUG_BASE = FIX_EARLY_DEBUG_TOP+(ALIGN(SZ_128K, PAGE_SIZE)/PAGE_SIZE)-1,

    FIX_KMAP_BEGIN,	/* reserved pte's for temporary kernel mappings */
    FIX_KMAP_END = FIX_KMAP_BEGIN + (KM_MAX_IDX * NR_CPUS) - 1,

// For IMMR we need an aligned 512K area

    FIX_IMMR_START,
    FIX_IMMR_BASE = __ALIGN_MASK(FIX_IMMR_START, FIX_IMMR_SIZE - 1) - 1 +
    FIX_IMMR_SIZE,

// For IMMR we need an aligned 2M area

    FIX_IMMR_START,
    FIX_IMMR_BASE = __ALIGN_MASK(FIX_IMMR_START, FIX_IMMR_SIZE - 1) - 1 +
    FIX_IMMR_SIZE,

// FIX_PCIE_MCFG,

    __end_of_permanent_fixed_addresses,

pub const FIX_BTMAPS_SLOTS: c_int = 16;

    FIX_BTMAP_END = __end_of_permanent_fixed_addresses,
    FIX_BTMAP_BEGIN = FIX_BTMAP_END + TOTAL_FIX_BTMAPS - 1,
    __end_of_fixed_addresses
}

