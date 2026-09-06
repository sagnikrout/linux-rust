//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_bprm_opts.c
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
// Copyright (C) 2020 Google LLC.
//

    static const char * const bash_envp[] = { "TMPDIR=shouldnotbeset", core::ptr::null_mut() };
#[no_mangle]
unsafe extern "C" fn update_storage(map_fd: c_int, secureexec: c_int) -> c_int {
    static int update_storage(int map_fd, int secureexec)
    {
    int task_fd, ret = 0;
    task_fd = sys_pidfd_open(getpid(), 0);
    if (task_fd < 0)
    return errno;
    ret = bpf_map_update_elem(map_fd, &task_fd, &secureexec, BPF_NOEXIST);
    if (ret)
    ret = errno;
    close(task_fd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn run_set_secureexec(map_fd: c_int, secureexec: c_int) -> c_int {
    static int run_set_secureexec(int map_fd, int secureexec)
    {
    int child_pid, child_status, ret, null_fd;
    child_pid = fork();
    if (child_pid == 0) {
    null_fd = open("/dev/null", O_WRONLY);
    if (null_fd == -1)
    exit(errno);
    dup2(null_fd, STDOUT_FILENO);
    dup2(null_fd, STDERR_FILENO);
    close(null_fd);
// Ensure that all executions from hereon are
// secure by setting a local storage which is read by
// the bprm_creds_for_exec hook and sets bprm->secureexec.
//
    ret = update_storage(map_fd, secureexec);
    if (ret)
    exit(ret);
// If the binary is executed with securexec=1, the dynamic
// loader ignores and unsets certain variables like LD_PRELOAD,
// TMPDIR etc. TMPDIR is used here to simplify the example, as
// LD_PRELOAD requires a real .so file.
//
// If the value of TMPDIR is set, the bash command returns 10
// and if the value is unset, it returns 20.
//
    execle("/bin/bash", "bash", "-c",
    "[[ -z \"${TMPDIR}\" ]] || exit 10 && exit 20", core::ptr::null_mut(),
    bash_envp);
    exit(errno);
    } else if (child_pid > 0) {
    waitpid(child_pid, &child_status, 0);
    ret = WEXITSTATUS(child_status);
// If a secureexec occurred, the exit status should be 20
    if (secureexec && ret == 20)
    return 0;
// If normal execution happened, the exit code should be 10
    if (!secureexec && ret == 10)
    return 0;
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn test_test_bprm_opts() {
    void test_test_bprm_opts(void)
    {
    int err, duration = 0;
    struct bprm_opts *skel = core::ptr::null_mut();
    skel = bprm_opts__open_and_load();
    if (CHECK(!skel, "skel_load", "skeleton failed\n"))
    goto close_prog;
    err = bprm_opts__attach(skel);
    if (CHECK(err, "attach", "attach failed: %d\n", err))
    goto close_prog;
// Run the test with the secureexec bit unset
    err = run_set_secureexec(bpf_map__fd(skel.maps.secure_exec_task_map),
    0 /* secureexec */);
    if (CHECK(err, "run_set_secureexec:0", "err = %d\n", err))
    goto close_prog;
// Run the test with the secureexec bit set
    err = run_set_secureexec(bpf_map__fd(skel.maps.secure_exec_task_map),
    1 /* secureexec */);
    if (CHECK(err, "run_set_secureexec:1", "err = %d\n", err))
    goto close_prog;
    close_prog:
    bprm_opts__destroy(skel);
    }
