//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/bpf_gen_internal.h
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
// Copyright (c) 2021 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksym_relo_desc {
    pub name: *const c_char,
    pub kind: c_int,
    pub insn_idx: c_int,
    pub is_weak: bool,
    pub is_typeless: bool,
    pub is_ld64: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksym_desc {
    pub name: *const c_char,
    pub ref: c_int,
    pub kind: c_int,
// used for kfunc
    pub off: c_int,
// used for typeless ksym
    pub typeless: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_gen {
    pub opts: *mut gen_loader_opts,
    pub data_start: *mut c_void,
    pub data_cur: *mut c_void,
    pub insn_start: *mut c_void,
    pub insn_cur: *mut c_void,
    pub swapped_endian: bool,
    pub cleanup_label: isize,
    pub nr_progs: __u32,
    pub nr_maps: __u32,
    pub log_level: c_int,
    pub error: c_int,
    pub relos: *mut ksym_relo_desc,
    pub relo_cnt: c_int,
    pub core_relos: *mut bpf_core_relo,
    pub core_relo_cnt: c_int,
    pub attach_target: [c_char; 128],
    pub attach_kind: c_int,
    pub ksyms: *mut ksym_desc,
    pub nr_ksyms: __u32,
    pub fd_array: c_int,
    pub nr_fd_array: c_int,
}

extern "C" {
    pub fn bpf_gen__init(gen: *mut bpf_gen, log_level: c_int, nr_progs: c_int, nr_maps: c_int);
}
extern "C" {
    pub fn bpf_gen__finish(gen: *mut bpf_gen, nr_progs: c_int, nr_maps: c_int) -> c_int;
}
extern "C" {
    pub fn bpf_gen__free(gen: *mut bpf_gen);
}
extern "C" {
    pub fn bpf_gen__load_btf(gen: *mut bpf_gen, raw_data: *const c_void, raw_size: __u32);
}
extern "C" {
    pub fn bpf_gen__map_freeze(gen: *mut bpf_gen, map_idx: c_int);
}
extern "C" {
    pub fn bpf_gen__record_attach_target(gen: *mut bpf_gen, name: *const c_char, type: bpf_attach_type);
}
extern "C" {
    pub fn bpf_gen__record_relo_core(gen: *mut bpf_gen, core_relo: *const bpf_core_relo);
}
extern "C" {
    pub fn bpf_gen__populate_outer_map(gen: *mut bpf_gen, outer_map_idx: c_int, key: c_int, inner_map_idx: c_int);
}
