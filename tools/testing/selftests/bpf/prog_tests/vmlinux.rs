//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/vmlinux.c
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

pub const MY_TV_NSEC: c_int = 1337;
#[no_mangle]
unsafe extern "C" fn nsleep() {
    static void nsleep()
    {
    let mut ts: timespec = { .tv_nsec = MY_TV_NSEC };
    (void)syscall(__NR_nanosleep, &ts, core::ptr::null_mut());
    }
    static const char *hrtimer_func = "hrtimer_start_range_ns";
#[no_mangle]
unsafe extern "C" fn setup_hrtimer_progs(skel: *mut test_vmlinux) -> c_int {
    static int setup_hrtimer_progs(struct test_vmlinux *skel)
    {
    int err;
    if (libbpf_find_vmlinux_btf_id("hrtimer_start_range_ns_user", BPF_TRACE_FENTRY) > 0)
    hrtimer_func = "hrtimer_start_range_ns_user";
    err = bpf_program__set_attach_target(skel.progs.handle__fentry, 0, hrtimer_func);
    if (err)
    return err;
//
// Bare SEC("kprobe") has no target function, so attach it manually
// later after selecting the hrtimer function to probe.
//
    bpf_program__set_autoattach(skel.progs.handle__kprobe, false);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_vmlinux() {
    void test_vmlinux(void)
    {
    int err;
    struct test_vmlinux* skel;
    struct test_vmlinux__bss *bss;
    struct bpf_link *kprobe_link = core::ptr::null_mut();
    skel = test_vmlinux__open();
    if (!ASSERT_OK_PTR(skel, "test_vmlinux__open"))
    return;
    err = setup_hrtimer_progs(skel);
    if (!ASSERT_OK(err, "setup_hrtimer_progs"))
    goto cleanup;
    err = test_vmlinux__load(skel);
    if (!ASSERT_OK(err, "test_vmlinux__load"))
    goto cleanup;
    bss = skel.bss;
    err = test_vmlinux__attach(skel);
    if (!ASSERT_OK(err, "test_vmlinux__attach"))
    goto cleanup;
// manually attach kprobe with the selected function
    if (hrtimer_func) {
    kprobe_link = bpf_program__attach_kprobe(skel.progs.handle__kprobe,
    false /* retprobe */, hrtimer_func);
    if (!ASSERT_OK_PTR(kprobe_link, "bpf_program__attach_kprobe"))
    goto cleanup;
    }
// trigger everything
    nsleep();
    ASSERT_TRUE(bss.tp_called, "tp");
    ASSERT_TRUE(bss.raw_tp_called, "raw_tp");
    ASSERT_TRUE(bss.tp_btf_called, "tp_btf");
    ASSERT_TRUE(bss.kprobe_called, "kprobe");
    ASSERT_TRUE(bss.fentry_called, "fentry");
    cleanup:
    bpf_link__destroy(kprobe_link);
    test_vmlinux__destroy(skel);
    }
