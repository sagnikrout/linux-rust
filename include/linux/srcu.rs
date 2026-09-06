//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/srcu.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Sleepable Read-Copy Update mechanism for mutual exclusion
//
// Copyright (C) IBM Corporation, 2006
// Copyright (C) Fujitsu, 2012
//
// Author: Paul McKenney <paulmck@linux.ibm.com>
// Lai Jiangshan <laijs@cn.fujitsu.com>
//
// For detailed explanation of Read-Copy Update mechanism see -
// Documentation/RCU/ *.txt
//

extern "C" {
    pub fn init_srcu_struct_lockdep(_arg: ssp, _arg: name, _arg: key) -> return;
}

extern "C" {
    pub fn __init_srcu_struct_fast(ssp: *mut srcu_struct, name: *const c_char, key: *mut lock_class_key) -> c_int;
}

extern "C" {
    pub fn init_srcu_struct_generic(ssp: *mut srcu_struct) -> c_int;
}
extern "C" {
    pub fn init_srcu_struct_generic(_arg: ssp) -> return;
}

extern "C" {
    pub fn init_srcu_struct_fast(ssp: *mut srcu_struct) -> c_int;
}
extern "C" {
    pub fn init_srcu_struct_fast_updown(ssp: *mut srcu_struct) -> c_int;
}

// Macro flag: #define __SRCU_DEP_MAP_INIT(srcu_name)

// Values for SRCU Tree srcu_data ->srcu_reader_flavor, but also used by rcutorture.
pub const SRCU_READ_FLAVOR_NORMAL: c_uint = 0x1		// srcu_read_lock().;
pub const SRCU_READ_FLAVOR_NMI: c_uint = 0x2		// srcu_read_lock_nmisafe().;
// 0x4		// SRCU-lite is no longer with us.
pub const SRCU_READ_FLAVOR_FAST: c_uint = 0x4		// srcu_read_lock_fast(), also NMI-safe.;
pub const SRCU_READ_FLAVOR_FAST_UPDOWN: c_uint = 0x8		// srcu_read_lock_fast_updown().;

// All of the above.

// Flavors requiring synchronize_rcu()
// instead of smp_mb().
extern "C" {
    pub fn __srcu_read_unlock(ssp: *mut srcu_struct, __releases_shared(ssp: int idx));
}

extern "C" {
    pub fn cleanup_srcu_struct(ssp: *mut srcu_struct);
}
extern "C" {
    pub fn synchronize_srcu(ssp: *mut srcu_struct);
}
pub const SRCU_GET_STATE_COMPLETED: c_uint = 0x1;
//
// get_completed_synchronize_srcu - Return a pre-completed polled state cookie
//
// Returns a value that poll_state_synchronize_srcu() will always treat
// as a cookie whose grace period has already completed.
//
extern "C" {
    pub fn get_state_synchronize_srcu(ssp: *mut srcu_struct) -> c_ulong;
}
extern "C" {
    pub fn start_poll_synchronize_srcu(ssp: *mut srcu_struct) -> c_ulong;
}
extern "C" {
    pub fn poll_state_synchronize_srcu(ssp: *mut srcu_struct, cookie: c_ulong) -> bool;
}
// Maximum number of unsigned long values corresponding to
// not-yet-completed SRCU grace periods.
pub const NUM_ACTIVE_SRCU_POLL_OLDSTATE: c_int = 2;
//
// same_state_synchronize_srcu - Are two old-state values identical?
// @oldstate1: First old-state value.
// @oldstate2: Second old-state value.
//
// The two old-state values must have been obtained from either
// get_state_synchronize_srcu(), start_poll_synchronize_srcu(), or
// get_completed_synchronize_srcu().  Returns @true if the two values are
// identical and @false otherwise.  This allows structures whose lifetimes
// are tracked by old-state values to push these values to a list header,
// allowing those structures to be slightly smaller.
//

extern "C" {
    pub fn __srcu_read_lock_nmisafe(__acquires_shared(ssp: *mut *mut srcu_ssp)) -> c_int;
}
extern "C" {
    pub fn __srcu_read_unlock_nmisafe(ssp: *mut srcu_struct, __releases_shared(ssp: int idx));
}

