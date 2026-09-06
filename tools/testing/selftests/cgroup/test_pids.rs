//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/cgroup/test_pids.c
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
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn run_success(cgroup: *const c_char, arg: *mut c_void) -> c_int {
    static int run_success(const char *cgroup, void *arg)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn run_pause(cgroup: *const c_char, arg: *mut c_void) -> c_int {
    static int run_pause(const char *cgroup, void *arg)
    {
    return pause();
    }
//
// This test checks that pids.max prevents forking new children above the
// specified limit in the cgroup.
//
#[no_mangle]
unsafe extern "C" fn test_pids_max(root: *const c_char) -> c_int {
    static int test_pids_max(const char *root)
    {
    let mut ret: c_int = KSFT_FAIL;
    char *cg_pids;
    int pid;
    cg_pids = cg_name(root, "pids_test");
    if (!cg_pids)
    goto cleanup;
    if (cg_create(cg_pids))
    goto cleanup;
    if (cg_read_strcmp(cg_pids, "pids.max", "max\n"))
    goto cleanup;
    if (cg_write(cg_pids, "pids.max", "2"))
    goto cleanup;
    if (cg_enter_current(cg_pids))
    goto cleanup;
    pid = cg_run_nowait(cg_pids, run_pause, core::ptr::null_mut());
    if (pid < 0)
    goto cleanup;
    if (cg_run_nowait(cg_pids, run_success, core::ptr::null_mut()) != -1 || errno != EAGAIN)
    goto cleanup;
    if (kill(pid, SIGINT))
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    cg_enter_current(root);
    cg_destroy(cg_pids);
    free(cg_pids);
    return ret;
    }
//
// This test checks that pids.events are counted in cgroup associated with pids.max
//
#[no_mangle]
unsafe extern "C" fn test_pids_events(root: *const c_char) -> c_int {
    static int test_pids_events(const char *root)
    {
    let mut ret: c_int = KSFT_FAIL;
    char *cg_parent = core::ptr::null_mut(), *cg_child = core::ptr::null_mut();
    int pid;
    if (cgroup_feature("pids_localevents") <= 0)
    return KSFT_SKIP;
    cg_parent = cg_name(root, "pids_parent");
    cg_child = cg_name(cg_parent, "pids_child");
    if (!cg_parent || !cg_child)
    goto cleanup;
    if (cg_create(cg_parent))
    goto cleanup;
    if (cg_write(cg_parent, "cgroup.subtree_control", "+pids"))
    goto cleanup;
    if (cg_create(cg_child))
    goto cleanup;
    if (cg_write(cg_parent, "pids.max", "2"))
    goto cleanup;
    if (cg_read_strcmp(cg_child, "pids.max", "max\n"))
    goto cleanup;
    if (cg_enter_current(cg_child))
    goto cleanup;
    pid = cg_run_nowait(cg_child, run_pause, core::ptr::null_mut());
    if (pid < 0)
    goto cleanup;
    if (cg_run_nowait(cg_child, run_success, core::ptr::null_mut()) != -1 || errno != EAGAIN)
    goto cleanup;
    if (kill(pid, SIGINT))
    goto cleanup;
    if (cg_read_key_long(cg_child, "pids.events", "max ") != 0)
    goto cleanup;
    if (cg_read_key_long(cg_parent, "pids.events", "max ") != 1)
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    cg_enter_current(root);
    if (cg_child)
    cg_destroy(cg_child);
    if (cg_parent)
    cg_destroy(cg_parent);
    free(cg_child);
    free(cg_parent);
    return ret;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pids_test {
    pub root): *const *const int (fn)(char,
    pub name: *const c_char,
    } tests[] = {
    T(test_pids_max),
    T(test_pids_events),
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    char root[PATH_MAX];
    ksft_print_header();
    if (cg_find_unified_root(root, sizeof(root), core::ptr::null_mut()))
    ksft_exit_skip("cgroup v2 isn't mounted\n");
//
// Check that pids controller is available:
// pids is listed in cgroup.controllers
//
    if (cg_read_strstr(root, "cgroup.controllers", "pids"))
    ksft_exit_skip("pids controller isn't available\n");
    if (cg_read_strstr(root, "cgroup.subtree_control", "pids"))
    if (cg_write(root, "cgroup.subtree_control", "+pids"))
    ksft_exit_skip("Failed to set pids controller\n");
    ksft_set_plan(ARRAY_SIZE(tests));
    for (int i = 0; i < ARRAY_SIZE(tests); i++) {
    switch (tests[i].fn(root)) {
    case KSFT_PASS:
    ksft_test_result_pass("%s\n", tests[i].name);
    break;
    case KSFT_SKIP:
    ksft_test_result_skip("%s\n", tests[i].name);
    break;
    default:
    ksft_test_result_fail("%s\n", tests[i].name);
    break;
    }
    }
    ksft_finished();
    }
