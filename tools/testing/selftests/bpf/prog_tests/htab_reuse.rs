//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/htab_reuse.c
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
// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htab_op_ctx {
    pub fd: c_int,
    pub loop: c_int,
    pub stop: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htab_val {
    pub lock: c_uint,
    pub data: c_uint,
}

    static void *htab_lookup_fn(void *arg)
    {
    struct htab_op_ctx *ctx = arg;
    let mut i: c_int = 0;
    while (i++ < ctx.loop && !ctx.stop) {
    struct htab_val value;
    unsigned int key;
// Use BPF_F_LOCK to use spin-lock in map value.
    key = 7;
    bpf_map_lookup_elem_flags(ctx.fd, &key, &value, BPF_F_LOCK);
    }
    return core::ptr::null_mut();
    }
    static void *htab_update_fn(void *arg)
    {
    struct htab_op_ctx *ctx = arg;
    let mut i: c_int = 0;
    while (i++ < ctx.loop && !ctx.stop) {
    struct htab_val value;
    unsigned int key;
    key = 7;
    value.lock = 0;
    value.data = key;
    bpf_map_update_elem(ctx.fd, &key, &value, BPF_F_LOCK);
    bpf_map_delete_elem(ctx.fd, &key);
    key = 24;
    value.lock = 0;
    value.data = key;
    bpf_map_update_elem(ctx.fd, &key, &value, BPF_F_LOCK);
    bpf_map_delete_elem(ctx.fd, &key);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn test_htab_reuse_basic() {
    static void test_htab_reuse_basic(void)
    {
    unsigned int i, wr_nr = 1, rd_nr = 4;
    pthread_t tids[wr_nr + rd_nr];
    struct htab_reuse *skel;
    struct htab_op_ctx ctx;
    int err;
    skel = htab_reuse__open_and_load();
    if (!ASSERT_OK_PTR(skel, "htab_reuse__open_and_load"))
    return;
    ctx.fd = bpf_map__fd(skel.maps.htab);
    ctx.loop = 500;
    ctx.stop = false;
    memset(tids, 0, sizeof(tids));
    for (i = 0; i < wr_nr; i++) {
    err = pthread_create(&tids[i], core::ptr::null_mut(), htab_update_fn, &ctx);
    if (!ASSERT_OK(err, "pthread_create")) {
    ctx.stop = true;
    goto reap;
    }
    }
    for (i = 0; i < rd_nr; i++) {
    err = pthread_create(&tids[i + wr_nr], core::ptr::null_mut(), htab_lookup_fn, &ctx);
    if (!ASSERT_OK(err, "pthread_create")) {
    ctx.stop = true;
    goto reap;
    }
    }
    reap:
    for (i = 0; i < wr_nr + rd_nr; i++) {
    if (!tids[i])
    continue;
    pthread_join(tids[i], core::ptr::null_mut());
    }
    htab_reuse__destroy(skel);
    }
//
// Writes consistency test for BPF_F_LOCK update
//
// The race:
// 1. Thread A: BPF_F_LOCK|BPF_EXIST update
// 2. Thread B: delete element then update it with BPF_ANY
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htab_val_large {
    pub lock: bpf_spin_lock,
    pub seq: __u32,
    pub data: [__u64; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct consistency_ctx {
    pub fd: c_int,
    pub start_fd: c_int,
    pub loop: c_int,
    pub torn_write: volatile bool,
}

#[no_mangle]
unsafe extern "C" fn wait_for_start(fd: c_int) {
    static void wait_for_start(int fd)
    {
    char buf;
    read(fd, &buf, 1);
    }
    static void *locked_update_fn(void *arg)
    {
    struct consistency_ctx *ctx = arg;
    struct htab_val_large value;
    let mut key: c_uint = 1;
    int i;
    memset(&value, 0xAA, sizeof(value));
    wait_for_start(ctx.start_fd);
    for (i = 0; i < ctx.loop; i++) {
    value.seq = i;
    bpf_map_update_elem(ctx.fd, &key, &value,
    BPF_F_LOCK | BPF_EXIST);
    }
    return core::ptr::null_mut();
    }
// Delete + update: removes the element then re-creates it with BPF_ANY.
    static void *delete_update_fn(void *arg)
    {
    struct consistency_ctx *ctx = arg;
    struct htab_val_large value;
    let mut key: c_uint = 1;
    int i;
    memset(&value, 0xBB, sizeof(value));
    wait_for_start(ctx.start_fd);
    for (i = 0; i < ctx.loop; i++) {
    value.seq = i;
    bpf_map_delete_elem(ctx.fd, &key);
    bpf_map_update_elem(ctx.fd, &key, &value, BPF_ANY | BPF_F_LOCK);
    }
    return core::ptr::null_mut();
    }
    static void *locked_lookup_fn(void *arg)
    {
    struct consistency_ctx *ctx = arg;
    struct htab_val_large value;
    let mut key: c_uint = 1;
    int i, j;
    wait_for_start(ctx.start_fd);
    for (i = 0; i < ctx.loop && !ctx.torn_write; i++) {
    if (bpf_map_lookup_elem_flags(ctx.fd, &key, &value, BPF_F_LOCK))
    continue;
    for (j = 0; j < 256; j++) {
    if (value.data[j] != value.data[0]) {
    ctx.torn_write = true;
    return core::ptr::null_mut();
    }
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn test_htab_reuse_consistency() {
    static void test_htab_reuse_consistency(void)
    {
    let mut threads_total: c_int = 6, threads = 2;
    pthread_t tids[threads_total];
    struct consistency_ctx ctx;
    struct htab_val_large seed;
    struct htab_reuse *skel;
    let mut key: c_uint = 1, i;
    int pipefd[2];
    int err;
    skel = htab_reuse__open_and_load();
    if (!ASSERT_OK_PTR(skel, "htab_reuse__open_and_load"))
    return;
    if (!ASSERT_OK(pipe(pipefd), "pipe"))
    goto out;
    ctx.fd = bpf_map__fd(skel.maps.htab_lock_consistency);
    ctx.start_fd = pipefd[0];
    ctx.loop = 100000;
    ctx.torn_write = false;
// Seed the element so locked updaters have something to find
    memset(&seed, 0xBB, sizeof(seed));
    err = bpf_map_update_elem(ctx.fd, &key, &seed, BPF_ANY);
    if (!ASSERT_OK(err, "seed_element"))
    goto close_pipe;
    memset(tids, 0, sizeof(tids));
    for (i = 0; i < threads; i++) {
    err = pthread_create(&tids[i], core::ptr::null_mut(), locked_update_fn, &ctx);
    if (!ASSERT_OK(err, "pthread_create"))
    goto stop;
    }
    for (i = 0; i < threads; i++) {
    err = pthread_create(&tids[threads + i], core::ptr::null_mut(), delete_update_fn, &ctx);
    if (!ASSERT_OK(err, "pthread_create"))
    goto stop;
    }
    for (i = 0; i < threads; i++) {
    err = pthread_create(&tids[threads * 2 + i], core::ptr::null_mut(), locked_lookup_fn, &ctx);
    if (!ASSERT_OK(err, "pthread_create"))
    goto stop;
    }
// Release all threads simultaneously
    close(pipefd[1]);
    pipefd[1] = -1;
    stop:
    for (i = 0; i < threads_total; i++) {
    if (!tids[i])
    continue;
    pthread_join(tids[i], core::ptr::null_mut());
    }
    ASSERT_FALSE(ctx.torn_write, "no torn writes detected");
    close_pipe:
    if (pipefd[1] >= 0)
    close(pipefd[1]);
    close(pipefd[0]);
    out:
    htab_reuse__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_htab_reuse() {
    void test_htab_reuse(void)
    {
    if (test__start_subtest("basic"))
    test_htab_reuse_basic();
    if (test__start_subtest("consistency"))
    test_htab_reuse_consistency();
    }
