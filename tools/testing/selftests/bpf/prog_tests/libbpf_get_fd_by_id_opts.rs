//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/libbpf_get_fd_by_id_opts.c
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
// Copyright (C) 2022 Huawei Technologies Duesseldorf GmbH
//
// Author: Roberto Sassu <roberto.sassu@huawei.com>
//

#[no_mangle]
pub unsafe extern "C" fn test_libbpf_get_fd_by_id_opts() {
    void test_libbpf_get_fd_by_id_opts(void)
    {
    struct test_libbpf_get_fd_by_id_opts *skel;
    let mut info_m: bpf_map_info = {};
    let mut len: __u32 = sizeof(info_m), value;
    int ret, zero = 0, fd = -1;
    LIBBPF_OPTS(bpf_get_fd_by_id_opts, fd_opts_rdonly,
    .open_flags = BPF_F_RDONLY,
    );
    skel = test_libbpf_get_fd_by_id_opts__open_and_load();
    if (!ASSERT_OK_PTR(skel,
    "test_libbpf_get_fd_by_id_opts__open_and_load"))
    return;
    ret = test_libbpf_get_fd_by_id_opts__attach(skel);
    if (!ASSERT_OK(ret, "test_libbpf_get_fd_by_id_opts__attach"))
    goto close_prog;
    ret = bpf_map_get_info_by_fd(bpf_map__fd(skel.maps.data_input),
    &info_m, &len);
    if (!ASSERT_OK(ret, "bpf_map_get_info_by_fd"))
    goto close_prog;
    fd = bpf_map_get_fd_by_id(info_m.id);
    if (!ASSERT_LT(fd, 0, "bpf_map_get_fd_by_id"))
    goto close_prog;
    fd = bpf_map_get_fd_by_id_opts(info_m.id, core::ptr::null_mut());
    if (!ASSERT_LT(fd, 0, "bpf_map_get_fd_by_id_opts"))
    goto close_prog;
    fd = bpf_map_get_fd_by_id_opts(info_m.id, &fd_opts_rdonly);
    if (!ASSERT_GE(fd, 0, "bpf_map_get_fd_by_id_opts"))
    goto close_prog;
// Map lookup should work with read-only fd.
    ret = bpf_map_lookup_elem(fd, &zero, &value);
    if (!ASSERT_OK(ret, "bpf_map_lookup_elem"))
    goto close_prog;
    if (!ASSERT_EQ(value, 0, "map value mismatch"))
    goto close_prog;
// Map update should not work with read-only fd.
    ret = bpf_map_update_elem(fd, &zero, &len, BPF_ANY);
    if (!ASSERT_LT(ret, 0, "bpf_map_update_elem"))
    goto close_prog;
// Map update should work with read-write fd.
    ret = bpf_map_update_elem(bpf_map__fd(skel.maps.data_input), &zero,
    &len, BPF_ANY);
    if (!ASSERT_OK(ret, "bpf_map_update_elem"))
    goto close_prog;
// Prog get fd with opts set should not work (no kernel support).
    ret = bpf_prog_get_fd_by_id_opts(0, &fd_opts_rdonly);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_prog_get_fd_by_id_opts"))
    goto close_prog;
// Link get fd with opts set should not work (no kernel support).
    ret = bpf_link_get_fd_by_id_opts(0, &fd_opts_rdonly);
    if (!ASSERT_EQ(ret, -EINVAL, "bpf_link_get_fd_by_id_opts"))
    goto close_prog;
// BTF get fd with opts set should not work (no kernel support).
    ret = bpf_btf_get_fd_by_id_opts(0, &fd_opts_rdonly);
    ASSERT_EQ(ret, -EINVAL, "bpf_btf_get_fd_by_id_opts");
    close_prog:
    if (fd >= 0)
    close(fd);
    test_libbpf_get_fd_by_id_opts__destroy(skel);
    }
