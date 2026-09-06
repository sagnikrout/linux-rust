//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_struct_ops_id_ops_mapping.c
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

#[no_mangle]
unsafe extern "C" fn test_st_ops_id_ops_mapping() {
    static void test_st_ops_id_ops_mapping(void)
    {
    struct struct_ops_id_ops_mapping1 *skel1 = core::ptr::null_mut();
    struct struct_ops_id_ops_mapping2 *skel2 = core::ptr::null_mut();
    let mut info: bpf_map_info = {};
    let mut len: __u32 = sizeof(info);
    int err, pid, prog1_fd, prog2_fd;
    skel1 = struct_ops_id_ops_mapping1__open_and_load();
    if (!ASSERT_OK_PTR(skel1, "struct_ops_id_ops_mapping1__open"))
    goto out;
    skel2 = struct_ops_id_ops_mapping2__open_and_load();
    if (!ASSERT_OK_PTR(skel2, "struct_ops_id_ops_mapping2__open"))
    goto out;
    err = bpf_map_get_info_by_fd(bpf_map__fd(skel1.maps.st_ops_map),
    &info, &len);
    if (!ASSERT_OK(err, "bpf_map_get_info_by_fd"))
    goto out;
    skel1.bss.st_ops_id = info.id;
    err = bpf_map_get_info_by_fd(bpf_map__fd(skel2.maps.st_ops_map),
    &info, &len);
    if (!ASSERT_OK(err, "bpf_map_get_info_by_fd"))
    goto out;
    skel2.bss.st_ops_id = info.id;
    err = struct_ops_id_ops_mapping1__attach(skel1);
    if (!ASSERT_OK(err, "struct_ops_id_ops_mapping1__attach"))
    goto out;
    err = struct_ops_id_ops_mapping2__attach(skel2);
    if (!ASSERT_OK(err, "struct_ops_id_ops_mapping2__attach"))
    goto out;
// run tracing prog that calls .test_1 and checks return
    pid = getpid();
    skel1.bss.test_pid = pid;
    skel2.bss.test_pid = pid;
    sys_gettid();
    skel1.bss.test_pid = 0;
    skel2.bss.test_pid = 0;
// run syscall_prog that calls .test_1 and checks return
    prog1_fd = bpf_program__fd(skel1.progs.syscall_prog);
    err = bpf_prog_test_run_opts(prog1_fd, core::ptr::null_mut());
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    prog2_fd = bpf_program__fd(skel2.progs.syscall_prog);
    err = bpf_prog_test_run_opts(prog2_fd, core::ptr::null_mut());
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    ASSERT_EQ(skel1.bss.test_err, 0, "skel1.bss.test_err");
    ASSERT_EQ(skel2.bss.test_err, 0, "skel2.bss.test_err");
    out:
    struct_ops_id_ops_mapping1__destroy(skel1);
    struct_ops_id_ops_mapping2__destroy(skel2);
    }
#[no_mangle]
pub unsafe extern "C" fn test_struct_ops_id_ops_mapping() {
    void test_struct_ops_id_ops_mapping(void)
    {
    if (test__start_subtest("st_ops_id_ops_mapping"))
    test_st_ops_id_ops_mapping();
    }
