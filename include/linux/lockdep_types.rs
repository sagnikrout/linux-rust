//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lockdep_types.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lockdep_wait_type {
    LD_WAIT_INV = 0,	/* not checked, catch all */

    LD_WAIT_FREE,		/* wait free, rcu etc.. */
    LD_WAIT_SPIN,		/* spin loops, raw_spinlock_t etc.. */

    LD_WAIT_CONFIG,		/* preemptible in PREEMPT_RT, spinlock_t etc.. */

    LD_WAIT_CONFIG = LD_WAIT_SPIN,

    LD_WAIT_SLEEP,		/* sleeping locks, mutex_t etc.. */

    LD_WAIT_MAX,		/* must be last */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lockdep_lock_type {
    LD_LOCK_NORMAL = 0,	/* normal, catch all */
    LD_LOCK_PERCPU,		/* percpu */
    LD_LOCK_WAIT_OVERRIDE,	/* annotation */
    LD_LOCK_MAX,
}

//
// We'd rather not expose kernel/lockdep_states.h this wide, but we do need
// the total number of states... :-(
//
// XXX_LOCK_USAGE_STATES is the number of lines in lockdep_states.h, for each
// of those we generates 4 states, Additionally we report on USED and USED_READ.
//
pub const XXX_LOCK_USAGE_STATES: c_int = 2;

//
// NR_LOCKDEP_CACHING_CLASSES ... Number of classes
// cached in the instance of lockdep_map
//
// Currently main class (subclass == 0) and single depth subclass
// are cached in lockdep_map. This optimization is mainly targeting
// on rq->lock. double_rq_lock() acquires this highly competitive with
// single depth.
//
pub const NR_LOCKDEP_CACHING_CLASSES: c_int = 2;
//
// A lockdep key is associated with each lock object. For static locks we use
// the lock address itself as the key. Dynamically allocated lock objects can
// have a statically or dynamically allocated key. Dynamically allocated lock
// keys must be registered before being used and must be unregistered before
// the key memory is freed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lockdep_subclass_key {
    pub __one_byte: c_char,
// C attribute field omitted
// hash_entry is used to keep track of dynamically allocated keys.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_class_key {
    pub hash_entry: hlist_node,
    pub subkeys: [lockdep_subclass_key; MAX_LOCKDEP_SUBCLASSES],
}

pub const LOCKSTAT_POINTS: c_int = 4;
extern "C" {
    pub fn void(map: *const *const lock_print_fn)(struct lockdep_map) -> typedef;
}
//
// The lock-class itself. The order of the structure members matters.
// reinit_class() zeroes the key member and all subsequent members.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_class {
//
// class-hash:
//
    pub hash_entry: hlist_node,
//
// Entry in all_lock_classes when in use. Entry in free_lock_classes
// when not in use. Instances that are being freed are on one of the
// zapped_classes lists.
//
    pub lock_entry: list_head,
//
// These fields represent a directed graph of lock dependencies,
// to every node we attach a list of "forward" and a list of
// "backward" graph nodes.
//
    pub locks_before: list_head locks_after,,
    pub key: *const lockdep_subclass_key,
    pub cmp_fn: lock_cmp_fn,
    pub print_fn: lock_print_fn,
    pub subclass: c_uint,
    pub dep_gen_id: c_uint,
//
// IRQ/softirq usage tracking bits:
//
    pub usage_mask: c_ulong,
    pub usage_traces: [*const lock_trace; LOCK_TRACE_STATES],
    pub name: *const c_char,
//
// Generation counter, when doing certain classes of graph walking,
// to ensure that we check one node only once:
//
    pub name_version: c_int,
    pub wait_type_inner: u8,
    pub wait_type_outer: u8,
    pub lock_type: u8,
