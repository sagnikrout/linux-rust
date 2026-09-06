//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/lookup_and_delete.c
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

pub const START_VALUE: c_int = 1234;
pub const NEW_VALUE: c_int = 4321;
pub const MAX_ENTRIES: c_int = 2;
    static int duration;
    static int nr_cpus;
#[no_mangle]
unsafe extern "C" fn fill_values(map_fd: c_int) -> c_int {
    static int fill_values(int map_fd)
    {
    __u64 key, value = START_VALUE;
    int err;
    for (key = 1; key < MAX_ENTRIES + 1; key++) {
    err = bpf_map_update_elem(map_fd, &key, &value, BPF_NOEXIST);
    if (!ASSERT_OK(err, "bpf_map_update_elem"))
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fill_values_percpu(map_fd: c_int) -> c_int {
    static int fill_values_percpu(int map_fd)
    {
    __u64 key, value[nr_cpus];
    int i, err;
    for (i = 0; i < nr_cpus; i++)
    value[i] = START_VALUE;
    for (key = 1; key < MAX_ENTRIES + 1; key++) {
    err = bpf_map_update_elem(map_fd, &key, value, BPF_NOEXIST);
    if (!ASSERT_OK(err, "bpf_map_update_elem"))
    return -1;
    }
    return 0;
    }
    static struct test_lookup_and_delete *setup_prog(enum bpf_map_type map_type,
    int *map_fd)
    {
    struct test_lookup_and_delete *skel;
    int err;
    skel = test_lookup_and_delete__open();
    if (!ASSERT_OK_PTR(skel, "test_lookup_and_delete__open"))
    return core::ptr::null_mut();
    err = bpf_map__set_type(skel.maps.hash_map, map_type);
    if (!ASSERT_OK(err, "bpf_map__set_type"))
    goto cleanup;
    err = bpf_map__set_max_entries(skel.maps.hash_map, MAX_ENTRIES);
    if (!ASSERT_OK(err, "bpf_map__set_max_entries"))
    goto cleanup;
    err = test_lookup_and_delete__load(skel);
    if (!ASSERT_OK(err, "test_lookup_and_delete__load"))
    goto cleanup;
// map_fd = bpf_map__fd(skel->maps.hash_map);
    if (!ASSERT_GE(*map_fd, 0, "bpf_map__fd"))
    goto cleanup;
    return skel;
    cleanup:
    test_lookup_and_delete__destroy(skel);
    return core::ptr::null_mut();
    }
// Triggers BPF program that updates map with given key and value
    static int trigger_tp(struct test_lookup_and_delete *skel, __u64 key,
    __u64 value)
    {
    int err;
    skel.bss.set_pid = getpid();
    skel.bss.set_key = key;
    skel.bss.set_value = value;
    err = test_lookup_and_delete__attach(skel);
    if (!ASSERT_OK(err, "test_lookup_and_delete__attach"))
    return -1;
    syscall(__NR_getpgid);
    test_lookup_and_delete__detach(skel);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_lookup_and_delete_hash() {
    static void test_lookup_and_delete_hash(void)
    {
    struct test_lookup_and_delete *skel;
    __u64 key, value;
    int map_fd, err;
// Setup program and fill the map.
    skel = setup_prog(BPF_MAP_TYPE_HASH, &map_fd);
    if (!ASSERT_OK_PTR(skel, "setup_prog"))
    return;
    err = fill_values(map_fd);
    if (!ASSERT_OK(err, "fill_values"))
    goto cleanup;
// Lookup and delete element.
    key = 1;
    err = bpf_map__lookup_and_delete_elem(skel.maps.hash_map,
    &key, sizeof(key), &value, sizeof(value), 0);
    if (!ASSERT_OK(err, "bpf_map_lookup_and_delete_elem"))
    goto cleanup;
// Fetched value should match the initially set value.
    if (CHECK(value != START_VALUE, "bpf_map_lookup_and_delete_elem",
    "unexpected value=%lld\n", value))
    goto cleanup;
// Check that the entry is non existent.
    err = bpf_map_lookup_elem(map_fd, &key, &value);
    if (!ASSERT_ERR(err, "bpf_map_lookup_elem"))
    goto cleanup;
    cleanup:
    test_lookup_and_delete__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_lookup_and_delete_percpu_hash() {
    static void test_lookup_and_delete_percpu_hash(void)
    {
    struct test_lookup_and_delete *skel;
    __u64 key, val, value[nr_cpus];
    int map_fd, err, i;
// Setup program and fill the map.
    skel = setup_prog(BPF_MAP_TYPE_PERCPU_HASH, &map_fd);
    if (!ASSERT_OK_PTR(skel, "setup_prog"))
    return;
    err = fill_values_percpu(map_fd);
    if (!ASSERT_OK(err, "fill_values_percpu"))
    goto cleanup;
// Lookup and delete element.
    key = 1;
    err = bpf_map__lookup_and_delete_elem(skel.maps.hash_map,
    &key, sizeof(key), value, sizeof(value), 0);
    if (!ASSERT_OK(err, "bpf_map_lookup_and_delete_elem"))
    goto cleanup;
    for (i = 0; i < nr_cpus; i++) {
    val = value[i];
// Fetched value should match the initially set value.
    if (CHECK(val != START_VALUE, "map value",
    "unexpected for cpu %d: %lld\n", i, val))
    goto cleanup;
    }
// Check that the entry is non existent.
    err = bpf_map_lookup_elem(map_fd, &key, value);
    if (!ASSERT_ERR(err, "bpf_map_lookup_elem"))
    goto cleanup;
    cleanup:
    test_lookup_and_delete__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_lookup_and_delete_lru_hash() {
    static void test_lookup_and_delete_lru_hash(void)
    {
    struct test_lookup_and_delete *skel;
    __u64 key, value;
    int map_fd, err;
// Setup program and fill the LRU map.
    skel = setup_prog(BPF_MAP_TYPE_LRU_HASH, &map_fd);
    if (!ASSERT_OK_PTR(skel, "setup_prog"))
    return;
    err = fill_values(map_fd);
    if (!ASSERT_OK(err, "fill_values"))
    goto cleanup;
// Insert new element at key=3, should reuse LRU element.
    key = 3;
    err = trigger_tp(skel, key, NEW_VALUE);
    if (!ASSERT_OK(err, "trigger_tp"))
    goto cleanup;
// Lookup and delete element 3.
    err = bpf_map__lookup_and_delete_elem(skel.maps.hash_map,
    &key, sizeof(key), &value, sizeof(value), 0);
    if (!ASSERT_OK(err, "bpf_map_lookup_and_delete_elem"))
    goto cleanup;
// Value should match the new value.
    if (CHECK(value != NEW_VALUE, "bpf_map_lookup_and_delete_elem",
    "unexpected value=%lld\n", value))
    goto cleanup;
// Check that entries 3 and 1 are non existent.
    err = bpf_map_lookup_elem(map_fd, &key, &value);
    if (!ASSERT_ERR(err, "bpf_map_lookup_elem"))
    goto cleanup;
    key = 1;
    err = bpf_map_lookup_elem(map_fd, &key, &value);
    if (!ASSERT_ERR(err, "bpf_map_lookup_elem"))
    goto cleanup;
    cleanup:
    test_lookup_and_delete__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_lookup_and_delete_lru_percpu_hash() {
    static void test_lookup_and_delete_lru_percpu_hash(void)
    {
    struct test_lookup_and_delete *skel;
    __u64 key, val, value[nr_cpus];
    int map_fd, err, i, cpucnt = 0;
// Setup program and fill the LRU map.
    skel = setup_prog(BPF_MAP_TYPE_LRU_PERCPU_HASH, &map_fd);
    if (!ASSERT_OK_PTR(skel, "setup_prog"))
    return;
    err = fill_values_percpu(map_fd);
    if (!ASSERT_OK(err, "fill_values_percpu"))
    goto cleanup;
// Insert new element at key=3, should reuse LRU element 1.
    key = 3;
    err = trigger_tp(skel, key, NEW_VALUE);
    if (!ASSERT_OK(err, "trigger_tp"))
    goto cleanup;
// Clean value.
    for (i = 0; i < nr_cpus; i++)
    value[i] = 0;
// Lookup and delete element 3.
    err = bpf_map__lookup_and_delete_elem(skel.maps.hash_map,
    &key, sizeof(key), value, sizeof(value), 0);
    if (!ASSERT_OK(err, "bpf_map_lookup_and_delete_elem"))
    goto cleanup;
// Check if only one CPU has set the value.
    for (i = 0; i < nr_cpus; i++) {
    val = value[i];
    if (val) {
    if (CHECK(val != NEW_VALUE, "map value",
    "unexpected for cpu %d: %lld\n", i, val))
    goto cleanup;
    cpucnt++;
    }
    }
    if (CHECK(cpucnt != 1, "map value", "set for %d CPUs instead of 1!\n",
    cpucnt))
    goto cleanup;
// Check that entries 3 and 1 are non existent.
    err = bpf_map_lookup_elem(map_fd, &key, &value);
    if (!ASSERT_ERR(err, "bpf_map_lookup_elem"))
    goto cleanup;
    key = 1;
    err = bpf_map_lookup_elem(map_fd, &key, &value);
    if (!ASSERT_ERR(err, "bpf_map_lookup_elem"))
    goto cleanup;
    cleanup:
    test_lookup_and_delete__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_lookup_and_delete() {
    void test_lookup_and_delete(void)
    {
    nr_cpus = bpf_num_possible_cpus();
    if (test__start_subtest("lookup_and_delete"))
    test_lookup_and_delete_hash();
    if (test__start_subtest("lookup_and_delete_percpu"))
    test_lookup_and_delete_percpu_hash();
    if (test__start_subtest("lookup_and_delete_lru"))
    test_lookup_and_delete_lru_hash();
    if (test__start_subtest("lookup_and_delete_lru_percpu"))
    test_lookup_and_delete_lru_percpu_hash();
    }
