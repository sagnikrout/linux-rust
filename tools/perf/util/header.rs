//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/header.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_header_version {
    PERF_HEADER_VERSION_1,
    PERF_HEADER_VERSION_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_file_section {
    pub offset: u64,
    pub size: u64,
}

//
// struct perf_file_header: Header representation on disk.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_file_header {
// @magic: Holds "PERFILE2".
    pub magic: u64,
// @size: Size of this header - sizeof(struct perf_file_header).
    pub size: u64,
//
// @attr_size: Size of attrs entries - sizeof(struct perf_event_attr) +
// sizeof(struct perf_file_section).
//
    pub attr_size: u64,
// @attrs: Offset and size of file section holding attributes.
    pub attrs: perf_file_section,
// @data: Offset and size of file section holding regular event data.
    pub data: perf_file_section,
// @event_types: Ignored.
    pub event_types: perf_file_section,
//
// @adds_features: Bitmap of features. The features are immediately after the data section.
//
    pub HEADER_FEAT_BITS): DECLARE_BITMAP(adds_features,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_pipe_file_header {
    pub magic: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_header {
    pub version: perf_header_version,
    pub needs_swap: bool,
    pub data_offset: u64,
    pub data_size: u64,
    pub feat_offset: u64,
    pub HEADER_FEAT_BITS): DECLARE_BITMAP(adds_features,,
    pub last_feat: c_int,
    pub env: perf_env,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct feat_fd {
    pub ph: *mut perf_header,
    pub fd: c_int,
    pub /: *mut *mut *mut void buf; / Either buf != NULL or fd >= 0,
    pub offset: isize,
    pub size: usize,
    pub events: *mut evsel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_header_feature_ops {
    pub evlist): *mut *mut *mut int (write)(struct feat_fd ff, struct evlist,
    pub fp): *mut *mut *mut void (print)(struct feat_fd ff, FILE,
    pub data): *mut *mut *mut int (process)(struct feat_fd ff, void,
    pub name: *const c_char,
    pub full_only: bool,
    pub synthesize: bool,
}

extern "C" {
    pub fn perf_session__read_header(session: *mut perf_session) -> c_int;
}
extern "C" {
    pub fn perf_header__write_pipe(fd: c_int) -> c_int;
}
// feat_writer writes a feature section to output
#[repr(C)]
#[derive(Copy, Clone)]
pub struct feat_writer {
    pub sz): *mut *mut *mut *mut int (write)(struct feat_writer fw, void buf, size_t,
}

// feat_copier copies a feature section using feat_writer to output
#[repr(C)]
#[derive(Copy, Clone)]
pub struct feat_copier {
    pub fw): *mut *mut *mut int (copy)(struct feat_copier fc, int feat, struct feat_writer,
}

extern "C" {
    pub fn perf_session__data_offset(evlist: *mut evlist) -> usize;
}
extern "C" {
    pub fn perf_header__set_feat(header: *mut perf_header, feat: c_int);
}
extern "C" {
    pub fn perf_header__clear_feat(header: *mut perf_header, feat: c_int);
}
extern "C" {
    pub fn perf_header__has_feat(header: *const perf_header, feat: c_int) -> bool;
}
extern "C" {
    pub fn perf_header__set_cmdline(argc: c_int, argv: *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_header__fprintf_info(s: *mut perf_session, fp: *mut FILE, full: bool) -> c_int;
}
extern "C" {
    pub fn perf_event__fprintf_attr(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_event_update(event: *mut perf_event, fp: *mut FILE) -> usize;
}

extern "C" {
    pub fn is_perf_magic(magic: u64) -> bool;
}
pub const NAME_ALIGN: c_int = 64;
extern "C" {
    pub fn do_write(fd: *mut feat_fd, buf: *const c_void, size: usize) -> c_int;
}
pub const MAX_CACHE_LVL: c_int = 4;
extern "C" {
    pub fn build_caches_for_cpu(cpu: u32, caches[]: cpu_cache_level, cntp: *mut u32) -> c_int;
}
pub const DEFAULT_CACHELINE_SIZE: c_int = 64;
//
// arch specific callback
//
extern "C" {
    pub fn get_cpuid(buffer: *mut c_char, sz: usize, cpu: perf_cpu) -> c_int;
}
extern "C" {
    pub fn strcmp_cpuid_str(s1: *const c_char, s2: *const c_char) -> c_int;
}
