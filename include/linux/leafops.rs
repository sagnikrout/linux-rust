//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/leafops.h
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
// Describes operations that can be performed on software-defined page table
// leaf entries. These are abstracted from the hardware page table entries
// themselves by the softleaf_t type, see mm_types.h.
//

// Temporary until swp_entry_t eliminated.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum softleaf_type {
// Fundamental types.
    SOFTLEAF_NONE,
    SOFTLEAF_SWAP,
// Migration types.
    SOFTLEAF_MIGRATION_READ,
    SOFTLEAF_MIGRATION_READ_EXCLUSIVE,
    SOFTLEAF_MIGRATION_WRITE,
// Device types.
    SOFTLEAF_DEVICE_PRIVATE_READ,
    SOFTLEAF_DEVICE_PRIVATE_WRITE,
    SOFTLEAF_DEVICE_EXCLUSIVE,
// H/W posion types.
    SOFTLEAF_HWPOISON,
// Marker types.
    SOFTLEAF_MARKER,
}

//
// softleaf_mk_none() - Create an empty ('none') leaf entry.
// Returns: empty leaf entry.
//
// softleaf_from_pte() - Obtain a leaf entry from a PTE entry.
// @pte: PTE entry.
//
// If @pte is present (therefore not a leaf entry) the function returns an empty
// leaf entry. Otherwise, it returns a leaf entry.
//
// Returns: Leaf entry.
//
extern "C" {
    pub fn softleaf_mk_none() -> return;
}
// Temporary until swp_entry_t eliminated.
extern "C" {
    pub fn swp_entry(_arg: __swp_type(arch_entry), _arg: __swp_offset(arch_entry)) -> return;
}
//
// softleaf_to_pte() - Obtain a PTE entry from a leaf entry.
// @entry: Leaf entry.
//
// This generates an architecture-specific PTE entry that can be utilised to
// encode the metadata the leaf entry encodes.
//
// Returns: Architecture-specific PTE entry encoding leaf entry.
//
// Temporary until swp_entry_t eliminated.
extern "C" {
    pub fn swp_entry_to_pte(_arg: entry) -> return;
}

//
// softleaf_from_pmd() - Obtain a leaf entry from a PMD entry.
// @pmd: PMD entry.
//
// If @pmd is present (therefore not a leaf entry) the function returns an empty
// leaf entry. Otherwise, it returns a leaf entry.
//
// Returns: Leaf entry.
//
extern "C" {
    pub fn softleaf_mk_none() -> return;
}
// Temporary until swp_entry_t eliminated.
extern "C" {
    pub fn swp_entry(_arg: __swp_type(arch_entry), _arg: __swp_offset(arch_entry)) -> return;
}
//
// softleaf_to_pmd() - Obtain a PMD entry from a leaf entry.
// @entry: Leaf entry.
//
// This generates an architecture-specific PMD entry that can be utilised to
// encode the metadata the leaf entry encodes.
//
// Returns: Architecture-specific PMD entry encoding leaf entry.
//
// Temporary until swp_entry_t eliminated.
extern "C" {
    pub fn swp_entry_to_pmd(_arg: entry) -> return;
}

extern "C" {
    pub fn softleaf_mk_none() -> return;
}
extern "C" {
    pub fn __pmd(_arg: 0) -> return;
}

//
// softleaf_is_none() - Is the leaf entry empty?
// @entry: Leaf entry.
//
// Empty entries are typically the result of a 'none' page table leaf entry
// being converted to a leaf entry.
//
// Returns: true if the entry is empty, false otherwise.
//
// softleaf_type() - Identify the type of leaf entry.
// @entry: Leaf entry.
//
// Returns: the leaf entry type associated with @entry.
//

