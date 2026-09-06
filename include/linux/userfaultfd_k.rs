//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/userfaultfd_k.h
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
// include/linux/userfaultfd_k.h
//
// Copyright (C) 2015  Red Hat, Inc.
//

// The set of all possible UFFD-related VM flags.

//
// CAREFUL: Check include/uapi/asm-generic/fcntl.h when defining
// new flags, since they might collide with O_* ones. We want
// to re-use O_* flags that couldn't possibly have a meaning
// from userfaultfd, in order to leave a free define-space for
// shared O_* flags.
//

//
// Start with fault_pending_wqh and fault_wqh so they're more likely
// to be in the same cacheline.
//
// Locking order:
// fd_wqh.lock
// fault_pending_wqh.lock
// fault_wqh.lock
// event_wqh.lock
//
// To avoid deadlocks, IRQs must be disabled when taking any of the above locks,
// since fd_wqh.lock is taken by aio_poll() while it's holding a lock that's
// also taken in IRQ context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct userfaultfd_ctx {
// waitqueue head for the pending (i.e. not read) userfaults
    pub fault_pending_wqh: wait_queue_head_t,
// waitqueue head for the userfaults
    pub fault_wqh: wait_queue_head_t,
// waitqueue head for the pseudo fd to wakeup poll/read
    pub fd_wqh: wait_queue_head_t,
// waitqueue head for events
    pub event_wqh: wait_queue_head_t,
// a refile sequence protected by fault_pending_wqh lock
    pub refile_seq: seqcount_spinlock_t,
// pseudo fd refcounting
    pub refcount: refcount_t,
// userfaultfd syscall flags
    pub flags: c_uint,
// features requested from the userspace
    pub features: c_uint,
// released
    pub released: bool,
//
// Prevents userfaultfd operations (fill/move/wp) from happening while
// some non-cooperative event(s) is taking place. Increments are done
// in write-mode. Whereas, userfaultfd operations, which includes
// reading mmap_changing, is done under read-mode.
//
    pub map_changing_lock: rw_semaphore,
// memory mappings are changing because of non-cooperative event
    pub mmap_changing: core::sync::atomic::AtomicI32,
// mm with one ore more vmas attached to this userfaultfd_ctx
    pub mm: *mut mm_struct,
}

extern "C" {
    pub fn handle_userfault(vmf: *mut vm_fault, reason: c_ulong) -> vm_fault_t;
}
// VMA userfaultfd operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_uffd_ops {
// Checks if a VMA can support userfaultfd
    pub vm_flags): *mut *mut *mut bool (can_userfault)(struct vm_area_struct vma, vm_flags_t,
//
// Called to resolve UFFDIO_CONTINUE request.
// Should return the folio found at pgoff in the VMA's pagecache if it
// exists or ERR_PTR otherwise.
// The returned folio is locked and with reference held.
//
    pub pgoff): *mut *mut *mut *mut folio (get_folio_noalloc)(inode inode, pgoff_t,
//
// Called during resolution of UFFDIO_COPY request.
// Should allocate and return a folio or NULL if allocation fails.
//
    pub addr): c_ulong,
//
// Called during resolution of UFFDIO_COPY request.
// Should only be called with a folio returned by alloc_folio() above.
// The folio will be set to locked.
// Returns 0 on success, error code on failure.
//
    pub addr): c_ulong,
//
// Called during resolution of UFFDIO_COPY request on the error
// handling path.
// Should revert the operation of ->filemap_add().
//
    pub vma): *mut *mut *mut void (filemap_remove)(struct folio folio, struct vm_area_struct,
}

// A combined operation mode + behavior flags.
pub type uffd_flags_t = u32;
// Mutually exclusive modes of operation.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mfill_atomic_mode {
    MFILL_ATOMIC_COPY,
    MFILL_ATOMIC_ZEROPAGE,
    MFILL_ATOMIC_CONTINUE,
    MFILL_ATOMIC_POISON,
    NR_MFILL_ATOMIC_MODES,
}

// Flags controlling behavior. These behavior changes are mode-independent.

