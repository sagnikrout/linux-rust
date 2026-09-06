//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/perf/include/perf/bpf_perf.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

//
// bpf_perf uses a hashmap, the attr_map, to track all the leader programs.
// The hashmap is pinned in bpffs. flock() on this file is used to ensure
// no concurrent access to the attr_map.  The key of attr_map is struct
// perf_event_attr, and the value is struct perf_event_attr_map_entry.
//
// struct perf_event_attr_map_entry contains two __u32 IDs, bpf_link of the
// leader prog, and the diff_map. Each perf-stat session holds a reference
// to the bpf_link to make sure the leader prog is attached to sched_switch
// tracepoint.
//
// Since the hashmap only contains IDs of the bpf_link and diff_map, it
// does not hold any references to the leader program. Once all perf-stat
// sessions of these events exit, the leader prog, its maps, and the
// perf_events will be freed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_attr_map_entry {
    pub link_id: __u32,
    pub diff_map_id: __u32,
}

// default attr_map name

