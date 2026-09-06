//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/32/pgtable.h
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
// The "classic" 32-bit implementation of the PowerPC MMU uses a hash
// table containing PTEs, together with a set of 16 segment registers,
// to define the virtual to physical address mapping.
//
// We use the hash table as an extended TLB, i.e. a cache of currently
// active mappings.  We maintain a two-level page table tree, much
// like that used by the i386, for the sake of the Linux memory
// management code.  Low-level assembler code in hash_low_32.S
// (procedure hash_page) is responsible for extracting ptes from the
// tree and putting them into the hash table when necessary, and
// updating the accessed and modified bits in the page table tree.
//
pub const _PAGE_PRESENT: c_uint = 0x001	/* software: pte contains a translation */;
pub const _PAGE_HASHPTE: c_uint = 0x002	/* hash_page has made an HPTE for this pte */;
pub const _PAGE_READ: c_uint = 0x004	/* software: read access allowed */;
pub const _PAGE_GUARDED: c_uint = 0x008	/* G: prohibit speculative access */;
pub const _PAGE_COHERENT: c_uint = 0x010	/* M: enforce memory coherence (SMP systems) */;
pub const _PAGE_NO_CACHE: c_uint = 0x020	/* I: cache inhibit */;
pub const _PAGE_WRITETHRU: c_uint = 0x040	/* W: cache write-through */;
pub const _PAGE_DIRTY: c_uint = 0x080	/* C: page changed */;
pub const _PAGE_ACCESSED: c_uint = 0x100	/* R: page referenced */;
pub const _PAGE_EXEC: c_uint = 0x200	/* software: exec allowed */;
pub const _PAGE_WRITE: c_uint = 0x400	/* software: user write access allowed */;
pub const _PAGE_SPECIAL: c_uint = 0x800	/* software: Special page */;

// We never clear the high word of the pte

pub const _PMD_PRESENT: c_int = 0;

// We borrow the _PAGE_READ bit to store the exclusive marker in swap PTEs.

// And here we include common definitions

//
// Location of the PFN in the PTE. Most 32-bit platforms use the same
// as _PAGE_SHIFT here (ie, naturally aligned).
// Platform who don't just pre-define the value so we don't override it here.
//

//
// The mask covered by the RPN must be a ULL on 32-bit platforms with
// 64-bit PTEs.
//

pub const MAX_POSSIBLE_PHYSMEM_BITS: c_int = 36;

pub const MAX_POSSIBLE_PHYSMEM_BITS: c_int = 32;

//
// _PAGE_CHG_MASK masks of bits that are to be preserved across
// pgprot changes.
//

//
// We define 2 sets of base prot bits, one for basic pages (ie,
// cacheable kernel and user pages) and one for non cacheable
// pages. We always set _PAGE_COHERENT when SMP is enabled or
// the processor might need it for DMA coherency.
//

// Permission masks used for kernel mappings

pub const PMD_INDEX_SIZE: c_int = 0;
pub const PUD_INDEX_SIZE: c_int = 0;

pub const PMD_TABLE_SIZE: c_int = 0;
pub const PUD_TABLE_SIZE: c_int = 0;

// Bits to mask out from a PMD to get to the PTE page

//
// The normal case is that PTEs are 32-bits and we have a 1-page
// 1024-entry pgdir pointing to 1-page 1024-entry PTE pages.  -- paulus
//
// For any >32-bit physical address platform, we can use the following
// two level page table layout where the pgdir is 8KB and the MS 13 bits
// are an index to the second level table.  The combined pgdir/pmd first
// level has 2048 entries and the second level has 512 64-bit PTE entries.
// -Matt
//
// PGDIR_SHIFT determines what a top-level page table entry can map

extern "C" {
    pub fn map_kernel_page(va: c_ulong, pa: phys_addr_t, prot: pgprot_t) -> c_int;
}
extern "C" {
    pub fn unmap_kernel_page(va: c_ulong);
}

//
// This is the bottom of the PKMAP area with HIGHMEM or an arbitrary
// value (for now) on others, from where we can start layout kernel
// virtual space that goes below PKMAP and FIXMAP
//
pub const FIXADDR_SIZE: c_int = 0;

//
// ioremap_bot starts at that address. Early ioremaps move down from there,
// until mem_init() at which point this becomes the top of the vmalloc
// and ioremap space
//

// PPC32 shares vmalloc area with ioremap

//
// Just any arbitrary offset to the start of the vmalloc VM area: the
// current 16MB value just means that there will be a 64MB "hole" after the
// physical memory until the kernel virtual memory starts.  That means that
// any out-of-bounds memory accesses will hopefully be caught.
// The vmalloc() routines leaves a hole of 4kB between each vmalloced
// area for the same reason. ;)
//
// We no longer map larger than phys RAM with the BATs so we don't have
// to worry about the VMALLOC_OFFSET causing problems.  We do have to worry
// about clashes between our early calls to ioremap() that start growing down
// from ioremap_base being run into the VM area allocations (growing upwards
// from VMALLOC_START).  For this reason we have ioremap_bot to check when
// we actually run into our mappings setup in the early boot with the VM
// system.  This really does become a problem for machines with good amounts
// of RAM.  -- Cort
//

