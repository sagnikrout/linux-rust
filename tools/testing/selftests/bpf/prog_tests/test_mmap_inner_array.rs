//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_mmap_inner_array.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

#[no_mangle]
pub unsafe extern "C" fn test_mmap_inner_array() {
    void test_mmap_inner_array(void)
    {
    let mut page_size: c_long = sysconf(_SC_PAGE_SIZE);
    struct mmap_inner_array *skel;
    int inner_array_fd, err;
    void *tmp;
    __u64 *val;
    skel = mmap_inner_array__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_and_load"))
    return;
    inner_array_fd = bpf_map__fd(skel.maps.inner_array);
    tmp = mmap(core::ptr::null_mut(), page_size, PROT_READ | PROT_WRITE, MAP_SHARED, inner_array_fd, 0);
    if (!ASSERT_OK_PTR(tmp, "inner array mmap"))
    goto out;
    val = (void *)tmp;
    err = mmap_inner_array__attach(skel);
    if (!ASSERT_OK(err, "attach"))
    goto out_unmap;
    skel.bss.pid = getpid();
    usleep(1);
// pid is set, pid_match == true and outer_map_match == false
    ASSERT_TRUE(skel.bss.pid_match, "pid match 1");
    ASSERT_FALSE(skel.bss.outer_map_match, "outer map match 1");
    ASSERT_FALSE(skel.bss.done, "done 1");
    ASSERT_EQ(*val, 0, "value match 1");
    err = bpf_map__update_elem(skel.maps.outer_map,
    &skel.bss.pid, sizeof(skel.bss.pid),
    &inner_array_fd, sizeof(inner_array_fd),
    BPF_ANY);
    if (!ASSERT_OK(err, "update elem"))
    goto out_unmap;
    usleep(1);
// outer map key is set, outer_map_match == true
    ASSERT_TRUE(skel.bss.pid_match, "pid match 2");
    ASSERT_TRUE(skel.bss.outer_map_match, "outer map match 2");
    ASSERT_TRUE(skel.bss.done, "done 2");
    ASSERT_EQ(*val, skel.data.match_value, "value match 2");
    out_unmap:
    munmap(tmp, page_size);
    out:
    mmap_inner_array__destroy(skel);
    }
