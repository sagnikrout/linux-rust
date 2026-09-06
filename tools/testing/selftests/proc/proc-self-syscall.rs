//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/proc/proc-self-syscall.c
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
// Copyright © 2018 Alexey Dobriyan <adobriyan@gmail.com>
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

#[no_mangle]
pub unsafe extern "C" fn sys_read(fd: c_int, buf: *mut c_void, len: usize) -> isize {
    static inline ssize_t sys_read(int fd, void *buf, size_t len)
    {
    return syscall(SYS_read, fd, buf, len);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    char buf1[64];
    char buf2[64];
    int fd;
    ssize_t rv;
    fd = open("/proc/self/syscall", O_RDONLY);
    if (fd == -1) {
    if (errno == ENOENT)
    return 4;
    return 1;
    }
// Do direct system call as libc can wrap anything.
    snprintf(buf1, sizeof(buf1), "%ld 0x%lx 0x%lx 0x%lx",
    (long)SYS_read, (long)fd, (long)buf2, (long)sizeof(buf2));
    memset(buf2, 0, sizeof(buf2));
    rv = sys_read(fd, buf2, sizeof(buf2));
    if (rv < 0)
    return 1;
    if (rv < strlen(buf1))
    return 1;
    if (strncmp(buf1, buf2, strlen(buf1)) != 0)
    return 1;
    return 0;
    }
