//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/locking/lockdep_internals.h
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
// kernel/lockdep_internals.h
//
// Runtime locking correctness validator
//
// lockdep subsystem internal functions and variables.
//
// Lock-class usage-state bits:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lock_usage_bit {

    LOCK_USED_IN_##__STATE,		\
    LOCK_USED_IN_##__STATE##_READ,	\
    LOCK_ENABLED_##__STATE,		\
    LOCK_ENABLED_##__STATE##_READ,

    LOCK_USED,
    LOCK_USED_READ,
    LOCK_USAGE_STATES,
}

// states after LOCK_USED_READ are not traced and printed
pub const LOCK_USAGE_READ_MASK: c_int = 1;
pub const LOCK_USAGE_DIR_MASK: c_int = 2;

//
// Usage-state bitmasks:
//

//
// CONFIG_LOCKDEP_SMALL is defined for sparc. Sparc requires .text,
// .data and .bss to fit in required 32MB limit for the kernel. With
// CONFIG_LOCKDEP we could go over this limit and cause system boot-up problems.
// So, reduce the static allocations for lockdeps related structures so that
// everything fits in current required size limit.
//

//
// MAX_LOCKDEP_ENTRIES is the maximum number of lock dependencies
// we track.
//
// We use the per-lock dependency maps in two ways: we grow it by adding
// every to-be-taken lock to all currently held lock's own dependency
// table (if it's not there yet), and we check it for lock order
// conflicts and deadlocks.
//

pub const MAX_LOCKDEP_CHAINS_BITS: c_int = 15;

pub const STACK_TRACE_HASH_SIZE: c_int = 8192;

//
// Stack-trace: tightly packed array of stack backtrace
// addresses. Protected by the hash_lock.
//

//
// Bit definitions for lock_chain.irq_context
//

pub const AVG_LOCKDEP_CHAIN_DEPTH: c_int = 5;

extern "C" {
    pub fn lockdep_next_lockchain(i: c_long) -> c_long;
}
extern "C" {
    pub fn lock_chain_count() -> c_ulong;
}

extern "C" {
    pub fn lockdep_count_forward_deps(: *mut lock_class) -> c_ulong;
}
extern "C" {
    pub fn lockdep_count_backward_deps(: *mut lock_class) -> c_ulong;
}

extern "C" {
    pub fn lockdep_stack_trace_count() -> u64;
}
extern "C" {
    pub fn lockdep_stack_hash_count() -> u64;
}

//
// Various lockdep statistics.
// We want them per cpu as they are often accessed in fast path
// and we want to avoid too much cache bouncing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lockdep_stats {
    pub chain_lookup_hits: c_ulong,
    pub chain_lookup_misses: c_uint,
    pub hardirqs_on_events: c_ulong,
    pub hardirqs_off_events: c_ulong,
    pub redundant_hardirqs_on: c_ulong,
    pub redundant_hardirqs_off: c_ulong,
    pub softirqs_on_events: c_ulong,
    pub softirqs_off_events: c_ulong,
    pub redundant_softirqs_on: c_ulong,
    pub redundant_softirqs_off: c_ulong,
    pub nr_unused_locks: c_int,
    pub nr_redundant_checks: c_uint,
    pub nr_redundant: c_uint,
    pub nr_cyclic_checks: c_uint,
    pub nr_find_usage_forwards_checks: c_uint,
    pub nr_find_usage_backwards_checks: c_uint,
//
// Per lock class locking operation stat counts
//
    pub lock_class_ops: [c_ulong; MAX_LOCKDEP_KEYS],
}

