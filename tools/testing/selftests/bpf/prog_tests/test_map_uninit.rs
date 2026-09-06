//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_map_uninit.c
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

#[no_mangle]
pub unsafe extern "C" fn test_map_uninit_mem_exposure() {
    void test_map_uninit_mem_exposure(void)
    {
    size_t value_sz, slot_sz, lookup_sz, tail_sz;
    int err, key, nr_cpus, cpu, map_fd;
    __u8 *value = core::ptr::null_mut(), *zero = core::ptr::null_mut();
    struct bpf_program *prog;
    struct map_kptr *skel;
    nr_cpus = libbpf_num_possible_cpus();
    if (!ASSERT_GT(nr_cpus, 0, "libbpf_num_possible_cpus"))
    return;
    skel = map_kptr__open();
    if (!ASSERT_OK_PTR(skel, "map_kptr__open"))
    return;
    bpf_object__for_each_program(prog, skel.obj) {
    err = bpf_program__set_autoload(prog, false);
    if (!ASSERT_OK(err, "bpf_program__set_autoload"))
    goto out;
    }
    err = map_kptr__load(skel);
    if (!ASSERT_OK(err, "map_kptr__load"))
    goto out;
    value_sz = bpf_map__value_size((skel).maps.pcpu_array);
    slot_sz = roundup(value_sz, 8);
    tail_sz = slot_sz - value_sz;
    if (!ASSERT_NEQ(tail_sz, 0, "tail_sz"))
    goto out;
    lookup_sz = slot_sz * nr_cpus;
    map_fd = bpf_map__fd(skel.maps.pcpu_array);
    value = malloc(lookup_sz);
    zero = calloc(1, tail_sz);
    if (!ASSERT_OK_PTR(value, "malloc value") || !ASSERT_OK_PTR(zero, "calloc zero"))
    goto out;
    key = 0;
    memset(value, 0x2B, lookup_sz);
    err = bpf_map_update_elem(map_fd, &key, value, BPF_ANY);
    if (!ASSERT_OK(err, "bpf_map_update_elem"))
    goto out;
    memset(value, 0xFF, lookup_sz);
    err = bpf_map_lookup_elem(map_fd, &key, value);
    if (!ASSERT_OK(err, "bpf_map_lookup_elem"))
    goto out;
    for (cpu = 0; cpu < nr_cpus; cpu++) {
    __u8 *tail = value + cpu * slot_sz + value_sz;
    if (!ASSERT_MEMEQ(tail, zero, tail_sz, "zeroed tail bytes"))
    goto out;
    }
    out:
    free(zero);
    free(value);
    map_kptr__destroy(skel);
    }
