//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/stream.h
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
pub struct stream {
    pub cnode: *mut callchain_node,
    pub pair_cnode: *mut callchain_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evsel_streams {
    pub streams: *mut stream,
    pub evsel: *const evsel,
    pub nr_streams_max: c_int,
    pub nr_streams: c_int,
    pub streams_hits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evlist_streams {
    pub ev_streams: *mut evsel_streams,
    pub nr_evsel: c_int,
}

extern "C" {
    pub fn evlist_streams__delete(els: *mut evlist_streams);
}
