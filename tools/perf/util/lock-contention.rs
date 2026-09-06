//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/lock-contention.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_filter {
    pub nr_types: c_int,
    pub nr_addrs: c_int,
    pub nr_syms: c_int,
    pub nr_cgrps: c_int,
    pub nr_slabs: c_int,
    pub types: *mut c_uint,
    pub addrs: *mut c_ulong,
    pub syms: *mut c_char,
    pub cgrps: *mut u64,
    pub slabs: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_delay {
    pub sym: *mut c_char,
    pub addr: c_ulong,
    pub time: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_stat {
    pub hash_entry: hlist_node,
    pub /: *mut *mut rb_node rb; / used for sorting,
    pub /: *mut *mut u64 addr; / address of lockdep_map, used as ID,
    pub /: *const *const *const char name; / for strcpy(), we cannot use,
    pub callstack: *mut u64,
    pub nr_acquire: c_uint,
    pub nr_acquired: c_uint,
    pub nr_contended: c_uint,
    pub nr_release: c_uint,
    pub nr_readlock: c_uint,
    pub flags: c_uint,
}

// these times are in nano sec.
//
// States of lock_seq_stat
//
// UNINITIALIZED is required for detecting first event of acquire.
// As the nature of lock events, there is no guarantee
// that the first event for the locks are acquire,
// it can be acquired, contended or release.
//

pub const SEQ_STATE_RELEASED: c_int = 1;
pub const SEQ_STATE_ACQUIRING: c_int = 2;
pub const SEQ_STATE_ACQUIRED: c_int = 3;
pub const SEQ_STATE_READ_ACQUIRED: c_int = 4;
pub const SEQ_STATE_CONTENDED: c_int = 5;
//
// MAX_LOCK_DEPTH
// Imported from include/linux/sched.h.
// Should this be synchronized?
//
pub const MAX_LOCK_DEPTH: c_int = 48;
// based on kernel/lockdep.c
pub const LOCKHASH_BITS: c_int = 12;

//
// struct lock_seq_stat:
// Place to put on state of one lock sequence
// 1) acquire -> acquired -> release
// 2) acquire -> contended -> acquired -> release
// 3) acquire (with read or try) -> release
// 4) Are there other patterns?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_seq_stat {
    pub list: list_head,
    pub state: c_int,
    pub prev_event_time: u64,
    pub addr: u64,
    pub read_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_stat {
    pub rb: rb_node,
    pub tid: u32,
    pub seq_list: list_head,
}

//
// CONTENTION_STACK_DEPTH
// Number of stack trace entries to find callers
//
pub const CONTENTION_STACK_DEPTH: c_int = 8;
//
// CONTENTION_STACK_SKIP
// Number of stack trace entries to skip when finding callers.
// The first few entries belong to the locking implementation itself.
//
pub const CONTENTION_STACK_SKIP: c_int = 4;
//
// flags for lock:contention_begin
// Imported from include/trace/events/lock.h.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_contention_fails {
    pub task: c_int,
    pub stack: c_int,
    pub time: c_int,
    pub data: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_contention {
    pub evlist: *mut evlist,
    pub target: *mut target,
    pub machine: *mut machine,
    pub result: *mut hlist_head,
    pub filters: *mut lock_filter,
    pub delays: *mut lock_delay,
    pub fails: lock_contention_fails,
    pub cgroups: rb_root,
    pub btf: *mut c_void,
    pub map_nr_entries: c_ulong,
    pub max_stack: c_int,
    pub stack_skip: c_int,
    pub aggr_mode: c_int,
    pub owner: c_int,
    pub nr_filtered: c_int,
    pub nr_delays: c_int,
    pub save_callstack: bool,
}

extern "C" {
    pub fn parse_call_stack(opt: *const option, str: *const c_char, unset: c_int) -> c_int;
}
extern "C" {
    pub fn needs_callstack() -> bool;
}
extern "C" {
    pub fn match_callstack_filter(machine: *mut machine, callstack: *mut u64, max_stack_depth: c_int) -> bool;
}

extern "C" {
    pub fn lock_contention_prepare(con: *mut lock_contention) -> c_int;
}
extern "C" {
    pub fn lock_contention_start() -> c_int;
}
extern "C" {
    pub fn lock_contention_stop() -> c_int;
}
extern "C" {
    pub fn lock_contention_read(con: *mut lock_contention) -> c_int;
}
extern "C" {
    pub fn lock_contention_finish(con: *mut lock_contention) -> c_int;
}

