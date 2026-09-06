//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/perf/include/internal/mmap.h
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

// perf sample has 16 bits size limit

extern "C" {
    pub fn void(map: *mut *mut libperf_unmap_cb_t)(struct perf_mmap) -> typedef;
}
//
// struct perf_mmap - perf's ring buffer mmap details
//
// @refcnt - e.g. code using PERF_EVENT_IOC_SET_OUTPUT to share this
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_mmap {
    pub base: *mut c_void,
    pub mask: c_int,
    pub fd: c_int,
    pub cpu: perf_cpu,
    pub refcnt: refcount_t,
    pub prev: u64,
    pub start: u64,
    pub end: u64,
    pub overwrite: bool,
    pub flush: u64,
    pub unmap_cb: libperf_unmap_cb_t,
    pub event_copy: *mut c_void,
    pub event_copy_sz: usize,
    pub next: *mut perf_mmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_mmap_param {
    pub prot: c_int,
    pub mask: c_int,
}

extern "C" {
    pub fn perf_mmap__mmap_len(map: *mut perf_mmap) -> usize;
}
extern "C" {
    pub fn perf_mmap__munmap(map: *mut perf_mmap);
}
extern "C" {
    pub fn perf_mmap__get(map: *mut perf_mmap);
}
extern "C" {
    pub fn perf_mmap__put(map: *mut perf_mmap);
}
extern "C" {
    pub fn perf_mmap__read_head(map: *mut perf_mmap) -> u64;
}
extern "C" {
    pub fn perf_mmap__read_self(map: *mut perf_mmap, count: *mut perf_counts_values) -> c_int;
}
