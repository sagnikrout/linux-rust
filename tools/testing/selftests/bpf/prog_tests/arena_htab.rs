//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/arena_htab.c
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
unsafe extern "C" fn test_arena_htab_common(htab: *mut htab) {
    static void test_arena_htab_common(struct htab *htab)
    {
    int i;
    printf("htab %p buckets %p n_buckets %d\n", htab, htab.buckets, htab.n_buckets);
    ASSERT_OK_PTR(htab.buckets, "htab.buckets shouldn't be core::ptr::null_mut()");
    for (i = 0; htab.buckets && i < 16; i += 4) {
//
// Walk htab buckets and link lists since all pointers are correct,
// though they were written by bpf program.
//
    let mut val: c_int = htab_lookup_elem(htab, i);
    ASSERT_EQ(i, val, "key == value");
    }
    }
#[no_mangle]
unsafe extern "C" fn test_arena_htab_llvm() {
    static void test_arena_htab_llvm(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    struct arena_htab *skel;
    struct htab *htab;
    size_t arena_sz;
    void *area;
    int ret;
    skel = arena_htab__open_and_load();
    if (!ASSERT_OK_PTR(skel, "arena_htab__open_and_load"))
    return;
    area = bpf_map__initial_value(skel.maps.arena, &arena_sz);
// fault-in a page with pgoff == 0 as sanity check
// (volatile int *)area = 0x55aa;
// bpf prog will allocate more pages
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.arena_htab_llvm), &opts);
    ASSERT_OK(ret, "ret");
    ASSERT_OK(opts.retval, "retval");
    if (skel.bss.skip) {
    printf("%s:SKIP:compiler doesn't support arena_cast\n", __func__);
    test__skip();
    goto out;
    }
    htab = skel.bss.htab_for_user;
    test_arena_htab_common(htab);
    out:
    arena_htab__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_arena_htab_asm() {
    static void test_arena_htab_asm(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    struct arena_htab_asm *skel;
    struct htab *htab;
    int ret;
    skel = arena_htab_asm__open_and_load();
    if (!ASSERT_OK_PTR(skel, "arena_htab_asm__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.arena_htab_asm), &opts);
    ASSERT_OK(ret, "ret");
    ASSERT_OK(opts.retval, "retval");
    htab = skel.bss.htab_for_user;
    test_arena_htab_common(htab);
    arena_htab_asm__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_arena_htab() {
    void serial_test_arena_htab(void)
    {
    if (test__start_subtest("arena_htab_llvm"))
    test_arena_htab_llvm();
    if (test__start_subtest("arena_htab_asm"))
    test_arena_htab_asm();
    }
