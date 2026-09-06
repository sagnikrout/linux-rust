//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/mmu.h
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
// MMU features bit definitions
//
// MMU families
//

// Radix page table supported and enabled

//
// Individual features below.
//
// Supports KUAP feature
// key 0 controlling userspace addresses on radix
// Key 3 on hash
//

//
// Supports KUEP feature
// key 0 controlling userspace addresses on radix
// Key 3 on hash
//

//
// Support for memory protection keys.
//

// Guest Translation Shootdown Enable

//
// Support for 68 bit VA space. We added that from ISA 2.05
//

//
// Kernel read only support.
// We added the ppp value 0b110 in ISA 2.04.
//

//
// We need to clear top 16bits of va (from the remaining 64 bits )in
// tlbie* instructions
//

// Enable use of high BAT registers

// Enable >32-bit physical addresses on 32-bit processor, only used
// by CONFIG_PPC_BOOK3S_32 currently as BookE supports that from day 1
//

// Enable use of broadcast TLB invalidations. We don't always set it
// on processors that support it due to other constraints with the
// use of such invalidations
//

// Enable use of tlbilx invalidate instructions.
//

// This indicates that the processor cannot handle multiple outstanding
// broadcast tlbivax or tlbsync. This makes the code use a spinlock
// around such invalidate forms.
//

// This indicates that the processor doesn't handle way selection
// properly and needs SW to track and update the LRU state.  This
// is specific to an errata on e300c2/c3/c4 class parts
//

// Doesn't support the B bit (1T segment) in SLBIE
//

// Support 16M large pages
//

// Supports TLBIEL variant
//

// Supports tlbies w/o locking
//

// Large pages can be marked CI
//

// 1T segments available
//

// NX paste RMA reject in DSI

// MMU feature bit sets for various CPUs

// BOOK3S_64 options

pub const MMU_FTRS_ALWAYS: c_int = 0;

pub const NUM_MMU_FTR_KEYS: c_int = 32;
extern "C" {
    pub fn mmu_feature_keys_init();
}

extern "C" {
    pub fn early_mmu_has_feature(_arg: feature) -> return;
}

extern "C" {
    pub fn static_branch_likely(_arg: &mmu_feature_keys[i]) -> return;
}

extern "C" {
    pub fn early_mmu_has_feature(_arg: feature) -> return;
}

// This is our real memory area size on ppc64 server, on embedded, we
// make it match the size our of bolted TLB area
//
// Cleanup function used by kexec
extern "C" {
    pub fn mmu_cleanup_all();
}
extern "C" {
    pub fn radix__mmu_cleanup_all();
}
// Functions for creating and updating partition table on POWER9
extern "C" {
    pub fn mmu_partition_table_init();
}

extern "C" {
    pub fn assert_pte_locked(mm: *mut mm_struct, addr: c_ulong);
}

extern "C" {
    pub fn mmu_has_feature(_arg: MMU_FTR_TYPE_RADIX) -> return;
}
extern "C" {
    pub fn early_mmu_has_feature(_arg: MMU_FTR_TYPE_RADIX) -> return;
}
extern "C" {
    pub fn IS_ENABLED(strict_kernel_rwx_enabled(: CONFIG_STRICT_MODULE_RWX) &&) -> return;
}

// The kernel use the constants below to index in the page sizes array.
// The use of fixed constants for this purpose is better for performances
// of the low level hash refill handlers.
//
// A non supported page size has a "shift" field set to 0
//
// Any new page size being implemented can get a new entry in here. Whether
// the kernel will use it or not is a different matter though. The actual page
// size used by hugetlbfs is not defined here and may be made variable
//
// Note: This array ended up being a false good idea as it's growing to the
// point where I wonder if we should replace it with something different,
// to think about, feedback welcome. --BenH.
//
// These are #defines as they have to be used in assembly
pub const MMU_PAGE_4K: c_int = 0;
pub const MMU_PAGE_16K: c_int = 1;
pub const MMU_PAGE_64K: c_int = 2;

pub const MMU_PAGE_256K: c_int = 4;
pub const MMU_PAGE_512K: c_int = 5;
pub const MMU_PAGE_1M: c_int = 6;
pub const MMU_PAGE_2M: c_int = 7;
pub const MMU_PAGE_4M: c_int = 8;
pub const MMU_PAGE_8M: c_int = 9;
pub const MMU_PAGE_16M: c_int = 10;
pub const MMU_PAGE_64M: c_int = 11;
pub const MMU_PAGE_256M: c_int = 12;
pub const MMU_PAGE_1G: c_int = 13;
pub const MMU_PAGE_16G: c_int = 14;
pub const MMU_PAGE_64G: c_int = 15;
//
// N.B. we need to change the type of hpte_page_sizes if this gets to be > 16
// Also we need to change he type of mm_context.low/high_slices_psize.
//
pub const MMU_PAGE_COUNT: c_int = 16;

// MMU initialization
extern "C" {
    pub fn early_init_mmu();
}
extern "C" {
    pub fn early_init_mmu_secondary();
}

// 32-bit classic hash table MMU

