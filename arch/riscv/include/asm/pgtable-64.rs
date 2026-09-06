//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/pgtable-64.h
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
// Copyright (C) 2012 Regents of the University of California
//

pub const PGDIR_SHIFT_L3: c_int = 30;
pub const PGDIR_SHIFT_L4: c_int = 39;
pub const PGDIR_SHIFT_L5: c_int = 48;

// Size of region mapped by a page global directory

// p4d is folded into pgd in case of 4-level page table
pub const P4D_SHIFT_L3: c_int = 30;
pub const P4D_SHIFT_L4: c_int = 39;
pub const P4D_SHIFT_L5: c_int = 39;

// pud is folded into pgd in case of 3-level page table
pub const PUD_SHIFT: c_int = 30;

pub const PMD_SHIFT: c_int = 21;
// Size of region mapped by a page middle directory

// Page 4th Directory entry

// Page Upper Directory entry

// Page Middle Directory entry

pub const MAX_POSSIBLE_PHYSMEM_BITS: c_int = 56;
//
// rv64 PTE format:
// | 63 | 62 61 | 60 54 | 53  10 | 9             8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0
// N      MT     RSV    PFN      reserved for SW   D   A   G   U   X   W   R   V
//

//
// [63] Svnapot definitions:
// 0 Svnapot disabled
// 1 Svnapot enabled
//
pub const _PAGE_NAPOT_SHIFT: c_int = 63;

//
// Only 64KB (order 4) napot ptes supported.
//
pub const NAPOT_CONT_ORDER_BASE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum napot_cont_order {
    NAPOT_CONT64KB_ORDER = NAPOT_CONT_ORDER_BASE,
    NAPOT_ORDER_MAX,
}

pub const HUGE_MAX_HSTATE: c_int = 2;

//
// [62:61] Svpbmt Memory Type definitions:
//
// 00 - PMA    Normal Cacheable, No change to implied PMA memory type
// 01 - NC     Non-cacheable, idempotent, weakly-ordered Main Memory
// 10 - IO     Non-cacheable, non-idempotent, strongly-ordered I/O memory
// 11 - Rsvd   Reserved for future standard use
//

//
// [63:59] T-Head Memory Type definitions:
// bit[63] SO - Strong Order
// bit[62] C - Cacheable
// bit[61] B - Bufferable
// bit[60] SH - Shareable
// bit[59] Sec - Trustable
// 00110 - NC   Weakly-ordered, Non-cacheable, Bufferable, Shareable, Non-trustable
// 01110 - PMA  Weakly-ordered, Cacheable, Bufferable, Shareable, Non-trustable
// 10010 - IO   Strongly-ordered, Non-cacheable, Non-bufferable, Shareable, Non-trustable
//

// Set of bits to preserve across pte_modify()

extern "C" {
    pub fn pud_present(_PAGE_LEAF: pud) && (pud_val(pud) &) -> return;
}
extern "C" {
    pub fn __pud(pgprot_val(prot): (pfn << _PAGE_PFN_SHIFT) |) -> return;
}
extern "C" {
    pub fn __page_val_to_pfn(_arg: pud_val(pud)) -> return;
}
extern "C" {
    pub fn pfn_to_page(_arg: __page_val_to_pfn(pud_val(pud))) -> return;
}

extern "C" {
    pub fn __pmd(prot_val: (pfn << _PAGE_PFN_SHIFT) |) -> return;
}
extern "C" {
    pub fn __page_val_to_pfn(_arg: pmd_val(pmd)) -> return;
}

extern "C" {
    pub fn __p4d(pgprot_val(prot): (pfn << _PAGE_PFN_SHIFT) |) -> return;
}
extern "C" {
    pub fn __page_val_to_pfn(_arg: p4d_val(p4d)) -> return;
}

extern "C" {
    pub fn pfn_to_page(_arg: __page_val_to_pfn(p4d_val(p4d))) -> return;
}

extern "C" {
    pub fn pfn_to_page(_arg: __page_val_to_pfn(pgd_val(pgd))) -> return;
}

extern "C" {
    pub fn pmd_pte(pmd: pmd_t) -> pte_t;
}
extern "C" {
    pub fn pud_pte(pud: pud_t) -> pte_t;
}

