//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/bpf-event.h
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
pub struct bpf_metadata {
    pub event: *mut perf_event,
    pub prog_names: *mut c_char,
    pub nr_prog_names: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_info_node {
    pub info_linear: *mut perf_bpil,
    pub metadata: *mut bpf_metadata,
    pub rb_node: rb_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_node {
    pub rb_node: rb_node,
    pub id: u32,
    pub data_size: u32,
    pub data: [c_char; ],
}

extern "C" {
    pub fn evlist__add_bpf_sb_event(evlist: *mut evlist, env: *mut perf_env) -> c_int;
}
extern "C" {
    pub fn bpf_metadata_free(metadata: *mut bpf_metadata);
}

