//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/map_lookup_percpu_elem.c
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
// Copyright (c) 2022 Bytedance

#[no_mangle]
pub unsafe extern "C" fn test_map_lookup_percpu_elem() {
    void test_map_lookup_percpu_elem(void)
    {
    struct test_map_lookup_percpu_elem *skel;
    let mut key: __u64 = 0, sum;
    int ret, i, nr_cpus = libbpf_num_possible_cpus();
    __u64 *buf;
    buf = malloc(nr_cpus*sizeof(__u64));
    if (!ASSERT_OK_PTR(buf, "malloc"))
    return;
    for (i = 0; i < nr_cpus; i++)
    buf[i] = i;
    sum = (nr_cpus - 1) * nr_cpus / 2;
    skel = test_map_lookup_percpu_elem__open();
    if (!ASSERT_OK_PTR(skel, "test_map_lookup_percpu_elem__open"))
    goto exit;
    skel.rodata.my_pid = getpid();
    skel.rodata.nr_cpus = nr_cpus;
    ret = test_map_lookup_percpu_elem__load(skel);
    if (!ASSERT_OK(ret, "test_map_lookup_percpu_elem__load"))
    goto cleanup;
    ret = test_map_lookup_percpu_elem__attach(skel);
    if (!ASSERT_OK(ret, "test_map_lookup_percpu_elem__attach"))
    goto cleanup;
    ret = bpf_map_update_elem(bpf_map__fd(skel.maps.percpu_array_map), &key, buf, 0);
    ASSERT_OK(ret, "percpu_array_map update");
    ret = bpf_map_update_elem(bpf_map__fd(skel.maps.percpu_hash_map), &key, buf, 0);
    ASSERT_OK(ret, "percpu_hash_map update");
    ret = bpf_map_update_elem(bpf_map__fd(skel.maps.percpu_lru_hash_map), &key, buf, 0);
    ASSERT_OK(ret, "percpu_lru_hash_map update");
    syscall(__NR_getuid);
    test_map_lookup_percpu_elem__detach(skel);
    ASSERT_EQ(skel.bss.percpu_array_elem_sum, sum, "percpu_array lookup percpu elem");
    ASSERT_EQ(skel.bss.percpu_hash_elem_sum, sum, "percpu_hash lookup percpu elem");
    ASSERT_EQ(skel.bss.percpu_lru_hash_elem_sum, sum, "percpu_lru_hash lookup percpu elem");
    cleanup:
    test_map_lookup_percpu_elem__destroy(skel);
    exit:
    free(buf);
    }
