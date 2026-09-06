//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/swapops.h
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
// swapcache pages are stored in the swapper_space radix tree.  We want to
// get good packing density in that tree, so the index should be dense in
// the low-order bits.
//
// We arrange the `type' and `offset' fields so that `type' is at the six
// high-order bits of the swp_entry_t and `offset' is right-aligned in the
// remaining bits.  Although `type' itself needs only five bits, we allow for
// shmem/tmpfs to shift it all up a further one bit: see swp_to_radix_entry().
//
// swp_entry_t's are *never* stored anywhere in their arch-dependent format.
//

//
// Definitions only for PFN swap entries (see leafeant_has_pfn()).  To
// store PFN, we only need SWP_PFN_BITS bits.  Each of the pfn swap entries
// can use the extra bits to store other information besides PFN.
//

//
// Migration swap entry specific bitfield definitions.  Layout:
//
// |----------+--------------------|
// | swp_type | swp_offset         |
// |----------+--------+-+-+-------|
// |          | resv   |D|A|  PFN  |
// |----------+--------+-+-+-------|
//
// @SWP_MIG_YOUNG_BIT: Whether the page used to have young bit set (bit A)
// @SWP_MIG_DIRTY_BIT: Whether the page used to have dirty bit set (bit D)
//
// Note: A/D bits will be stored in migration entries iff there're enough
// free bits in arch specific swp offset.  By default we'll ignore A/D bits
// when migrating a page.  Please refer to migration_entry_supports_ad()
// for more information.  If there're more bits besides PFN and A/D bits,
// they should be reserved and always be zeros.
//

// Clear all flags but only keep swp_entry_t related information
//
// Store a type+offset into a swp_entry_t in an arch-independent format
//
// Extract the `type' field from a swp_entry_t.  The swp_entry_t is in
// arch-independent format
//
// Extract the `offset' field from a swp_entry_t.  The swp_entry_t is in
// arch-independent format
//
// Convert the arch-independent representation of a swp_entry_t into the
// arch-dependent pte representation.
//
extern "C" {
    pub fn __swp_entry_to_pte(_arg: arch_entry) -> return;
}
extern "C" {
    pub fn xa_mk_value(_arg: entry.val) -> return;
}

extern "C" {
    pub fn swp_entry(_arg: SWP_DEVICE_READ, _arg: offset) -> return;
}
extern "C" {
    pub fn swp_entry(_arg: SWP_DEVICE_WRITE, _arg: offset) -> return;
}
extern "C" {
    pub fn swp_entry(_arg: SWP_DEVICE_EXCLUSIVE, _arg: offset) -> return;
}

extern "C" {
    pub fn swp_entry(_arg: 0, _arg: 0) -> return;
}
extern "C" {
    pub fn swp_entry(_arg: 0, _arg: 0) -> return;
}
extern "C" {
    pub fn swp_entry(_arg: 0, _arg: 0) -> return;
}

extern "C" {
    pub fn swp_entry(_arg: SWP_MIGRATION_READ, _arg: offset) -> return;
}
extern "C" {
    pub fn swp_entry(_arg: SWP_MIGRATION_READ_EXCLUSIVE, _arg: offset) -> return;
}
extern "C" {
    pub fn swp_entry(_arg: SWP_MIGRATION_WRITE, _arg: offset) -> return;
}
//
// Returns whether the host has large enough swap offset field to support
// carrying over pgtable A/D bits for page migrations.  The result is
// pretty much arch specific.
//

extern "C" {
    pub fn migration_entry_wait_huge(vma: *mut vm_area_struct, addr: c_ulong, pte: *mut pte_t);
}

extern "C" {
    pub fn swp_entry(_arg: 0, _arg: 0) -> return;
}
extern "C" {
    pub fn swp_entry(_arg: 0, _arg: 0) -> return;
}
extern "C" {
    pub fn swp_entry(_arg: 0, _arg: 0) -> return;
}

//
// Support for hardware poisoned pages
//
extern "C" {
    pub fn swp_entry(_arg: SWP_HWPOISON, _arg: page_to_pfn(page)) -> return;
}

extern "C" {
    pub fn swp_entry(_arg: 0, _arg: 0) -> return;
}

pub type pte_marker = c_ulong;

//
// "Poisoned" here is meant in the very general sense of "future accesses are
// invalid", instead of referring very specifically to hardware memory errors.
// This marker is meant to represent any of various different causes of this.
//
// Note that, when encountered by the faulting logic, PTEs with this marker will
// result in VM_FAULT_HWPOISON and thus regardless trigger hardware memory error
// logic.
//

//
// Indicates that, on fault, this PTE will case a SIGSEGV signal to be
// sent. This means guard markers behave in effect as if the region were mapped
// PROT_NONE, rather than if they were a memory hole or equivalent.
//

extern "C" {
    pub fn swp_entry(_arg: SWP_PTE_MARKER, _arg: marker) -> return;
}
extern "C" {
    pub fn swp_entry_to_pte(_arg: make_pte_marker_entry(marker)) -> return;
}
extern "C" {
    pub fn make_pte_marker_entry(_arg: PTE_MARKER_POISONED) -> return;
}
extern "C" {
    pub fn make_pte_marker_entry(_arg: PTE_MARKER_GUARD) -> return;
}

extern "C" {
    pub fn pmd_migration_entry_wait(mm: *mut mm_struct, pmd: *mut pmd_t);
}
extern "C" {
    pub fn __swp_entry_to_pmd(_arg: arch_entry) -> return;
}

extern "C" {
    pub fn __pmd(_arg: 0) -> return;
}

