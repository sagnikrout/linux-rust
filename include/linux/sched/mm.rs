//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/mm.h
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
// Routines for handling mm_structs
//
// mmgrab() - Pin a &struct mm_struct.
// @mm: The &struct mm_struct to pin.
//
// Make sure that @mm will not get freed even after the owning task
// exits. This doesn't guarantee that the associated address space
// will still exist later on and mmget_not_zero() has to be used before
// accessing it.
//
// This is a preferred way to pin @mm for a longer/unbounded amount
// of time.
//
// Use mmdrop() to release the reference acquired by mmgrab().
//
// See also <Documentation/mm/active_mm.rst> for an in-depth explanation
// of &mm_struct.mm_count vs &mm_struct.mm_users.
//
extern "C" {
    pub fn __mmdrop(mm: *mut mm_struct);
}
//
// The implicit full barrier implied by atomic_dec_and_test() is
// required by the membarrier system call before returning to
// user-space, after storing to rq->curr.
//

//
// RCU callback for delayed mm drop. Not strictly RCU, but call_rcu() is
// by far the least expensive way to do that.
//
// Invoked from finish_task_switch(). Delegates the heavy lifting on RT
// kernels via RCU.
//
// Provides a full memory barrier. See mmdrop()

// Helpers for lazy TLB mm refcounting
//
// mmdrop_lazy_tlb must provide a full memory barrier, see the
// membarrier comment finish_task_switch which relies on this.
//
// mmget() - Pin the address space associated with a &struct mm_struct.
// @mm: The address space to pin.
//
// Make sure that the address space of the given &struct mm_struct doesn't
// go away. This does not protect against parts of the address space being
// modified or freed, however.
//
// Never use this function to pin this address space for an
// unbounded/indefinite amount of time.
//
// Use mmput() to release the reference acquired by mmget().
//
// See also <Documentation/mm/active_mm.rst> for an in-depth explanation
// of &mm_struct.mm_count vs &mm_struct.mm_users.
//
extern "C" {
    pub fn atomic_inc_not_zero(_arg: &mm->mm_users) -> return;
}
// mmput gets rid of the mappings and all user-space
extern "C" {
    pub fn mmput(: *mut mm_struct);
}

// same as above but performs the slow path from the async context. Can
// be called from the atomic context as well
//
extern "C" {
    pub fn mmput_async(: *mut mm_struct);
}

// Grab a reference to a task's mm, if it is not already going away
//
// Grab a reference to a task's mm, if it is not already going away
// and ptrace_may_access with the mode parameter passed to it
// succeeds.
//
// Remove the current tasks stale references to the old mm_struct on exit() and
// exec(). Cleans up futexes as well.
//
extern "C" {
    pub fn mm_exit_exec_release(: *mut task_struct, : *mut mm_struct);
}

extern "C" {
    pub fn mm_update_next_owner(mm: *mut mm_struct);
}

//
// need RCU to access ->real_parent if CLONE_VM was used along with
// CLONE_PARENT.
//
// We check real_parent->mm == tsk->mm because CLONE_VFORK does not
// imply CLONE_VM
//
// CLONE_VFORK can be used with CLONE_PARENT/CLONE_THREAD and thus
// ->real_parent is not necessarily the task doing vfork(), so in
// theory we can't rely on task_lock() if we want to dereference it.
//
// And in this case we can't trust the real_parent->mm == tsk->mm
// check, it can be false negative. But we do not care, if init or
// another oom-unkillable task does this it should blame itself.
//
// Applies per-task gfp context to the given allocation flags.
// PF_MEMALLOC_NOIO implies GFP_NOIO
// PF_MEMALLOC_NOFS implies GFP_NOFS
// PF_MEMALLOC_PIN  implies !GFP_MOVABLE
//
// NOIO implies both NOIO and NOFS and it is a weaker context
// so always make sure it makes precedence
//

extern "C" {
    pub fn __fs_reclaim_acquire(ip: c_ulong);
}
extern "C" {
    pub fn __fs_reclaim_release(ip: c_ulong);
}
extern "C" {
    pub fn fs_reclaim_acquire(gfp_mask: gfp_t);
}
extern "C" {
    pub fn fs_reclaim_release(gfp_mask: gfp_t);
}

