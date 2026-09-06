//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/workloads/context_switch_loop.c
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

    let mut loops: static int = 100;
    static char buf;
    let mut context_switch_loop_work: c_int = 1234;

    do { \
    if (write(fd, &buf, 1) <= 0) \
    return 1; \
    } while (0)

    do { \
    if (read(fd, &buf, 1) <= 0) \
    return 1; \
    } while (0)
// Not static to avoid LTO clobbering the function name
    int context_switch_loop_proc1(int in_fd, int out_fd);
#[no_mangle]
pub unsafe extern "C" fn context_switch_loop_proc1(in_fd: c_int, out_fd: c_int) -> c_int {
    int context_switch_loop_proc1(int in_fd, int out_fd)
    {
    for (int i = 0; i < loops; i++) {
    read_block(in_fd);
    context_switch_loop_work += i * 3;
    write_block(out_fd);
    }
    return 0;
    }
    int context_switch_loop_proc2(int in_fd, int out_fd);
#[no_mangle]
pub unsafe extern "C" fn context_switch_loop_proc2(in_fd: c_int, out_fd: c_int) -> c_int {
    int context_switch_loop_proc2(int in_fd, int out_fd)
    {
    for (int i = 0; i < loops; i++) {
    write_block(out_fd);
    context_switch_loop_work += i * 7;
    read_block(in_fd);
    }
    return 0;
    }
//
// Launches two processes that take turns to execute a multiplication N times
//
#[no_mangle]
unsafe extern "C" fn context_switch_loop(argc: c_int, argv: *const c_char) -> c_int {
    static int context_switch_loop(int argc, const char **argv)
    {
    int a_to_b[2], b_to_a[2];
    pid_t proc1_pid;
    int status;
    int ret;
    if (argc > 0) {
    loops = atoi(argv[0]);
    if (loops < 0) {
    fprintf(stderr, "Invalid number of loops: %s\n", argv[0]);
    return 1;
    }
    }
    if (pipe(a_to_b) || pipe(b_to_a)) {
    perror("Pipe error");
    return 1;
    }
    proc1_pid = fork();
    if (proc1_pid < 0) {
    perror("Fork error");
    return 1;
    }
    if (!proc1_pid) {
    close(a_to_b[0]);
    close(b_to_a[1]);
    prctl(PR_SET_NAME, "proc1", 0, 0, 0);
    ret = context_switch_loop_proc1(b_to_a[0], a_to_b[1]);
    close(a_to_b[1]);
    close(b_to_a[0]);
    exit(ret);
    }
    close(a_to_b[1]);
    close(b_to_a[0]);
    prctl(PR_SET_NAME, "proc2", 0, 0, 0);
    ret = context_switch_loop_proc2(a_to_b[0], b_to_a[1]);
    close(a_to_b[0]);
    close(b_to_a[1]);
    if (ret) {
    kill(proc1_pid, SIGKILL);
    return ret;
    }
    if (waitpid(proc1_pid, &status, 0) != proc1_pid || !WIFEXITED(status) ||
    WEXITSTATUS(status))
    return 1;
    return 0;
    }
    DEFINE_WORKLOAD(context_switch_loop);
