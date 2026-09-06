//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/pgtable-hwdef.h
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
// Copyright (C) 2012 ARM Ltd.
//

pub const PTDESC_ORDER: c_int = 3;
// Number of VA bits resolved by a single translation table level

//
// Number of page-table levels required to address 'va_bits' wide
// address, without section mapping. We resolve the top (va_bits - PAGE_SHIFT)
// bits with PTDESC_TABLE_SHIFT bits at each page table level. Hence:
//
// levels = DIV_ROUND_UP((va_bits - PAGE_SHIFT), PTDESC_TABLE_SHIFT)
//
// where DIV_ROUND_UP(n, d) => (((n) + (d) - 1) / (d))
//
// We cannot include linux/kernel.h which defines DIV_ROUND_UP here
// due to build issues. So we open code DIV_ROUND_UP here:
//
// ((((va_bits) - PAGE_SHIFT) + PTDESC_TABLE_SHIFT - 1) / PTDESC_TABLE_SHIFT)
//
// which gets simplified as :
//

//
// Size mapped by an entry at level n ( -1 <= n <= 3)
// We map PTDESC_TABLE_SHIFT at all translation levels and PAGE_SHIFT bits
// in the final page. The maximum number of translation levels supported by
// the architecture is 5. Hence, starting at level n, we have further
// ((4 - n) - 1) levels of translation excluding the offset within the page.
// So, the total number of bits mapped by an entry at level n is :
//
// ((4 - n) - 1) * PTDESC_TABLE_SHIFT + PAGE_SHIFT
//
// Rearranging it a bit we get :
// (4 - n) * PTDESC_TABLE_SHIFT + PTDESC_ORDER
//

//
// PMD_SHIFT determines the size a level 2 page table entry can map.
//

//
// PUD_SHIFT determines the size a level 1 page table entry can map.
//

//
// PGDIR_SHIFT determines the size a top-level page table entry can map
// (depending on the configuration, this level can be -1, 0, 1 or 2).
//

//
// Contiguous page definitions.
//

//
// Hardware page table definitions.
//
// Level -1 descriptor (PGD).
//

//
// Level 0 descriptor (P4D).
//

//
// Level 1 descriptor (PUD).
//

//
// Level 2 descriptor (PMD).
//

//
// Section
//

//
// AttrIndx[2:0] encoding (mapping attributes defined in the MAIR* registers).
//

//
// Level 3 descriptor (PTE).
//

pub const PTE_ADDR_HIGH_SHIFT: c_int = 36;

pub const PTE_ADDR_HIGH_SHIFT: c_int = 42;

//
// AttrIndx[2:0] encoding (mapping attributes defined in the MAIR* registers).
//

//
// PIIndex[3:0] encoding (Permission Indirection Extension)
//

//
// POIndex[2:0] encoding (Permission Overlay Extension)
//

//
// Memory Attribute override for Stage-2 (MemAttr[3:0])
//

//
// Hierarchical permission for Stage-1 tables
//

//
// TCR flags.
//

//
// TTBR.
//

//
// TTBR_ELx[1] is RES0 in this configuration.
//

// Must be at least 64-byte aligned to prevent corruption of the TTBR

