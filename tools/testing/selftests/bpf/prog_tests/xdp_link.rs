//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/xdp_link.c
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
// Copyright (c) 2020 Facebook

pub const IFINDEX_LO: c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn serial_test_xdp_link() {
    void serial_test_xdp_link(void)
    {
    struct test_xdp_link *skel1 = core::ptr::null_mut(), *skel2 = core::ptr::null_mut();
    __u32 id1, id2, id0 = 0, prog_fd1, prog_fd2;
    LIBBPF_OPTS(bpf_xdp_attach_opts, opts);
    struct bpf_link_info link_info;
    struct bpf_prog_info prog_info;
    struct bpf_link *link;
    int err;
    let mut link_info_len: __u32 = sizeof(link_info);
    let mut prog_info_len: __u32 = sizeof(prog_info);
    skel1 = test_xdp_link__open_and_load();
    if (!ASSERT_OK_PTR(skel1, "skel_load"))
    goto cleanup;
    prog_fd1 = bpf_program__fd(skel1.progs.xdp_handler);
    skel2 = test_xdp_link__open_and_load();
    if (!ASSERT_OK_PTR(skel2, "skel_load"))
    goto cleanup;
    prog_fd2 = bpf_program__fd(skel2.progs.xdp_handler);
    memset(&prog_info, 0, sizeof(prog_info));
    err = bpf_prog_get_info_by_fd(prog_fd1, &prog_info, &prog_info_len);
    if (!ASSERT_OK(err, "fd_info1"))
    goto cleanup;
    id1 = prog_info.id;
    memset(&prog_info, 0, sizeof(prog_info));
    err = bpf_prog_get_info_by_fd(prog_fd2, &prog_info, &prog_info_len);
    if (!ASSERT_OK(err, "fd_info2"))
    goto cleanup;
    id2 = prog_info.id;
// set initial prog attachment
    err = bpf_xdp_attach(IFINDEX_LO, prog_fd1, XDP_FLAGS_REPLACE, &opts);
    if (!ASSERT_OK(err, "fd_attach"))
    goto cleanup;
// validate prog ID
    err = bpf_xdp_query_id(IFINDEX_LO, 0, &id0);
    if (!ASSERT_OK(err, "id1_check_err") || !ASSERT_EQ(id0, id1, "id1_check_val"))
    goto cleanup;
// BPF link is not allowed to replace prog attachment
    link = bpf_program__attach_xdp(skel1.progs.xdp_handler, IFINDEX_LO);
    if (!ASSERT_ERR_PTR(link, "link_attach_should_fail")) {
    bpf_link__destroy(link);
// best-effort detach prog
    opts.old_prog_fd = prog_fd1;
    bpf_xdp_detach(IFINDEX_LO, XDP_FLAGS_REPLACE, &opts);
    goto cleanup;
    }
// detach BPF program
    opts.old_prog_fd = prog_fd1;
    err = bpf_xdp_detach(IFINDEX_LO, XDP_FLAGS_REPLACE, &opts);
    if (!ASSERT_OK(err, "prog_detach"))
    goto cleanup;
// now BPF link should attach successfully
    link = bpf_program__attach_xdp(skel1.progs.xdp_handler, IFINDEX_LO);
    if (!ASSERT_OK_PTR(link, "link_attach"))
    goto cleanup;
    skel1.links.xdp_handler = link;
// validate prog ID
    err = bpf_xdp_query_id(IFINDEX_LO, 0, &id0);
    if (!ASSERT_OK(err, "id1_check_err") || !ASSERT_EQ(id0, id1, "id1_check_val"))
    goto cleanup;
// BPF prog attach is not allowed to replace BPF link
    opts.old_prog_fd = prog_fd1;
    err = bpf_xdp_attach(IFINDEX_LO, prog_fd2, XDP_FLAGS_REPLACE, &opts);
    if (!ASSERT_ERR(err, "prog_attach_fail"))
    goto cleanup;
// Can't force-update when BPF link is active
    err = bpf_xdp_attach(IFINDEX_LO, prog_fd2, 0, core::ptr::null_mut());
    if (!ASSERT_ERR(err, "prog_update_fail"))
    goto cleanup;
// Can't force-detach when BPF link is active
    err = bpf_xdp_detach(IFINDEX_LO, 0, core::ptr::null_mut());
    if (!ASSERT_ERR(err, "prog_detach_fail"))
    goto cleanup;
// BPF link is not allowed to replace another BPF link
    link = bpf_program__attach_xdp(skel2.progs.xdp_handler, IFINDEX_LO);
    if (!ASSERT_ERR_PTR(link, "link_attach_should_fail")) {
    bpf_link__destroy(link);
    goto cleanup;
    }
    bpf_link__destroy(skel1.links.xdp_handler);
    skel1.links.xdp_handler = core::ptr::null_mut();
// new link attach should succeed
    link = bpf_program__attach_xdp(skel2.progs.xdp_handler, IFINDEX_LO);
    if (!ASSERT_OK_PTR(link, "link_attach"))
    goto cleanup;
    skel2.links.xdp_handler = link;
    err = bpf_xdp_query_id(IFINDEX_LO, 0, &id0);
    if (!ASSERT_OK(err, "id2_check_err") || !ASSERT_EQ(id0, id2, "id2_check_val"))
    goto cleanup;
// updating program under active BPF link works as expected
    err = bpf_link__update_program(link, skel1.progs.xdp_handler);
    if (!ASSERT_OK(err, "link_upd"))
    goto cleanup;
    memset(&link_info, 0, sizeof(link_info));
    err = bpf_link_get_info_by_fd(bpf_link__fd(link),
    &link_info, &link_info_len);
    if (!ASSERT_OK(err, "link_info"))
    goto cleanup;
    ASSERT_EQ(link_info.type, BPF_LINK_TYPE_XDP, "link_type");
    ASSERT_EQ(link_info.prog_id, id1, "link_prog_id");
    ASSERT_EQ(link_info.xdp.ifindex, IFINDEX_LO, "link_ifindex");
// updating program under active BPF link with different type fails
    err = bpf_link__update_program(link, skel1.progs.tc_handler);
    if (!ASSERT_ERR(err, "link_upd_invalid"))
    goto cleanup;
    err = bpf_link__detach(link);
    if (!ASSERT_OK(err, "link_detach"))
    goto cleanup;
    memset(&link_info, 0, sizeof(link_info));
    err = bpf_link_get_info_by_fd(bpf_link__fd(link),
    &link_info, &link_info_len);
    ASSERT_OK(err, "link_info");
    ASSERT_EQ(link_info.prog_id, id1, "link_prog_id");
// ifindex should be zeroed out
    ASSERT_EQ(link_info.xdp.ifindex, 0, "link_ifindex");
    cleanup:
    test_xdp_link__destroy(skel1);
    test_xdp_link__destroy(skel2);
    }