// Any memory-allocation retry loop should use
// memalloc_retry_wait(), and pass the flags for the most
// constrained allocation attempt that might have failed.
// This provides useful documentation of where loops are,
// and a central place to fine tune the waiting as the MM
// implementation changes.
//
// We use io_schedule_timeout because waiting for memory
// typically included waiting for dirty pages to be
// written out, which requires IO.
//
// Probably waited already, no need for much more
// Probably didn't wait, and has now released a lock,
// so now is a good time to wait
//
// might_alloc - Mark possible allocation sites
// @gfp_mask: gfp_t flags that would be used to allocate
//
// Similar to might_sleep() and other annotations, this can be used in functions
// that might allocate, but often don't. Compiles to nothing without
// CONFIG_LOCKDEP. Includes a conditional might_sleep() if @gfp allows blocking.
//
// memalloc_flags_save - Add a PF_* flag to current->flags, save old value
// @flags: Flags to add.
//
// This allows PF_* flags to be conveniently added, irrespective of current
// value, and then the old version restored with memalloc_flags_restore().
//
// memalloc_noio_save - Marks implicit GFP_NOIO allocation scope.
//
// This functions marks the beginning of the GFP_NOIO allocation scope.
// All further allocations will implicitly drop __GFP_IO flag and so
// they are safe for the IO critical section from the allocation recursion
// point of view. Use memalloc_noio_restore to end the scope with flags
// returned by this function.
//
// Context: This function is safe to be used from any context.
// Return: The saved flags to be passed to memalloc_noio_restore.
//
extern "C" {
    pub fn memalloc_flags_save(_arg: PF_MEMALLOC_NOIO) -> return;
}
//
// memalloc_noio_restore - Ends the implicit GFP_NOIO scope.
// @flags: Flags to restore.
//
// Ends the implicit GFP_NOIO scope started by memalloc_noio_save function.
// Always make sure that the given flags is the return value from the
// pairing memalloc_noio_save call.
//
// memalloc_nofs_save - Marks implicit GFP_NOFS allocation scope.
//
// This functions marks the beginning of the GFP_NOFS allocation scope.
// All further allocations will implicitly drop __GFP_FS flag and so
// they are safe for the FS critical section from the allocation recursion
// point of view. Use memalloc_nofs_restore to end the scope with flags
// returned by this function.
//
// Context: This function is safe to be used from any context.
// Return: The saved flags to be passed to memalloc_nofs_restore.
//
extern "C" {
    pub fn memalloc_flags_save(_arg: PF_MEMALLOC_NOFS) -> return;
}
//
// memalloc_nofs_restore - Ends the implicit GFP_NOFS scope.
// @flags: Flags to restore.
//
// Ends the implicit GFP_NOFS scope started by memalloc_nofs_save function.
// Always make sure that the given flags is the return value from the
// pairing memalloc_nofs_save call.
//
// memalloc_noreclaim_save - Marks implicit __GFP_MEMALLOC scope.
//
// This function marks the beginning of the __GFP_MEMALLOC allocation scope.
// All further allocations will implicitly add the __GFP_MEMALLOC flag, which
// prevents entering reclaim and allows access to all memory reserves. This
// should only be used when the caller guarantees the allocation will allow more
// memory to be freed very shortly, i.e. it needs to allocate some memory in
// the process of freeing memory, and cannot reclaim due to potential recursion.
//
// Users of this scope have to be extremely careful to not deplete the reserves
// completely and implement a throttling mechanism which controls the
// consumption of the reserve based on the amount of freed memory. Usage of a
// pre-allocated pool (e.g. mempool) should be always considered before using
// this scope.
//
// Individual allocations under the scope can opt out using __GFP_NOMEMALLOC
//
// Context: This function should not be used in an interrupt context as that one
// does not give PF_MEMALLOC access to reserves.
// See __gfp_pfmemalloc_flags().
// Return: The saved flags to be passed to memalloc_noreclaim_restore.
//
extern "C" {
    pub fn memalloc_flags_save(_arg: PF_MEMALLOC) -> return;
}
//
// memalloc_noreclaim_restore - Ends the implicit __GFP_MEMALLOC scope.
// @flags: Flags to restore.
//
// Ends the implicit __GFP_MEMALLOC scope started by memalloc_noreclaim_save
// function. Always make sure that the given flags is the return value from the
// pairing memalloc_noreclaim_save call.
//
// memalloc_pin_save - Marks implicit ~__GFP_MOVABLE scope.
//
// This function marks the beginning of the ~__GFP_MOVABLE allocation scope.
// All further allocations will implicitly remove the __GFP_MOVABLE flag, which
// will constraint the allocations to zones that allow long term pinning, i.e.
// not ZONE_MOVABLE zones.
//
// Return: The saved flags to be passed to memalloc_pin_restore.
//
extern "C" {
    pub fn memalloc_flags_save(_arg: PF_MEMALLOC_PIN) -> return;
}
//
// memalloc_pin_restore - Ends the implicit ~__GFP_MOVABLE scope.
// @flags: Flags to restore.
//
// Ends the implicit ~__GFP_MOVABLE scope started by memalloc_pin_save function.
// Always make sure that the given flags is the return value from the pairing
// memalloc_pin_save call.
//

//
// set_active_memcg - Starts the remote memcg charging scope.
// @memcg: memcg to charge.
//
// This function marks the beginning of the remote memcg charging scope. All the
// __GFP_ACCOUNT allocations till the end of the scope will be charged to the
// given memcg.
//
// Please, make sure that caller has a reference to the passed memcg structure,
// so its lifetime is guaranteed to exceed the scope between two
// set_active_memcg() calls.
//
// NOTE: This function can nest. Users must save the return value and
// reset the previous value after their own charging scope is over.
//

//
// The atomic_read() below prevents CSE. The following should
// help the compiler generate more efficient code on architectures
// where sync_core_before_usermode() is a no-op.
//
extern "C" {
    pub fn membarrier_exec_mmap(mm: *mut mm_struct);
}
extern "C" {
    pub fn membarrier_update_current_mm(next_mm: *mut mm_struct);
}

