//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/bpf_attr_size.c
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
// Copyright (c) 2026 Google LLC

#[no_mangle]
unsafe extern "C" fn test_query_size_boundaries() {
    static void test_query_size_boundaries(void)
    {
    struct cgroup_skb_direct_packet_access *skel;
    struct bpf_link *link = core::ptr::null_mut();
    union bpf_attr attr;
    let mut cg_fd: c_int = -1;
    int err;
    skel = cgroup_skb_direct_packet_access__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_load"))
    return;
    cg_fd = test__join_cgroup("/attr_size_cg");
    if (!ASSERT_GE(cg_fd, 0, "join_cgroup"))
    goto cleanup;
    link = bpf_program__attach_cgroup(skel.progs.direct_packet_access,
    cg_fd);
    if (!ASSERT_OK_PTR(link, "cg_attach"))
    goto cleanup;
    memset(&attr, 0, sizeof(attr));
    attr.query.target_fd = cg_fd;
    attr.query.attach_type = BPF_CGROUP_INET_INGRESS;
    attr.query.revision = 0xdeadbeefdeadbeefULL;
    err = syscall(__NR_bpf, BPF_PROG_QUERY, &attr, OLD_QUERY_SIZE);
    if (ASSERT_OK(err, "query_old_size")) {
    ASSERT_EQ(attr.query.prog_cnt, 1, "prog_cnt_written_old");
    ASSERT_EQ(attr.query.revision, 0xdeadbeefdeadbeefULL,
    "revision_not_written_old");
    }
    memset(&attr, 0, sizeof(attr));
    attr.query.target_fd = cg_fd;
    attr.query.attach_type = BPF_CGROUP_INET_INGRESS;
    err = syscall(__NR_bpf, BPF_PROG_QUERY, &attr, FULL_QUERY_SIZE);
    if (!ASSERT_OK(err, "query_full_size"))
    goto cleanup;
    ASSERT_EQ(attr.query.prog_cnt, 1, "prog_cnt_written");
    ASSERT_GT(attr.query.revision, 0, "revision_written");
    cleanup:
    if (link)
    bpf_link__destroy(link);
    if (cg_fd >= 0)
    close(cg_fd);
    cgroup_skb_direct_packet_access__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_map_info_tail_zero() {
    static void test_map_info_tail_zero(void)
    {
    LIBBPF_OPTS(bpf_map_create_opts, map_opts);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_info_fake {
    pub hash_size)]: __u8 info[offsetofend(struct bpf_map_info,,
    pub pad: __u32,
    } info = {
    .pad = 1,
}

    int map_fd, err;
    __u32 info_len;
    map_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, "arr", sizeof(int), 1, 1, &map_opts);
    if (!ASSERT_GE(map_fd, 0, "bpf_map_create"))
    return;
    info_len = sizeof(info);
    err = bpf_obj_get_info_by_fd(map_fd, &info, &info_len);
    ASSERT_EQ(err, -E2BIG, "bpf_obj_get_info_by_fd");
    close(map_fd);
    }
#[no_mangle]
unsafe extern "C" fn test_prog_info_tail_zero() {
    static void test_prog_info_tail_zero(void)
    {
    LIBBPF_OPTS(bpf_prog_load_opts, prog_opts);
    struct bpf_insn insns[] = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_info_fake {
    pub attach_btf_id)]: __u8 info[offsetofend(struct bpf_prog_info,,
    pub pad: __u32,
    } info = {
    .pad = 1,
}

    int prog_fd, err;
    __u32 info_len;
    prog_fd = bpf_prog_load(BPF_PROG_TYPE_SOCKET_FILTER, "test_prog", "GPL", insns,
    ARRAY_SIZE(insns), &prog_opts);
    if (!ASSERT_GE(prog_fd, 0, "bpf_prog_load"))
    return;
    info_len = sizeof(info);
    err = bpf_obj_get_info_by_fd(prog_fd, &info, &info_len);
    ASSERT_EQ(err, -E2BIG, "bpf_obj_get_info_by_fd");
    close(prog_fd);
    }
#[no_mangle]
pub unsafe extern "C" fn test_bpf_attr_size() {
    void test_bpf_attr_size(void)
    {
    if (test__start_subtest("query_size_boundaries"))
    test_query_size_boundaries();
    if (test__start_subtest("map_info_tail_zero"))
    test_map_info_tail_zero();
    if (test__start_subtest("prog_info_tail_zero"))
    test_prog_info_tail_zero();
    }
