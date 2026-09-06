//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/libarena/include/bpf_arena_spin_lock.h
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

pub const EBUSY: c_int = 16;
pub const EOPNOTSUPP: c_int = 95;
pub const ETIMEDOUT: c_int = 110;
//
// Typically, we'd just rely on the definition in vmlinux.h for qspinlock, but
// PowerPC overrides the definition to define lock->val as u32 instead of
// atomic_t, leading to compilation errors.  Import a local definition below so
// that we don't depend on the vmlinux.h version.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __qspinlock {
    pub val: core::sync::atomic::AtomicI32,

    pub locked: u8,
    pub pending: u8,
}

// FIXME: Using typedef causes CO-RE relocation error
// typedef struct qspinlock arena_spinlock_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arena_mcs_spinlock {
    pub next: *mut arena_mcs_spinlock __arena,
    pub locked: c_int,
    pub count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arena_qnode {
    pub mcs: arena_mcs_spinlock,
}

pub const _Q_MAX_NODES: c_int = 4;
pub const _Q_PENDING_LOOPS: c_int = 1;
//
// Bitfields in the atomic value:
//
// 0- 7: locked byte
// 8: pending
// 9-15: not used
// 16-17: tail index
// 18-31: tail cpu (+1)
//
pub const _Q_MAX_CPUS: c_int = 1024;

pub const _Q_LOCKED_OFFSET: c_int = 0;
pub const _Q_LOCKED_BITS: c_int = 8;

pub const _Q_PENDING_BITS: c_int = 8;

pub const _Q_TAIL_IDX_BITS: c_int = 2;

//
// xchg_tail - Put in the new queue tail code word & retrieve previous one
// @lock : Pointer to queued spinlock structure
// @tail : The new queue tail code word
// Return: The previous queue tail code word
//
// xchg(lock, tail)
//
// p,*,* -> n,*,* ; prev = xchg(lock, node)
//
// We can use relaxed semantics since the caller ensures that
// the MCS node is properly initialized before updating the
// tail.
//
// These loops are not expected to stall, but we still need to
// prove to the verifier they will terminate eventually.
//
// clear_pending - clear the pending bit.
// @lock: Pointer to queued spinlock structure
//
// *,1,* -> *,0,
//
// clear_pending_set_locked - take ownership and clear the pending bit.
// @lock: Pointer to queued spinlock structure
//
// *,1,0 -> *,0,1
//
// Lock stealing is not allowed if this function is used.
//
// set_locked - Set the lock bit and own the lock
// @lock: Pointer to queued spinlock structure
//
// *,*,0 -> *,0,1
//
// These loops are not expected to stall, but we still need to
// prove to the verifier they will terminate eventually.
//
// arena_spin_trylock - try to acquire the queued spinlock
// @lock : Pointer to queued spinlock structure
// Return: 1 if lock acquired, 0 if failed
//
extern "C" {
    pub fn likely(_arg: atomic_try_cmpxchg_acquire(&lock->val, _arg: &val, _arg: _Q_LOCKED_VAL)) -> return;
}
//
// Wait for in-progress pending->locked hand-overs with a bounded
// number of spins so that we guarantee forward progress.
//
// 0,1,0 -> 0,0,1
//
// If we observe any contention; queue.
//
// trylock || pending
//
// 0,0,* -> 0,1,* -> 0,0,1 pending, trylock
//
// If we observe contention, there is a concurrent locker.
//
// Undo and queue; our setting of PENDING might have made the
// n,0,0 -> 0,0,0 transition fail and it will now be waiting
// on @next to become !NULL.
//
// Undo PENDING if we set it.
//
// We're pending, wait for the owner to go away.
//
// 0,1,1 -> *,1,0
//
// this wait loop must be a load-acquire such that we match the
// store-release that clears the locked bit and create lock
// sequentiality; this is because not all
// clear_pending_set_locked() implementations imply full
// barriers.
//
// take ownership and clear the pending bit.
//
// 0,1,0 -> 0,0,1
//
// End of pending bit optimistic spinning and beginning of MCS
// queuing.
//
// 4 nodes are allocated based on the assumption that there will not be
// nested NMIs taking spinlocks. That may not be true in some
// architectures even though the chance of needing more than 4 nodes
// will still be extremely unlikely. When that happens, we simply return
// an error. Original qspinlock has a trylock fallback in this case.
//
// Ensure that we increment the head node->count before initialising
// the actual node. If the compiler is kind enough to reorder these
// stores, then an IRQ could overwrite our assignments.
//
// We touched a (possibly) cold cacheline in the per-cpu queue node;
// attempt the trylock once more in the hope someone let go while we
// weren't watching.
//
// Ensure that the initialisation of @node is complete before we
// publish the updated tail via xchg_tail() and potentially link
// @node into the waitqueue via WRITE_ONCE(prev->next, node) below.
//
// Publish the updated tail.
// We have already touched the queueing cacheline; don't bother with
// pending stuff.
//
// p,*,* -> n,*,
//
// if there was a previous node; link it and wait until reaching the
// head of the waitqueue.
//
// Link @node into the waitqueue.
//
// While waiting for the MCS lock, the next pointer may have
// been set by another lock waiter. We cannot prefetch here
// due to lack of equivalent instruction in BPF ISA.
//
// we're at the head of the waitqueue, wait for the owner & pending to
// go away.
//
// *,x,y -> *,0,0
//
// this wait loop must use a load-acquire such that we match the
// store-release that clears the locked bit and create lock
// sequentiality; this is because the set_locked() function below
// does not imply a full barrier.
//
// claim the lock:
//
// n,0,0 -> 0,0,1 : lock, uncontended
// *,*,0 -> *,*,1 : lock, contended
//
// If the queue head is the only one in the queue (lock value == tail)
// and nobody is pending, clear the tail code and grab the lock.
// Otherwise, we only need to grab the lock.
//
// In the PV case we might already have _Q_LOCKED_VAL set, because
// of lock stealing; therefore we must also allow:
//
// n,0,1 -> 0,0,1
//
// Note: at this point: (val & _Q_PENDING_MASK) == 0, because of the
// above wait condition, therefore any concurrent setting of
// PENDING will make the uncontended transition fail.
//
// Either somebody is queued behind us or _Q_PENDING_VAL got set
// which will then detect the remaining tail and queue behind us
// ensuring we'll see a @next.
//
// contended path; wait for next if not observed yet, release.
//
// release the node
//
// Doing a normal dec vs this_cpu_dec is fine. An upper context always
// decrements count it incremented before returning, thus we're fine.
// For contexts interrupting us, they either observe our dec or not.
// Just ensure the compiler doesn't reorder this statement, as a
// this_cpu_dec implicitly implied that.
//
// arena_spin_lock - acquire a queued spinlock
// @lock: Pointer to queued spinlock structure
//
// On error, returned value will be negative.
// On success, zero is returned.
//
// The return value _must_ be tested against zero for success,
// instead of checking it against negative, for passing the
// BPF verifier.
//
// The user should do:
// if (arena_spin_lock(...) != 0) // failure
// or
// if (arena_spin_lock(...) == 0) // success
// or
// if (arena_spin_lock(...)) // failure
// or
// if (!arena_spin_lock(...)) // success
// instead of:
// if (arena_spin_lock(...) < 0) // failure
//
// The return value can still be inspected later.
//
// FIXME: bpf_assert_range(-MAX_ERRNO, 0) once we have it working for all cases.
//
// arena_spin_unlock - release a queued spinlock
// @lock : Pointer to queued spinlock structure
//
// unlock() needs release semantics:
//

