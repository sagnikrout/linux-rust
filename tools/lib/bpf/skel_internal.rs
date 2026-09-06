//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/skel_internal.h
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

// This file is a base header for auto-generated *.lskel.h files.
// Its contents will change and may become part of auto-generation in the future.
//
// The layout of bpf_[map|prog]_desc and bpf_loader_ctx is feature dependent
// and will change from one version of libbpf to another and features
// requested during loader program generation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_desc {
// output of the loader prog
    pub map_fd: c_int,
// input for the loader prog
    pub max_entries: __u32,
    pub initial_value: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_desc {
    pub prog_fd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_loader_ctx {
    pub sz: __u32,
    pub flags: __u32,
    pub log_level: __u32,
    pub log_size: __u32,
    pub log_buf: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_load_and_run_opts {
    pub ctx: *mut bpf_loader_ctx,
    pub data: *const c_void,
    pub insns: *const c_void,
    pub data_sz: __u32,
    pub insns_sz: __u32,
    pub errstr: *const c_char,
    pub signature: *mut c_void,
    pub signature_sz: __u32,
    pub keyring_id: __s32,
    pub excl_prog_hash: *mut c_void,
    pub excl_prog_hash_sz: __u32,
}

extern "C" {
    pub fn kern_sys_bpf(cmd: __u32, attr: *mut c_void, attr_size: __u32) -> c_long;
}

extern "C" {
    pub fn kern_sys_bpf(_arg: cmd, _arg: attr, _arg: size) -> return;
}

extern "C" {
    pub fn syscall(_arg: __NR_bpf, _arg: cmd, _arg: attr, _arg: size) -> return;
}

extern "C" {
    pub fn close_fd(_arg: fd) -> return;
}
// skel->bss/rodata maps are populated the following way:
//
// For kernel use:
// skel_prep_map_data() allocates kernel memory that kernel module can directly access.
// Generated lskel stores the pointer in skel->rodata and in skel->maps.rodata.initial_value.
// The loader program will perform probe_read_kernel() from maps.rodata.initial_value.
// skel_finalize_map_data() sets skel->rodata to point to actual value in a bpf map and
// does maps.rodata.initial_value = ~0ULL to signal skel_free_map_data() that kvfree
// is not necessary.
//
// For user space:
// skel_prep_map_data() mmaps anon memory into skel->rodata that can be accessed directly.
// Generated lskel stores the pointer in skel->rodata and in skel->maps.rodata.initial_value.
// The loader program will perform copy_from_user() from maps.rodata.initial_value.
// skel_finalize_map_data() remaps bpf array map value from the kernel memory into
// skel->rodata address.
//
// The "bpftool gen skeleton -L" command generates lskel.h that is suitable for
// both kernel and user space. The generated loader program does
// either bpf_probe_read_kernel() or bpf_copy_from_user() from initial_value
// depending on bpf_loader_ctx->flags.
//
// When addr == ~0ULL the init buffer has already been released.
// For skel_finalize_map_data(), 'p' points to
// ((struct bpf_array *)map)->value.
//
// init_val = ~0ULL;
// At this point bpf_load_and_run() finished without error and
// 'fd' is a valid bpf map FD. All sanity checks below should succeed.
//
// the addr stays valid, since FD is not closed
// init_val = ~0ULL;

extern "C" {
    pub fn calloc(_arg: 1, _arg: size) -> return;
}

extern "C" {
    pub fn close(_arg: fd) -> return;
}

extern "C" {
    pub fn skel_sys_bpf(_arg: BPF_MAP_CREATE, _arg: &attr, _arg: attr_sz) -> return;
}
extern "C" {
    pub fn skel_sys_bpf(_arg: BPF_MAP_UPDATE_ELEM, _arg: &attr, _arg: attr_sz) -> return;
}
extern "C" {
    pub fn skel_sys_bpf(_arg: BPF_MAP_DELETE_ELEM, _arg: &attr, _arg: attr_sz) -> return;
}
extern "C" {
    pub fn skel_sys_bpf(_arg: BPF_MAP_GET_FD_BY_ID, _arg: &attr, _arg: attr_sz) -> return;
}
extern "C" {
    pub fn skel_sys_bpf(_arg: BPF_RAW_TRACEPOINT_OPEN, _arg: &attr, _arg: attr_sz) -> return;
}
extern "C" {
    pub fn skel_sys_bpf(_arg: BPF_LINK_CREATE, _arg: &attr, _arg: attr_sz) -> return;
}
extern "C" {
    pub fn skel_sys_bpf(_arg: BPF_MAP_FREEZE, _arg: &attr, _arg: attr_sz) -> return;
}

// Macro flag: #define set_err

