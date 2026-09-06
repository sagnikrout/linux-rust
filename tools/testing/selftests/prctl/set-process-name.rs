//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/prctl/set-process-name.c
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
// This test covers the PR_SET_NAME functionality of prctl calls
//

pub const TASK_COMM_LEN: c_int = 16;
pub const MAX_PATH_LEN: c_int = 50;
#[no_mangle]
pub unsafe extern "C" fn set_name(name: *mut c_char) -> c_int {
    int set_name(char *name)
    {
    int res;
    res = prctl(PR_SET_NAME, name, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if (res < 0)
    return -errno;
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn check_is_name_correct(check_name: *mut c_char) -> c_int {
    int check_is_name_correct(char *check_name)
    {
    char name[TASK_COMM_LEN];
    int res;
    res = prctl(PR_GET_NAME, name, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if (res < 0)
    return -errno;
    return !strcmp(name, check_name);
    }
#[no_mangle]
pub unsafe extern "C" fn check_null_pointer(check_name: *mut c_char) -> c_int {
    int check_null_pointer(char *check_name)
    {
    char *name = core::ptr::null_mut();
    int res;
    res = prctl(PR_GET_NAME, name, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn check_name() -> c_int {
    int check_name(void)
    {
    int pid;
    pid = getpid();
    FILE *fptr = core::ptr::null_mut();
    char path[MAX_PATH_LEN] = {};
    char name[TASK_COMM_LEN] = {};
    char output[TASK_COMM_LEN] = {};
    int j;
    j = snprintf(path, MAX_PATH_LEN, "/proc/self/task/%d/comm", pid);
    fptr = fopen(path, "r");
    if (!fptr)
    return -EIO;
    fscanf(fptr, "%s", output);
    if (ferror(fptr))
    return -EIO;
    let mut res: c_int = prctl(PR_GET_NAME, name, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if (res < 0)
    return -errno;
    return !strcmp(output, name);
    }
    TEST(rename_process) {
    EXPECT_GE(set_name(CHANGE_NAME), 0);
    EXPECT_TRUE(check_is_name_correct(CHANGE_NAME));
    EXPECT_GE(set_name(EMPTY_NAME), 0);
    EXPECT_TRUE(check_is_name_correct(EMPTY_NAME));
    EXPECT_GE(set_name(CHANGE_NAME), 0);
    EXPECT_LT(check_null_pointer(CHANGE_NAME), 0);
    EXPECT_TRUE(check_name());
    }
    TEST_HARNESS_MAIN
