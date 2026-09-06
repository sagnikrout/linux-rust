//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/fd_htab_lookup.c
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
// Copyright (C) 2025. Huawei Technologies Co., Ltd
// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htab_op_ctx {
    pub fd: c_int,
    pub loop: c_int,
    pub entries: c_uint,
    pub stop: bool,
}

    static void *htab_lookup_fn(void *arg)
    {
    struct htab_op_ctx *ctx = arg;
    let mut i: c_int = 0;
    while (i++ < ctx.loop && !ctx.stop) {
    unsigned int j;
    for (j = 0; j < ctx.entries; j++) {
    let mut key: c_uint = j, zero = 0, value;
    int inner_fd, err;
    err = bpf_map_lookup_elem(ctx.fd, &key, &value);
    if (err) {
    ctx.stop = true;
    return ERR_TO_RETVAL(1, err);
    }
    inner_fd = bpf_map_get_fd_by_id(value);
    if (inner_fd < 0) {
// The old map has been freed
    if (inner_fd == -ENOENT)
    continue;
    ctx.stop = true;
    return ERR_TO_RETVAL(2, inner_fd);
    }
    err = bpf_map_lookup_elem(inner_fd, &zero, &value);
    if (err) {
    close(inner_fd);
    ctx.stop = true;
    return ERR_TO_RETVAL(3, err);
    }
    close(inner_fd);
    if (value != key) {
    ctx.stop = true;
    return ERR_TO_RETVAL(4, -EINVAL);
    }
    }
    }
    return core::ptr::null_mut();
    }
    static void *htab_update_fn(void *arg)
    {
    struct htab_op_ctx *ctx = arg;
    let mut i: c_int = 0;
    while (i++ < ctx.loop && !ctx.stop) {
    unsigned int j;
    for (j = 0; j < ctx.entries; j++) {
    let mut key: c_uint = j, zero = 0;
    int inner_fd, err;
    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, core::ptr::null_mut(), 4, 4, 1, core::ptr::null_mut());
    if (inner_fd < 0) {
    ctx.stop = true;
    return ERR_TO_RETVAL(1, inner_fd);
    }
    err = bpf_map_update_elem(inner_fd, &zero, &key, 0);
    if (err) {
    close(inner_fd);
    ctx.stop = true;
    return ERR_TO_RETVAL(2, err);
    }
    err = bpf_map_update_elem(ctx.fd, &key, &inner_fd, BPF_EXIST);
    if (err) {
    close(inner_fd);
    ctx.stop = true;
    return ERR_TO_RETVAL(3, err);
    }
    close(inner_fd);
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn setup_htab(fd: c_int, entries: c_uint) -> c_int {
    static int setup_htab(int fd, unsigned int entries)
    {
    unsigned int i;
    for (i = 0; i < entries; i++) {
    let mut key: c_uint = i, zero = 0;
    int inner_fd, err;
    inner_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, core::ptr::null_mut(), 4, 4, 1, core::ptr::null_mut());
    if (!ASSERT_OK_FD(inner_fd, "new array"))
    return -1;
    err = bpf_map_update_elem(inner_fd, &zero, &key, 0);
    if (!ASSERT_OK(err, "init array")) {
    close(inner_fd);
    return -1;
    }
    err = bpf_map_update_elem(fd, &key, &inner_fd, 0);
    if (!ASSERT_OK(err, "init outer")) {
    close(inner_fd);
    return -1;
    }
    close(inner_fd);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_int_from_env(name: *const c_char, dft: c_int) -> c_int {
    static int get_int_from_env(const char *name, int dft)
    {
    const char *value;
    value = getenv(name);
    if (!value)
    return dft;
    return atoi(value);
    }
#[no_mangle]
pub unsafe extern "C" fn test_fd_htab_lookup() {
    void test_fd_htab_lookup(void)
    {
    unsigned int i, wr_nr = 8, rd_nr = 16;
    pthread_t tids[wr_nr + rd_nr];
    struct fd_htab_lookup *skel;
    struct htab_op_ctx ctx;
    int err;
    skel = fd_htab_lookup__open_and_load();
    if (!ASSERT_OK_PTR(skel, "fd_htab_lookup__open_and_load"))
    return;
    ctx.fd = bpf_map__fd(skel.maps.outer_map);
    ctx.loop = get_int_from_env("FD_HTAB_LOOP_NR", 5);
    ctx.stop = false;
    ctx.entries = 8;
    err = setup_htab(ctx.fd, ctx.entries);
    if (err)
    goto destroy;
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
    void *ret = core::ptr::null_mut();
    char desc[32];
    if (!tids[i])
    continue;
    snprintf(desc, sizeof(desc), "thread %u", i + 1);
    err = pthread_join(tids[i], &ret);
    ASSERT_OK(err, desc);
    ASSERT_EQ(ret, core::ptr::null_mut(), desc);
    }
    destroy:
    fd_htab_lookup__destroy(skel);
    }
