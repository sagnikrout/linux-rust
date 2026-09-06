//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/kmem_cache_iter.c
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
// Copyright (c) 2024 Google

    char _license[] SEC("license") = "GPL";
pub const SLAB_NAME_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmem_cache_result {
    pub name: [c_char; SLAB_NAME_MAX],
    pub obj_size: c_long,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(void *));
    __uint(value_size, SLAB_NAME_MAX);
    __uint(max_entries, 1);
    } slab_hash SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(key_size, sizeof(int));
    __uint(value_size, sizeof(struct kmem_cache_result));
    __uint(max_entries, 1024);
    } slab_result SEC(".maps");
    extern struct kmem_cache *bpf_get_kmem_cache(u64 addr) __ksym;
// Result, will be checked by userspace
    int task_struct_found;
    int kmem_cache_seen;
    int open_coded_seen;
    SEC("iter/kmem_cache")
#[no_mangle]
pub unsafe extern "C" fn slab_info_collector(ctx: *mut bpf_iter__kmem_cache) -> c_int {
    int slab_info_collector(struct bpf_iter__kmem_cache *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    struct kmem_cache *s = ctx.s;
    struct kmem_cache_result *r;
    int idx;
    if (s) {
// To make sure if the slab_iter implements the seq interface
// properly and it's also useful for debugging.
//
    BPF_SEQ_PRINTF(seq, "%s: %u\n", s.name, s.size);
    idx = kmem_cache_seen;
    r = bpf_map_lookup_elem(&slab_result, &idx);
    if (r == core::ptr::null_mut())
    return 0;
    kmem_cache_seen++;
// Save name and size to match /proc/slabinfo
    bpf_probe_read_kernel_str(r.name, sizeof(r.name), s.name);
    r.obj_size = s.size;
    if (!bpf_strncmp(r.name, 11, "task_struct"))
    bpf_map_update_elem(&slab_hash, &s, r.name, BPF_NOEXIST);
    }
    return 0;
    }
    SEC("raw_tp/bpf_test_finish")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: check_task_struct) -> c_int {
    int BPF_PROG(check_task_struct)
    {
    let mut curr: u64 = bpf_get_current_task();
    struct kmem_cache *s;
    char *name;
    s = bpf_get_kmem_cache(curr);
    if (s == core::ptr::null_mut()) {
    task_struct_found = -1;
    return 0;
    }
    name = bpf_map_lookup_elem(&slab_hash, &s);
    if (name && !bpf_strncmp(name, 11, "task_struct"))
    task_struct_found = 1;
    else
    task_struct_found = -2;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn open_coded_iter(ctx: *const c_void) -> c_int {
    int open_coded_iter(const void *ctx)
    {
    struct kmem_cache *s;
    bpf_for_each(kmem_cache, s) {
    struct kmem_cache_result *r;
    r = bpf_map_lookup_elem(&slab_result, &open_coded_seen);
    if (!r)
    break;
    if (r.obj_size != s.size)
    break;
    open_coded_seen++;
    }
    return 0;
    }
