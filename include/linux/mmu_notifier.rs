//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmu_notifier.h
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
// enum mmu_notifier_event - reason for the mmu notifier callback
// @MMU_NOTIFY_UNMAP: either munmap() that unmap the range or a mremap() that
// move the range
//
// @MMU_NOTIFY_CLEAR: clear page table entry (many reasons for this like
// madvise() or replacing a page by another one, ...).
//
// @MMU_NOTIFY_PROTECTION_VMA: update is due to protection change for the range
// ie using the vma access permission (vm_page_prot) to update the whole range
// is enough no need to inspect changes to the CPU page table (mprotect()
// syscall)
//
// @MMU_NOTIFY_PROTECTION_PAGE: update is due to change in read/write flag for
// pages in the range so to mirror those changes the user must inspect the CPU
// page table (from the end callback).
//
// @MMU_NOTIFY_SOFT_DIRTY: soft dirty accounting (still same page and same
// access flags). User should soft dirty the page in the end callback to make
// sure that anyone relying on soft dirtiness catch pages that might be written
// through non CPU mappings.
//
// @MMU_NOTIFY_RELEASE: used during mmu_interval_notifier invalidate to signal
// that the mm refcount is zero and the range is no longer accessible.
//
// @MMU_NOTIFY_MIGRATE: used during migrate_vma_collect() invalidate to signal
// a device driver to possibly ignore the invalidation if the
// owner field matches the driver's device private pgmap owner.
//
// @MMU_NOTIFY_EXCLUSIVE: conversion of a page table entry to device-exclusive.
// The owner is initialized to the value provided by the caller of
// make_device_exclusive(), such that this caller can filter out these
// events.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmu_notifier_event {
    MMU_NOTIFY_UNMAP = 0,
    MMU_NOTIFY_CLEAR,
    MMU_NOTIFY_PROTECTION_VMA,
    MMU_NOTIFY_PROTECTION_PAGE,
    MMU_NOTIFY_SOFT_DIRTY,
    MMU_NOTIFY_RELEASE,
    MMU_NOTIFY_MIGRATE,
    MMU_NOTIFY_EXCLUSIVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_notifier_ops {
//
// Called either by mmu_notifier_unregister or when the mm is
// being destroyed by exit_mmap, always before all pages are
// freed. This can run concurrently with other mmu notifier
// methods (the ones invoked outside the mm context) and it
// should tear down all secondary mmu mappings and freeze the
// secondary mmu. If this method isn't implemented you've to
// be sure that nothing could possibly write to the pages
// through the secondary mmu by the time the last thread with
// tsk->mm == mm exits.
//
// As side note: the pages freed after ->release returns could
// be immediately reallocated by the gart at an alias physical
// address with a different cache model, so if ->release isn't
// implemented because all _software_ driven memory accesses
// through the secondary mmu are terminated by the time the
// last thread of this mm quits, you've also to be sure that
// speculative _hardware_ operations can't allocate dirty
// cachelines in the cpu that could not be snooped and made
// coherent with the other read and write operations happening
// through the gart alias address, so leading to memory
// corruption.
//
    pub mm): *mut mm_struct,
//
// clear_flush_young is called after the VM is
// test-and-clearing the young/accessed bitflag in the
// pte. This way the VM will provide proper aging to the
// accesses to the page through the secondary MMUs and not
// only to the ones through the Linux pte.
// Start-end is necessary in case the secondary MMU is mapping the page
// at a smaller granularity than the primary MMU.
//
    pub end): c_ulong,
//
// clear_young is a lightweight version of clear_flush_young. Like the
// latter, it is supposed to test-and-clear the young/accessed bitflag
// in the secondary pte, but it may omit flushing the secondary tlb.
//
    pub end): c_ulong,
//
// test_young is called to check the young/accessed bitflag in
// the secondary pte. This is used to know if the page is
// frequently used without actually clearing the flag or tearing
// down the secondary mapping on the page.
//
    pub address): c_ulong,
