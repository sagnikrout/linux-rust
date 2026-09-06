//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/ksyms.c
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
// Copyright (c) 2019 Facebook

#[no_mangle]
pub unsafe extern "C" fn test_ksyms() {
    void test_ksyms(void)
    {
    const char *btf_path = "/sys/kernel/btf/vmlinux";
    struct test_ksyms *skel;
    struct test_ksyms__data *data;
    __u64 link_fops_addr, per_cpu_start_addr;
    struct stat st;
    __u64 btf_size;
    int err;
    err = kallsyms_find("bpf_link_fops", &link_fops_addr);
    if (!ASSERT_NEQ(err, -EINVAL, "bpf_link_fops: kallsyms_fopen"))
    return;
    if (!ASSERT_NEQ(err, -ENOENT, "bpf_link_fops: ksym_find"))
    return;
    err = kallsyms_find("__per_cpu_start", &per_cpu_start_addr);
    if (!ASSERT_NEQ(err, -EINVAL, "__per_cpu_start: kallsyms_fopen"))
    return;
    if (!ASSERT_NEQ(err, -ENOENT, "__per_cpu_start: ksym_find"))
    return;
    if (!ASSERT_OK(stat(btf_path, &st), "stat_btf"))
    return;
    btf_size = st.st_size;
    skel = test_ksyms__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_ksyms__open_and_load"))
    return;
    err = test_ksyms__attach(skel);
    if (!ASSERT_OK(err, "test_ksyms__attach"))
    goto cleanup;
// trigger tracepoint
    usleep(1);
    data = skel.data;
    ASSERT_EQ(data.out__bpf_link_fops, link_fops_addr, "bpf_link_fops");
    ASSERT_EQ(data.out__bpf_link_fops1, 0, "bpf_link_fops1");
    ASSERT_EQ(data.out__btf_size, btf_size, "btf_size");
    ASSERT_EQ(data.out__per_cpu_start, per_cpu_start_addr, "__per_cpu_start");
    cleanup:
    test_ksyms__destroy(skel);
    }
