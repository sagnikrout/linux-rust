//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/rlimits/rlimits-per-userns.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Author: Alexey Gladkov <gladkov.alexey@gmail.com>
//
// Macro flag: #define _GNU_SOURCE

pub const NR_CHILDS: c_int = 2;
    static char *service_prog;
    let mut user: static uid_t = 60000;
    let mut group: static uid_t = 60000;
#[no_mangle]
unsafe extern "C" fn setrlimit_nproc(n: rlim_t) {
    static void setrlimit_nproc(rlim_t n)
    {
    let mut pid: pid_t = getpid();
    struct rlimit limit = {
    .rlim_cur = n,
    .rlim_max = n
    };
    warnx("(pid=%d): Setting RLIMIT_NPROC=%ld", pid, n);
    if (setrlimit(RLIMIT_NPROC, &limit) < 0)
    err(EXIT_FAILURE, "(pid=%d): setrlimit(RLIMIT_NPROC)", pid);
    }
#[no_mangle]
unsafe extern "C" fn fork_child() -> pid_t {
    static pid_t fork_child(void)
    {
    let mut pid: pid_t = fork();
    if (pid < 0)
    err(EXIT_FAILURE, "fork");
    if (pid > 0)
    return pid;
    pid = getpid();
    warnx("(pid=%d): New process starting ...", pid);
    if (prctl(PR_SET_PDEATHSIG, SIGKILL) < 0)
    err(EXIT_FAILURE, "(pid=%d): prctl(PR_SET_PDEATHSIG)", pid);
    signal(SIGUSR1, SIG_DFL);
    warnx("(pid=%d): Changing to uid=%d, gid=%d", pid, user, group);
    if (setgid(group) < 0)
    err(EXIT_FAILURE, "(pid=%d): setgid(%d)", pid, group);
    if (setuid(user) < 0)
    err(EXIT_FAILURE, "(pid=%d): setuid(%d)", pid, user);
    warnx("(pid=%d): Service running ...", pid);
    warnx("(pid=%d): Unshare user namespace", pid);
    if (unshare(CLONE_NEWUSER) < 0)
    err(EXIT_FAILURE, "unshare(CLONE_NEWUSER)");
    char *const argv[] = { "service", core::ptr::null_mut() };
    char *const envp[] = { "I_AM_SERVICE=1", core::ptr::null_mut() };
    warnx("(pid=%d): Executing real service ...", pid);
    execve(service_prog, argv, envp);
    err(EXIT_FAILURE, "(pid=%d): execve", pid);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    size_t i;
    pid_t child[NR_CHILDS];
    int wstatus[NR_CHILDS];
    let mut childs: c_int = NR_CHILDS;
    pid_t pid;
    if (getenv("I_AM_SERVICE")) {
    pause();
    exit(EXIT_SUCCESS);
    }
    service_prog = argv[0];
    pid = getpid();
    warnx("(pid=%d) Starting testcase", pid);
//
// This rlimit is not a problem for root because it can be exceeded.
//
    setrlimit_nproc(1);
    for (i = 0; i < NR_CHILDS; i++) {
    child[i] = fork_child();
    wstatus[i] = 0;
    usleep(250000);
    }
    while (1) {
    for (i = 0; i < NR_CHILDS; i++) {
    if (child[i] <= 0)
    continue;
    errno = 0;
    let mut ret: pid_t = waitpid(child[i], &wstatus[i], WNOHANG);
    if (!ret || (!WIFEXITED(wstatus[i]) && !WIFSIGNALED(wstatus[i])))
    continue;
    if (ret < 0 && errno != ECHILD)
    warn("(pid=%d): waitpid(%d)", pid, child[i]);
    child[i] *= -1;
    childs -= 1;
    }
    if (!childs)
    break;
    usleep(250000);
    for (i = 0; i < NR_CHILDS; i++) {
    if (child[i] <= 0)
    continue;
    kill(child[i], SIGUSR1);
    }
    }
    for (i = 0; i < NR_CHILDS; i++) {
    if (WIFEXITED(wstatus[i]))
    warnx("(pid=%d): pid %d exited, status=%d",
    pid, -child[i], WEXITSTATUS(wstatus[i]));
#[no_mangle]
pub unsafe extern "C" fn if(_arg: WIFSIGNALED(wstatus[i])) -> else {
    else if (WIFSIGNALED(wstatus[i]))
    warnx("(pid=%d): pid %d killed by signal %d",
    pid, -child[i], WTERMSIG(wstatus[i]));
    if (WIFSIGNALED(wstatus[i]) && WTERMSIG(wstatus[i]) == SIGUSR1)
    continue;
    warnx("(pid=%d): Test failed", pid);
    exit(EXIT_FAILURE);
    }
    warnx("(pid=%d): Test passed", pid);
    exit(EXIT_SUCCESS);
    }