//
// invalidate_range_start() and invalidate_range_end() must be
// paired and are called only when the mmap_lock and/or the
// locks protecting the reverse maps are held. If the subsystem
// can't guarantee that no additional references are taken to
// the pages in the range, it has to implement the
// invalidate_range() notifier to remove any references taken
// after invalidate_range_start().
//
// Invalidation of multiple concurrent ranges may be
// optionally permitted by the driver. Either way the
// establishment of sptes is forbidden in the range passed to
// invalidate_range_start/end for the whole duration of the
// invalidate_range_start/end critical section.
//
// invalidate_range_start() is called when all pages in the
// range are still mapped and have at least a refcount of one.
//
// invalidate_range_end() is called when all pages in the
// range have been unmapped and the pages have been freed by
// the VM.
//
// The VM will remove the page table entries and potentially
// the page between invalidate_range_start() and
// invalidate_range_end(). If the page must not be freed
// because of pending I/O or other circumstances then the
// invalidate_range_start() callback (or the initial mapping
// by the driver) must make sure that the refcount is kept
// elevated.
//
// If the driver increases the refcount when the pages are
// initially mapped into an address space then either
// invalidate_range_start() or invalidate_range_end() may
// decrease the refcount. If the refcount is decreased on
// invalidate_range_start() then the VM can free pages as page
// table entries are removed.  If the refcount is only
// dropped on invalidate_range_end() then the driver itself
// will drop the last refcount but it must take care to flush
// any secondary tlb before doing the final free on the
// page. Pages will no longer be referenced by the linux
// address space but may still be referenced by sptes until
// the last refcount is dropped.
//
// If blockable argument is set to false then the callback cannot
// sleep and has to return with -EAGAIN if sleeping would be required.
// 0 should be returned otherwise. Please note that notifiers that can
// fail invalidate_range_start are not allowed to implement
// invalidate_range_end, as there is no mechanism for informing the
// notifier that its start failed.
//
    pub range): *const mmu_notifier_range,
    pub range): *const mmu_notifier_range,
//
// arch_invalidate_secondary_tlbs() is used to manage a non-CPU TLB
// which shares page-tables with the CPU. The
// invalidate_range_start()/end() callbacks should not be implemented as
// invalidate_secondary_tlbs() already catches the points in time when
// an external TLB needs to be flushed.
//
// This requires arch_invalidate_secondary_tlbs() to be called while
// holding the ptl spin-lock and therefore this callback is not allowed
// to sleep.
//
// This is called by architecture code whenever invalidating a TLB
// entry. It is assumed that any secondary TLB has the same rules for
// when invalidations are required. If this is not the case architecture
// code will need to call this explicitly when required for secondary
// TLB invalidation.
//
    pub end): c_ulong,
//
// These callbacks are used with the get/put interface to manage the
// lifetime of the mmu_notifier memory. alloc_notifier() returns a new
// notifier for use with the mm.
//
// free_notifier() is only called after the mmu_notifier has been
// fully put, calls to any ops callback are prevented and no ops
// callbacks are currently running. It is called from a SRCU callback
// and cannot sleep.
//
    pub mm): *mut *mut *mut mmu_notifier (alloc_notifier)(mm_struct,
    pub subscription): *mut *mut void (free_notifier)(struct mmu_notifier,
}

//
// The notifier chains are protected by mmap_lock and/or the reverse map
// semaphores. Notifier chains are only changed when all reverse maps and
// the mmap_lock locks are taken.
//
// Therefore notifier chains can only be traversed when either
//
// 1. mmap_lock is held.
// 2. One of the reverse map locks is held (i_mmap_rwsem or anon_vma->rwsem).
// 3. No other concurrent thread can access the list (release)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_notifier {
    pub hlist: hlist_node,
    pub ops: *const mmu_notifier_ops,
    pub mm: *mut mm_struct,
    pub rcu: rcu_head,
    pub users: c_uint,
}

//
// struct mmu_interval_notifier_finish - mmu_interval_notifier two-pass abstraction
// @link: Lockless list link for the notifiers pending pass list
// @notifier: The mmu_interval_notifier for which the finish pass is called.
//
// Allocate, typically using GFP_NOWAIT in the interval notifier's start pass.
// Note that with a large number of notifiers implementing two passes,
// allocation with GFP_NOWAIT will become increasingly likely to fail, so consider
// implementing a small pool instead of using kmalloc() allocations.
//
// If the implementation needs to pass data between the start and the finish passes,
// the recommended way is to embed struct mmu_interval_notifier_finish into a larger
// structure that also contains the data needed to be shared. Keep in mind that
// a notifier callback can be invoked in parallel, and each invocation needs its
// own struct mmu_interval_notifier_finish.
//
// If allocation fails, then the &mmu_interval_notifier_ops->invalidate_start op
// needs to implements the full notifier functionality. Please refer to its
// documentation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_interval_notifier_finish {
    pub link: llist_node,
    pub notifier: *mut mmu_interval_notifier,
}

