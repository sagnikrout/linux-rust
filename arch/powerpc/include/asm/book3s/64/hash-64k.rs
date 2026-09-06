//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/64/hash-64k.h
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
// If we store section details in page->flags we can't increase the MAX_PHYSMEM_BITS
// if we increase SECTIONS_WIDTH we will not store node details in page->flags and
// page_to_nid does a page->section->node lookup
// Hence only increase for VMEMMAP. Further depending on SPARSEMEM_EXTREME reduce
// memory requirements with large number of sections.
// 51 bits is the max physical real address on POWER9
//

pub const H_MAX_PHYSMEM_BITS: c_int = 51;

pub const H_MAX_PHYSMEM_BITS: c_int = 46;

//
// Each context is 512TB size. SLB miss for first context/default context
// is handled in the hotpath.
//
pub const MAX_EA_BITS_PER_CONTEXT: c_int = 49;

//
// We use one context for each MAP area.
//

//
// Define the address range of the kernel non-linear virtual area
// 2PB
//

//
// 64k aligned address free up few of the lower bits of RPN for us
// We steal that here. For more deatils look at pte_pfn/pfn_pte()
//

// memory key bits.

//
// We need to differentiate between explicit huge page and THP huge
// page, since THP huge page also need to track real subpage details
//

// PTE flags to conserve for HPTE identification

//
// We use a 2K PTE page fragment and another 2K for storing
// real_pte_t hash index
// 8 bytes per each pte entry and another 8 bytes for storing
// slot details.
//

//
// With 64K pages on hash table, we have a special PTE format that
// uses a second "half" of the page table to encode sub-page information
// in order to deal with 64K made of 4K HW pages. Thus we override the
// generic accessors and iterators here
//

//
// Ensure that we do not read the hidx before we read the PTE. Because
// the writer side is expected to finish writing the hidx first followed
// by the PTE, by using smp_wmb(). pte_set_hash_slot() ensures that.
//
// shift the hidx representation by one-modulo-0xf; i.e hidx 0 is respresented
// as 1, 1 as 2,... , and 0xf as 0.  This convention lets us represent a
// invalid hidx 0xf with a 0x0 bit value. PTEs are anyway zero'd when
// allocated. We dont have to zero them gain; thus save on the initialization.
//

pub const INVALID_RPTE_HIDX: c_uint = 0x0UL;
extern "C" {
    pub fn HIDX_UNSHIFT_BY_ONE(_arg: BITS_TO_HIDX(rpte.hidx, _arg: index)) -> return;
}
//
// Commit the hidx and return PTE bits that needs to be modified. The caller is
// expected to modify the PTE bits accordingly and commit the PTE to memory.
//
// hidxp = rpte.hidx  | HIDX_BITS(HIDX_SHIFT_BY_ONE(hidx), subpg_index);
//
// Anyone reading PTE must ensure hidx bits are read after reading the
// PTE by using the read-side barrier smp_rmb(). __real_pte() can be
// used for that.
//
// No PTE bits to be modified, return 0x0UL

extern "C" {
    pub fn __rpte_sub_valid(rpte: real_pte_t, index: c_ulong) -> bool;
}
//
// Trick: we set __end to va + 64k, which happens works for
// a 16M page as well as we want only one iteration
//

//
// The hpte hindex is stored in the pgtable whose address is in the
// second half of the PMD
//
// Order this load with the test for pmd_trans_huge in the caller
//
// The linux hugepage PMD now include the pmd entries followed by the address
// to the stashed pgtable_t. The stashed pgtable_t contains the hpte bits.
// [ 000 | 1 bit secondary | 3 bit hidx | 1 bit valid]. We use one byte per
// each HPTE entry. With 16MB hugepage and 64K HPTE we need 256 entries and
// with 4K HPTE we need 4096 entries. Both will fit in a 4K pgtable_t.
//
// The top three bits are intentionally left as zero. This memory location
// are also used as normal page PTE pointers. So if we have any pointers
// left around while we collapse a hugepage, we need to make sure
// _PAGE_PRESENT bit of that is zero when we look at them
//
// For core kernel code by design pmd_trans_huge is never run on any hugetlbfs
// page. The hugetlbfs page table walking and mangling paths are totally
// separated form the core VM paths and they're differentiated by
// VM_HUGETLB being set on vm_flags well before any pmd_trans_huge could run.
//
// pmd_trans_huge() is defined as false at build time if
// CONFIG_TRANSPARENT_HUGEPAGE=n to optimize away code blocks at build
// time in such case.
//
// For ppc64 we need to differntiate from explicit hugepages from THP, because
// for THP we also track the subpage details at the pmd level. We don't do
// that for explicit huge pages.
//
extern "C" {
    pub fn __pmd(H_PAGE_THP_HUGE): pmd_val(pmd) | (_PAGE_PTE |) -> return;
}
extern "C" {
    pub fn hash__pgtable_trans_huge_withdraw(mm: *mut mm_struct, pmdp: *mut pmd_t) -> pgtable_t;
}
extern "C" {
    pub fn hash__has_transparent_hugepage() -> c_int;
}

