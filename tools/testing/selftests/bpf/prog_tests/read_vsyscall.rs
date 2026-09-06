//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/read_vsyscall.c
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
// Copyright (C) 2024. Huawei Technologies Co., Ltd

// For VSYSCALL_ADDR

// To prevent build failure on non-x86 arch

#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_ret_desc {
    pub name: *const c_char,
    pub ret: c_int,
    } all_read[] = {
    { .name = "probe_read_kernel", .ret = -ERANGE },
    { .name = "probe_read_kernel_str", .ret = -ERANGE },
    { .name = "probe_read", .ret = -ERANGE },
    { .name = "probe_read_str", .ret = -ERANGE },
    { .name = "probe_read_user", .ret = -EFAULT },
    { .name = "probe_read_user_str", .ret = -EFAULT },
    { .name = "copy_from_user", .ret = -EFAULT },
    { .name = "copy_from_user_task", .ret = -EFAULT },
    { .name = "copy_from_user_str", .ret = -EFAULT },
    { .name = "copy_from_user_task_str", .ret = -EFAULT },
}

#[no_mangle]
pub unsafe extern "C" fn test_read_vsyscall() {
    void test_read_vsyscall(void)
    {
    struct read_vsyscall *skel;
    unsigned int i;
    int err;

    test__skip();
    return;

    skel = read_vsyscall__open_and_load();
    if (!ASSERT_OK_PTR(skel, "read_vsyscall open_load"))
    return;
    skel.bss.target_pid = getpid();
    err = read_vsyscall__attach(skel);
    if (!ASSERT_EQ(err, 0, "read_vsyscall attach"))
    goto out;
// userspace may don't have vsyscall page due to LEGACY_VSYSCALL_NONE,
// but it doesn't affect the returned error codes.
//
    skel.bss.user_ptr = (void *)VSYSCALL_ADDR;
    usleep(1);
    for (i = 0; i < ARRAY_SIZE(all_read); i++)
    ASSERT_EQ(skel.bss.read_ret[i], all_read[i].ret, all_read[i].name);
    out:
    read_vsyscall__destroy(skel);
    }
