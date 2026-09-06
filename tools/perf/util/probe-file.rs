//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/probe-file.h
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

// Cache of probe definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe_cache_entry {
    pub node: list_head,
    pub sdt: bool,
    pub pev: perf_probe_event,
    pub spev: *mut c_char,
    pub tevlist: *mut strlist,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe_cache {
    pub fd: c_int,
    pub entries: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum probe_type {
    PROBE_TYPE_U = 0,
    PROBE_TYPE_S,
    PROBE_TYPE_X,
    PROBE_TYPE_STRING,
    PROBE_TYPE_BITFIELD,
    PROBE_TYPE_END,
}

pub const PF_FL_UPROBE: c_int = 1;
pub const PF_FL_RW: c_int = 2;

// probe-file.c depends on libelf

extern "C" {
    pub fn open_trace_file(trace_file: *const c_char, readwrite: bool) -> c_int;
}
extern "C" {
    pub fn probe_file__open(flag: c_int) -> c_int;
}
extern "C" {
    pub fn probe_file__open_both(kfd: *mut c_int, ufd: *mut c_int, flag: c_int) -> c_int;
}
extern "C" {
    pub fn probe_file__add_event(fd: c_int, tev: *mut probe_trace_event) -> c_int;
}
extern "C" {
    pub fn probe_file__del_strlist(fd: c_int, namelist: *mut strlist) -> c_int;
}
extern "C" {
    pub fn probe_cache__scan_sdt(pcache: *mut probe_cache, pathname: *const c_char) -> c_int;
}
extern "C" {
    pub fn probe_cache__commit(pcache: *mut probe_cache) -> c_int;
}
extern "C" {
    pub fn probe_cache__purge(pcache: *mut probe_cache);
}
extern "C" {
    pub fn probe_cache__delete(pcache: *mut probe_cache);
}
extern "C" {
    pub fn probe_cache__show_all_caches(filter: *mut strfilter) -> c_int;
}
extern "C" {
    pub fn probe_type_is_available(type: probe_type) -> bool;
}
extern "C" {
    pub fn kretprobe_offset_is_supported() -> bool;
}
extern "C" {
    pub fn uprobe_ref_ctr_is_supported() -> bool;
}
extern "C" {
    pub fn user_access_is_supported() -> bool;
}
extern "C" {
    pub fn multiprobe_event_is_supported() -> bool;
}
extern "C" {
    pub fn immediate_value_is_supported() -> bool;
}

