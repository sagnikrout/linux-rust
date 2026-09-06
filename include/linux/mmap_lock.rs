//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmap_lock.h
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
// Avoid a dependency loop by declaring here.
extern "C" {
    pub fn rcuwait_wake_up(w: *mut rcuwait) -> c_int;
}

extern "C" {
    pub fn __mmap_lock_do_trace_start_locking(mm: *mut mm_struct, write: bool);
}
extern "C" {
    pub fn __mmap_lock_do_trace_released(mm: *mut mm_struct, write: bool);
}

//
// VMA locks do not behave like most ordinary locks found in the kernel, so we
// cannot quite have full lockdep tracking in the way we would ideally prefer.
//
// Read locks act as shared locks which exclude an exclusive lock being
// taken. We therefore mark these accordingly on read lock acquire/release.
//
// Write locks are acquired exclusively per-VMA, but released in a shared
// fashion, that is upon vma_end_write_all(), we update the mmap's seqcount such
// that write lock is released.
//
// We therefore cannot track write locks per-VMA, nor do we try. Mitigating this
// is the fact that, of course, we do lockdep-track the mmap lock rwsem which
// must be held when taking a VMA write lock.
//
// We do, however, want to indicate that during either acquisition of a VMA
// write lock or detachment of a VMA that we require the lock held be exclusive,
// so we utilise lockdep to do so.
//

// Only meaningful if CONFIG_LOCK_STAT is defined.

//
// Since mmap_lock is a sleeping lock, and waiting for it to become
// unlocked is more or less equivalent with taking it ourselves, don't
// bother with the speculative path if mmap_lock is already write-locked
// and take the slow path, which takes the lock.
//
extern "C" {
    pub fn raw_seqcount_try_begin(_arg: &mm->mm_lock_seq, _arg: *mut seq) -> return;
}
extern "C" {
    pub fn read_seqcount_retry(_arg: &mm->mm_lock_seq, _arg: seq) -> return;
}

