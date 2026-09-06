//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/proc/thread-self.c
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
// Test that /proc/thread-self gives correct TGID/PID.

#[no_mangle]
pub unsafe extern "C" fn f(arg: *mut c_void) -> c_int {
    int f(void *arg)
    {
    char buf1[64], buf2[64];
    pid_t pid, tid;
    ssize_t rv;
    pid = sys_getpid();
    tid = sys_gettid();
    snprintf(buf1, sizeof(buf1), "%u/task/%u", pid, tid);
    rv = readlink("/proc/thread-self", buf2, sizeof(buf2));
    assert(rv == strlen(buf1));
    buf2[rv] = '\0';
    assert(streq(buf1, buf2));
    if (arg)
    exit(0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    let mut PAGE_SIZE: c_int = sysconf(_SC_PAGESIZE);
    pid_t pid;
    void *stack;
// main thread
    f((void *)0);
    stack = mmap(core::ptr::null_mut(), 2 * PAGE_SIZE, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
    assert(stack != MAP_FAILED);
// side thread
    pid = clone(f, stack + PAGE_SIZE, CLONE_THREAD|CLONE_SIGHAND|CLONE_VM, (void *)1);
    assert(pid > 0);
    pause();
    return 0;
    }
