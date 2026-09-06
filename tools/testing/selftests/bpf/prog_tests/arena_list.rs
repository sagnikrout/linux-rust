//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/arena_list.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub node: arena_list_node,
    pub value: __u64,
}

#[no_mangle]
unsafe extern "C" fn list_sum(head: *mut arena_list_head) -> c_int {
    static int list_sum(struct arena_list_head *head)
    {
    struct elem __arena *n;
    let mut sum: c_int = 0;
    list_for_each_entry(n, head, node)
    sum += n.value;
    return sum;
    }
#[no_mangle]
unsafe extern "C" fn test_arena_list_add_del(cnt: c_int, nonsleepable: bool) {
    static void test_arena_list_add_del(int cnt, bool nonsleepable)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    struct arena_list *skel;
    let mut expected_sum: c_int = (u64)cnt * (cnt - 1) / 2;
    int ret, sum;
    skel = arena_list__open();
    if (!ASSERT_OK_PTR(skel, "arena_list__open"))
    return;
    skel.rodata.nonsleepable = nonsleepable;
    ret = arena_list__load(skel);
    if (!ASSERT_OK(ret, "arena_list__load"))
    goto out;
    skel.bss.cnt = cnt;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.arena_list_add), &opts);
    ASSERT_OK(ret, "ret_add");
    ASSERT_OK(opts.retval, "retval");
    if (skel.bss.skip) {
    printf("%s:SKIP:compiler doesn't support arena_cast\n", __func__);
    test__skip();
    goto out;
    }
    sum = list_sum(skel.bss.list_head);
    ASSERT_EQ(sum, expected_sum, "sum of elems");
    ASSERT_EQ(skel.arena.arena_sum, expected_sum, "__arena sum of elems");
    ASSERT_EQ(skel.arena.test_val, cnt + 1, "num of elems");
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.arena_list_del), &opts);
    ASSERT_OK(ret, "ret_del");
    sum = list_sum(skel.bss.list_head);
    ASSERT_EQ(sum, 0, "sum of list elems after del");
    ASSERT_EQ(skel.bss.list_sum, expected_sum, "sum of list elems computed by prog");
    ASSERT_EQ(skel.arena.arena_sum, expected_sum, "__arena sum of elems");
    out:
    arena_list__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_arena_list() {
    void serial_test_arena_list(void)
    {
    if (test__start_subtest("arena_list_1"))
    test_arena_list_add_del(1, false);
    if (test__start_subtest("arena_list_1000"))
    test_arena_list_add_del(1000, false);
    if (test__start_subtest("arena_list_1_nonsleepable"))
    test_arena_list_add_del(1, true);
    if (test__start_subtest("arena_list_1000_nonsleepable"))
    test_arena_list_add_del(1000, true);
    }
