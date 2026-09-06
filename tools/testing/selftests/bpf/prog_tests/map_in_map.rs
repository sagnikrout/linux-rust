//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/map_in_map.c
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
pub struct thread_ctx {
    pub barrier: pthread_barrier_t,
    pub outer_map_fd: c_int,
    pub abort: int start,,
    pub err: int loop,,
}

#[no_mangle]
unsafe extern "C" fn wait_for_start_or_abort(ctx: *mut thread_ctx) -> c_int {
    static int wait_for_start_or_abort(struct thread_ctx *ctx)
    {
    while (!ctx.start && !ctx.abort)
    usleep(1);
    return ctx.abort ? -1 : 0;
    }
    static void *update_map_fn(void *data)
    {
    struct thread_ctx *ctx = data;
    let mut loop: c_int = ctx.loop, err = 0;
    if (wait_for_start_or_abort(ctx) < 0)
    return core::ptr::null_mut();
    pthread_barrier_wait(&ctx.barrier);
    while (loop-- > 0) {
    int fd, zero = 0;
    fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, core::ptr::null_mut(), 4, 4, 1, core::ptr::null_mut());
    if (fd < 0) {
    err |= 1;
    pthread_barrier_wait(&ctx.barrier);
    continue;
    }
// Remove the old inner map
    if (bpf_map_update_elem(ctx.outer_map_fd, &zero, &fd, 0) < 0)
    err |= 2;
    close(fd);
    pthread_barrier_wait(&ctx.barrier);
    }
    ctx.err = err;
    return core::ptr::null_mut();
    }
    static void *access_map_fn(void *data)
    {
    struct thread_ctx *ctx = data;
    let mut loop: c_int = ctx.loop;
    if (wait_for_start_or_abort(ctx) < 0)
    return core::ptr::null_mut();
    pthread_barrier_wait(&ctx.barrier);
    while (loop-- > 0) {
// Access the old inner map
    syscall(SYS_getpgid);
    pthread_barrier_wait(&ctx.barrier);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn test_map_in_map_access(prog_name: *const c_char, map_name: *const c_char) {
    static void test_map_in_map_access(const char *prog_name, const char *map_name)
    {
    struct access_map_in_map *skel;
    struct bpf_map *outer_map;
    struct bpf_program *prog;
    struct thread_ctx ctx;
    pthread_t tid[2];
    int err;
    skel = access_map_in_map__open();
    if (!ASSERT_OK_PTR(skel, "access_map_in_map open"))
    return;
    prog = bpf_object__find_program_by_name(skel.obj, prog_name);
    if (!ASSERT_OK_PTR(prog, "find program"))
    goto out;
    bpf_program__set_autoload(prog, true);
    outer_map = bpf_object__find_map_by_name(skel.obj, map_name);
    if (!ASSERT_OK_PTR(outer_map, "find map"))
    goto out;
    err = access_map_in_map__load(skel);
    if (!ASSERT_OK(err, "access_map_in_map load"))
    goto out;
    err = access_map_in_map__attach(skel);
    if (!ASSERT_OK(err, "access_map_in_map attach"))
    goto out;
    skel.bss.tgid = getpid();
    memset(&ctx, 0, sizeof(ctx));
    pthread_barrier_init(&ctx.barrier, core::ptr::null_mut(), 2);
    ctx.outer_map_fd = bpf_map__fd(outer_map);
    ctx.loop = 4;
    err = pthread_create(&tid[0], core::ptr::null_mut(), update_map_fn, &ctx);
    if (!ASSERT_OK(err, "close_thread"))
    goto out;
    err = pthread_create(&tid[1], core::ptr::null_mut(), access_map_fn, &ctx);
    if (!ASSERT_OK(err, "read_thread")) {
    ctx.abort = 1;
    pthread_join(tid[0], core::ptr::null_mut());
    goto out;
    }
    ctx.start = 1;
    pthread_join(tid[0], core::ptr::null_mut());
    pthread_join(tid[1], core::ptr::null_mut());
    ASSERT_OK(ctx.err, "err");
    out:
    access_map_in_map__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn add_del_fd_htab(outer_fd: c_int) {
    static void add_del_fd_htab(int outer_fd)
    {
    int inner_fd, err;
    let mut key: c_int = 1;
    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, "arr1", 4, 4, 1, core::ptr::null_mut());
    if (!ASSERT_OK_FD(inner_fd, "inner1"))
    return;
    err = bpf_map_update_elem(outer_fd, &key, &inner_fd, BPF_NOEXIST);
    close(inner_fd);
    if (!ASSERT_OK(err, "add"))
    return;
// Delete
    err = bpf_map_delete_elem(outer_fd, &key);
    ASSERT_OK(err, "del");
    }
#[no_mangle]
unsafe extern "C" fn overwrite_fd_htab(outer_fd: c_int) {
    static void overwrite_fd_htab(int outer_fd)
    {
    int inner_fd, err;
    let mut key: c_int = 1;
    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, "arr1", 4, 4, 1, core::ptr::null_mut());
    if (!ASSERT_OK_FD(inner_fd, "inner1"))
    return;
    err = bpf_map_update_elem(outer_fd, &key, &inner_fd, BPF_NOEXIST);
    close(inner_fd);
    if (!ASSERT_OK(err, "add"))
    return;
// Overwrite
    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, "arr2", 4, 4, 1, core::ptr::null_mut());
    if (!ASSERT_OK_FD(inner_fd, "inner2"))
    goto out;
    err = bpf_map_update_elem(outer_fd, &key, &inner_fd, BPF_EXIST);
    close(inner_fd);
    if (!ASSERT_OK(err, "overwrite"))
    goto out;
    err = bpf_map_delete_elem(outer_fd, &key);
    ASSERT_OK(err, "del");
    return;
    out:
    bpf_map_delete_elem(outer_fd, &key);
    }
#[no_mangle]
unsafe extern "C" fn lookup_delete_fd_htab(outer_fd: c_int) {
    static void lookup_delete_fd_htab(int outer_fd)
    {
    let mut key: c_int = 1, value;
    int inner_fd, err;
    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, "arr1", 4, 4, 1, core::ptr::null_mut());
    if (!ASSERT_OK_FD(inner_fd, "inner1"))
    return;
    err = bpf_map_update_elem(outer_fd, &key, &inner_fd, BPF_NOEXIST);
    close(inner_fd);
    if (!ASSERT_OK(err, "add"))
    return;
// lookup_and_delete is not supported for htab of maps
    err = bpf_map_lookup_and_delete_elem(outer_fd, &key, &value);
    ASSERT_EQ(err, -ENOTSUPP, "lookup_del");
    err = bpf_map_delete_elem(outer_fd, &key);
    ASSERT_OK(err, "del");
    }
#[no_mangle]
unsafe extern "C" fn batched_lookup_delete_fd_htab(outer_fd: c_int) {
    static void batched_lookup_delete_fd_htab(int outer_fd)
    {
    int keys[2] = {1, 2}, values[2];
    unsigned int cnt, batch;
    int inner_fd, err;
    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, "arr1", 4, 4, 1, core::ptr::null_mut());
    if (!ASSERT_OK_FD(inner_fd, "inner1"))
    return;
    err = bpf_map_update_elem(outer_fd, &keys[0], &inner_fd, BPF_NOEXIST);
    close(inner_fd);
    if (!ASSERT_OK(err, "add1"))
    return;
    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, "arr2", 4, 4, 1, core::ptr::null_mut());
    if (!ASSERT_OK_FD(inner_fd, "inner2"))
    goto out;
    err = bpf_map_update_elem(outer_fd, &keys[1], &inner_fd, BPF_NOEXIST);
    close(inner_fd);
    if (!ASSERT_OK(err, "add2"))
    goto out;
// batched lookup_and_delete
    cnt = ARRAY_SIZE(keys);
    err = bpf_map_lookup_and_delete_batch(outer_fd, core::ptr::null_mut(), &batch, keys, values, &cnt, core::ptr::null_mut());
    ASSERT_TRUE((!err || err == -ENOENT), "delete_batch ret");
    ASSERT_EQ(cnt, ARRAY_SIZE(keys), "delete_batch cnt");
    out:
    bpf_map_delete_elem(outer_fd, &keys[0]);
    }
#[no_mangle]
unsafe extern "C" fn test_update_map_in_htab(preallocate: bool) {
    static void test_update_map_in_htab(bool preallocate)
    {
    struct update_map_in_htab *skel;
    int err, fd;
    skel = update_map_in_htab__open();
    if (!ASSERT_OK_PTR(skel, "open"))
    return;
    err = update_map_in_htab__load(skel);
    if (!ASSERT_OK(err, "load"))
    goto out;
    fd = preallocate ? bpf_map__fd(skel.maps.outer_htab_map) :
    bpf_map__fd(skel.maps.outer_alloc_htab_map);
    add_del_fd_htab(fd);
    overwrite_fd_htab(fd);
    lookup_delete_fd_htab(fd);
    batched_lookup_delete_fd_htab(fd);
    out:
    update_map_in_htab__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_map_in_map() {
    void test_map_in_map(void)
    {
    if (test__start_subtest("acc_map_in_array"))
    test_map_in_map_access("access_map_in_array", "outer_array_map");
    if (test__start_subtest("sleepable_acc_map_in_array"))
    test_map_in_map_access("sleepable_access_map_in_array", "outer_array_map");
    if (test__start_subtest("acc_map_in_htab"))
    test_map_in_map_access("access_map_in_htab", "outer_htab_map");
    if (test__start_subtest("sleepable_acc_map_in_htab"))
    test_map_in_map_access("sleepable_access_map_in_htab", "outer_htab_map");
    if (test__start_subtest("update_map_in_htab"))
    test_update_map_in_htab(true);
    if (test__start_subtest("update_map_in_alloc_htab"))
    test_update_map_in_htab(false);
    }