// Unknown entry type.
//
// softleaf_is_swap() - Is this leaf entry a swap entry?
// @entry: Leaf entry.
//
// Returns: true if the leaf entry is a swap entry, otherwise false.
//
// softleaf_is_migration_write() - Is this leaf entry a writable migration entry?
// @entry: Leaf entry.
//
// Returns: true if the leaf entry is a writable migration entry, otherwise
// false.
//
// softleaf_is_migration_read() - Is this leaf entry a readable migration entry?
// @entry: Leaf entry.
//
// Returns: true if the leaf entry is a readable migration entry, otherwise
// false.
//
// softleaf_is_migration_read_exclusive() - Is this leaf entry an exclusive
// readable migration entry?
// @entry: Leaf entry.
//
// Returns: true if the leaf entry is an exclusive readable migration entry,
// otherwise false.
//
// softleaf_is_migration() - Is this leaf entry a migration entry?
// @entry: Leaf entry.
//
// Returns: true if the leaf entry is a migration entry, otherwise false.
//
// softleaf_is_device_private_write() - Is this leaf entry a device private
// writable entry?
// @entry: Leaf entry.
//
// Returns: true if the leaf entry is a device private writable entry, otherwise
// false.
//
// softleaf_is_device_private() - Is this leaf entry a device private entry?
// @entry: Leaf entry.
//
// Returns: true if the leaf entry is a device private entry, otherwise false.
//
// softleaf_is_device_exclusive() - Is this leaf entry a device-exclusive entry?
// @entry: Leaf entry.
//
// Returns: true if the leaf entry is a device-exclusive entry, otherwise false.
//
// softleaf_is_hwpoison() - Is this leaf entry a hardware poison entry?
// @entry: Leaf entry.
//
// Returns: true if the leaf entry is a hardware poison entry, otherwise false.
//
// softleaf_is_marker() - Is this leaf entry a marker?
// @entry: Leaf entry.
//
// Returns: true if the leaf entry is a marker entry, otherwise false.
//
// softleaf_to_marker() - Obtain marker associated with leaf entry.
// @entry: Leaf entry, softleaf_is_marker(@entry) must return true.
//
// Returns: Marker associated with the leaf entry.
//
// softleaf_has_pfn() - Does this leaf entry encode a valid PFN number?
// @entry: Leaf entry.
//
// A pfn swap entry is a special type of swap entry that always has a pfn stored
// in the swap offset. They can either be used to represent unaddressable device
// memory, to restrict access to a page undergoing migration or to represent a
// pfn which has been hwpoisoned and unmapped.
//
// Returns: true if the leaf entry encodes a PFN, otherwise false.
//
// Make sure the swp offset can always store the needed fields.
//
// softleaf_to_pfn() - Obtain PFN encoded within leaf entry.
// @entry: Leaf entry, softleaf_has_pfn(@entry) must return true.
//
// Returns: The PFN associated with the leaf entry.
//
// Temporary until swp_entry_t eliminated.
//
// Ensure we do not race with split, which might alter tail pages into new
// folios and thus result in observing an unlocked folio.
// This matches the write barrier in __split_folio_to_order().
//
// Any use of migration entries may only occur while the
// corresponding page is locked
//
// softleaf_to_page() - Obtains struct page for PFN encoded within leaf entry.
// @entry: Leaf entry, softleaf_has_pfn(@entry) must return true.
//
// Returns: Pointer to the struct page associated with the leaf entry's PFN.
//
// softleaf_to_folio() - Obtains struct folio for PFN encoded within leaf entry.
// @entry: Leaf entry, softleaf_has_pfn(@entry) must return true.
//
// Returns: Pointer to the struct folio associated with the leaf entry's PFN.
//
// softleaf_is_poison_marker() - Is this leaf entry a poison marker?
// @entry: Leaf entry.
//
// The poison marker is set via UFFDIO_POISON. Userfaultfd-specific.
//
// Returns: true if the leaf entry is a poison marker, otherwise false.
//
// softleaf_is_guard_marker() - Is this leaf entry a guard region marker?
// @entry: Leaf entry.
//
// Returns: true if the leaf entry is a guard marker, otherwise false.
//
// softleaf_is_uffd_wp_marker() - Is this leaf entry a userfautlfd write protect
// marker?
// @entry: Leaf entry.
//
// Userfaultfd-specific.
//
// Returns: true if the leaf entry is a UFFD WP marker, otherwise false.
//