extern "C" {
    pub fn __srcu_read_lock(_arg: ssp) -> return;
}

extern "C" {
    pub fn srcu_init();
}

//
// srcu_read_lock_held - might we be in SRCU read-side critical section?
// @ssp: The srcu_struct structure to check
//
// If CONFIG_DEBUG_LOCK_ALLOC is selected, returns nonzero iff in an SRCU
// read-side critical section.  In absence of CONFIG_DEBUG_LOCK_ALLOC,
// this assumes we are in an SRCU read-side critical section unless it can
// prove otherwise.
//
// Checks debug_lockdep_rcu_enabled() to prevent false positives during boot
// and while lockdep is disabled.
//
// Note that SRCU is based on its own statemachine and it doesn't
// relies on normal RCU, it can be called from the CPU which
// is in the idle loop from an RCU point of view or offline.
//
extern "C" {
    pub fn lock_is_held(_arg: &ssp->dep_map) -> return;
}
//
// Annotations provide deadlock detection for SRCU.
//
// Similar to other lockdep annotations, except there is an additional
// srcu_lock_sync(), which is basically an empty *write*-side critical section,
// see lock_sync() for more information.
//
// Annotates a srcu_read_lock()
// Annotates a synchronize_srcu()

//
// No-op helper to denote that ssp must be held. Because SRCU-protected pointers
// should still be marked with __rcu_guarded, and we do not want to mark them
// with __guarded_by(ssp) as it would complicate annotations for writers, we
// choose the following strategy: srcu_dereference_check() calls this helper
// that checks that the passed ssp is held, and then fake-acquires 'RCU'.
//
// srcu_dereference_check - fetch SRCU-protected pointer for later dereferencing
// @p: the pointer to fetch and protect for later dereferencing
// @ssp: pointer to the srcu_struct, which is used to check that we
// really are in an SRCU read-side critical section.
// @c: condition to check for update-side use
//
// If PROVE_RCU is enabled, invoking this outside of an RCU read-side
// critical section will result in an RCU-lockdep splat, unless @c evaluates
// to 1.  The @c argument will normally be a logical expression containing
// lockdep_is_held() calls.
//

//
// srcu_dereference - fetch SRCU-protected pointer for later dereferencing
// @p: the pointer to fetch and protect for later dereferencing
// @ssp: pointer to the srcu_struct, which is used to check that we
// really are in an SRCU read-side critical section.
//
// Makes rcu_dereference_check() do the dirty work.  If PROVE_RCU
// is enabled, invoking this outside of an RCU read-side critical
// section will result in an RCU-lockdep splat.
//

//
// srcu_dereference_notrace - no tracing and no lockdep calls from here
// @p: the pointer to fetch and protect for later dereferencing
// @ssp: pointer to the srcu_struct, which is used to check that we
// really are in an SRCU read-side critical section.
//

