//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_bpf_ma.c
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
// Copyright (C) 2023. Huawei Technologies Co., Ltd

#[repr(C)]
#[derive(Copy, Clone)]
pub struct generic_map_value {
    pub data: *mut c_void,
}

    char _license[] SEC("license") = "GPL";
    const unsigned int data_sizes[] = {16, 32, 64, 96, 128, 192, 256, 512, 1024, 2048, 4096};
    const volatile unsigned int data_btf_ids[ARRAY_SIZE(data_sizes)] = {};
    const unsigned int percpu_data_sizes[] = {8, 16, 32, 64, 96, 128, 192, 256, 512};
    const volatile unsigned int percpu_data_btf_ids[ARRAY_SIZE(data_sizes)] = {};
    let mut err: c_int = 0;
    let mut pid: u32 = 0;

    struct bin_data_##_size { \
    char data[_size - sizeof(void *)]; \
    }; \
// See Commit 5d8d6634ccc, force btf generation for type bin_data_##_size */	\
    struct bin_data_##_size *__bin_data_##_size; \
    struct map_value_##_size { \
    struct bin_data_##_size __kptr * data; \
    }; \
    struct { \
    __uint(type, BPF_MAP_TYPE_ARRAY); \
    __type(key, int); \
    __type(value, struct map_value_##_size); \
    __uint(max_entries, 128); \
    } array_##_size SEC(".maps")

    struct percpu_bin_data_##_size { \
    char data[_size]; \
    }; \
    struct percpu_bin_data_##_size *__percpu_bin_data_##_size; \
    struct map_value_percpu_##_size { \
    struct percpu_bin_data_##_size __percpu_kptr * data; \
    }; \
    struct { \
    __uint(type, BPF_MAP_TYPE_ARRAY); \
    __type(key, int); \
    __type(value, struct map_value_percpu_##_size); \
    __uint(max_entries, 128); \
    } array_percpu_##_size SEC(".maps")
#[no_mangle]
unsafe extern "C" fn batch_alloc(map: *mut bpf_map, batch: c_uint, idx: c_uint) -> __always_inline void {
    static __always_inline void batch_alloc(struct bpf_map *map, unsigned int batch, unsigned int idx)
    {
    struct generic_map_value *value;
    unsigned int i, key;
    void *old, *new;
    for (i = 0; i < batch; i++) {
    key = i;
    value = bpf_map_lookup_elem(map, &key);
    if (!value) {
    err = 1;
    return;
    }
    new = bpf_obj_new_impl(data_btf_ids[idx], core::ptr::null_mut());
    if (!new) {
    err = 2;
    return;
    }
    old = bpf_kptr_xchg(&value.data, new);
    if (old) {
    bpf_obj_drop(old);
    err = 3;
    return;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn batch_free(map: *mut bpf_map, batch: c_uint, idx: c_uint) -> __always_inline void {
    static __always_inline void batch_free(struct bpf_map *map, unsigned int batch, unsigned int idx)
    {
    struct generic_map_value *value;
    unsigned int i, key;
    void *old;
    for (i = 0; i < batch; i++) {
    key = i;
    value = bpf_map_lookup_elem(map, &key);
    if (!value) {
    err = 4;
    return;
    }
    old = bpf_kptr_xchg(&value.data, core::ptr::null_mut());
    if (!old) {
    err = 5;
    return;
    }
    bpf_obj_drop(old);
    }
    }
    static __always_inline void batch_percpu_alloc(struct bpf_map *map, unsigned int batch,
    unsigned int idx)
    {
    struct generic_map_value *value;
    unsigned int i, key;
    void *old, *new;
    for (i = 0; i < batch; i++) {
    key = i;
    value = bpf_map_lookup_elem(map, &key);
    if (!value) {
    err = 1;
    return;
    }
// per-cpu allocator may not be able to refill in time
    new = bpf_percpu_obj_new_impl(percpu_data_btf_ids[idx], core::ptr::null_mut());
    if (!new)
    continue;
    old = bpf_kptr_xchg(&value.data, new);
    if (old) {
    bpf_percpu_obj_drop(old);
    err = 2;
    return;
    }
    }
    }
    static __always_inline void batch_percpu_free(struct bpf_map *map, unsigned int batch,
    unsigned int idx)
    {
    struct generic_map_value *value;
    unsigned int i, key;
    void *old;
    for (i = 0; i < batch; i++) {
    key = i;
    value = bpf_map_lookup_elem(map, &key);
    if (!value) {
    err = 3;
    return;
    }
    old = bpf_kptr_xchg(&value.data, core::ptr::null_mut());
    if (!old)
    continue;
    bpf_percpu_obj_drop(old);
    }
    }

    batch_alloc((struct bpf_map *)(&array_##size), batch, idx)

    do { \
    batch_alloc((struct bpf_map *)(&array_##size), batch, idx); \
    batch_free((struct bpf_map *)(&array_##size), batch, idx); \
    } while (0)

    batch_percpu_alloc((struct bpf_map *)(&array_percpu_##size), batch, idx)

    do { \
    batch_percpu_alloc((struct bpf_map *)(&array_percpu_##size), batch, idx); \
    batch_percpu_free((struct bpf_map *)(&array_percpu_##size), batch, idx); \
    } while (0)
// kptr doesn't support bin_data_8 which is a zero-sized array
    DEFINE_ARRAY_WITH_KPTR(16);
    DEFINE_ARRAY_WITH_KPTR(32);
    DEFINE_ARRAY_WITH_KPTR(64);
    DEFINE_ARRAY_WITH_KPTR(96);
    DEFINE_ARRAY_WITH_KPTR(128);
    DEFINE_ARRAY_WITH_KPTR(192);
    DEFINE_ARRAY_WITH_KPTR(256);
    DEFINE_ARRAY_WITH_KPTR(512);
    DEFINE_ARRAY_WITH_KPTR(1024);
    DEFINE_ARRAY_WITH_KPTR(2048);
    DEFINE_ARRAY_WITH_KPTR(4096);
    DEFINE_ARRAY_WITH_PERCPU_KPTR(8);
    DEFINE_ARRAY_WITH_PERCPU_KPTR(16);
    DEFINE_ARRAY_WITH_PERCPU_KPTR(32);
    DEFINE_ARRAY_WITH_PERCPU_KPTR(64);
    DEFINE_ARRAY_WITH_PERCPU_KPTR(96);
    DEFINE_ARRAY_WITH_PERCPU_KPTR(128);
    DEFINE_ARRAY_WITH_PERCPU_KPTR(192);
    DEFINE_ARRAY_WITH_PERCPU_KPTR(256);
    DEFINE_ARRAY_WITH_PERCPU_KPTR(512);
    SEC("?fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_batch_alloc_free(ctx: *mut c_void) -> c_int {
    int test_batch_alloc_free(void *ctx)
    {
    if ((u32)bpf_get_current_pid_tgid() != pid)
    return 0;
// Alloc 128 16-bytes objects in batch to trigger refilling,
// then free 128 16-bytes objects in batch to trigger freeing.
//
    CALL_BATCH_ALLOC_FREE(16, 128, 0);
    CALL_BATCH_ALLOC_FREE(32, 128, 1);
    CALL_BATCH_ALLOC_FREE(64, 128, 2);
    CALL_BATCH_ALLOC_FREE(96, 128, 3);
    CALL_BATCH_ALLOC_FREE(128, 128, 4);
    CALL_BATCH_ALLOC_FREE(192, 128, 5);
    CALL_BATCH_ALLOC_FREE(256, 128, 6);
    CALL_BATCH_ALLOC_FREE(512, 64, 7);
    CALL_BATCH_ALLOC_FREE(1024, 32, 8);
    CALL_BATCH_ALLOC_FREE(2048, 16, 9);
    CALL_BATCH_ALLOC_FREE(4096, 8, 10);
    return 0;
    }
    SEC("?fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_free_through_map_free(ctx: *mut c_void) -> c_int {
    int test_free_through_map_free(void *ctx)
    {
    if ((u32)bpf_get_current_pid_tgid() != pid)
    return 0;
// Alloc 128 16-bytes objects in batch to trigger refilling,
// then free these objects through map free.
//
    CALL_BATCH_ALLOC(16, 128, 0);
    CALL_BATCH_ALLOC(32, 128, 1);
    CALL_BATCH_ALLOC(64, 128, 2);
    CALL_BATCH_ALLOC(96, 128, 3);
    CALL_BATCH_ALLOC(128, 128, 4);
    CALL_BATCH_ALLOC(192, 128, 5);
    CALL_BATCH_ALLOC(256, 128, 6);
    CALL_BATCH_ALLOC(512, 64, 7);
    CALL_BATCH_ALLOC(1024, 32, 8);
    CALL_BATCH_ALLOC(2048, 16, 9);
    CALL_BATCH_ALLOC(4096, 8, 10);
    return 0;
    }
    SEC("?fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_batch_percpu_alloc_free(ctx: *mut c_void) -> c_int {
    int test_batch_percpu_alloc_free(void *ctx)
    {
    if ((u32)bpf_get_current_pid_tgid() != pid)
    return 0;
// Alloc 128 8-bytes per-cpu objects in batch to trigger refilling,
// then free 128 8-bytes per-cpu objects in batch to trigger freeing.
//
    CALL_BATCH_PERCPU_ALLOC_FREE(8, 128, 0);
    CALL_BATCH_PERCPU_ALLOC_FREE(16, 128, 1);
    CALL_BATCH_PERCPU_ALLOC_FREE(32, 128, 2);
    CALL_BATCH_PERCPU_ALLOC_FREE(64, 128, 3);
    CALL_BATCH_PERCPU_ALLOC_FREE(96, 128, 4);
    CALL_BATCH_PERCPU_ALLOC_FREE(128, 128, 5);
    CALL_BATCH_PERCPU_ALLOC_FREE(192, 128, 6);
    CALL_BATCH_PERCPU_ALLOC_FREE(256, 128, 7);
    CALL_BATCH_PERCPU_ALLOC_FREE(512, 64, 8);
    return 0;
    }
    SEC("?fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_percpu_free_through_map_free(ctx: *mut c_void) -> c_int {
    int test_percpu_free_through_map_free(void *ctx)
    {
    if ((u32)bpf_get_current_pid_tgid() != pid)
    return 0;
// Alloc 128 8-bytes per-cpu objects in batch to trigger refilling,
// then free these object through map free.
//
    CALL_BATCH_PERCPU_ALLOC(8, 128, 0);
    CALL_BATCH_PERCPU_ALLOC(16, 128, 1);
    CALL_BATCH_PERCPU_ALLOC(32, 128, 2);
    CALL_BATCH_PERCPU_ALLOC(64, 128, 3);
    CALL_BATCH_PERCPU_ALLOC(96, 128, 4);
    CALL_BATCH_PERCPU_ALLOC(128, 128, 5);
    CALL_BATCH_PERCPU_ALLOC(192, 128, 6);
    CALL_BATCH_PERCPU_ALLOC(256, 128, 7);
    CALL_BATCH_PERCPU_ALLOC(512, 64, 8);
    return 0;
    }
