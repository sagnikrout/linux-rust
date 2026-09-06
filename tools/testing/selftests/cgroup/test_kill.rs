//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/cgroup/test_kill.c
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
// Kill the given cgroup and wait for the inotify signal.
// If there are no events in 10 seconds, treat this as an error.
// Then check that the cgroup is in the desired state.
//
#[no_mangle]
unsafe extern "C" fn cg_kill_wait(cgroup: *const c_char) -> c_int {
    static int cg_kill_wait(const char *cgroup)
    {
    int fd, ret = -1;
    fd = cg_prepare_for_wait(cgroup);
    if (fd < 0)
    return fd;
    ret = cg_write(cgroup, "cgroup.kill", "1");
    if (ret)
    goto out;
    ret = cg_wait_for(fd);
    if (ret)
    goto out;
    out:
    close(fd);
    return ret;
    }
//
// A simple process running in a sleep loop until being
// re-parented.
//
#[no_mangle]
unsafe extern "C" fn child_fn(cgroup: *const c_char, arg: *mut c_void) -> c_int {
    static int child_fn(const char *cgroup, void *arg)
    {
    let mut ppid: c_int = getppid();
    while (getppid() == ppid)
    usleep(1000);
    return getppid() == ppid;
    }
#[no_mangle]
unsafe extern "C" fn test_cgkill_simple(root: *const c_char) -> c_int {
    static int test_cgkill_simple(const char *root)
    {
    pid_t pids[100];
    let mut ret: c_int = KSFT_FAIL;
    char *cgroup = core::ptr::null_mut();
    int i;
    cgroup = cg_name(root, "cg_test_simple");
    if (!cgroup)
    goto cleanup;
    if (cg_create(cgroup))
    goto cleanup;
    for (i = 0; i < 100; i++)
    pids[i] = cg_run_nowait(cgroup, child_fn, core::ptr::null_mut());
    if (cg_wait_for_proc_count(cgroup, 100))
    goto cleanup;
    if (cg_read_strcmp(cgroup, "cgroup.events", "populated 1\n"))
    goto cleanup;
    if (cg_kill_wait(cgroup))
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    for (i = 0; i < 100; i++)
    wait_for_pid(pids[i]);
    if (ret == KSFT_PASS &&
    cg_read_strcmp_wait(cgroup, "cgroup.events", "populated 0\n"))
    ret = KSFT_FAIL;
    if (cgroup)
    cg_destroy(cgroup);
    free(cgroup);
    return ret;
    }
//
// The test creates the following hierarchy:
// A
// / / \ \
// B  E  I K
// /\  |
// C  D F
// |
// G
// |
// H
//
// with a process in C, H and 3 processes in K.
// Then it tries to kill the whole tree.
//
#[no_mangle]
unsafe extern "C" fn test_cgkill_tree(root: *const c_char) -> c_int {
    static int test_cgkill_tree(const char *root)
    {
    pid_t pids[5];
    char *cgroup[10] = {0};
    let mut ret: c_int = KSFT_FAIL;
    int i;
    cgroup[0] = cg_name(root, "cg_test_tree_A");
    if (!cgroup[0])
    goto cleanup;
    cgroup[1] = cg_name(cgroup[0], "B");
    if (!cgroup[1])
    goto cleanup;
    cgroup[2] = cg_name(cgroup[1], "C");
    if (!cgroup[2])
    goto cleanup;
    cgroup[3] = cg_name(cgroup[1], "D");
    if (!cgroup[3])
    goto cleanup;
    cgroup[4] = cg_name(cgroup[0], "E");
    if (!cgroup[4])
    goto cleanup;
    cgroup[5] = cg_name(cgroup[4], "F");
    if (!cgroup[5])
    goto cleanup;
    cgroup[6] = cg_name(cgroup[5], "G");
    if (!cgroup[6])
    goto cleanup;
    cgroup[7] = cg_name(cgroup[6], "H");
    if (!cgroup[7])
    goto cleanup;
    cgroup[8] = cg_name(cgroup[0], "I");
    if (!cgroup[8])
    goto cleanup;
    cgroup[9] = cg_name(cgroup[0], "K");
    if (!cgroup[9])
    goto cleanup;
    for (i = 0; i < 10; i++)
    if (cg_create(cgroup[i]))
    goto cleanup;
    pids[0] = cg_run_nowait(cgroup[2], child_fn, core::ptr::null_mut());
    pids[1] = cg_run_nowait(cgroup[7], child_fn, core::ptr::null_mut());
    pids[2] = cg_run_nowait(cgroup[9], child_fn, core::ptr::null_mut());
    pids[3] = cg_run_nowait(cgroup[9], child_fn, core::ptr::null_mut());
    pids[4] = cg_run_nowait(cgroup[9], child_fn, core::ptr::null_mut());
//
// Wait until all child processes will enter
// corresponding cgroups.
//
    if (cg_wait_for_proc_count(cgroup[2], 1) ||
    cg_wait_for_proc_count(cgroup[7], 1) ||
    cg_wait_for_proc_count(cgroup[9], 3))
    goto cleanup;
//
// Kill A and check that we get an empty notification.
//
    if (cg_kill_wait(cgroup[0]))
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    for (i = 0; i < 5; i++)
    wait_for_pid(pids[i]);
    if (ret == KSFT_PASS &&
    cg_read_strcmp_wait(cgroup[0], "cgroup.events",
    "populated 0\n"))
    ret = KSFT_FAIL;
    for (i = 9; i >= 0 && cgroup[i]; i--) {
    cg_destroy(cgroup[i]);
    free(cgroup[i]);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn forkbomb_fn(cgroup: *const c_char, arg: *mut c_void) -> c_int {
    static int forkbomb_fn(const char *cgroup, void *arg)
    {
    int ppid;
    fork();
    fork();
    ppid = getppid();
    while (getppid() == ppid)
    usleep(1000);
    return getppid() == ppid;
    }
//
// The test runs a fork bomb in a cgroup and tries to kill it.
//
#[no_mangle]
unsafe extern "C" fn test_cgkill_forkbomb(root: *const c_char) -> c_int {
    static int test_cgkill_forkbomb(const char *root)
    {
    let mut ret: c_int = KSFT_FAIL;
    char *cgroup = core::ptr::null_mut();
    let mut pid: pid_t = -ESRCH;
    cgroup = cg_name(root, "cg_forkbomb_test");
    if (!cgroup)
    goto cleanup;
    if (cg_create(cgroup))
    goto cleanup;
    pid = cg_run_nowait(cgroup, forkbomb_fn, core::ptr::null_mut());
    if (pid < 0)
    goto cleanup;
    usleep(100000);
    if (cg_kill_wait(cgroup))
    goto cleanup;
    if (cg_wait_for_proc_count(cgroup, 0))
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    if (pid > 0)
    wait_for_pid(pid);
    if (ret == KSFT_PASS &&
    cg_read_strcmp_wait(cgroup, "cgroup.events", "populated 0\n"))
    ret = KSFT_FAIL;
    if (cgroup)
    cg_destroy(cgroup);
    free(cgroup);
    return ret;
    }
//
// Test that a cgroup that was killed in the past can still be the target
// of clone3(CLONE_INTO_CGROUP): writing cgroup.kill must only kill the
// tasks in the cgroup at the time of the write, not tasks cloned into
// it afterwards.
//
#[no_mangle]
unsafe extern "C" fn test_cgkill_clone_into_killed(root: *const c_char) -> c_int {
    static int test_cgkill_clone_into_killed(const char *root)
    {
    pid_t pid;
    let mut cgroup_fd: c_int = -EBADF;
    let mut ret: c_int = KSFT_FAIL;
    char *cgroup = core::ptr::null_mut();
    cgroup = cg_name(root, "cg_test_clone_into_killed");
    if (!cgroup)
    goto cleanup;
    if (cg_create(cgroup))
    goto cleanup;
// Kill the cgroup while it is still empty.
    if (cg_write(cgroup, "cgroup.kill", "1"))
    goto cleanup;
    cgroup_fd = dirfd_open_opath(cgroup);
    if (cgroup_fd < 0)
    goto cleanup;
    pid = clone_into_cgroup(cgroup_fd);
    if (pid < 0) {
    if (errno == ENOSYS)
    ret = KSFT_SKIP;
    goto cleanup;
    }
    if (pid == 0)
    exit(EXIT_SUCCESS);
// The child must not be SIGKILLed; it has to exit cleanly.
    if (clone_reap(pid, WEXITED) != EXIT_SUCCESS)
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    if (cgroup_fd >= 0)
    close(cgroup_fd);
    if (cgroup)
    cg_destroy(cgroup);
    free(cgroup);
    return ret;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgkill_test {
    pub root): *const *const int (fn)(char,
    pub name: *const c_char,
    } tests[] = {
    T(test_cgkill_simple),
    T(test_cgkill_tree),
    T(test_cgkill_forkbomb),
    T(test_cgkill_clone_into_killed),
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    char root[PATH_MAX];
    int i;
    ksft_print_header();
    if (cg_find_unified_root(root, sizeof(root), core::ptr::null_mut()))
    ksft_exit_skip("cgroup v2 isn't mounted\n");
    ksft_set_plan(ARRAY_SIZE(tests));
    for (i = 0; i < ARRAY_SIZE(tests); i++) {
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
