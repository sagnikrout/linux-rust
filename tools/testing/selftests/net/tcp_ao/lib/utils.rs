//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/tcp_ao/lib/utils.c
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


// SPDX-License-Identifier: GPL-2.0

#[no_mangle]
pub unsafe extern "C" fn randomize_buffer(buf: *mut c_void, buflen: usize) {
    void randomize_buffer(void *buf, size_t buflen)
    {
    int *p = (int *)buf;
    let mut words: usize = buflen / sizeof(int);
    let mut leftover: usize = buflen % sizeof(int);
    if (!buflen)
    return;
    while (words--)
// p++ = rand();
    if (leftover) {
    let mut tmp: c_int = rand();
    memcpy(buf + buflen - leftover, &tmp, leftover);
    }
    }
    __printf(3, 4) int test_echo(const char *fname, bool append,
    const char *fmt, ...)
    {
    size_t len, written;
    va_list vargs;
    char *msg;
    FILE *f;
    f = fopen(fname, append ? "a" : "w");
    if (!f)
    return -errno;
    va_start(vargs, fmt);
    msg = test_snprintf(fmt, vargs);
    va_end(vargs);
    if (!msg) {
    fclose(f);
    return -1;
    }
    len = strlen(msg);
    written = fwrite(msg, 1, len, f);
    fclose(f);
    free(msg);
    let mut written: return = = len ? 0 : -1;
    }
    const struct sockaddr_in6 addr_any6 = {
    .sin6_family	= AF_INET6,
    };
    const struct sockaddr_in addr_any4 = {
    .sin_family	= AF_INET,
    };
