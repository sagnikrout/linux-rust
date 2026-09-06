//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/proc/proc-self-map-files-001.c
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
// Test readlink /proc/self/map_files/...

#[no_mangle]
unsafe extern "C" fn pass(fmt: *const c_char, a: c_ulong, b: c_ulong) {
    static void pass(const char *fmt, unsigned long a, unsigned long b)
    {
    char name[64];
    char buf[64];
    snprintf(name, sizeof(name), fmt, a, b);
    if (readlink(name, buf, sizeof(buf)) == -1)
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn fail(fmt: *const c_char, a: c_ulong, b: c_ulong) {
    static void fail(const char *fmt, unsigned long a, unsigned long b)
    {
    char name[64];
    char buf[64];
    snprintf(name, sizeof(name), fmt, a, b);
    if (readlink(name, buf, sizeof(buf)) == -1 && errno == ENOENT)
    return;
    exit(1);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    let mut PAGE_SIZE: c_uint = sysconf(_SC_PAGESIZE);
    void *p;
    int fd;
    unsigned long a, b;
    fd = open("/dev/zero", O_RDONLY);
    if (fd == -1)
    return 1;
    p = mmap(core::ptr::null_mut(), PAGE_SIZE, PROT_NONE, MAP_PRIVATE|MAP_FILE, fd, 0);
    if (p == MAP_FAILED)
    return 1;
    a = (unsigned long)p;
    b = (unsigned long)p + PAGE_SIZE;
    pass("/proc/self/map_files/%lx-%lx", a, b);
    fail("/proc/self/map_files/ %lx-%lx", a, b);
    fail("/proc/self/map_files/%lx -%lx", a, b);
    fail("/proc/self/map_files/%lx- %lx", a, b);
    fail("/proc/self/map_files/%lx-%lx ", a, b);
    fail("/proc/self/map_files/0%lx-%lx", a, b);
    fail("/proc/self/map_files/%lx-0%lx", a, b);
    if (sizeof(long) == 4) {
    fail("/proc/self/map_files/100000000%lx-%lx", a, b);
    fail("/proc/self/map_files/%lx-100000000%lx", a, b);
    } else if (sizeof(long) == 8) {
    fail("/proc/self/map_files/10000000000000000%lx-%lx", a, b);
    fail("/proc/self/map_files/%lx-10000000000000000%lx", a, b);
    } else
    return 1;
    return 0;
    }
