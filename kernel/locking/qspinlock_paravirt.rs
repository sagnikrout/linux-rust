//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/locking/qspinlock_paravirt.h
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
// Implement paravirt qspinlocks; the general idea is to halt the vcpus instead
// of spinning them.
//
// This relies on the architecture to provide two paravirt hypercalls:
//
// pv_wait(u8 *ptr, u8 val) -- suspends the vcpu if *ptr == val
// pv_kick(cpu)             -- wakes a suspended vcpu
//
// Using these we implement __pv_queued_spin_lock_slowpath() and
// __pv_queued_spin_unlock() to replace native_queued_spin_lock_slowpath() and
// native_queued_spin_unlock().
//

//
// Queue Node Adaptive Spinning
//
// A queue node vCPU will stop spinning if the vCPU in the previous node is
// not running. The one lock stealing attempt allowed at slowpath entry
// mitigates the slight slowdown for non-overcommitted guest with this
// aggressive wait-early mechanism.
//
// The status of the previous node will be checked at fixed interval
// controlled by PV_PREV_CHECK_MASK. This is to ensure that we won't
// pound on the cacheline of the previous node too heavily.
//
pub const PV_PREV_CHECK_MASK: c_uint = 0xff;
//
// Queue node uses: VCPU_RUNNING & VCPU_HALTED.
// Queue head uses: VCPU_RUNNING & VCPU_HASHED.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcpu_state {
    VCPU_RUNNING = 0,
    VCPU_HALTED,		/* Used only in pv_wait_node */
    VCPU_HASHED,		/* = pv_hash'ed + VCPU_HALTED */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pv_node {
    pub mcs: mcs_spinlock,
    pub cpu: c_int,
    pub state: u8,
}

//
// Hybrid PV queued/unfair lock
//
// By replacing the regular queued_spin_trylock() with the function below,
// it will be called once when a lock waiter enter the PV slowpath before
// being queued.
//
// The pending bit is set by the queue head vCPU of the MCS wait queue in
// pv_wait_head_or_lock() to signal that it is ready to spin on the lock.
// When that bit becomes visible to the incoming waiters, no lock stealing
// is allowed. The function will return immediately to make the waiters
// enter the MCS wait queue. So lock starvation shouldn't happen as long
// as the queued mode vCPUs are actively running to set the pending bit
// and hence disabling lock stealing.
//
// When the pending bit isn't set, the lock waiters will stay in the unfair
// mode spinning on the lock unless the MCS wait queue is empty. In this
// case, the lock waiters will enter the queued mode slowpath trying to
// become the queue head and set the pending bit.
//
// This hybrid PV queued/unfair lock combines the best attributes of a
// queued lock (no lock starvation) and an unfair lock (good performance
// on not heavily contended locks).
//

//
// Stay in unfair lock mode as long as queued mode waiters are
// present in the MCS wait queue but the pending bit isn't set.
//
// The pending bit is used by the queue head vCPU to indicate that it
// is actively spinning on the lock and no lock stealing is allowed.
//

//
// The pending bit check in pv_queued_spin_steal_lock() isn't a memory
// barrier. Therefore, an atomic cmpxchg_acquire() is used to acquire the
// lock just to be sure that it will get it.
//

//
// Try to clear pending bit & set locked bit
//

//
// Lock and MCS node addresses hash table for fast lookup
//
// Hashing is done on a per-cacheline basis to minimize the need to access
// more than one cacheline.
//
// Dynamically allocate a hash table big enough to hold at least 4X the
// number of possible cpus in the system. Allocation is done on page
// granularity. So the minimum number of hash buckets should be at least
// 256 (64-bit) or 512 (32-bit) to fully utilize a 4k page.
//
// Since we should not be holding locks from NMI context (very rare indeed) the
// max load factor is 0.75, which is around the point where open addressing
// breaks down.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pv_hash_entry {
    pub lock: *mut qspinlock,
    pub node: *mut pv_node,
}

//
// Allocate memory for the PV qspinlock hash buckets
//
// This function should be called from the paravirt spinlock initialization
// routine.
//
// Allocate space from bootmem which should be page-size aligned
// and hence cacheline aligned.
//

