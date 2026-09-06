//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/nsfs/owner.c
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

pub const NSIO: c_uint = 0xb7;

    ({ \
    fprintf(stderr, "%s:%d:" fmt ": %m\n", \
    __func__, __LINE__, ##__VA_ARGS__); \
    1; \
    })
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argvp[]: *mut c_char) -> c_int {
    int main(int argc, char *argvp[])
    {
    int pfd[2], ns, uns, init_uns;
    struct stat st1, st2;
    char path[128];
    pid_t pid;
    char c;
    if (pipe(pfd))
    return 1;
    pid = fork();
    if (pid < 0)
    return pr_err("fork");
    if (pid == 0) {
    prctl(PR_SET_PDEATHSIG, SIGKILL);
    if (unshare(CLONE_NEWUTS | CLONE_NEWUSER))
    return pr_err("unshare");
    close(pfd[0]);
    close(pfd[1]);
    while (1)
    sleep(1);
    return 0;
    }
    close(pfd[1]);
    if (read(pfd[0], &c, 1) != 0)
    return pr_err("Unable to read from pipe");
    close(pfd[0]);
    snprintf(path, sizeof(path), "/proc/%d/ns/uts", pid);
    ns = open(path, O_RDONLY);
    if (ns < 0)
    return pr_err("Unable to open %s", path);
    uns = ioctl(ns, NS_GET_USERNS);
    if (uns < 0)
    return pr_err("Unable to get an owning user namespace");
    if (fstat(uns, &st1))
    return pr_err("fstat");
    snprintf(path, sizeof(path), "/proc/%d/ns/user", pid);
    if (stat(path, &st2))
    return pr_err("stat");
    if (st1.st_ino != st2.st_ino)
    return pr_err("NS_GET_USERNS returned a wrong namespace");
    init_uns = ioctl(uns, NS_GET_USERNS);
    if (uns < 0)
    return pr_err("Unable to get an owning user namespace");
    if (ioctl(init_uns, NS_GET_USERNS) >= 0 || errno != EPERM)
    return pr_err("Don't get EPERM");
    if (unshare(CLONE_NEWUSER))
    return pr_err("unshare");
    if (ioctl(ns, NS_GET_USERNS) >= 0 || errno != EPERM)
    return pr_err("Don't get EPERM");
    if (ioctl(init_uns, NS_GET_USERNS) >= 0 || errno != EPERM)
    return pr_err("Don't get EPERM");
    kill(pid, SIGKILL);
    wait(core::ptr::null_mut());
    return 0;
    }
