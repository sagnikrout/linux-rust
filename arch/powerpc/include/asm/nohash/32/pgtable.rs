//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/nohash/32/pgtable.h
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

pub const PMD_INDEX_SIZE: c_int = 0;
pub const PUD_INDEX_SIZE: c_int = 0;

pub const PMD_TABLE_SIZE: c_int = 0;
pub const PUD_TABLE_SIZE: c_int = 0;

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

// Bits to mask out from a PGD to get to the PUD page
pub const PGD_MASKED_BITS: c_int = 0;

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
// from IOREMAP_TOP being run into the VM area allocations (growing upwards
// from VMALLOC_START).  For this reason we have ioremap_bot to check when
// we actually run into our mappings setup in the early boot with the VM
// system.  This really does become a problem for machines with good amounts
// of RAM.  -- Cort
//

//
// Bits in a linux-style PTE.  These match the bits in the
// (hardware-defined) PowerPC PTE as closely as possible.
//

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

// pmdp = __pmd(0);
//
// Note that on Book E processors, the pmd contains the kernel virtual
// (lowmem) address of the pte page.  The physical address is less useful
// because everything runs with translation enabled (even the TLB miss
// handler).  On everything else the pmd contains the physical address
// of the pte page.  -- paulus
//

//
// Encode/decode swap entries and swap PTEs. Swap PTEs are all PTEs that
// are !pte_none() && !pte_present().
//
// Format of swap PTEs (32bit PTEs):
//
// 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// <------------------ offset -------------------> < type -> E 0 0
//
// E is the exclusive marker that is not stored in swap entries.
//
// For 64bit PTEs, the offset is extended by 32bit.
//

// We borrow LSB 2 to store the exclusive marker in swap PTEs.
pub const _PAGE_SWP_EXCLUSIVE: c_uint = 0x000004;