//
// struct mmu_interval_notifier_ops - callback for range notification
// @invalidate: Upon return the caller must stop using any SPTEs within this
// range. This function can sleep. Return false only if sleeping
// was required but mmu_notifier_range_blockable(range) is false.
// @invalidate_start: Similar to @invalidate, but intended for two-pass notifier
// callbacks where the call to @invalidate_start is the first
// pass and any struct mmu_interval_notifier_finish pointer
// returned in the @finish parameter describes the finish pass.
// If *@finish is %NULL on return, then no final pass will be
// called, and @invalidate_start needs to implement the full
// notifier, behaving like @invalidate. The value of *@finish
// is guaranteed to be %NULL at function entry.
// @invalidate_finish: Called as the second pass for any notifier that returned
// a non-NULL *@finish from @invalidate_start. The @finish
// pointer passed here is the same one returned by
// @invalidate_start.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_interval_notifier_ops {
    pub cur_seq): c_ulong,
    pub finish): *mut mmu_interval_notifier_finish,
    pub finish): *mut *mut void (invalidate_finish)(struct mmu_interval_notifier_finish,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_interval_notifier {
    pub interval_tree: interval_tree_node,
    pub ops: *const mmu_interval_notifier_ops,
    pub mm: *mut mm_struct,
    pub deferred_item: hlist_node,
    pub invalidate_seq: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_notifier_range {
    pub mm: *mut mm_struct,
    pub start: c_ulong,
    pub end: c_ulong,
    pub flags: unsigned,
    pub event: mmu_notifier_event,
    pub owner: *mut c_void,
}

extern "C" {
    pub fn unlikely(_arg: mm->notifier_subscriptions) -> return;
}
extern "C" {
    pub fn mmu_notifier_put(subscription: *mut mmu_notifier);
}
extern "C" {
    pub fn mmu_notifier_synchronize();
}
extern "C" {
    pub fn mmu_interval_notifier_remove(interval_sub: *mut mmu_interval_notifier);
}
//
// mmu_interval_set_seq - Save the invalidation sequence
// @interval_sub: The subscription passed to invalidate
// @cur_seq: The cur_seq passed to the invalidate() callback
//
// This must be called unconditionally from the invalidate callback of a
// struct mmu_interval_notifier_ops under the same lock that is used to call
// mmu_interval_read_retry(). It updates the sequence number for later use by
// mmu_interval_read_retry(). The provided cur_seq will always be odd.
//
// If the caller does not call mmu_interval_read_begin() or
// mmu_interval_read_retry() then this call is not required.
//
// mmu_interval_read_retry - End a read side critical section against a VA range
// @interval_sub: The subscription
// @seq: The return of the paired mmu_interval_read_begin()
//
// This MUST be called under a user provided lock that is also held
// unconditionally by op->invalidate() when it calls mmu_interval_set_seq().
//
// Each call should be paired with a single mmu_interval_read_begin() and
// should be used to conclude the read side.
//
// Returns: true if an invalidation collided with this critical section, and
// the caller should retry.
//
// mmu_interval_check_retry - Test if a collision has occurred
// @interval_sub: The subscription
// @seq: The return of the matching mmu_interval_read_begin()
//
// This can be used in the critical section between mmu_interval_read_begin()
// and mmu_interval_read_retry().
//
// This call can be used as part of loops and other expensive operations to
// expedite a retry.
// It can be called many times and does not have to hold the user
// provided lock.
//
// Returns: true indicates an invalidation has collided with this critical
// region and a future mmu_interval_read_retry() will return true.
// False is not reliable and only suggests a collision may not have
// occurred.
//
// Pairs with the WRITE_ONCE in mmu_interval_set_seq()
extern "C" {
    pub fn __mmu_notifier_subscriptions_destroy(mm: *mut mm_struct);
}
extern "C" {
    pub fn __mmu_notifier_release(mm: *mut mm_struct);
}
extern "C" {
    pub fn __mmu_notifier_invalidate_range_start(r: *mut mmu_notifier_range) -> c_int;
}
extern "C" {
    pub fn __mmu_notifier_invalidate_range_end(r: *mut mmu_notifier_range);
}
extern "C" {
    pub fn __mmu_notifier_clear_flush_young(_arg: mm, _arg: start, _arg: end) -> return;
}
extern "C" {
    pub fn __mmu_notifier_clear_young(_arg: mm, _arg: start, _arg: end) -> return;
}
extern "C" {
    pub fn __mmu_notifier_test_young(_arg: mm, _arg: address) -> return;
}
//
// This version of mmu_notifier_invalidate_range_start() avoids blocking, but it
// can return an error if a notifier can't proceed without blocking, in which
// case you're not allowed to modify PTEs in the specified range.
//
// This is mainly intended for OOM handling.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_notifier_range {
    pub start: c_ulong,
    pub end: c_ulong,
}

