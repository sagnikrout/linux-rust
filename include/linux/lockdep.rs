//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lockdep.h
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
// Runtime locking correctness validator
//
// Copyright (C) 2006,2007 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
// Copyright (C) 2007 Red Hat, Inc., Peter Zijlstra
//
// see Documentation/locking/lockdep-design.rst for more details.
//

// to = *from;
//
// Since the class cache can be modified concurrently we could observe
// half pointers (64bit arch using 32bit copy insns). Therefore clear
// the caches and take the performance hit.
//
// XXX it doesn't work well with lockdep_set_class_and_subclass(), since
// that relies on cache abuse.
//
// Every lock has a list of other locks that were taken after it.
// We only grow the list, never remove from it:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_list {
    pub entry: list_head,
    pub class: *mut lock_class,
    pub links_to: *mut lock_class,
    pub trace: *const lock_trace,
    pub distance: u16,
// bitmap of different dependencies from head to this
    pub dep: u8,
// used by BFS to record whether "prev -> this" only has -(*R)->
    pub only_xr: u8,
//
// The parent field is used to implement breadth-first search, and the
// bit 0 is reused to indicate if the lock has been accessed in BFS.
//
    pub parent: *mut lock_list,
}

//
// struct lock_chain - lock dependency chain record
//
// @irq_context: the same as irq_context in held_lock below
// @depth:       the number of held locks in this chain
// @base:        the index in chain_hlocks for this chain
// @entry:       the collided lock chains in lock_chain hash list
// @chain_key:   the hash key of this lock_chain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_chain {
// see BUILD_BUG_ON()s in add_chain_cache()
    pub 24: base :,
// 4 byte hole
    pub entry: hlist_node,
    pub chain_key: u64,
}

//
// Initialization, self-test and debugging-output methods:
//
extern "C" {
    pub fn lockdep_init();
}
extern "C" {
    pub fn lockdep_reset();
}
extern "C" {
    pub fn lockdep_reset_lock(lock: *mut lockdep_map);
}
extern "C" {
    pub fn lockdep_free_key_range(start: *mut c_void, size: c_ulong);
}
extern "C" {
    pub fn lockdep_sys_exit() -> asmlinkage void;
}
extern "C" {
    pub fn lockdep_set_selftest_task(task: *mut task_struct);
}
extern "C" {
    pub fn lockdep_init_task(task: *mut task_struct);
}
//
// Split the recursion counter in two to readily detect 'off' vs recursion.
//
pub const LOCKDEP_RECURSION_BITS: c_int = 16;

//
// lockdep_{off,on}() are macros to avoid tracing and kprobes; not inlines due
// to header dependencies.
//

extern "C" {
    pub fn lockdep_register_key(key: *mut lock_class_key);
}
extern "C" {
    pub fn lockdep_unregister_key(key: *mut lock_class_key);
}
//
// These methods are used by specific locking variants (spinlocks,
// rwlocks, mutexes and rwsems) to pass init/acquire/release events
// to lockdep:
//
// Reinitialize a lock key - for cases where there is special locking or
// special initialization of locks so that the validator gets the scope
// of dependencies wrong: they are either too broad (they need a class-split)
// or they are too narrow (they suffer from a false class-split):
//

//
// lockdep_set_novalidate_class: disable checking of lock ordering on a given
// lock
// @lock: Lock to mark
//
// Lockdep will still record that this lock has been taken, and print held
// instances when dumping locks
//

//
// lockdep_set_notrack_class: disable lockdep tracking of a given lock entirely
// @lock: Lock to mark
//
// Bigger hammer than lockdep_set_novalidate_class: so far just for bcachefs,
// which takes more locks than lockdep is able to track (48).
//

//
// Compare locking classes
//

//
// Acquire a lock.
//
// Values for "read":
//
// 0: exclusive (write) acquire
// 1: read-acquire (no recursion allowed)
// 2: read-acquire with same-instance recursion allowed
//
// Values for check:
//
// 0: simple checks (freeing, held-at-exit-time, etc.)
// 1: full validation
//
extern "C" {
    pub fn lock_release(lock: *mut lockdep_map, ip: c_ulong);
}
// lock_is_held_type() returns

pub const LOCK_STATE_NOT_HELD: c_int = 0;
pub const LOCK_STATE_HELD: c_int = 1;
//
// Same "read" as for lock_acquire(), except -1 means any.
//
extern "C" {
    pub fn lock_is_held_type(lock: *const lockdep_map, read: c_int) -> c_int;
}
extern "C" {
    pub fn lock_is_held_type(_arg: lock, _arg: -1) -> return;
}

extern "C" {
    pub fn lock_downgrade(lock: *mut lockdep_map, ip: c_ulong);
}

extern "C" {
    pub fn lock_pin_lock(lock: *mut lockdep_map) -> pin_cookie;
}
extern "C" {
    pub fn lock_repin_lock(lock: *mut lockdep_map, pin_cookie: struct);
}
extern "C" {
    pub fn lock_unpin_lock(lock: *mut lockdep_map, pin_cookie: struct);
}
extern "C" {
    pub fn lock_sequence(lock: *mut lockdep_map) -> u32;
}

//
// Must use lock_map_aquire_try() with override maps to avoid
// lockdep thinking they participate in the block chain.
//

//
// We don't define lockdep_match_class() and lockdep_match_key() for !LOCKDEP
// case since the result is not well defined and the caller should rather
// #ifdef the call himself.
//

//
// Dummy forward declarations, allow users to write less ifdef-y code
// and depend on dead code elimination.
//
extern "C" {
    pub fn lock_is_held(: *const c_void) -> c_int;
}
extern "C" {
    pub fn lockdep_is_held(: *const c_void) -> c_int;
}

extern "C" {
    pub fn lockdep_set_lock_cmp_fn(: *mut lockdep_map, _arg: lock_cmp_fn, _arg: lock_print_fn);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xhlock_context_t {
    XHLOCK_HARD,
    XHLOCK_SOFT,
    XHLOCK_CTX_NR,
}

//
// To initialize a lockdep_map statically use this macro.
// Note that _name must not be NULL.
//

extern "C" {
    pub fn lock_contended(lock: *mut lockdep_map, ip: c_ulong);
}
extern "C" {
    pub fn lock_acquired(lock: *mut lockdep_map, ip: c_ulong);
}

extern "C" {
    pub fn print_irqtrace_events(curr: *mut task_struct);
}

// Variable used to make lockdep treat read_lock() as recursive in selftests

pub const force_read_lock_recursive: c_int = 0;

extern "C" {
    pub fn read_lock_is_recursive() -> bool;
}

// If !LOCKDEP, the value is meaningless
pub const read_lock_is_recursive(): c_int = 0;

//
// For trivial one-depth nesting of a lock-class, the following
// global define can be used. (Subsystems with multiple levels
// of nesting should define their own lock-nesting subclasses.)
//
pub const SINGLE_DEPTH_NESTING: c_int = 1;
//
// Map the dependency ops to NOP or to real lockdep ops, depending
// on the per lock-class debug mode:
//

//
// Acceptable for protecting per-CPU resources accessed from BH.
// Much like in_softirq() - semantics are ambiguous, use carefully.
//

extern "C" {
    pub fn lockdep_assert_in_softirq_func();
}

extern "C" {
    pub fn lockdep_rcu_suspicious(file: *const c_char, line: c_int, s: *const c_char);
}