// move_pages
extern "C" {
    pub fn double_pt_lock(ptl1: *mut spinlock_t, ptl2: *mut spinlock_t);
}
extern "C" {
    pub fn double_pt_unlock(ptl1: *mut spinlock_t, ptl2: *mut spinlock_t);
}
// mm helpers
//
// Never enable huge pmd sharing on some uffd registered vmas:
//
// - VM_UFFD_WP and VM_UFFD_RWP VMAs, because the write protect / access
// tracking information is per pgtable entry.
//
// - VM_UFFD_MINOR VMAs, because otherwise we would never get minor faults for
// VMAs which share huge pmds. (If you have two mappings to the same
// underlying pages, and fault in the non-UFFD-registered one with a write,
// with huge pmd sharing this would *also* setup the second UFFD-registered
// mapping, and we'd not get minor faults.)
//
// Don't do fault around for WP, RWP or MINOR registered uffd range.  For
// MINOR registered range, fault around will be a total disaster and ptes can
// be installed without notifications; for WP it should mostly be fine as long
// as the fault around checks for pte_none() before the installation, however
// to be super safe we just forbid it; for RWP, pre-faulted neighbours would
// be indistinguishable from accessed pages in PAGEMAP_SCAN (PAGE_IS_ACCESSED)
// and pollute the tracked working set, so each page must be populated by its
// own fault.
//
extern "C" {
    pub fn vma_test_any_mask(_arg: vma, _arg: VMA_UFFD_MISSING) -> return;
}
extern "C" {
    pub fn vma_test_any_mask(_arg: vma, _arg: VMA_UFFD_WP) -> return;
}
extern "C" {
    pub fn vma_test_any_mask(_arg: vma, _arg: VMA_UFFD_MINOR) -> return;
}
//
// Callers gate PAGE_NONE usage on this; PAGE_NONE is a BUILD_BUG()
// without CONFIG_ARCH_HAS_PTE_PROTNONE, so fold to false.
//
extern "C" {
    pub fn vma_test_single_mask(_arg: vma, _arg: VMA_UFFD_RWP) -> return;
}
extern "C" {
    pub fn userfaultfd_wp(userfaultfd_rwp(vma: vma) ||) -> return;
}
extern "C" {
    pub fn userfaultfd_wp(pte_uffd(pte: vma) &&) -> return;
}
extern "C" {
    pub fn userfaultfd_wp(pmd_uffd(pmd: vma) &&) -> return;
}
extern "C" {
    pub fn userfaultfd_rwp(pte_uffd(pte: vma) &&) -> return;
}
extern "C" {
    pub fn userfaultfd_rwp(pmd_uffd(pmd: vma) &&) -> return;
}
extern "C" {
    pub fn vma_test_any_mask(_arg: vma, _arg: __VMA_UFFD_FLAGS) -> return;
}
extern "C" {
    pub fn dup_userfaultfd(: *mut vm_area_struct, : *mut list_head) -> c_int;
}
extern "C" {
    pub fn dup_userfaultfd_complete(: *mut list_head);
}
extern "C" {
    pub fn dup_userfaultfd_fail(: *mut list_head);
}
extern "C" {
    pub fn mremap_userfaultfd_fail(: *mut vm_userfaultfd_ctx);
}
extern "C" {
    pub fn userfaultfd_wp_unpopulated(vma: *mut vm_area_struct) -> bool;
}
extern "C" {
    pub fn userfaultfd_wp_async(vma: *mut vm_area_struct) -> bool;
}
extern "C" {
    pub fn userfaultfd_rwp_async(vma: *mut vm_area_struct) -> bool;
}
// Only wr-protect mode uses pte markers
// File-based uffd-wp always need markers
//
// Anonymous uffd-wp only needs the markers if WP_UNPOPULATED
// enabled (to apply markers on zero pages).
//
extern "C" {
    pub fn userfaultfd_wp_unpopulated(_arg: vma) -> return;
}
//
// Returns true if this swap pte carries uffd-tracked state in either
// form (pte marker or a normal swap pte), false otherwise.
//

// mm helpers
//
// Returns true if this swap pte carries uffd-tracked state in either
// form (pte marker or a normal swap pte), false otherwise.
//