// u8				hole;
    pub contention_point: [c_ulong; LOCKSTAT_POINTS],
    pub contending_point: [c_ulong; LOCKSTAT_POINTS],
    pub __no_randomize_layout: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_time {
    pub min: i64,
    pub max: i64,
    pub total: i64,
    pub nr: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bounce_type {
    bounce_acquired_write,
    bounce_acquired_read,
    bounce_contended_write,
    bounce_contended_read,
    nr_bounce_types,

    bounce_acquired = bounce_acquired_write,
    bounce_contended = bounce_contended_write,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_class_stats {
    pub contention_point: [c_ulong; LOCKSTAT_POINTS],
    pub contending_point: [c_ulong; LOCKSTAT_POINTS],
    pub read_waittime: lock_time,
    pub write_waittime: lock_time,
    pub read_holdtime: lock_time,
    pub write_holdtime: lock_time,
    pub bounces: [c_ulong; nr_bounce_types],
}

extern "C" {
    pub fn lock_stats(class: *mut lock_class, stats: *mut lock_class_stats);
}
extern "C" {
    pub fn clear_lock_stats(class: *mut lock_class);
}

//
// Map the lock object (the lock instance) to the lock-class object.
// This is embedded into specific lock instances:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lockdep_map {
    pub key: *mut lock_class_key,
    pub class_cache: [*mut lock_class; NR_LOCKDEP_CACHING_CLASSES],
    pub name: *const c_char,
    pub /: *mut *mut u8 wait_type_outer; / can be taken in this context,
    pub /: *mut *mut u8 wait_type_inner; / presents this context,
    pub lock_type: u8,
// u8				hole;

    pub cpu: c_int,
    pub ip: c_ulong,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pin_cookie {
pub const MAX_LOCKDEP_KEYS_BITS: c_int = 13;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct held_lock {
//
// One-way hash of the dependency chain up to this point. We
// hash the hashes step by step as the dependency chain grows.
//
// We use it for dependency-caching and we skip detection
// passes and dependency-updates if there is a cache-hit, so
// it is absolutely critical for 100% coverage of the validator
// to have a unique key value for every unique dependency path
// that can occur in the system, to make a unique hash value
// as likely as possible - hence the 64-bit width.
//
// The task struct holds the current hash value (initialized
// with zero), here we store the previous hash value:
//
    pub prev_chain_key: u64,
    pub acquire_ip: c_ulong,
    pub instance: *mut lockdep_map,
    pub nest_lock: *mut lockdep_map,

    pub waittime_stamp: u64,
    pub holdtime_stamp: u64,

//
// class_idx is zero-indexed; it points to the element in
// lock_classes this held lock instance belongs to. class_idx is in
// the range from 0 to (MAX_LOCKDEP_KEYS-1) inclusive.
//
    pub class_idx:MAX_LOCKDEP_KEYS_BITS: c_uint,
//
// The lock-stack is unified in that the lock chains of interrupt
// contexts nest ontop of process context chains, but we 'separate'
// the hashes by starting with 0 if we cross into an interrupt
// context, and we also keep do not add cross-context lock
// dependencies - the lock usage graph walking covers that area
// anyway, and we'd just unnecessarily increase the number of
// dependencies otherwise. [Note: hardirq and softirq contexts
// are separated from each other too.]
//
// The following field is used to detect when we cross into an
// interrupt context:
//
    pub /: *mut *mut unsigned int irq_context:2; / bit 0 - soft, bit 1 - hard,
    pub /: *mut *mut unsigned int trylock:1; / 16 bits,
    pub /: *mut *mut unsigned int read:2; / see lock_acquire() comment,
    pub /: *mut *mut unsigned int check:1; / see lock_acquire() comment,
    pub hardirqs_off:1: c_uint,
    pub sync:1: c_uint,
    pub /: *mut *mut unsigned int references:11; / 32 bits,
    pub pin_count:24: c_uint,
    pub seq_count:8: c_uint,
}

//
// The class key takes no space if lockdep is disabled:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_class_key {
//
// The lockdep_map takes no space if lockdep is disabled:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lockdep_map {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pin_cookie {