// Bits to mask out from a PGD to get to the PUD page
pub const PGD_MASKED_BITS: c_int = 0;

//
// Bits in a linux-style PTE.  These match the bits in the
// (hardware-defined) PowerPC PTE as closely as possible.
//

// pmdp = __pmd(0);
//
// When flushing the tlb entry for a page, we also need to flush the hash
// table entry.  flush_hash_pages is assembler (for speed) in hashtable.S.
//
// Add an HPTE to the hash table
// Flush an entry from the TLB/hash table
//
// PTE updates. This function is called whenever an existing
// valid PTE is updated. This does -not- include set_pte_at()
// which nowadays only sets a new PTE.
//
// Depending on the type of MMU, we may need to use atomic updates
// and the PTE may be either 32 or 64 bit wide. In the later case,
// when using atomic updates, only the low part of the PTE is
// accessed atomically.
//

// p = __pte((old & ~(pte_basic_t)clr) | set);
//
// 2.6 calls this without flushing the TLB entry; this is wrong
// for our hash-based implementation, we fix that up here.
//

//
// Encode/decode swap entries and swap PTEs. Swap PTEs are all PTEs that
// are !pte_none() && !pte_present().
//
// Format of swap PTEs (32bit PTEs):
//
// 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// <----------------- offset --------------------> < type -> E H P
//
// E is the exclusive marker that is not stored in swap entries.
// _PAGE_PRESENT (P) and __PAGE_HASHPTE (H) must be 0.
//
// For 64bit PTEs, the offset is extended by 32bit.
//

extern "C" {
    pub fn __pte(_PAGE_SWP_EXCLUSIVE: pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(~_PAGE_SWP_EXCLUSIVE: pte_val(pte) &) -> return;
}
// Generic accessors to PTE bits
//
// We only find page table entry in the last level
// Hence no need for other accessors
//

//
// A read-only access is controlled by _PAGE_READ bit.
// We have _PAGE_READ set for WRITE
//
extern "C" {
    pub fn pte_present(!is_kernel_addr(addr: pte) &&) -> return;
}
// Conversion functions: convert a page and protection to a page entry,
// and a page entry and page directory to the page they refer to.
//
// Even if PTEs can be unsigned long long, a PFN is always an unsigned
// long for now.
//
// Generic modifiers for PTE bits
extern "C" {
    pub fn __pte(~_PAGE_WRITE: pte_val(pte) &) -> return;
}
extern "C" {
    pub fn __pte(~_PAGE_EXEC: pte_val(pte) &) -> return;
}
extern "C" {
    pub fn __pte(~_PAGE_DIRTY: pte_val(pte) &) -> return;
}
extern "C" {
    pub fn __pte(~_PAGE_ACCESSED: pte_val(pte) &) -> return;
}
extern "C" {
    pub fn __pte(_PAGE_EXEC: pte_val(pte) |) -> return;
}
//
// write implies read, hence set both
//
extern "C" {
    pub fn __pte(_PAGE_RW: pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(_PAGE_DIRTY: pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(_PAGE_ACCESSED: pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(_PAGE_SPECIAL: pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(pgprot_val(newprot): (pte_val(pte) & _PAGE_CHG_MASK) |) -> return;
}
// This low level function performs the actual PTE insertion
// Setting the PTE depends on the MMU type and other factors.
//
// First case is 32-bit in UP mode with 32-bit PTEs, we need to preserve
// the _PAGE_HASHPTE bit since we may not have invalidated the previous
// translation in the hash yet (done in a subsequent flush_tlb_xxx())
// and see we need to keep track that this PTE needs invalidating.
//
// Second case is 32-bit with 64-bit PTE.  In this case, we
// can just store as long as we do the two halves in the right order
// with a barrier in between. This is possible because we take care,
// in the hash code, to pre-invalidate if the PTE was already hashed,
// which synchronizes us with any concurrent invalidation.
// In the percpu case, we fallback to the simple update preserving
// the hash bits (ie, same as the non-SMP case).
//
// Third case is 32-bit in SMP mode with 32-bit PTEs. We use the
// helper pte_update() which does an atomic update. We need to do that
// because a concurrent invalidation can clear _PAGE_HASHPTE. If it's a
// per-CPU PTE such as a kmap_atomic, we also do a simple update preserving
// the hash bits instead.
//
// ptep = __pte((pte_val(*ptep) & _PAGE_HASHPTE) |
//
// Macro to mark a page protection value as "uncacheable".
//

extern "C" {
    pub fn __pgprot(~_PAGE_CACHE_CTL: pgprot_val(prot) &) -> return;
}

extern "C" {
    pub fn pgprot_noncached_wc(_arg: prot) -> return;
}

