//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pgtable_types.h
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

// Shared with _PAGE_BIT_UFFD which is not supported on 32 bit

// If _PAGE_BIT_PRESENT is clear, we use these:
// - if the user mapped it with PROT_NONE; pte_present gives true

pub const _PAGE_KNL_ERRATUM_MASK: c_int = 0;

//
// Tracking soft dirty bit when a page goes to a swap is tricky.
// We need a bit which can be stored in pte _and_ not conflict
// with swap entry format. On x86 bits 1-4 are *not* involved
// into swap entry computation, but bit 7 is used for thp migration,
// so we borrow bit 1 for soft dirty tracking.
//
// Please note that this bit must be treated as swap dirty page
// mark if and only if the PTE/PMD has present bit clear!
//

//
// The hardware requires shadow stack to be Write=0,Dirty=1. However,
// there are valid cases where the kernel might create read-only PTEs that
// are dirty (e.g., fork(), mprotect(), userfaultfd, soft-dirty tracking). In
// this case, the _PAGE_SAVED_DIRTY bit is used instead of the HW-dirty bit,
// to avoid creating a wrong "shadow stack" PTEs. Such PTEs have
// (Write=0,SavedDirty=1,Dirty=0) set.
//

//
// Set of bits not changed in pte_modify.  The pte's
// protection key is treated like _PAGE_RW, for
// instance, and is *not* included in this mask since
// pte_modify() does modify it.
//

//
// The cache modes defined here are used to translate between pure SW usage
// and the HW defined cache mode bits and/or PAT entries.
//
// The resulting bits for PWT, PCD and PAT should be chosen in a way
// to have the WB mode at index 0 (all bits clear). This is the default
// right now and likely would break too much if changed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum page_cache_mode {
    _PAGE_CACHE_MODE_WB       = 0,
    _PAGE_CACHE_MODE_WC       = 1,
    _PAGE_CACHE_MODE_UC_MINUS = 2,
    _PAGE_CACHE_MODE_UC       = 3,
    _PAGE_CACHE_MODE_WT       = 4,
    _PAGE_CACHE_MODE_WP       = 5,

    _PAGE_CACHE_MODE_NUM      = 8
}

//
// Page tables needs to have Write=1 in order for any lower PTEs to be
// writable. This includes shadow stack memory (Write=0, Dirty=1)
//

//
// early identity mapping  pte attrib macros.
//

pub const PTE_IDENT_ATTR: c_uint = 0x003		/* PRESENT+RW */;
pub const PDE_IDENT_ATTR: c_uint = 0x063		/* PRESENT+RW+DIRTY+ACCESSED */;
pub const PGD_IDENT_ATTR: c_uint = 0x001		/* PRESENT (no other attributes) */;

// Extracts the PFN from a (pte|pmd|pud|pgd)val_t of a 4KB page

//
// Extracts the flags from a (pte|pmd|pud|pgd)val_t
// This includes the protection key value.
//

extern "C" {
    pub fn __pgprot(_PAGE_NX: pgprot_val(prot) |) -> return;
}

//
// PHYSICAL_PAGE_MASK might be non-constant when SME is compiled in, so we can't
// use it here.
//

//
// PAE allows Base Address, P, PWT, PCD and AVL bits to be set in PGD entries.
// All other bits are Reserved MBZ
//

// No need to mask any bits for !PAE

extern "C" {
    pub fn native_pgd_val(_arg: p4d.pgd) -> return;
}

extern "C" {
    pub fn native_pgd_val(_arg: pud.p4d.pgd) -> return;
}

extern "C" {
    pub fn native_pgd_val(_arg: pmd.pud.p4d.pgd) -> return;
}

// No 512 GiB huge pages yet
extern "C" {
    pub fn native_p4d_val(p4d_flags_mask(p4d: p4d) &) -> return;
}
extern "C" {
    pub fn native_pud_val(pud_flags_mask(pud: pud) &) -> return;
}
extern "C" {
    pub fn native_pmd_val(pmd_flags_mask(pmd: pmd) &) -> return;
}

extern "C" {
    pub fn cachemode2protval(pcm: page_cache_mode) -> c_ulong;
}
extern "C" {
    pub fn __pgprot(_arg: protval_4k_2_large(pgprot_val(pgprot))) -> return;
}
extern "C" {
    pub fn __pgprot(_arg: protval_large_2_4k(pgprot_val(pgprot))) -> return;
}

extern "C" {
    pub fn pgprot_writecombine(prot: pgprot_t) -> pgprot_t;
}

extern "C" {
    pub fn pgprot_writethrough(prot: pgprot_t) -> pgprot_t;
}
// Indicate that x86 has its own track and untrack pfn vma functions
// Install a pte for a particular vaddr in kernel space.
extern "C" {
    pub fn set_pte_vaddr(vaddr: c_ulong, pte: pte_t);
}

extern "C" {
    pub fn native_pagetable_init();
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pg_level {
    PG_LEVEL_NONE,
    PG_LEVEL_4K,
    PG_LEVEL_2M,
    PG_LEVEL_1G,
    PG_LEVEL_512G,
    PG_LEVEL_256T,
    PG_LEVEL_NUM
}

extern "C" {
    pub fn update_page_count(level: c_int, pages: c_ulong);
}

//
// Helper function that returns the kernel pagetable entry controlling
// the virtual address 'address'. NULL means no pagetable entry present.
// NOTE: the return type is pte_t but if the pmd is PSE then we return it
// as a pte too.
//
extern "C" {
    pub fn slow_virt_to_phys(__address: *mut c_void) -> phys_addr_t;
}

