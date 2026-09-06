//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/64/hash.h
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
// Common bits between 4K and 64K pages in a linux-style PTE.
// Additional bits may be defined in pgtable-hash64-*.h
//

// Bits to set in a PMD/PUD/PGD entry valid bit

//
// Size of EA range mapped by our pagetables.
//

//
// Top 2 bits are ignored in page table walk.
//

//
// We store the slot details in the second half of page table.
// Increase the pud level table so that hugetlb ptes can be stored
// at pud level.
//

//
// +------------------------------+
// |                              |
// +------------------------------+  Kernel virtual map end (0xc00e000000000000)
// |                              |
// |      512TB/16TB of vmemmap   |
// |                              |
// +------------------------------+  Kernel vmemmap  start
// |                              |
// |      512TB/16TB of IO map    |
// |                              |
// +------------------------------+  Kernel IO map start
// |                              |
// |      512TB/16TB of vmap      |
// |                              |
// +------------------------------+  Kernel virt start (0xc008000000000000)
// |                              |
// +------------------------------+  Kernel linear (0xc.....)
//

//
// Region IDs
//
pub const USER_REGION_ID: c_int = 0;
pub const LINEAR_MAP_REGION_ID: c_int = 1;

//
// Defines the address of the vmemap area, in its own region on
// hash table CPUs.
//
// PTEIDX nibble
pub const _PTEIDX_SECONDARY: c_uint = 0x8;
pub const _PTEIDX_GROUP_IX: c_uint = 0x7;

//
// pud comparison that will work with both pte and page table pointer.
//

extern "C" {
    pub fn hash__mark_rodata_ro();
}
extern "C" {
    pub fn hash__mark_initmem_nx();
}

extern "C" {
    pub fn htab_convert_pte_flags(pteflags: c_ulong, flags: c_ulong) -> c_ulong;
}
// Atomic PTE updates
extern "C" {
    pub fn be64_to_cpu(_arg: old_be) -> return;
}
// huge pages use the old page table lock
// Set the dirty and/or accessed bits atomically in a linux PTE, this
// function doesn't need to flush the hash entry
//
// This low level function performs the actual PTE insertion
// Setting the PTE depends on the MMU type and other factors. It's
// an horrible mess that I'm not going to try to clean up now but
// I'm keeping it in one place rather than spread around
//
// Anything else just stores the PTE normally. That covers all 64-bit
// cases, and 32-bit non-hash with 32-bit PTEs.
//
// ptep = pte;

extern "C" {
    pub fn hash__map_kernel_page(ea: c_ulong, pa: c_ulong, prot: pgprot_t) -> c_int;
}
extern "C" {
    pub fn hash__remove_section_mapping(start: c_ulong, end: c_ulong) -> c_int;
}

