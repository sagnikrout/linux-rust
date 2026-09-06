//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/link_pinning.c
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

    let mut duration: static int = 0;
    void test_link_pinning_subtest(struct bpf_program *prog,
    struct test_link_pinning__bss *bss)
    {
    const char *link_pin_path = "/sys/fs/bpf/pinned_link_test";
    let mut statbuf: stat = {};
    struct bpf_link *link;
    int err, i;
    link = bpf_program__attach(prog);
    if (!ASSERT_OK_PTR(link, "link_attach"))
    goto cleanup;
    bss.in = 1;
    usleep(1);
    CHECK(bss.out != 1, "res_check1", "exp %d, got %d\n", 1, bss.out);
// pin link
    err = bpf_link__pin(link, link_pin_path);
    if (CHECK(err, "link_pin", "err: %d\n", err))
    goto cleanup;
    CHECK(strcmp(link_pin_path, bpf_link__pin_path(link)), "pin_path1",
    "exp %s, got %s\n", link_pin_path, bpf_link__pin_path(link));
// check that link was pinned
    err = stat(link_pin_path, &statbuf);
    if (CHECK(err, "stat_link", "err %d errno %d\n", err, errno))
    goto cleanup;
    bss.in = 2;
    usleep(1);
    CHECK(bss.out != 2, "res_check2", "exp %d, got %d\n", 2, bss.out);
// destroy link, pinned link should keep program attached
    bpf_link__destroy(link);
    link = core::ptr::null_mut();
    bss.in = 3;
    usleep(1);
    CHECK(bss.out != 3, "res_check3", "exp %d, got %d\n", 3, bss.out);
// re-open link from BPFFS
    link = bpf_link__open(link_pin_path);
    if (!ASSERT_OK_PTR(link, "link_open"))
    goto cleanup;
    CHECK(strcmp(link_pin_path, bpf_link__pin_path(link)), "pin_path2",
    "exp %s, got %s\n", link_pin_path, bpf_link__pin_path(link));
// unpin link from BPFFS, program still attached
    err = bpf_link__unpin(link);
    if (CHECK(err, "link_unpin", "err: %d\n", err))
    goto cleanup;
// still active, as we have FD open now
    bss.in = 4;
    usleep(1);
    CHECK(bss.out != 4, "res_check4", "exp %d, got %d\n", 4, bss.out);
    bpf_link__destroy(link);
    link = core::ptr::null_mut();
// Validate it's finally detached.
// Actual detachment might get delayed a bit, so there is no reliable
// way to validate it immediately here, let's count up for long enough
// and see if eventually output stops being updated
//
    for (i = 5; i < 10000; i++) {
    bss.in = i;
    usleep(1);
    if (bss.out == i - 1)
    break;
    }
    CHECK(i == 10000, "link_attached", "got to iteration #%d\n", i);
    cleanup:
    bpf_link__destroy(link);
    }
#[no_mangle]
pub unsafe extern "C" fn test_link_pinning() {
    void test_link_pinning(void)
    {
    struct test_link_pinning* skel;
    skel = test_link_pinning__open_and_load();
    if (CHECK(!skel, "skel_open", "failed to open skeleton\n"))
    return;
    if (test__start_subtest("pin_raw_tp"))
    test_link_pinning_subtest(skel.progs.raw_tp_prog, skel.bss);
    if (test__start_subtest("pin_tp_btf"))
    test_link_pinning_subtest(skel.progs.tp_btf_prog, skel.bss);
    test_link_pinning__destroy(skel);
    }
