//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/proc/proc-fsconfig-hidepid.c
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
pub unsafe extern "C" fn fsopen(fsname: *const c_char, flags: c_uint) -> c_int {
    static inline int fsopen(const char *fsname, unsigned int flags)
    {
    return syscall(__NR_fsopen, fsname, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn fsconfig(fd: c_int, cmd: c_uint, key: *const c_char, val: *const c_void, aux: c_int) -> c_int {
    static inline int fsconfig(int fd, unsigned int cmd, const char *key, const void *val, int aux)
    {
    return syscall(__NR_fsconfig, fd, cmd, key, val, aux);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int fsfd, ret;
    let mut hidepid: c_int = 2;
    assert((fsfd = fsopen("proc", 0)) != -1);
    ret = fsconfig(fsfd, FSCONFIG_SET_BINARY, "hidepid", &hidepid, 0);
    assert(ret == -1);
    assert(errno == EINVAL);
    assert(!fsconfig(fsfd, FSCONFIG_SET_STRING, "hidepid", "2", 0));
    assert(!fsconfig(fsfd, FSCONFIG_SET_STRING, "hidepid", "invisible", 0));
    assert(!close(fsfd));
    return 0;
    }