//
// srcu_read_lock - register a new reader for an SRCU-protected structure.
// @ssp: srcu_struct in which to register the new reader.
//
// Enter an SRCU read-side critical section.  Note that SRCU read-side
// critical sections may be nested.  However, it is illegal to
// call anything that waits on an SRCU grace period for the same
// srcu_struct, whether directly or indirectly.  Please note that
// one way to indirectly wait on an SRCU grace period is to acquire
// a mutex that is held elsewhere while calling synchronize_srcu() or
// synchronize_srcu_expedited().
//
// The return value from srcu_read_lock() is guaranteed to be
// non-negative.  This value must be passed unaltered to the matching
// srcu_read_unlock().  Note that srcu_read_lock() and the matching
// srcu_read_unlock() must occur in the same context, for example, it is
// illegal to invoke srcu_read_unlock() in an irq handler if the matching
// srcu_read_lock() was invoked in process context.  Or, for that matter to
// invoke srcu_read_unlock() from one task and the matching srcu_read_lock()
// from another.
//
// srcu_read_lock_fast - register a new reader for an SRCU-protected structure.
// @ssp: srcu_struct in which to register the new reader.
//
// Enter an SRCU read-side critical section, but for a light-weight
// smp_mb()-free reader.  See srcu_read_lock() for more information.  This
// function is NMI-safe, in a manner similar to srcu_read_lock_nmisafe().
//
// For srcu_read_lock_fast() to be used on an srcu_struct structure,
// that structure must have been defined using either DEFINE_SRCU_FAST()
// or DEFINE_STATIC_SRCU_FAST() on the one hand or initialized with
// init_srcu_struct_fast() on the other.  Such an srcu_struct structure
// cannot be passed to any non-fast variant of srcu_read_{,un}lock() or
// srcu_{down,up}_read().  In kernels built with CONFIG_PROVE_RCU=y,
// __srcu_check_read_flavor() will complain bitterly if you ignore this
// restriction.
//
// Grace-period auto-expediting is disabled for SRCU-fast srcu_struct
// structures because SRCU-fast expedited grace periods invoke
// synchronize_rcu_expedited(), IPIs and all.  If you need expedited
// SRCU-fast grace periods, use synchronize_srcu_expedited().
//
// The srcu_read_lock_fast() function can be invoked only from those
// contexts where RCU is watching, that is, from contexts where it would
// be legal to invoke rcu_read_lock().  Otherwise, lockdep will complain.
//
// srcu_read_lock_fast_updown - register a new reader for an SRCU-fast-updown structure.
// @ssp: srcu_struct in which to register the new reader.
//
// Enter an SRCU read-side critical section, but for a light-weight
// smp_mb()-free reader.  See srcu_read_lock() for more information.
// This function is compatible with srcu_down_read_fast(), but is not
// NMI-safe.
//
// For srcu_read_lock_fast_updown() to be used on an srcu_struct
// structure, that structure must have been defined using either
// DEFINE_SRCU_FAST_UPDOWN() or DEFINE_STATIC_SRCU_FAST_UPDOWN() on the one
// hand or initialized with init_srcu_struct_fast_updown() on the other.
// Such an srcu_struct structure cannot be passed to any non-fast-updown
// variant of srcu_read_{,un}lock() or srcu_{down,up}_read().  In kernels
// built with CONFIG_PROVE_RCU=y, __srcu_check_read_flavor() will complain
// bitterly if you ignore this * restriction.
//
// Grace-period auto-expediting is disabled for SRCU-fast-updown
// srcu_struct structures because SRCU-fast-updown expedited grace periods
// invoke synchronize_rcu_expedited(), IPIs and all.  If you need expedited
// SRCU-fast-updown grace periods, use synchronize_srcu_expedited().
//
// The srcu_read_lock_fast_updown() function can be invoked only from
// those contexts where RCU is watching, that is, from contexts where
// it would be legal to invoke rcu_read_lock().  Otherwise, lockdep will
// complain.
//
// Used by tracing, cannot be traced and cannot call lockdep.
// See srcu_read_lock_fast() for more information.
//
// srcu_down_read_fast - register a new reader for an SRCU-protected structure.
// @ssp: srcu_struct in which to register the new reader.
//
// Enter a semaphore-like SRCU read-side critical section, but for
// a light-weight smp_mb()-free reader.  See srcu_read_lock_fast() and
// srcu_down_read() for more information.
//
// The same srcu_struct may be used concurrently by srcu_down_read_fast()
// and srcu_read_lock_fast().  However, the same definition/initialization
// requirements called out for srcu_read_lock_fast_updown() apply.
//
extern "C" {
    pub fn __srcu_read_lock_fast_updown(_arg: ssp) -> return;
}
//
// srcu_read_lock_nmisafe - register a new reader for an SRCU-protected structure.
// @ssp: srcu_struct in which to register the new reader.
//
// Enter an SRCU read-side critical section, but in an NMI-safe manner.
// See srcu_read_lock() for more information.
//
// If srcu_read_lock_nmisafe() is ever used on an srcu_struct structure,
// then none of the other flavors may be used, whether before, during,
// or after.
//
// Used by tracing, cannot be traced and cannot invoke lockdep.
//
// srcu_down_read - register a new reader for an SRCU-protected structure.
// @ssp: srcu_struct in which to register the new reader.
//
// Enter a semaphore-like SRCU read-side critical section.  Note that
// SRCU read-side critical sections may be nested.  However, it is
// illegal to call anything that waits on an SRCU grace period for the
// same srcu_struct, whether directly or indirectly.  Please note that
// one way to indirectly wait on an SRCU grace period is to acquire
// a mutex that is held elsewhere while calling synchronize_srcu() or
// synchronize_srcu_expedited().  But if you want lockdep to help you
// keep this stuff straight, you should instead use srcu_read_lock().
//
// The semaphore-like nature of srcu_down_read() means that the matching
// srcu_up_read() can be invoked from some other context, for example,
// from some other task or from an irq handler.  However, neither
// srcu_down_read() nor srcu_up_read() may be invoked from an NMI handler.
//
// Calls to srcu_down_read() may be nested, similar to the manner in
// which calls to down_read() may be nested.  The same srcu_struct may be
// used concurrently by srcu_down_read() and srcu_read_lock().
//
extern "C" {
    pub fn __srcu_read_lock(_arg: ssp) -> return;
}
//
// srcu_read_unlock - unregister a old reader from an SRCU-protected structure.
// @ssp: srcu_struct in which to unregister the old reader.
// @idx: return value from corresponding srcu_read_lock().
//
// Exit an SRCU read-side critical section.
//
// srcu_read_unlock_fast - unregister a old reader from an SRCU-protected structure.
// @ssp: srcu_struct in which to unregister the old reader.
// @scp: return value from corresponding srcu_read_lock_fast().
//
// Exit a light-weight SRCU read-side critical section.
//
// srcu_read_unlock_fast_updown - unregister a old reader from an SRCU-fast-updown structure.
// @ssp: srcu_struct in which to unregister the old reader.
// @scp: return value from corresponding srcu_read_lock_fast_updown().
//
// Exit an SRCU-fast-updown read-side critical section.
//
// Used by tracing, cannot be traced and cannot call lockdep.
// See srcu_read_unlock_fast() for more information.
//
// srcu_up_read_fast - unregister a old reader from an SRCU-protected structure.
// @ssp: srcu_struct in which to unregister the old reader.
// @scp: return value from corresponding srcu_read_lock_fast().
//
// Exit an SRCU read-side critical section, but not necessarily from
// the same context as the maching srcu_down_read_fast().
//
// srcu_read_unlock_nmisafe - unregister a old reader from an SRCU-protected structure.
// @ssp: srcu_struct in which to unregister the old reader.
// @idx: return value from corresponding srcu_read_lock_nmisafe().
//
// Exit an SRCU read-side critical section, but in an NMI-safe manner.
//
// Used by tracing, cannot be traced and cannot call lockdep.
//
// srcu_up_read - unregister a old reader from an SRCU-protected structure.
// @ssp: srcu_struct in which to unregister the old reader.
// @idx: return value from corresponding srcu_read_lock().
//
// Exit an SRCU read-side critical section, but not necessarily from
// the same context as the maching srcu_down_read().
//
// smp_mb__after_srcu_read_unlock - ensure full ordering after srcu_read_unlock
//
// Converts the preceding srcu_read_unlock into a two-way memory barrier.
//
// Call this after srcu_read_unlock, to guarantee that all memory operations
// that occur after smp_mb__after_srcu_read_unlock will appear to happen after
// the preceding srcu_read_unlock.
//
// __srcu_read_unlock has smp_mb() internally so nothing to do here.
//
// smp_mb__after_srcu_read_lock - ensure full ordering after srcu_read_lock
//
// Converts the preceding srcu_read_lock into a two-way memory barrier.
//
// Call this after srcu_read_lock, to guarantee that all memory operations
// that occur after smp_mb__after_srcu_read_lock will appear to happen after
// the preceding srcu_read_lock.
//
// __srcu_read_lock has smp_mb() internally so nothing to do here.

