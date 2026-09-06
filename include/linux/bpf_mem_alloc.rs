//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bpf_mem_alloc.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_mem_alloc {
    pub caches: *mut bpf_mem_caches __percpu,
    pub cache: *mut bpf_mem_cache __percpu,
    pub objcg: *mut obj_cgroup,
    pub percpu: bool,
    pub work: work_struct,
    pub ctx): *mut *mut void (dtor_ctx_free)(void,
    pub dtor_ctx: *mut c_void,
}

// 'size != 0' is for bpf_mem_alloc which manages fixed-size objects.
// Alloc and free are done with bpf_mem_cache_{alloc,free}().
//
// 'size = 0' is for bpf_mem_alloc which manages many fixed-size objects.
// Alloc and free are done with bpf_mem_{alloc,free}() and the size of
// the returned object is given by the size argument of bpf_mem_alloc().
// If percpu equals true, error will be returned in order to avoid
// large memory consumption and the below bpf_mem_alloc_percpu_unit_init()
// should be used to do on-demand per-cpu allocation for each size.
//
extern "C" {
    pub fn bpf_mem_alloc_init(ma: *mut bpf_mem_alloc, size: c_int, percpu: bool) -> c_int;
}
// Initialize a non-fix-size percpu memory allocator
extern "C" {
    pub fn bpf_mem_alloc_percpu_init(ma: *mut bpf_mem_alloc, objcg: *mut obj_cgroup) -> c_int;
}
// The percpu allocation with a specific unit size.
extern "C" {
    pub fn bpf_mem_alloc_percpu_unit_init(ma: *mut bpf_mem_alloc, size: c_int) -> c_int;
}
extern "C" {
    pub fn bpf_mem_alloc_destroy(ma: *mut bpf_mem_alloc);
}
// Check the allocation size for kmalloc equivalent allocator
extern "C" {
    pub fn bpf_mem_alloc_check_size(percpu: bool, size: usize) -> c_int;
}
// kmalloc/kfree equivalent:
extern "C" {
    pub fn bpf_mem_free(ma: *mut bpf_mem_alloc, ptr: *mut c_void);
}
extern "C" {
    pub fn bpf_mem_free_rcu(ma: *mut bpf_mem_alloc, ptr: *mut c_void);
}
// kmem_cache_alloc/free equivalent:
extern "C" {
    pub fn bpf_mem_cache_free(ma: *mut bpf_mem_alloc, ptr: *mut c_void);
}
extern "C" {
    pub fn bpf_mem_cache_free_rcu(ma: *mut bpf_mem_alloc, ptr: *mut c_void);
}
extern "C" {
    pub fn bpf_mem_cache_raw_free(ptr: *mut c_void);
}
