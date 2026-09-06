//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/proc/proc-tid0.c
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


//
// Copyright (c) 2021 Alexey Dobriyan <adobriyan@gmail.com>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// Test that /proc/*/task never contains "0".

    let mut pid: static pid_t = -1;
#[no_mangle]
unsafe extern "C" fn atexit_hook() {
    static void atexit_hook(void)
    {
    if (pid > 0) {
    kill(pid, SIGKILL);
    }
    }
    static void *f(void *_)
    {
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sigalrm(_: c_int) {
    static void sigalrm(int _)
    {
    exit(0);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    pid = fork();
    if (pid == 0) {
// child
    while (1) {
    pthread_t pth;
    pthread_create(&pth, core::ptr::null_mut(), f, core::ptr::null_mut());
    pthread_join(pth, core::ptr::null_mut());
    }
    } else if (pid > 0) {
// parent
    atexit(atexit_hook);
    char buf[64];
    snprintf(buf, sizeof(buf), "/proc/%u/task", pid);
    signal(SIGALRM, sigalrm);
    alarm(1);
    while (1) {
    DIR *d = opendir(buf);
    struct dirent *de;
    while ((de = readdir(d))) {
    if (strcmp(de.d_name, "0") == 0) {
    exit(1);
    }
    }
    closedir(d);
    }
    return 0;
    } else {
    perror("fork");
    return 1;
    }
    }
