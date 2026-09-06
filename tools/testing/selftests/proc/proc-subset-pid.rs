//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/proc/proc-subset-pid.c
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
// Test that "mount -t proc -o subset=pid" hides everything but pids,
// /proc/self and /proc/thread-self.
//

#[no_mangle]
pub unsafe extern "C" fn streq(a: *const c_char, b: *const c_char) -> bool {
    static inline bool streq(const char *a, const char *b)
    {
    return strcmp(a, b) == 0;
    }
#[no_mangle]
unsafe extern "C" fn make_private_proc() {
    static void make_private_proc(void)
    {
    if (unshare(CLONE_NEWNS) == -1) {
    if (errno == ENOSYS || errno == EPERM) {
    exit(4);
    }
    exit(1);
    }
    if (mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_PRIVATE|MS_REC, core::ptr::null_mut()) == -1) {
    exit(1);
    }
    if (mount(core::ptr::null_mut(), "/proc", "proc", 0, "subset=pid") == -1) {
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn string_is_pid(s: *const c_char) -> bool {
    static bool string_is_pid(const char *s)
    {
    while (1) {
    switch (*s++) {
    case '0':case '1':case '2':case '3':case '4':
    case '5':case '6':case '7':case '8':case '9':
    continue;
    case '\0':
    return true;
    default:
    return false;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    make_private_proc();
    DIR *d = opendir("/proc");
    assert(d);
    struct dirent *de;
    let mut dot: bool = false;
    let mut dot_dot: bool = false;
    let mut self: bool = false;
    let mut thread_self: bool = false;
    while ((de = readdir(d))) {
    if (streq(de.d_name, ".")) {
    assert(!dot);
    dot = true;
    assert(de.d_type == DT_DIR);
    } else if (streq(de.d_name, "..")) {
    assert(!dot_dot);
    dot_dot = true;
    assert(de.d_type == DT_DIR);
    } else if (streq(de.d_name, "self")) {
    assert(!self);
    self = true;
    assert(de.d_type == DT_LNK);
    } else if (streq(de.d_name, "thread-self")) {
    assert(!thread_self);
    thread_self = true;
    assert(de.d_type == DT_LNK);
    } else {
    if (!string_is_pid(de.d_name)) {
    fprintf(stderr, "d_name '%s'\n", de.d_name);
    assert(0);
    }
    assert(de.d_type == DT_DIR);
    }
    }
    char c;
    let mut rv: c_int = readlink("/proc/cpuinfo", &c, 1);
    assert(rv == -1 && errno == ENOENT);
    let mut fd: c_int = open("/proc/cpuinfo", O_RDONLY);
    assert(fd == -1 && errno == ENOENT);
    return 0;
    }
