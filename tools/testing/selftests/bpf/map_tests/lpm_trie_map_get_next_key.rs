//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/map_tests/lpm_trie_map_get_next_key.c
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
// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_lpm_key {
    pub prefix: __u32,
    pub data: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_next_key_ctx {
    pub key: test_lpm_key,
    pub start: bool,
    pub stop: bool,
    pub map_fd: c_int,
    pub loop: c_int,
}

    static void *get_next_key_fn(void *arg)
    {
    struct get_next_key_ctx *ctx = arg;
    struct test_lpm_key next_key;
    let mut i: c_int = 0;
    while (!ctx.start)
    usleep(1);
    while (!ctx.stop && i++ < ctx.loop)
    bpf_map_get_next_key(ctx.map_fd, &ctx.key, &next_key);
    return core::ptr::null_mut();
    }
    static void abort_get_next_key(struct get_next_key_ctx *ctx, pthread_t *tids,
    unsigned int nr)
    {
    unsigned int i;
    ctx.stop = true;
    ctx.start = true;
    for (i = 0; i < nr; i++)
    pthread_join(tids[i], core::ptr::null_mut());
    }
// This test aims to prevent regression of future. As long as the kernel does
// not panic, it is considered as success.
//
#[no_mangle]
pub unsafe extern "C" fn test_lpm_trie_map_get_next_key() {
    void test_lpm_trie_map_get_next_key(void)
    {
pub const MAX_NR_THREADS: c_int = 8;
    LIBBPF_OPTS(bpf_map_create_opts, create_opts,
    .map_flags = BPF_F_NO_PREALLOC);
    let mut key: test_lpm_key = {};
    let mut val: __u32 = 0;
    int map_fd;
    let mut max_prefixlen: __u32 = 8 * (sizeof(key) - sizeof(key.prefix));
    let mut max_entries: __u32 = max_prefixlen + 1;
    unsigned int i, nr = MAX_NR_THREADS, loop = 65536;
    pthread_t tids[MAX_NR_THREADS];
    struct get_next_key_ctx ctx;
    int err;
    map_fd = bpf_map_create(BPF_MAP_TYPE_LPM_TRIE, "lpm_trie_map",
    sizeof(struct test_lpm_key), sizeof(__u32),
    max_entries, &create_opts);
    CHECK(map_fd == -1, "bpf_map_create()", "error:%s\n",
    strerror(errno));
    for (i = 0; i <= max_prefixlen; i++) {
    key.prefix = i;
    err = bpf_map_update_elem(map_fd, &key, &val, BPF_ANY);
    CHECK(err, "bpf_map_update_elem()", "error:%s\n",
    strerror(errno));
    }
    ctx.start = false;
    ctx.stop = false;
    ctx.map_fd = map_fd;
    ctx.loop = loop;
    memcpy(&ctx.key, &key, sizeof(key));
    for (i = 0; i < nr; i++) {
    err = pthread_create(&tids[i], core::ptr::null_mut(), get_next_key_fn, &ctx);
    if (err) {
    abort_get_next_key(&ctx, tids, i);
    CHECK(err, "pthread_create", "error %d\n", err);
    }
    }
    ctx.start = true;
    for (i = 0; i < nr; i++)
    pthread_join(tids[i], core::ptr::null_mut());
    printf("%s:PASS\n", __func__);
    close(map_fd);
    }
