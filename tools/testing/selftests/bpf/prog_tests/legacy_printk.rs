//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/legacy_printk.c
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
unsafe extern "C" fn execute_one_variant(legacy: bool) -> c_int {
    static int execute_one_variant(bool legacy)
    {
    struct test_legacy_printk *skel;
    int err, zero = 0, my_pid = getpid(), res, map_fd;
    skel = test_legacy_printk__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return -errno;
    bpf_program__set_autoload(skel.progs.handle_legacy, legacy);
    bpf_program__set_autoload(skel.progs.handle_modern, !legacy);
    err = test_legacy_printk__load(skel);
// no ASSERT_OK, we expect one of two variants can fail here
    if (err)
    goto err_out;
    if (legacy) {
    map_fd = bpf_map__fd(skel.maps.my_pid_map);
    err = bpf_map_update_elem(map_fd, &zero, &my_pid, BPF_ANY);
    if (!ASSERT_OK(err, "my_pid_map_update"))
    goto err_out;
    err = bpf_map_lookup_elem(map_fd, &zero, &res);
    } else {
    skel.bss.my_pid_var = my_pid;
    }
    err = test_legacy_printk__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto err_out;
    usleep(1); /* trigger */
    if (legacy) {
    map_fd = bpf_map__fd(skel.maps.res_map);
    err = bpf_map_lookup_elem(map_fd, &zero, &res);
    if (!ASSERT_OK(err, "res_map_lookup"))
    goto err_out;
    } else {
    res = skel.bss.res_var;
    }
    if (!ASSERT_GT(res, 0, "res")) {
    err = -EINVAL;
    goto err_out;
    }
    err_out:
    test_legacy_printk__destroy(skel);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn test_legacy_printk() {
    void test_legacy_printk(void)
    {
// legacy variant should work everywhere
    ASSERT_OK(execute_one_variant(true /* legacy */), "legacy_case");
// execute modern variant, can fail the load on old kernels
    execute_one_variant(false);
    }
