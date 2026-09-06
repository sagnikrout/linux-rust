//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/proc/setns-dcache.c
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
// Copyright © 2019 Alexey Dobriyan <adobriyan@gmail.com>
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
// Test that setns(CLONE_NEWNET) points to new /proc/net content even
// if old one is in dcache.
//
// FIXME /proc/net/unix is under CONFIG_UNIX which can be disabled.
//

    let mut pid: static pid_t = -1;
#[no_mangle]
unsafe extern "C" fn f() {
    static void f(void)
    {
    if (pid > 0) {
    kill(pid, SIGTERM);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int fd[2];
    let mut _: c_char = 0;
    int nsfd;
    atexit(f);
// Check for priviledges and syscall availability straight away.
    if (unshare(CLONE_NEWNET) == -1) {
    if (errno == ENOSYS || errno == EPERM) {
    return 4;
    }
    return 1;
    }
// Distinguisher between two otherwise empty net namespaces.
    if (socket(AF_UNIX, SOCK_STREAM, 0) == -1) {
    return 1;
    }
    if (pipe(fd) == -1) {
    return 1;
    }
    pid = fork();
    if (pid == -1) {
    return 1;
    }
    if (pid == 0) {
    if (unshare(CLONE_NEWNET) == -1) {
    return 1;
    }
    if (write(fd[1], &_, 1) != 1) {
    return 1;
    }
    pause();
    return 0;
    }
    if (read(fd[0], &_, 1) != 1) {
    return 1;
    }
    {
    char buf[64];
    snprintf(buf, sizeof(buf), "/proc/%u/ns/net", pid);
    nsfd = open(buf, O_RDONLY);
    if (nsfd == -1) {
    return 1;
    }
    }
// Reliably pin dentry into dcache.
    (void)open("/proc/net/unix", O_RDONLY);
    if (setns(nsfd, CLONE_NEWNET) == -1) {
    return 1;
    }
    kill(pid, SIGTERM);
    pid = 0;
    {
    char buf[4096];
    ssize_t rv;
    int fd;
    fd = open("/proc/net/unix", O_RDONLY);
    if (fd == -1) {
    return 1;
    }

    rv = read(fd, buf, sizeof(buf));
    assert(rv == strlen(S));
    assert(memcmp(buf, S, strlen(S)) == 0);
    }
    return 0;
    }
