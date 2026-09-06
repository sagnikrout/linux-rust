//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/tlb.h
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
// TLB flushing on s390 is complicated. The following requirement
// from the principles of operation is the most arduous:
//
// "A valid table entry must not be changed while it is attached
// to any CPU and may be used for translation by that CPU except to
// (1) invalidate the entry by using INVALIDATE PAGE TABLE ENTRY,
// or INVALIDATE DAT TABLE ENTRY, (2) alter bits 56-63 of a page
// table entry, or (3) make a change by means of a COMPARE AND SWAP
// AND PURGE instruction that purges the TLB."
//
// The modification of a pte of an active mm struct therefore is
// a two step process: i) invalidate the pte, ii) store the new pte.
// This is true for the page protection bit as well.
// The only possible optimization is to flush at the beginning of
// a tlb_gather_mmu cycle if the mm_struct is currently not in use.
//
// Pages used for the page tables is a different story. FIXME: more
//
extern "C" {
    pub fn tlb_flush(tlb: *mut mmu_gather);
}

//
// Release the page cache reference for a pte removed by
// tlb_ptep_clear_flush. In both flush modes the tlb for a page cache page
// has already been freed, so just do free_folio_and_swap_cache.
//
// s390 doesn't delay rmap removal.
//
// pte_free_tlb frees a pte table and clears the CRSTE for the
// page table from the tlb.
//
// pmd_free_tlb frees a pmd table and clears the CRSTE for the
// segment table entry from the tlb.
// If the mm uses a two level page table the single pmd is freed
// as the pgd. pmd_free_tlb checks the asce_limit against 2GB
// to avoid the double free of the pmd in this case.
//
// p4d_free_tlb frees a pud table and clears the CRSTE for the
// region second table entry from the tlb.
// If the mm uses a four level page table the single p4d is freed
// as the pgd. p4d_free_tlb checks the asce_limit against 8PB
// to avoid the double free of the p4d in this case.
//
// pud_free_tlb frees a pud table and clears the CRSTE for the
// region third table entry from the tlb.
// If the mm uses a three level page table the single pud is freed
// as the pgd. pud_free_tlb checks the asce_limit against 4TB
// to avoid the double free of the pud in this case.
//
