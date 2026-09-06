//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/migrate.h
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

extern "C" {
    pub fn free_folio_t(folio: *mut folio, private: c_ulong) -> typedef void;
}
//
// struct movable_operations - Driver page migration
// @isolate_page:
// The VM calls this function to prepare the page to be moved.  The page
// is locked and the driver should not unlock it.  The driver should
// return ``true`` if the page is movable and ``false`` if it is not
// currently movable.  After this function returns, the VM uses the
// page->lru field, so the driver must preserve any information which
// is usually stored here.
//
// @migrate_page:
// After isolation, the VM calls this function with the isolated
// @src page.  The driver should copy the contents of the
// @src page to the @dst page and set up the fields of @dst page.
// Both pages are locked.
// If page migration is successful, the driver should return 0.
// If the driver cannot migrate the page at the moment, it can return
// -EAGAIN.  The VM interprets this as a temporary migration failure and
// will retry it later.  Any other error value is a permanent migration
// failure and migration will not be retried.
// The driver shouldn't touch the @src->lru field while in the
// migrate_page() function.  It may write to @dst->lru.
//
// @putback_page:
// If migration fails on the isolated page, the VM informs the driver
// that the page is no longer a candidate for migration by calling
// this function.  The driver should put the isolated page back into
// its own data structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct movable_operations {
    pub isolate_mode_t): *mut *mut *mut bool (isolate_page)(struct page ,,
    pub migrate_mode): enum,
    pub ): *mut *mut void (putback_page)(struct page,
}

// Defined in mm/debug.c:

extern "C" {
    pub fn putback_movable_pages(l: *mut list_head);
}
extern "C" {
    pub fn isolate_movable_ops_page(page: *mut page, mode: isolate_mode_t) -> bool;
}
extern "C" {
    pub fn isolate_folio_to_list(folio: *mut folio, list: *mut list_head) -> bool;
}
extern "C" {
    pub fn folio_migrate_flags(newfolio: *mut folio, folio: *mut folio);
}
extern "C" {
    pub fn set_movable_ops(ops: *const movable_operations, type: pagetype) -> c_int;
}

extern "C" {
    pub fn migrate_misplaced_folio(folio: *mut folio, node: c_int) -> c_int;
}

//
// Watch out for PAE architecture, which has an unsigned long, and might not
// have enough bits to store all physical address and flags. So far we have
// enough room for all our flags.
//

pub const MIGRATE_PFN_SHIFT: c_int = 6;
extern "C" {
    pub fn pfn_to_page(MIGRATE_PFN_SHIFT: mpfn >>) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum migrate_vma_direction {
    MIGRATE_VMA_SELECT_SYSTEM = 1 << 0,
    MIGRATE_VMA_SELECT_DEVICE_PRIVATE = 1 << 1,
    MIGRATE_VMA_SELECT_DEVICE_COHERENT = 1 << 2,
    MIGRATE_VMA_SELECT_COMPOUND = 1 << 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct migrate_vma {
    pub vma: *mut vm_area_struct,
//
// Both src and dst array must be big enough for
// (end - start) >> PAGE_SHIFT entries.
//
// The src array must not be modified by the caller after
// migrate_vma_setup(), and must not change the dst array after
// migrate_vma_pages() returns.
//
    pub dst: *mut c_ulong,
    pub src: *mut c_ulong,
    pub cpages: c_ulong,
    pub npages: c_ulong,
    pub start: c_ulong,
    pub end: c_ulong,
//
// Set to the owner value also stored in page_pgmap(page)->owner
// for migrating out of device private memory. The flags also need to
// be set to MIGRATE_VMA_SELECT_DEVICE_PRIVATE.
// The caller should always set this field when using mmu notifier
// callbacks to avoid device MMU invalidations for device private
// pages that are not being migrated.
//
    pub pgmap_owner: *mut c_void,
    pub flags: c_ulong,
//
// Set to vmf->page if this is being called to migrate a page as part of
// a migrate_to_ram() callback.
//
    pub fault_page: *mut page,
}

extern "C" {
    pub fn migrate_vma_setup(args: *mut migrate_vma) -> c_int;
}
extern "C" {
    pub fn migrate_vma_pages(migrate: *mut migrate_vma);
}
extern "C" {
    pub fn migrate_vma_finalize(migrate: *mut migrate_vma);
}
extern "C" {
    pub fn migrate_device_pfns(src_pfns: *mut c_ulong, npages: c_ulong) -> c_int;
}