//
// Hard assume there is a free entry for us.
//
// This is guaranteed by ensuring every blocked lock only ever consumes
// a single entry, and since we only have 4 nesting levels per CPU
// and allocated 4*nr_possible_cpus(), this must be so.
//
// The single entry is guaranteed by having the lock owner unhash
// before it releases.
//
// Hard assume we'll find an entry.
//
// This guarantees a limited lookup time and is itself guaranteed by
// having the lock owner do the unhash -- IFF the unlock sees the
// SLOW flag, there MUST be a hash entry.
//
// Return true if when it is time to check the previous node which is not
// in a running state.
//
// Initialize the PV part of the mcs_spinlock node.
//
// Wait for node->locked to become true, halt the vcpu after a short spin.
// pv_kick_node() is used to set _Q_SLOW_VAL and fill in hash table on its
// behalf.
//
// Order pn->state vs pn->locked thusly:
//
// [S] pn->state = VCPU_HALTED	  [S] next->locked = 1
// MB			      MB
// [L] pn->locked		[RmW] pn->state = VCPU_HASHED
//
// Matches the cmpxchg() from pv_kick_node().
//
// If pv_kick_node() changed us to VCPU_HASHED, retain that
// value so that pv_wait_head_or_lock() knows to not also try
// to hash this lock.
//
// If the locked flag is still not set after wakeup, it is a
// spurious wakeup and the vCPU should wait again. However,
// there is a pretty high overhead for CPU halting and kicking.
// So it is better to spin for a while in the hope that the
// MCS lock will be released soon.
//
// By now our node->locked should be 1 and our caller will not actually
// spin-wait for it. We do however rely on our caller to do a
// load-acquire for us.
//
// Called after setting next->locked = 1 when we're the lock owner.
//
// Instead of waking the waiters stuck in pv_wait_node() advance their state
// such that they're waiting in pv_wait_head_or_lock(), this avoids a
// wake/sleep cycle.
//
// If the vCPU is indeed halted, advance its state to match that of
// pv_wait_node(). If OTOH this fails, the vCPU was running and will
// observe its next->locked value and advance itself.
//
// Matches with smp_store_mb() and cmpxchg() in pv_wait_node()
//
// The write to next->locked in arch_mcs_spin_unlock_contended()
// must be ordered before the read of pn->state in the cmpxchg()
// below for the code to work correctly. To guarantee full ordering
// irrespective of the success or failure of the cmpxchg(),
// a relaxed version with explicit barrier is used. The control
// dependency will order the reading of pn->state before any
// subsequent writes.
//
// Put the lock into the hash table and set the _Q_SLOW_VAL.
//
// As this is the same vCPU that will check the _Q_SLOW_VAL value and
// the hash table later on at unlock time, no atomic instruction is
// needed.
//
// Wait for l->locked to become clear and acquire the lock;
// halt the vcpu after a short spin.
// __pv_queued_spin_unlock() will wake us.
//
// The current value of the lock will be returned for additional processing.
//
// If pv_kick_node() already advanced our state, we don't need to
// insert ourselves into the hash table anymore.
//
// Tracking # of slowpath locking operations
//
// Set correct vCPU state to be used by queue node wait-early
// mechanism.
//
// Set the pending bit in the active lock spinning loop to
// disable lock stealing before attempting to acquire the lock.
//
// We must hash before setting _Q_SLOW_VAL, such that
// when we observe _Q_SLOW_VAL in __pv_queued_spin_unlock()
// we'll be sure to be able to observe our hash entry.
//
// [S] <hash>                 [Rmw] l->locked == _Q_SLOW_VAL
// MB                           RMB
// [RmW] l->locked = _Q_SLOW_VAL  [L] <unhash>
//
// Matches the smp_rmb() in __pv_queued_spin_unlock().
//
// The lock was free and now we own the lock.
// Change the lock value back to _Q_LOCKED_VAL
// and unhash the table.
//
// Because of lock stealing, the queue head vCPU may not be
// able to acquire the lock before it has to wait again.
//
// The cmpxchg() or xchg() call before coming here provides the
// acquire semantics for locking. The dummy ORing of _Q_LOCKED_VAL
// here is to indicate to the compiler that the value will always
// be nozero to enable better code optimization.
//
// Include the architecture specific callee-save thunk of the
// __pv_queued_spin_unlock(). This thunk is put together with
// __pv_queued_spin_unlock() to make the callee-save thunk and the real unlock
// function close to each other sharing consecutive instruction cachelines.
// Alternatively, architecture specific version of __pv_queued_spin_unlock()
// can be defined.
//

//
// PV versions of the unlock fastpath and slowpath functions to be used
// instead of queued_spin_unlock().
//
// A failed cmpxchg doesn't provide any memory-ordering guarantees,
// so we need a barrier to order the read of the node data in
// pv_unhash *after* we've read the lock being _Q_SLOW_VAL.
//
// Matches the cmpxchg() in pv_wait_head_or_lock() setting _Q_SLOW_VAL.
//
// Since the above failed to release, this must be the SLOW path.
// Therefore start by looking up the blocked node and unhashing it.
//
// Now that we have a reference to the (likely) blocked pv_node,
// release the lock.
//
// At this point the memory pointed at by lock can be freed/reused,
// however we can still use the pv_node to kick the CPU.
// The other vCPU may not really be halted, but kicking an active
// vCPU is harmless other than the additional latency in completing
// the unlock.
//

//
// We must not unlock if SLOW, because in that case we must first
// unhash. Otherwise it would be possible to have multiple @lock
// entries, which would be BAD.
//
