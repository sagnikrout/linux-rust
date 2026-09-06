//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/perf/include/internal/evlist.h
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

pub const PERF_EVLIST__HLIST_BITS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_evlist {
    pub entries: list_head,
    pub nr_entries: c_int,
    pub has_user_cpus: bool,
    pub needs_map_propagation: bool,
//
// The cpus passed from the command line or all online CPUs by
// default.
//
    pub user_requested_cpus: *mut perf_cpu_map,
// The union of all evsel cpu maps.
    pub all_cpus: *mut perf_cpu_map,
    pub threads: *mut perf_thread_map,
    pub nr_mmaps: c_int,
    pub mmap_len: usize,
    pub pollfd: fdarray,
    pub heads: [hlist_head; PERF_EVLIST__HLIST_SIZE],
    pub mmap: *mut perf_mmap,
    pub mmap_ovw: *mut perf_mmap,
    pub mmap_first: *mut perf_mmap,
    pub mmap_ovw_first: *mut perf_mmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_evlist_mmap_ops {
    pub idx: perf_evlist_mmap__cb_idx_t,
    pub get: perf_evlist_mmap__cb_get_t,
    pub mmap: perf_evlist_mmap__cb_mmap_t,
}

extern "C" {
    pub fn perf_evlist__alloc_pollfd(evlist: *mut perf_evlist) -> c_int;
}
extern "C" {
    pub fn perf_evlist__init(evlist: *mut perf_evlist);
}
extern "C" {
    pub fn perf_evlist__exit(evlist: *mut perf_evlist);
}
//
// __perf_evlist__for_each_entry - iterate thru all the evsels
// @list: list_head instance to iterate
// @evsel: struct perf_evsel iterator
//

//
// evlist__for_each_entry - iterate thru all the evsels
// @evlist: perf_evlist instance to iterate
// @evsel: struct perf_evsel iterator
//

//
// __perf_evlist__for_each_entry_reverse - iterate thru all the evsels in reverse order
// @list: list_head instance to iterate
// @evsel: struct evsel iterator
//

//
// perf_evlist__for_each_entry_reverse - iterate thru all the evsels in reverse order
// @evlist: evlist instance to iterate
// @evsel: struct evsel iterator
//

//
// __perf_evlist__for_each_entry_safe - safely iterate thru all the evsels
// @list: list_head instance to iterate
// @tmp: struct evsel temp iterator
// @evsel: struct evsel iterator
//

//
// perf_evlist__for_each_entry_safe - safely iterate thru all the evsels
// @evlist: evlist instance to iterate
// @evsel: struct evsel iterator
// @tmp: struct evsel temp iterator
//

extern "C" {
    pub fn list_entry(_arg: evlist->entries.next, perf_evsel: struct, _arg: node) -> return;
}
extern "C" {
    pub fn list_entry(_arg: evlist->entries.prev, perf_evsel: struct, _arg: node) -> return;
}
extern "C" {
    pub fn perf_evlist__read_format(evlist: *mut perf_evlist) -> u64;
}
extern "C" {
    pub fn perf_evlist__reset_id_hash(evlist: *mut perf_evlist);
}
extern "C" {
    pub fn __perf_evlist__set_leader(list: *mut list_head, leader: *mut perf_evsel);
}
extern "C" {
    pub fn perf_evlist__go_system_wide(evlist: *mut perf_evlist, evsel: *mut perf_evsel);
}
