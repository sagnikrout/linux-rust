//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/unistd.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// unistd function definitions for NOLIBC
// Copyright (C) 2017-2022 Willy Tarreau <w@1wt.eu>
//
// make sure to include all global symbols

pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;
pub const F_OK: c_int = 0;
pub const X_OK: c_int = 1;
pub const W_OK: c_int = 2;
pub const R_OK: c_int = 4;
//
// int access(const char *path, int amode);
// int faccessat(int fd, const char *path, int amode, int flag);
//
extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_faccessat, _arg: fd, _arg: path, _arg: amode, _arg: flag) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_faccessat(fd, _arg: path, _arg: amode, _arg: flag)) -> return;
}
extern "C" {
    pub fn faccessat(_arg: AT_FDCWD, _arg: path, _arg: amode, _arg: 0) -> return;
}

extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_ftruncate64, _arg: fd, _arg: length0, _arg: length1) -> return;
}

extern "C" {
    pub fn _sys_ftruncate64(_arg: fd, _arg: __NOLIBC_LLARGPART(length, _arg: 0), _arg: __NOLIBC_LLARGPART(length, _arg: 1)) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_ftruncate, _arg: fd, _arg: length) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_ftruncate(fd, _arg: length)) -> return;
}
//
// char *getcwd(char *buf, size_t size);
//
extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_getcwd, _arg: buf, _arg: size) -> return;
}
// Unlike other libc's we don't handle passing NULL for buf
// On error return NULL, __sysret() above will have set errno
// Handle no path being written or the kernel putting
// "(unreachable)" into the buffer instead of a path.
// This matches what musl is doing.
//
// ret must be the number of bytes written at this point,
// so return the pointer to buf.
//
// ssize_t readlink(const char *path, char *buf, size_t bufsiz);
//
extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_readlinkat, _arg: AT_FDCWD, _arg: path, _arg: buf, _arg: bufsiz) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_readlink(path, _arg: buf, _arg: bufsiz)) -> return;
}
extern "C" {
    pub fn _sys_select(_arg: 0, _arg: NULL, _arg: NULL, _arg: NULL, _arg: &my_timeval) -> return;
}
extern "C" {
    pub fn ioctl(_arg: fd, _arg: TIOCSPGRP, _arg: &pid) -> return;
}
