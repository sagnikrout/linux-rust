//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/timer_mim.c
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
// Copyright (c) 2021 Facebook

#[no_mangle]
unsafe extern "C" fn timer_mim(timer_skel: *mut timer_mim) -> c_int {
    static int timer_mim(struct timer_mim *timer_skel)
    {
    __u64 cnt1, cnt2;
    int err, prog_fd, key1 = 1;
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    err = timer_mim__attach(timer_skel);
    if (!ASSERT_OK(err, "timer_attach"))
    return err;
    prog_fd = bpf_program__fd(timer_skel.progs.test1);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 0, "test_run");
    timer_mim__detach(timer_skel);
// check that timer_cb[12] are incrementing 'cnt'
    cnt1 = READ_ONCE(timer_skel.bss.cnt);
    for (int i = 0; i < 100; i++) {
    cnt2 = READ_ONCE(timer_skel.bss.cnt);
    if (cnt2 != cnt1)
    break;
    usleep(200); /* 100 times more than interval */
    }
    ASSERT_GT(cnt2, cnt1, "cnt");
    ASSERT_EQ(timer_skel.bss.err, 0, "err");
// check that code paths completed
    ASSERT_EQ(timer_skel.bss.ok, 1 | 2, "ok");
    close(bpf_map__fd(timer_skel.maps.inner_htab));
    err = bpf_map__delete_elem(timer_skel.maps.outer_arr, &key1, sizeof(key1), 0);
    ASSERT_EQ(err, 0, "delete inner map");
// check that timer_cb[12] are no longer running
    cnt1 = READ_ONCE(timer_skel.bss.cnt);
    for (int i = 0; i < 100; i++) {
    usleep(200); /* 100 times more than interval */
    cnt2 = READ_ONCE(timer_skel.bss.cnt);
    if (cnt2 == cnt1)
    break;
    }
    ASSERT_EQ(cnt2, cnt1, "cnt");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_timer_mim() {
    void serial_test_timer_mim(void)
    {
    struct timer_mim_reject *timer_reject_skel = core::ptr::null_mut();
    let mut old_print_fn: libbpf_print_fn_t = core::ptr::null_mut();
    struct timer_mim *timer_skel = core::ptr::null_mut();
    int err;
    old_print_fn = libbpf_set_print(core::ptr::null_mut());
    timer_reject_skel = timer_mim_reject__open_and_load();
    libbpf_set_print(old_print_fn);
    if (!ASSERT_ERR_PTR(timer_reject_skel, "timer_reject_skel_load"))
    goto cleanup;
    timer_skel = timer_mim__open_and_load();
    if (!timer_skel && errno == EOPNOTSUPP) {
    test__skip();
    return;
    }
    if (!ASSERT_OK_PTR(timer_skel, "timer_skel_load"))
    goto cleanup;
    err = timer_mim(timer_skel);
    ASSERT_OK(err, "timer_mim");
    cleanup:
    timer_mim__destroy(timer_skel);
    timer_mim_reject__destroy(timer_reject_skel);
    }