//
// softleaf_is_migration_young() - Does this migration entry contain an accessed
// bit?
// @entry: Leaf entry.
//
// If the architecture can support storing A/D bits in migration entries, this
// determines whether the accessed (or 'young') bit was set on the migrated page
// table entry.
//
// Returns: true if the entry contains an accessed bit, otherwise false.
//
// Keep the old behavior of aging page after migration
//
// softleaf_is_migration_dirty() - Does this migration entry contain a dirty bit?
// @entry: Leaf entry.
//
// If the architecture can support storing A/D bits in migration entries, this
// determines whether the dirty bit was set on the migrated page table entry.
//
// Returns: true if the entry contains a dirty bit, otherwise false.
//
// Keep the old behavior of clean page after migration

//
// pte_is_marker() - Does the PTE entry encode a marker leaf entry?
// @pte: PTE entry.
//
// Returns: true if this PTE is a marker leaf entry, otherwise false.
//
extern "C" {
    pub fn softleaf_is_marker(_arg: softleaf_from_pte(pte)) -> return;
}
//
// pte_is_uffd_wp_marker() - Does this PTE entry encode a userfaultfd write
// protect marker leaf entry?
// @pte: PTE entry.
//
// Returns: true if this PTE is a UFFD WP marker leaf entry, otherwise false.
//
extern "C" {
    pub fn softleaf_is_uffd_wp_marker(_arg: entry) -> return;
}
//
// pte_is_uffd_marker() - Does this PTE entry encode a userfault-specific marker
// leaf entry?
// @pte: PTE entry.
//
// It's useful to be able to determine which leaf entries encode UFFD-specific
// markers so we can handle these correctly.
//
// Returns: true if this PTE entry is a UFFD-specific marker, otherwise false.
//
// UFFD WP, poisoned swap entries are UFFD-handled.

//
// pmd_is_device_private_entry() - Check if PMD contains a device private swap
// entry.
// @pmd: The PMD to check.
//
// Returns true if the PMD contains a swap entry that represents a device private
// page mapping. This is used for zone device private pages that have been
// swapped out but still need special handling during various memory management
// operations.
//
// Return: true if PMD contains device private entry, false otherwise
//
extern "C" {
    pub fn softleaf_is_device_private(_arg: softleaf_from_pmd(pmd)) -> return;
}

//
// pmd_is_migration_entry() - Does this PMD entry encode a migration entry?
// @pmd: PMD entry.
//
// Returns: true if the PMD encodes a migration entry, otherwise false.
//
extern "C" {
    pub fn softleaf_is_migration(_arg: softleaf_from_pmd(pmd)) -> return;
}
//
// softleaf_is_valid_pmd_entry() - Is the specified softleaf entry obtained from
// a PMD one that we support at PMD level?
// @entry: Entry to check.
// Returns: true if the softleaf entry is valid at PMD, otherwise false.
//
// Only device private, migration entries valid for PMD.
//
// pmd_is_valid_softleaf() - Is this PMD entry a valid softleaf entry?
// @pmd: PMD entry.
//
// PMD leaf entries are valid only if they are device private or migration
// entries. This function asserts that a PMD leaf entry is valid in this
// respect.
//
// Returns: true if the PMD entry is a valid leaf entry, otherwise false.
//
extern "C" {
    pub fn softleaf_is_valid_pmd_entry(_arg: entry) -> return;
}
//
// pmd_to_softleaf_folio() - Convert the PMD entry to a folio.
// @pmd: PMD entry.
//
// The PMD entry is expected to be a valid PMD softleaf entry.
//
// Returns: the folio the softleaf entry references if this is a valid softleaf
// entry, otherwise NULL.
//
extern "C" {
    pub fn softleaf_to_folio(_arg: entry) -> return;
}

