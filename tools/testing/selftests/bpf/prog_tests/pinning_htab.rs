//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/pinning_htab.c
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
unsafe extern "C" fn unpin_map(map_name: *const c_char, pin_path: *const c_char) {
    static void unpin_map(const char *map_name, const char *pin_path)
    {
    struct test_pinning_htab *skel;
    struct bpf_map *map;
    int err;
    skel = test_pinning_htab__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel open_and_load"))
    return;
    map = bpf_object__find_map_by_name(skel.obj, map_name);
    if (!ASSERT_OK_PTR(map, "bpf_object__find_map_by_name"))
    goto out;
    err = bpf_map__pin(map, pin_path);
    if (!ASSERT_OK(err, "bpf_map__pin"))
    goto out;
    err = bpf_map__unpin(map, pin_path);
    ASSERT_OK(err, "bpf_map__unpin");
    out:
    test_pinning_htab__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_pinning_htab() {
    void test_pinning_htab(void)
    {
    if (test__start_subtest("timer_prealloc"))
    unpin_map("timer_prealloc", "/sys/fs/bpf/timer_prealloc");
    if (test__start_subtest("timer_no_prealloc"))
    unpin_map("timer_no_prealloc", "/sys/fs/bpf/timer_no_prealloc");
    }
