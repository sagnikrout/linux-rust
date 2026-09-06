//! Automatically rewritten from C to Rust
//! Source: samples/pidfd/pidfd-metadata.c
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

pub const CLONE_PIDFD: c_uint = 0x00001000;

#[no_mangle]
unsafe extern "C" fn do_child(args: *mut c_void) -> c_int {
    static int do_child(void *args)
    {
    printf("%d\n", getpid());
    _exit(EXIT_SUCCESS);
    }
#[no_mangle]
unsafe extern "C" fn pidfd_clone(flags: c_int, pidfd: *mut c_int) -> pid_t {
    static pid_t pidfd_clone(int flags, int *pidfd)
    {
    let mut stack_size: usize = 1024;
    char *stack[1024] = { 0 };

    return __clone2(do_child, stack, stack_size, flags | SIGCHLD, core::ptr::null_mut(), pidfd);

    return clone(do_child, stack + stack_size, flags | SIGCHLD, core::ptr::null_mut(), pidfd);

    }
    static inline int sys_pidfd_send_signal(int pidfd, int sig, siginfo_t *info,
    unsigned int flags)
    {
    return syscall(__NR_pidfd_send_signal, pidfd, sig, info, flags);
    }
#[no_mangle]
unsafe extern "C" fn pidfd_metadata_fd(pid: pid_t, pidfd: c_int) -> c_int {
    static int pidfd_metadata_fd(pid_t pid, int pidfd)
    {
    int procfd, ret;
    char path[100];
    snprintf(path, sizeof(path), "/proc/%d", pid);
    procfd = open(path, O_DIRECTORY | O_RDONLY | O_CLOEXEC);
    if (procfd < 0) {
    warn("Failed to open %s\n", path);
    return -1;
    }
//
// Verify that the pid has not been recycled and our /proc/<pid> handle
// is still valid.
//
    ret = sys_pidfd_send_signal(pidfd, 0, core::ptr::null_mut(), 0);
    if (ret < 0) {
    switch (errno) {
    case EPERM:
// Process exists, just not allowed to signal it.
    break;
    default:
    warn("Failed to signal process\n");
    close(procfd);
    procfd = -1;
    }
    }
    return procfd;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut pidfd: c_int = -1, ret = EXIT_FAILURE;
    char buf[4096] = { 0 };
    pid_t pid;
    int procfd, statusfd;
    ssize_t bytes;
    pid = pidfd_clone(CLONE_PIDFD, &pidfd);
    if (pid < 0)
    err(ret, "CLONE_PIDFD");
    if (pidfd == -1) {
    warnx("CLONE_PIDFD is not supported by the kernel");
    goto out;
    }
    procfd = pidfd_metadata_fd(pid, pidfd);
    close(pidfd);
    if (procfd < 0)
    goto out;
    statusfd = openat(procfd, "status", O_RDONLY | O_CLOEXEC);
    close(procfd);
    if (statusfd < 0)
    goto out;
    bytes = read(statusfd, buf, sizeof(buf));
    if (bytes > 0)
    bytes = write(STDOUT_FILENO, buf, bytes);
    close(statusfd);
    ret = EXIT_SUCCESS;
    out:
    (void)wait(core::ptr::null_mut());
    exit(ret);
    }
