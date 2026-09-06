//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/kfunc_module_order.c
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

    static int test_run_prog(const struct bpf_program *prog,
    struct bpf_test_run_opts *opts)
    {
    int err;
    err = bpf_prog_test_run_opts(bpf_program__fd(prog), opts);
    if (!ASSERT_OK(err, "bpf_prog_test_run_opts"))
    return err;
    if (!ASSERT_EQ((int)opts.retval, 0, bpf_program__name(prog)))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_kfunc_module_order() {
    void test_kfunc_module_order(void)
    {
    struct kfunc_module_order *skel;
    char pkt_data[64] = {};
    let mut err: c_int = 0;
    DECLARE_LIBBPF_OPTS(bpf_test_run_opts, test_opts, .data_in = pkt_data,
    .data_size_in = sizeof(pkt_data));
    err = load_module("bpf_test_modorder_x.ko",
    env_verbosity > VERBOSE_NONE);
    if (!ASSERT_OK(err, "load bpf_test_modorder_x.ko"))
    return;
    err = load_module("bpf_test_modorder_y.ko",
    env_verbosity > VERBOSE_NONE);
    if (!ASSERT_OK(err, "load bpf_test_modorder_y.ko"))
    goto exit_modx;
    skel = kfunc_module_order__open_and_load();
    if (!ASSERT_OK_PTR(skel, "kfunc_module_order__open_and_load()")) {
    err = -EINVAL;
    goto exit_mods;
    }
    test_run_prog(skel.progs.call_kfunc_xy, &test_opts);
    test_run_prog(skel.progs.call_kfunc_yx, &test_opts);
    kfunc_module_order__destroy(skel);
    exit_mods:
    unload_module("bpf_test_modorder_y", env_verbosity > VERBOSE_NONE);
    exit_modx:
    unload_module("bpf_test_modorder_x", env_verbosity > VERBOSE_NONE);
    }
