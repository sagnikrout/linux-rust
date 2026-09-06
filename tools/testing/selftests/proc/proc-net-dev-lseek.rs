//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/proc/proc-net-dev-lseek.c
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
// Copyright (c) 2025 Alexey Dobriyan <adobriyan@gmail.com>
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

// Macro flag: #define _GNU_SOURCE

//
// Test that lseek("/proc/net/dev/", 0, SEEK_SET)
// a) works,
// b) does what you think it does.
//
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
// /proc/net/dev output is deterministic in fresh netns only.
    if (unshare(CLONE_NEWNET) == -1) {
    if (errno == ENOSYS || errno == EPERM) {
    return 4;
    }
    return 1;
    }
    let mut fd: c_int = open("/proc/net/dev", O_RDONLY);
    assert(fd >= 0);
    char buf1[4096];
    let mut rv1: isize = read(fd, buf1, sizeof(buf1));
//
// Not "<=", this file can't be empty:
// there is header, "lo" interface with some zeroes.
//
    assert(0 < rv1);
    assert(rv1 <= sizeof(buf1));
// Believe it or not, this line broke one day.
    assert(lseek(fd, 0, SEEK_SET) == 0);
    char buf2[4096];
    let mut rv2: isize = read(fd, buf2, sizeof(buf2));
// Not "<=", see above.
    assert(0 < rv2);
    assert(rv2 <= sizeof(buf2));
// Test that lseek rewinds to the beginning of the file.
    assert(rv1 == rv2);
    assert(memcmp(buf1, buf2, rv1) == 0);
// Contents of the file is not validated: this test is about lseek().
    return 0;
    }
