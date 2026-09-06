//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/inner_array_lookup.c
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

#[no_mangle]
pub unsafe extern "C" fn test_inner_array_lookup() {
    void test_inner_array_lookup(void)
    {
    int map1_fd, err;
    let mut key: c_int = 3;
    let mut val: c_int = 1;
    struct inner_array_lookup *skel;
    skel = inner_array_lookup__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_load_skeleton"))
    return;
    err = inner_array_lookup__attach(skel);
    if (!ASSERT_OK(err, "skeleton_attach"))
    goto cleanup;
    map1_fd = bpf_map__fd(skel.maps.inner_map1);
    bpf_map_update_elem(map1_fd, &key, &val, 0);
// Probe should have set the element at index 3 to 2
    bpf_map_lookup_elem(map1_fd, &key, &val);
    ASSERT_EQ(val, 2, "value_is_2");
    cleanup:
    inner_array_lookup__destroy(skel);
    }
