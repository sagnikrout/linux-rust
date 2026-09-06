//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/lsm/lsm_list_modules_test.c
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
//
// Linux Security Module infrastructure tests
// Tests for the lsm_list_modules system call
//
// Copyright © 2022 Casey Schaufler <casey@schaufler-ca.com>
//
// Macro flag: #define _GNU_SOURCE

    TEST(size_null_lsm_list_modules)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    __u64 *syscall_lsms = calloc(page_size, 1);
    ASSERT_NE(core::ptr::null_mut(), syscall_lsms);
    errno = 0;
    ASSERT_EQ(-1, lsm_list_modules(syscall_lsms, core::ptr::null_mut(), 0));
    ASSERT_EQ(EFAULT, errno);
    free(syscall_lsms);
    }
    TEST(ids_null_lsm_list_modules)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    let mut size: __u32 = page_size;
    errno = 0;
    ASSERT_EQ(-1, lsm_list_modules(core::ptr::null_mut(), &size, 0));
    ASSERT_EQ(EFAULT, errno);
    ASSERT_NE(1, size);
    }
    TEST(size_too_small_lsm_list_modules)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    __u64 *syscall_lsms = calloc(page_size, 1);
    let mut size: __u32 = 1;
    ASSERT_NE(core::ptr::null_mut(), syscall_lsms);
    errno = 0;
    ASSERT_EQ(-1, lsm_list_modules(syscall_lsms, &size, 0));
    ASSERT_EQ(E2BIG, errno);
    ASSERT_NE(1, size);
    free(syscall_lsms);
    }
    TEST(flags_set_lsm_list_modules)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    __u64 *syscall_lsms = calloc(page_size, 1);
    let mut size: __u32 = page_size;
    ASSERT_NE(core::ptr::null_mut(), syscall_lsms);
    errno = 0;
    ASSERT_EQ(-1, lsm_list_modules(syscall_lsms, &size, 7));
    ASSERT_EQ(EINVAL, errno);
    ASSERT_EQ(page_size, size);
    free(syscall_lsms);
    }
    TEST(correct_lsm_list_modules)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    let mut size: __u32 = page_size;
    __u64 *syscall_lsms = calloc(page_size, 1);
    char *sysfs_lsms = calloc(page_size, 1);
    char *name;
    char *cp;
    int count;
    int i;
    ASSERT_NE(core::ptr::null_mut(), sysfs_lsms);
    ASSERT_NE(core::ptr::null_mut(), syscall_lsms);
    ASSERT_EQ(0, read_sysfs_lsms(sysfs_lsms, page_size));
    count = lsm_list_modules(syscall_lsms, &size, 0);
    ASSERT_LE(1, count);
    cp = sysfs_lsms;
    for (i = 0; i < count; i++) {
    switch (syscall_lsms[i]) {
    case LSM_ID_CAPABILITY:
    name = "capability";
    break;
    case LSM_ID_SELINUX:
    name = "selinux";
    break;
    case LSM_ID_SMACK:
    name = "smack";
    break;
    case LSM_ID_TOMOYO:
    name = "tomoyo";
    break;
    case LSM_ID_APPARMOR:
    name = "apparmor";
    break;
    case LSM_ID_YAMA:
    name = "yama";
    break;
    case LSM_ID_LOADPIN:
    name = "loadpin";
    break;
    case LSM_ID_SAFESETID:
    name = "safesetid";
    break;
    case LSM_ID_LOCKDOWN:
    name = "lockdown";
    break;
    case LSM_ID_BPF:
    name = "bpf";
    break;
    case LSM_ID_LANDLOCK:
    name = "landlock";
    break;
    case LSM_ID_IMA:
    name = "ima";
    break;
    case LSM_ID_EVM:
    name = "evm";
    break;
    case LSM_ID_IPE:
    name = "ipe";
    break;
    default:
    name = "INVALID";
    break;
    }
    ASSERT_EQ(0, strncmp(cp, name, strlen(name)));
    cp += strlen(name) + 1;
    }
    free(sysfs_lsms);
    free(syscall_lsms);
    }
    TEST_HARNESS_MAIN
