//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/exec/recursion-depth.c
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
// Copyright (c) 2019 Alexey Dobriyan <adobriyan@gmail.com>
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
// Test that pointing #! script interpreter to self doesn't recurse.

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int fd, rv;
    ksft_print_header();
    ksft_set_plan(1);
    if (unshare(CLONE_NEWNS) == -1) {
    if (errno == ENOSYS || errno == EPERM) {
    ksft_test_result_skip("error: unshare, errno %d\n", errno);
    ksft_finished();
    }
    ksft_exit_fail_perror("error: unshare");
    }
    if (mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_PRIVATE | MS_REC, core::ptr::null_mut()) == -1)
    ksft_exit_fail_perror("error: mount '/'");
// Require "exec" filesystem.
    if (mount(core::ptr::null_mut(), "/tmp", "ramfs", 0, core::ptr::null_mut()) == -1)
    ksft_exit_fail_perror("error: mount ramfs");

    fd = creat(FILENAME, 0700);
    if (fd == -1)
    ksft_exit_fail_perror("error: creat");

    if (write(fd, S, strlen(S)) != strlen(S))
    ksft_exit_fail_perror("error: write");
    close(fd);
    rv = execve(FILENAME, core::ptr::null_mut(), core::ptr::null_mut());
    ksft_test_result(rv == -1 && errno == ELOOP,
    "execve failed as expected (ret %d, errno %d)\n", rv, errno);
    ksft_finished();
    }
