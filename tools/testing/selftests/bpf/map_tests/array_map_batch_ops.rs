//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/map_tests/array_map_batch_ops.c
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

    static int nr_cpus;
    static void map_batch_update(int map_fd, __u32 max_entries, int *keys,
    __s64 *values, bool is_pcpu)
    {
    int i, j, err;
    let mut cpu_offset: c_int = 0;
    DECLARE_LIBBPF_OPTS(bpf_map_batch_opts, opts,
    .elem_flags = 0,
    .flags = 0,
    );
    for (i = 0; i < max_entries; i++) {
    keys[i] = i;
    if (is_pcpu) {
    cpu_offset = i * nr_cpus;
    for (j = 0; j < nr_cpus; j++)
    (values + cpu_offset)[j] = i + 1 + j;
    } else {
    values[i] = i + 1;
    }
    }
    err = bpf_map_update_batch(map_fd, keys, values, &max_entries, &opts);
    CHECK(err, "bpf_map_update_batch()", "error:%s\n", strerror(errno));
    }
    static void map_batch_verify(int *visited, __u32 max_entries, int *keys,
    __s64 *values, bool is_pcpu)
    {
    int i, j;
    let mut cpu_offset: c_int = 0;
    memset(visited, 0, max_entries * sizeof(*visited));
    for (i = 0; i < max_entries; i++) {
    if (is_pcpu) {
    cpu_offset = i * nr_cpus;
    for (j = 0; j < nr_cpus; j++) {
    let mut value: __s64 = (values + cpu_offset)[j];
    CHECK(keys[i] + j + 1 != value,
    "key/value checking",
    "error: i %d j %d key %d value %lld\n", i,
    j, keys[i], value);
    }
    } else {
    CHECK(keys[i] + 1 != values[i], "key/value checking",
    "error: i %d key %d value %lld\n", i, keys[i],
    values[i]);
    }
    visited[i] = 1;
    }
    for (i = 0; i < max_entries; i++) {
    CHECK(visited[i] != 1, "visited checking",
    "error: keys array at index %d missing\n", i);
    }
    }
#[no_mangle]
unsafe extern "C" fn __test_map_lookup_and_update_batch(is_pcpu: bool) {
    static void __test_map_lookup_and_update_batch(bool is_pcpu)
    {
    int map_fd, *keys, *visited;
    __u32 count, total, total_success;
    let mut max_entries: __u32 = 10;
    let mut batch: __u64 = 0;
    int err, step, value_size;
    void *values;
    DECLARE_LIBBPF_OPTS(bpf_map_batch_opts, opts,
    .elem_flags = 0,
    .flags = 0,
    );
    map_fd = bpf_map_create(is_pcpu ? BPF_MAP_TYPE_PERCPU_ARRAY : BPF_MAP_TYPE_ARRAY,
    "array_map", sizeof(int), sizeof(__s64), max_entries, core::ptr::null_mut());
    CHECK(map_fd == -1,
    "bpf_map_create()", "error:%s\n", strerror(errno));
    value_size = sizeof(__s64);
    if (is_pcpu)
    value_size *= nr_cpus;
    keys = calloc(max_entries, sizeof(*keys));
    values = calloc(max_entries, value_size);
    visited = calloc(max_entries, sizeof(*visited));
    CHECK(!keys || !values || !visited, "malloc()", "error:%s\n",
    strerror(errno));
// test 1: lookup in a loop with various steps.
    total_success = 0;
    for (step = 1; step < max_entries; step++) {
    map_batch_update(map_fd, max_entries, keys, values, is_pcpu);
    map_batch_verify(visited, max_entries, keys, values, is_pcpu);
    memset(keys, 0, max_entries * sizeof(*keys));
    memset(values, 0, max_entries * value_size);
    batch = 0;
    total = 0;
// iteratively lookup/delete elements with 'step'
// elements each.
//
    count = step;
    while (true) {
    err = bpf_map_lookup_batch(map_fd,
    total ? &batch : core::ptr::null_mut(),
    &batch, keys + total,
    values + total * value_size,
    &count, &opts);
    CHECK((err && errno != ENOENT), "lookup with steps",
    "error: %s\n", strerror(errno));
    total += count;
    if (err)
    break;
    }
    CHECK(total != max_entries, "lookup with steps",
    "total = %u, max_entries = %u\n", total, max_entries);
    map_batch_verify(visited, max_entries, keys, values, is_pcpu);
    total_success++;
    }
    CHECK(total_success == 0, "check total_success",
    "unexpected failure\n");
    free(keys);
    free(values);
    free(visited);
    close(map_fd);
    }
#[no_mangle]
unsafe extern "C" fn array_map_batch_ops() {
    static void array_map_batch_ops(void)
    {
    __test_map_lookup_and_update_batch(false);
    printf("test_%s:PASS\n", __func__);
    }
#[no_mangle]
unsafe extern "C" fn array_percpu_map_batch_ops() {
    static void array_percpu_map_batch_ops(void)
    {
    __test_map_lookup_and_update_batch(true);
    printf("test_%s:PASS\n", __func__);
    }
#[no_mangle]
pub unsafe extern "C" fn test_array_map_batch_ops() {
    void test_array_map_batch_ops(void)
    {
    nr_cpus = libbpf_num_possible_cpus();
    CHECK(nr_cpus < 0, "nr_cpus checking",
    "error: get possible cpus failed");
    array_map_batch_ops();
    array_percpu_map_batch_ops();
    }