//
// This function determines whether the input VMA reference count describes a
// VMA which has excluded all VMA read locks.
//
// In the case of a detached VMA, we may incorrectly indicate that readers are
// excluded when one remains, because in that scenario we target a refcount of
// VM_REFCNT_EXCLUDE_READERS_FLAG, rather than the attached target of
// VM_REFCNT_EXCLUDE_READERS_FLAG + 1.
//
// However, the race window for that is very small so it is unlikely.
//
// Returns: true if readers are excluded, false otherwise.
//
// See the comment describing the vm_area_struct->vm_refcnt field for
// details of possible refcnt values.
//
// Actually decrement the VMA reference count.
//
// The function returns the reference count as it was immediately after the
// decrement took place. If it returns zero, the VMA is now detached.
//
// vma_refcount_put() - Drop reference count in VMA vm_refcnt field due to a
// read-lock being dropped.
// @vma: The VMA whose reference count we wish to decrement.
//
// If we were the last reader, wake up threads waiting to obtain an exclusive
// lock.
//
// Use a copy of vm_mm in case vma is freed after we drop vm_refcnt.
//
// __vma_start_exclude_readers() may be sleeping waiting for readers to
// drop their reference count, so wake it up if we were the last reader
// blocking it from being acquired.
//
// We may be raced by other readers temporarily incrementing the
// reference count, though the race window is very small, this might
// cause spurious wakeups.
//
// Use only while holding mmap read lock which guarantees that locking will not
// fail (nobody can concurrently write-lock the vma). vma_start_read() should
// not be used in such cases because it might fail due to mm_lock_seq overflow.
// This functionality is used to obtain vma read lock and drop the mmap read lock.
//
// Use only while holding mmap read lock which guarantees that locking will not
// fail (nobody can concurrently write-lock the vma). vma_start_read() should
// not be used in such cases because it might fail due to mm_lock_seq overflow.
// This functionality is used to obtain vma read lock and drop the mmap read lock.
//
extern "C" {
    pub fn vma_start_read_locked_nested(_arg: vma, _arg: 0) -> return;
}
// We must hold an exclusive write lock for this access to be valid.
//
// Determine whether a VMA is write-locked. Must be invoked ONLY if the mmap
// write lock is held.
//
// Returns true if write-locked, otherwise false.
//
// current task is holding mmap_write_lock, both vma->vm_lock_seq and
// mm->mm_lock_seq can't be concurrently modified.
//
extern "C" {
    pub fn __vma_start_write(vma: *mut vm_area_struct, state: c_int) -> c_int;
}
//
// Begin writing to a VMA.
// Exclude concurrent readers under the per-VMA lock until the currently
// write-locked mmap_lock is dropped or downgraded.
//
// vma_start_write_killable - Begin writing to a VMA.
// @vma: The VMA we are going to modify.
//
// Exclude concurrent readers under the per-VMA lock until the currently
// write-locked mmap_lock is dropped or downgraded.
//
// Context: May sleep while waiting for readers to drop the vma read lock.
// Caller must already hold the mmap_lock for write.
//
// Return: 0 for a successful acquisition.  -EINTR if a fatal signal was
// received.
//
extern "C" {
    pub fn __vma_start_write(_arg: vma, _arg: TASK_KILLABLE) -> return;
}
//
// vma_assert_write_locked() - assert that @vma holds a VMA write lock.
// @vma: The VMA to assert.
//
// vma_assert_locked() - assert that @vma holds either a VMA read or a VMA write
// lock and is not detached.
// @vma: The VMA to assert.
//
// See the comment describing the vm_area_struct->vm_refcnt field for
// details of possible refcnt values.
//
// In this case we're either read-locked, write-locked with temporary
// readers, or in the midst of excluding readers, all of which means
// we're locked.
//
// It is a bug for the VMA to be detached here.
//
// OK, the VMA has a reference count of 1 which means it is either
// unlocked and attached or write-locked, so assert that it is
// write-locked.
//
// vma_assert_stabilised() - assert that this VMA cannot be changed from
// underneath us either by having a VMA or mmap lock held.
// @vma: The VMA whose stability we wish to assess.
//
// If lockdep is enabled we can precisely ensure stability via either an mmap
// lock owned by us or a specific VMA lock.
//
// With lockdep disabled we may sometimes race with other threads acquiring the
// mmap read lock simultaneous with our VMA read lock.
//
// If another thread owns an mmap lock, it may go away at any time, and
// thus is no guarantee of stability.
//
// If lockdep is enabled we can accurately determine if an mmap lock is
// held and owned by us. Otherwise we must approximate.
//
// It doesn't necessarily mean we are not stabilised however, as we may
// hold a VMA read lock (not a write lock as this would require an owned
// mmap lock).
//
// If (assuming lockdep is not enabled) we were to assert a VMA read
// lock first we may also run into issues, as other threads can hold VMA
// read locks simlutaneous to us.
//
// Therefore if lockdep is not enabled we risk a false negative (i.e. no
// assert fired). If accurate checking is required, enable lockdep.
//
// We're not stabilised by the mmap lock, so assert that we're
// stabilised by a VMA lock.
//
extern "C" {
    pub fn refcount_read(_arg: &vma->vm_refcnt) -> return;
}
//
// WARNING: to avoid racing with vma_mark_attached()/vma_mark_detached(), these
// assertions should be made either under mmap_write_lock or when the object
// has been isolated under mmap_write_lock, ensuring no competing writers.
//
extern "C" {
    pub fn __vma_exclude_readers_for_detach(vma: *mut vm_area_struct);
}
//
// The VMA still being attached (refcnt > 0) - is unlikely, because the
// vma has been already write-locked and readers can increment vm_refcnt
// only temporarily before they check vm_lock_seq, realize the vma is
// locked and drop back the vm_refcnt. That is a narrow window for
// observing a raised vm_refcnt.
//
// See the comment describing the vm_area_struct->vm_refcnt field for
// details of possible refcnt values.
//
// Locks next vma pointed by the iterator. Confirms the locked vma has not
// been modified and will retry under mmap_lock protection if modification
// was detected. Should be called from read RCU section.
// Returns either a valid locked VMA, NULL if no more VMAs or -EINTR if the
// process was interrupted.
//

// If no VMA locks, then either mmap lock suffices to stabilise.

//
// Drop all currently-held per-VMA locks.
// This is called from the mmap_lock implementation directly before releasing
// a write-locked mmap_lock (or downgrading it to read-locked).
// This should normally NOT be called manually from other places.
// If you want to call this manually anyway, keep in mind that this will release
// *all* VMA write locks, including ones from further up the stack.
//
extern "C" {
    pub fn rwsem_is_contended(_arg: &mm->mmap_lock) -> return;
}
