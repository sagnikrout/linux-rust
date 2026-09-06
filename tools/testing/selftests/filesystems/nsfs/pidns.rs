//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/nsfs/pidns.c
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

    ({ \
    fprintf(stderr, "%s:%d:" fmt ": %m\n", \
    __func__, __LINE__, ##__VA_ARGS__); \
    1; \
    })
pub const NSIO: c_uint = 0xb7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cr_clone_arg {
    pub __stack_aligned__: char stack[128],
    pub stack_ptr: [c_char; ],
}

#[no_mangle]
unsafe extern "C" fn child(args: *mut c_void) -> c_int {
    static int child(void *args)
    {
    prctl(PR_SET_PDEATHSIG, SIGKILL);
    while (1)
    sleep(1);
    exit(0);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    char *ns_strs[] = {"pid", "user"};
    char path[] = "/proc/0123456789/ns/pid";
    struct cr_clone_arg ca;
    struct stat st1, st2;
    int ns, pns, i;
    pid_t pid;
    pid = clone(child, ca.stack_ptr, CLONE_NEWUSER | CLONE_NEWPID | SIGCHLD, core::ptr::null_mut());
    if (pid < 0)
    return pr_err("clone");
    for (i = 0; i < 2; i++) {
    snprintf(path, sizeof(path), "/proc/%d/ns/%s", pid, ns_strs[i]);
    ns = open(path, O_RDONLY);
    if (ns < 0)
    return pr_err("Unable to open %s", path);
    pns = ioctl(ns, NS_GET_PARENT);
    if (pns < 0)
    return pr_err("Unable to get a parent pidns");
    snprintf(path, sizeof(path), "/proc/self/ns/%s", ns_strs[i]);
    if (stat(path, &st2))
    return pr_err("Unable to stat %s", path);
    if (fstat(pns, &st1))
    return pr_err("Unable to stat the parent pidns");
    if (st1.st_ino != st2.st_ino)
    return pr_err("NS_GET_PARENT returned a wrong namespace");
    if (ioctl(pns, NS_GET_PARENT) >= 0 || errno != EPERM)
    return pr_err("Don't get EPERM");
    }
    kill(pid, SIGKILL);
    wait(core::ptr::null_mut());
    return 0;
    }
