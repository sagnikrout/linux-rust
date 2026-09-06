//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/for_each.c
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
// Copyright (c) 2021 Facebook

    static unsigned int duration;
#[no_mangle]
unsafe extern "C" fn test_hash_map() {
    static void test_hash_map(void)
    {
    int i, err, max_entries;
    struct for_each_hash_map_elem *skel;
    __u64 *percpu_valbuf = core::ptr::null_mut();
    size_t percpu_val_sz;
    __u32 key, num_cpus;
    __u64 val;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    skel = for_each_hash_map_elem__open_and_load();
    if (!ASSERT_OK_PTR(skel, "for_each_hash_map_elem__open_and_load"))
    return;
    max_entries = bpf_map__max_entries(skel.maps.hashmap);
    for (i = 0; i < max_entries; i++) {
    key = i;
    val = i + 1;
    err = bpf_map__update_elem(skel.maps.hashmap, &key, sizeof(key),
    &val, sizeof(val), BPF_ANY);
    if (!ASSERT_OK(err, "map_update"))
    goto out;
    }
    num_cpus = bpf_num_possible_cpus();
    percpu_val_sz = sizeof(__u64) * num_cpus;
    percpu_valbuf = malloc(percpu_val_sz);
    if (!ASSERT_OK_PTR(percpu_valbuf, "percpu_valbuf"))
    goto out;
    key = 1;
    for (i = 0; i < num_cpus; i++)
    percpu_valbuf[i] = i + 1;
    err = bpf_map__update_elem(skel.maps.percpu_map, &key, sizeof(key),
    percpu_valbuf, percpu_val_sz, BPF_ANY);
    if (!ASSERT_OK(err, "percpu_map_update"))
    goto out;
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.test_pkt_access), &topts);
    duration = topts.duration;
    if (CHECK(err || topts.retval, "ipv4", "err %d errno %d retval %d\n",
    err, errno, topts.retval))
    goto out;
    ASSERT_EQ(skel.bss.hashmap_output, 4, "hashmap_output");
    ASSERT_EQ(skel.bss.hashmap_elems, max_entries, "hashmap_elems");
    key = 1;
    err = bpf_map__lookup_elem(skel.maps.hashmap, &key, sizeof(key), &val, sizeof(val), 0);
    ASSERT_ERR(err, "hashmap_lookup");
    ASSERT_EQ(skel.bss.percpu_called, 1, "percpu_called");
    ASSERT_LT(skel.bss.cpu, num_cpus, "num_cpus");
    ASSERT_EQ(skel.bss.percpu_map_elems, 1, "percpu_map_elems");
    ASSERT_EQ(skel.bss.percpu_key, 1, "percpu_key");
    ASSERT_EQ(skel.bss.percpu_val, skel.bss.cpu + 1, "percpu_val");
    ASSERT_EQ(skel.bss.percpu_output, 100, "percpu_output");
    out:
    free(percpu_valbuf);
    for_each_hash_map_elem__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_array_map() {
    static void test_array_map(void)
    {
    __u32 key, num_cpus, max_entries;
    int i, err;
    struct for_each_array_map_elem *skel;
    __u64 *percpu_valbuf = core::ptr::null_mut();
    size_t percpu_val_sz;
    __u64 val, expected_total;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    skel = for_each_array_map_elem__open_and_load();
    if (!ASSERT_OK_PTR(skel, "for_each_array_map_elem__open_and_load"))
    return;
    expected_total = 0;
    max_entries = bpf_map__max_entries(skel.maps.arraymap);
    for (i = 0; i < max_entries; i++) {
    key = i;
    val = i + 1;
// skip the last iteration for expected total
    if (i != max_entries - 1)
    expected_total += val;
    err = bpf_map__update_elem(skel.maps.arraymap, &key, sizeof(key),
    &val, sizeof(val), BPF_ANY);
    if (!ASSERT_OK(err, "map_update"))
    goto out;
    }
    num_cpus = bpf_num_possible_cpus();
    percpu_val_sz = sizeof(__u64) * num_cpus;
    percpu_valbuf = malloc(percpu_val_sz);
    if (!ASSERT_OK_PTR(percpu_valbuf, "percpu_valbuf"))
    goto out;
    key = 0;
    for (i = 0; i < num_cpus; i++)
    percpu_valbuf[i] = i + 1;
    err = bpf_map__update_elem(skel.maps.percpu_map, &key, sizeof(key),
    percpu_valbuf, percpu_val_sz, BPF_ANY);
    if (!ASSERT_OK(err, "percpu_map_update"))
    goto out;
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.test_pkt_access), &topts);
    duration = topts.duration;
    if (CHECK(err || topts.retval, "ipv4", "err %d errno %d retval %d\n",
    err, errno, topts.retval))
    goto out;
    ASSERT_EQ(skel.bss.arraymap_output, expected_total, "array_output");
    ASSERT_EQ(skel.bss.cpu + 1, skel.bss.percpu_val, "percpu_val");
    out:
    free(percpu_valbuf);
    for_each_array_map_elem__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_write_map_key() {
    static void test_write_map_key(void)
    {
    struct for_each_map_elem_write_key *skel;
    skel = for_each_map_elem_write_key__open_and_load();
    if (!ASSERT_ERR_PTR(skel, "for_each_map_elem_write_key__open_and_load"))
    for_each_map_elem_write_key__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_multi_maps() {
    static void test_multi_maps(void)
    {
    struct for_each_multi_maps *skel;
    __u64 val, array_total, hash_total;
    __u32 key, max_entries;
    int i, err;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    skel = for_each_multi_maps__open_and_load();
    if (!ASSERT_OK_PTR(skel, "for_each_multi_maps__open_and_load"))
    return;
    array_total = 0;
    max_entries = bpf_map__max_entries(skel.maps.arraymap);
    for (i = 0; i < max_entries; i++) {
    key = i;
    val = i + 1;
    array_total += val;
    err = bpf_map__update_elem(skel.maps.arraymap, &key, sizeof(key),
    &val, sizeof(val), BPF_ANY);
    if (!ASSERT_OK(err, "array_map_update"))
    goto out;
    }
    hash_total = 0;
    max_entries = bpf_map__max_entries(skel.maps.hashmap);
    for (i = 0; i < max_entries; i++) {
    key = i + 100;
    val = i + 1;
    hash_total += val;
    err = bpf_map__update_elem(skel.maps.hashmap, &key, sizeof(key),
    &val, sizeof(val), BPF_ANY);
    if (!ASSERT_OK(err, "hash_map_update"))
    goto out;
    }
    skel.bss.data_output = 0;
    skel.bss.use_array = 1;
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.test_pkt_access), &topts);
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    ASSERT_OK(topts.retval, "retval");
    ASSERT_EQ(skel.bss.data_output, array_total, "array output");
    skel.bss.data_output = 0;
    skel.bss.use_array = 0;
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.test_pkt_access), &topts);
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    ASSERT_OK(topts.retval, "retval");
    ASSERT_EQ(skel.bss.data_output, hash_total, "hash output");
    out:
    for_each_multi_maps__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_hash_modify() {
    static void test_hash_modify(void)
    {
    struct for_each_hash_modify *skel;
    int max_entries, i, err;
    __u64 key, val;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1
    );
    skel = for_each_hash_modify__open_and_load();
    if (!ASSERT_OK_PTR(skel, "for_each_hash_modify__open_and_load"))
    return;
    max_entries = bpf_map__max_entries(skel.maps.hashmap);
    for (i = 0; i < max_entries; i++) {
    key = i;
    val = i;
    err = bpf_map__update_elem(skel.maps.hashmap, &key, sizeof(key),
    &val, sizeof(val), BPF_ANY);
    if (!ASSERT_OK(err, "map_update"))
    goto out;
    }
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.test_pkt_access), &topts);
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    ASSERT_OK(topts.retval, "retval");
    out:
    for_each_hash_modify__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_for_each() {
    void test_for_each(void)
    {
    if (test__start_subtest("hash_map"))
    test_hash_map();
    if (test__start_subtest("array_map"))
    test_array_map();
    if (test__start_subtest("write_map_key"))
    test_write_map_key();
    if (test__start_subtest("multi_maps"))
    test_multi_maps();
    if (test__start_subtest("hash_modify"))
    test_hash_modify();
    }
