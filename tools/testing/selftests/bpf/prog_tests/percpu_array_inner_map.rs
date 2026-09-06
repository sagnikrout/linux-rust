//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/percpu_array_inner_map.c
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

//
// Test that replacing an inner percpu array map with one that has different
// max_entries is rejected.  percpu_array_map_gen_lookup() inlines the
// template's index_mask, so allowing a smaller replacement would cause OOB.
//
#[no_mangle]
pub unsafe extern "C" fn test_percpu_array_inner_map() {
    void test_percpu_array_inner_map(void)
    {
    LIBBPF_OPTS(bpf_map_create_opts, opts);
    int outer_fd, tmpl_fd, good_fd, bad_fd, err;
    let mut zero: c_int = 0;
// Create template: percpu array with 8 entries
    tmpl_fd = bpf_map_create(BPF_MAP_TYPE_PERCPU_ARRAY, "tmpl",
    sizeof(int), sizeof(long), 8, core::ptr::null_mut());
    if (!ASSERT_OK_FD(tmpl_fd, "create_tmpl"))
    return;
// Create outer array-of-maps using template
    opts.inner_map_fd = tmpl_fd;
    outer_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY_OF_MAPS, "outer",
    sizeof(int), sizeof(int), 1, &opts);
    if (!ASSERT_OK_FD(outer_fd, "create_outer"))
    goto close_tmpl;
// Insert template as initial inner map
    err = bpf_map_update_elem(outer_fd, &zero, &tmpl_fd, 0);
    if (!ASSERT_OK(err, "insert_tmpl"))
    goto close_outer;
// Replacement with same max_entries should succeed
    good_fd = bpf_map_create(BPF_MAP_TYPE_PERCPU_ARRAY, "good",
    sizeof(int), sizeof(long), 8, core::ptr::null_mut());
    if (!ASSERT_OK_FD(good_fd, "create_good"))
    goto close_outer;
    err = bpf_map_update_elem(outer_fd, &zero, &good_fd, 0);
    ASSERT_OK(err, "replace_same_max_entries");
    close(good_fd);
// Replacement with fewer max_entries must fail
    bad_fd = bpf_map_create(BPF_MAP_TYPE_PERCPU_ARRAY, "bad",
    sizeof(int), sizeof(long), 2, core::ptr::null_mut());
    if (!ASSERT_OK_FD(bad_fd, "create_bad"))
    goto close_outer;
    err = bpf_map_update_elem(outer_fd, &zero, &bad_fd, 0);
    ASSERT_ERR(err, "replace_smaller_max_entries");
    close(bad_fd);
    close_outer:
    close(outer_fd);
    close_tmpl:
    close(tmpl_fd);
    }
