//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/64/hash-4k.h
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
// Each context is 512TB. But on 4k we restrict our max TASK size to 64TB
// Hence also limit max EA bits to 64TB.
//
pub const MAX_EA_BITS_PER_CONTEXT: c_int = 46;
//
// Our page table limit us to 64TB. For 64TB physical memory, we only need 64GB
// of vmemmap space. To better support sparse memory layout, we use 61TB
// linear map range, 1TB of vmalloc, 1TB of I/O and 1TB of vmememmap.
//

//
// Limits the linear mapping range
//
pub const H_MAX_PHYSMEM_BITS: c_int = 46;
//
// Define the address range of the kernel non-linear virtual area (61TB)
//

// PTE flags to conserve for HPTE identification

//
// Not supported by 4k linux page size
//
pub const H_PAGE_4K_PFN: c_uint = 0x0;
pub const H_PAGE_THP_HUGE: c_uint = 0x0;
pub const H_PAGE_COMBO: c_uint = 0x0;
// 8 bytes per each pte entry

// memory key bits, only 8 keys supported
pub const H_PTE_PKEY_BIT4: c_int = 0;
pub const H_PTE_PKEY_BIT3: c_int = 0;

//
// On all 4K setups, remap_4k_pfn() equates to remap_pfn_range()
//

//
// With 4K page size the real_pte machinery is all nops.
//

//
// We expect this to be called only for user addresses or kernel virtual
// addresses other than the linear mapping.
//

//
// 4K PTE format is different from 64K PTE format. Saving the hash_slot is just
// a matter of returning the PTE bits that need to be modified. On 64K PTE,
// things are a little more involved and hence needs many more parameters to
// accomplish the same. However we want to abstract this out from the caller by
// keeping the prototype consistent across the two formats.
//

extern "C" {
    pub fn hash__pgtable_trans_huge_withdraw(mm: *mut mm_struct, pmdp: *mut pmd_t) -> pgtable_t;
}
extern "C" {
    pub fn hash__has_transparent_hugepage() -> c_int;
}

