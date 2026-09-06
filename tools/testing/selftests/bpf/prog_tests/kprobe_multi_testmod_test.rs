//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/kprobe_multi_testmod_test.c
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

    static struct ksyms *ksyms;
#[no_mangle]
unsafe extern "C" fn kprobe_multi_testmod_check(skel: *mut kprobe_multi) {
    static void kprobe_multi_testmod_check(struct kprobe_multi *skel)
    {
    ASSERT_EQ(skel.bss.kprobe_testmod_test1_result, 1, "kprobe_test1_result");
    ASSERT_EQ(skel.bss.kprobe_testmod_test2_result, 1, "kprobe_test2_result");
    ASSERT_EQ(skel.bss.kprobe_testmod_test3_result, 1, "kprobe_test3_result");
    ASSERT_EQ(skel.bss.kretprobe_testmod_test1_result, 1, "kretprobe_test1_result");
    ASSERT_EQ(skel.bss.kretprobe_testmod_test2_result, 1, "kretprobe_test2_result");
    ASSERT_EQ(skel.bss.kretprobe_testmod_test3_result, 1, "kretprobe_test3_result");
    }
#[no_mangle]
unsafe extern "C" fn test_testmod_attach_api(opts: *mut bpf_kprobe_multi_opts) {
    static void test_testmod_attach_api(struct bpf_kprobe_multi_opts *opts)
    {
    struct kprobe_multi *skel = core::ptr::null_mut();
    skel = kprobe_multi__open_and_load();
    if (!ASSERT_OK_PTR(skel, "fentry_raw_skel_load"))
    return;
    skel.bss.pid = getpid();
    skel.links.test_kprobe_testmod = bpf_program__attach_kprobe_multi_opts(
    skel.progs.test_kprobe_testmod,
    core::ptr::null_mut(), opts);
    if (!skel.links.test_kprobe_testmod)
    goto cleanup;
    opts.retprobe = true;
    skel.links.test_kretprobe_testmod = bpf_program__attach_kprobe_multi_opts(
    skel.progs.test_kretprobe_testmod,
    core::ptr::null_mut(), opts);
    if (!skel.links.test_kretprobe_testmod)
    goto cleanup;
    ASSERT_OK(trigger_module_test_read(1), "trigger_read");
    kprobe_multi_testmod_check(skel);
    cleanup:
    kprobe_multi__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_testmod_attach_api_addrs() {
    static void test_testmod_attach_api_addrs(void)
    {
    LIBBPF_OPTS(bpf_kprobe_multi_opts, opts);
    unsigned long long addrs[3];
    addrs[0] = ksym_get_addr_local(ksyms, "bpf_testmod_fentry_test1");
    ASSERT_NEQ(addrs[0], 0, "ksym_get_addr_local");
    addrs[1] = ksym_get_addr_local(ksyms, "bpf_testmod_fentry_test2");
    ASSERT_NEQ(addrs[1], 0, "ksym_get_addr_local");
    addrs[2] = ksym_get_addr_local(ksyms, "bpf_testmod_fentry_test3");
    ASSERT_NEQ(addrs[2], 0, "ksym_get_addr_local");
    opts.addrs = (const unsigned long *) addrs;
    opts.cnt = ARRAY_SIZE(addrs);
    test_testmod_attach_api(&opts);
    }
#[no_mangle]
unsafe extern "C" fn test_testmod_attach_api_syms() {
    static void test_testmod_attach_api_syms(void)
    {
    LIBBPF_OPTS(bpf_kprobe_multi_opts, opts);
    const char *syms[3] = {
    "bpf_testmod_fentry_test1",
    "bpf_testmod_fentry_test2",
    "bpf_testmod_fentry_test3",
    };
    opts.syms = syms;
    opts.cnt = ARRAY_SIZE(syms);
    test_testmod_attach_api(&opts);
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_kprobe_multi_testmod_test() {
    void serial_test_kprobe_multi_testmod_test(void)
    {
    ksyms = load_kallsyms_local();
    if (!ASSERT_OK_PTR(ksyms, "load_kallsyms_local"))
    return;
    if (test__start_subtest("testmod_attach_api_syms"))
    test_testmod_attach_api_syms();
    if (test__start_subtest("testmod_attach_api_addrs"))
    test_testmod_attach_api_addrs();
    free_kallsyms_local(ksyms);
    }
