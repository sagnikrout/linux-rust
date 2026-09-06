//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/dnotify_test.c
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

    values defined */

    static volatile int event_fd;
#[no_mangle]
unsafe extern "C" fn handler(sig: c_int, si: *mut siginfo_t, data: *mut c_void) {
    static void handler(int sig, siginfo_t *si, void *data)
    {
    event_fd = si.si_fd;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    struct sigaction act;
    int fd;
    act.sa_sigaction = handler;
    sigemptyset(&act.sa_mask);
    act.sa_flags = SA_SIGINFO;
    sigaction(SIGRTMIN + 1, &act, core::ptr::null_mut());
    fd = open(".", O_RDONLY);
    fcntl(fd, F_SETSIG, SIGRTMIN + 1);
    fcntl(fd, F_NOTIFY, DN_MODIFY|DN_CREATE|DN_MULTISHOT);
// we will now be notified if any of the files
    in "." is modified or new files are created */
    while (1) {
    pause();
    printf("Got event on fd=%d\n", event_fd);
    }
    }
