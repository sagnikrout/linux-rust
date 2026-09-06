//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/map_tests/lpm_trie_map_batch_ops.c
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
pub struct test_lpm_key {
    pub prefix: __u32,
    pub ipv4: in_addr,
}

    static void map_batch_update(int map_fd, __u32 max_entries,
    struct test_lpm_key *keys, int *values)
    {
    __u32 i;
    int err;
    char buff[16] = { 0 };
    DECLARE_LIBBPF_OPTS(bpf_map_batch_opts, opts,
    .elem_flags = 0,
    .flags = 0,
    );
    for (i = 0; i < max_entries; i++) {
    keys[i].prefix = 32;
    snprintf(buff, 16, "192.168.1.%d", i + 1);
    inet_pton(AF_INET, buff, &keys[i].ipv4);
    values[i] = i + 1;
    }
    err = bpf_map_update_batch(map_fd, keys, values, &max_entries, &opts);
    CHECK(err, "bpf_map_update_batch()", "error:%s\n", strerror(errno));
    }
    static void map_batch_verify(int *visited, __u32 max_entries,
    struct test_lpm_key *keys, int *values)
    {
    char buff[16] = { 0 };
    let mut lower_byte: c_int = 0;
    __u32 i;
    memset(visited, 0, max_entries * sizeof(*visited));
    for (i = 0; i < max_entries; i++) {
    inet_ntop(AF_INET, &keys[i].ipv4, buff, 32);
    CHECK(sscanf(buff, "192.168.1.%d", &lower_byte) == EOF,
    "sscanf()", "error: i %d\n", i);
    CHECK(lower_byte != values[i], "key/value checking",
    "error: i %d key %s value %d\n", i, buff, values[i]);
    visited[i] = 1;
    }
    for (i = 0; i < max_entries; i++) {
    CHECK(visited[i] != 1, "visited checking",
    "error: keys array at index %d missing\n", i);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn test_lpm_trie_map_batch_ops() {
    void test_lpm_trie_map_batch_ops(void)
    {
    LIBBPF_OPTS(bpf_map_create_opts, create_opts, .map_flags = BPF_F_NO_PREALLOC);
    struct test_lpm_key *keys, key;
    int map_fd, *values, *visited;
    __u32 step, count, total, total_success;
    let mut max_entries: __u32 = 10;
    let mut batch: __u64 = 0;
    int err;
    DECLARE_LIBBPF_OPTS(bpf_map_batch_opts, opts,
    .elem_flags = 0,
    .flags = 0,
    );
    map_fd = bpf_map_create(BPF_MAP_TYPE_LPM_TRIE, "lpm_trie_map",
    sizeof(struct test_lpm_key), sizeof(int),
    max_entries, &create_opts);
    CHECK(map_fd == -1, "bpf_map_create()", "error:%s\n",
    strerror(errno));
    keys = malloc(max_entries * sizeof(struct test_lpm_key));
    values = malloc(max_entries * sizeof(int));
    visited = malloc(max_entries * sizeof(int));
    CHECK(!keys || !values || !visited, "malloc()", "error:%s\n",
    strerror(errno));
    total_success = 0;
    for (step = 1; step < max_entries; step++) {
    map_batch_update(map_fd, max_entries, keys, values);
    map_batch_verify(visited, max_entries, keys, values);
    memset(keys, 0, max_entries * sizeof(*keys));
    memset(values, 0, max_entries * sizeof(*values));
    batch = 0;
    total = 0;
// iteratively lookup/delete elements with 'step'
// elements each.
//
    count = step;
    while (true) {
    err = bpf_map_lookup_batch(map_fd,
    total ? &batch : core::ptr::null_mut(), &batch,
    keys + total, values + total, &count, &opts);
    CHECK((err && errno != ENOENT), "lookup with steps",
    "error: %s\n", strerror(errno));
    total += count;
    if (err)
    break;
    }
    CHECK(total != max_entries, "lookup with steps",
    "total = %u, max_entries = %u\n", total, max_entries);
    map_batch_verify(visited, max_entries, keys, values);
    total = 0;
    count = step;
    while (total < max_entries) {
    if (max_entries - total < step)
    count = max_entries - total;
    err = bpf_map_delete_batch(map_fd, keys + total, &count,
    &opts);
    CHECK((err && errno != ENOENT), "delete batch",
    "error: %s\n", strerror(errno));
    total += count;
    if (err)
    break;
    }
    CHECK(total != max_entries, "delete with steps",
    "total = %u, max_entries = %u\n", total, max_entries);
// check map is empty, errno == ENOENT
    err = bpf_map_get_next_key(map_fd, core::ptr::null_mut(), &key);
    CHECK(!err || errno != ENOENT, "bpf_map_get_next_key()",
    "error: %s\n", strerror(errno));
    total_success++;
    }
    CHECK(total_success == 0, "check total_success",
    "unexpected failure\n");
    printf("%s:PASS\n", __func__);
    free(keys);
    free(values);
    free(visited);
    close(map_fd);
    }
