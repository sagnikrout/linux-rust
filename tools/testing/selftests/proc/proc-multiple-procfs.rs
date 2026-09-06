//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/proc/proc-multiple-procfs.c
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
// Copyright © 2020 Alexey Gladkov <gladkov.alexey@gmail.com>
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
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    struct stat proc_st1, proc_st2;
    char procbuff[] = "/tmp/proc.XXXXXX/meminfo";
    char procdir1[] = "/tmp/proc.XXXXXX";
    char procdir2[] = "/tmp/proc.XXXXXX";
    assert(mkdtemp(procdir1) != core::ptr::null_mut());
    assert(mkdtemp(procdir2) != core::ptr::null_mut());
    assert(!mount("proc", procdir1, "proc", 0, "hidepid=1"));
    assert(!mount("proc", procdir2, "proc", 0, "hidepid=2"));
    snprintf(procbuff, sizeof(procbuff), "%s/meminfo", procdir1);
    assert(!stat(procbuff, &proc_st1));
    snprintf(procbuff, sizeof(procbuff), "%s/meminfo", procdir2);
    assert(!stat(procbuff, &proc_st2));
    umount(procdir1);
    umount(procdir2);
    assert(proc_st1.st_dev != proc_st2.st_dev);
    return 0;
    }
