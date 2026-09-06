//! Automatically rewritten from C to Rust
//! Source: tools/lib/perf/lib.c
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

    unsigned int page_size;
#[no_mangle]
unsafe extern "C" fn ion(is_read: bool, fd: c_int, buf: *mut c_void, n: usize) -> isize {
    static ssize_t ion(bool is_read, int fd, void *buf, size_t n)
    {
    void *buf_start = buf;
    let mut left: usize = n;
    while (left) {
// buf must be treated as const if !is_read.
    ssize_t ret = is_read ? read(fd, buf, left) :
    write(fd, buf, left);
    if (ret < 0 && errno == EINTR)
    continue;
    if (ret <= 0)
    return ret;
    left -= ret;
    buf  += ret;
    }
    BUG_ON((size_t)(buf - buf_start) != n);
    return n;
    }
//
// Read exactly 'n' bytes or return an error.
//
#[no_mangle]
pub unsafe extern "C" fn readn(fd: c_int, buf: *mut c_void, n: usize) -> isize {
    ssize_t readn(int fd, void *buf, size_t n)
    {
    return ion(true, fd, buf, n);
    }
#[no_mangle]
pub unsafe extern "C" fn preadn(fd: c_int, buf: *mut c_void, n: usize, offs: off_t) -> isize {
    ssize_t preadn(int fd, void *buf, size_t n, off_t offs)
    {
    let mut left: usize = n;
    while (left) {
    let mut ret: isize = pread(fd, buf, left, offs);
    if (ret < 0 && errno == EINTR)
    continue;
    if (ret <= 0)
    return ret;
    left -= ret;
    buf  += ret;
    offs += ret;
    }
    return n;
    }
//
// Write exactly 'n' bytes or return an error.
//
#[no_mangle]
pub unsafe extern "C" fn writen(fd: c_int, buf: *const c_void, n: usize) -> isize {
    ssize_t writen(int fd, const void *buf, size_t n)
    {
// ion does not modify buf.
    return ion(false, fd, (void *)buf, n);
    }
