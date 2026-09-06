//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/sys/stat.h
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
// stat definition for NOLIBC
// Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
//
// make sure to include all global symbols

//
// int statx(int fd, const char *path, int flags, unsigned int mask, struct statx *buf);
// int stat(const char *path, struct stat *buf);
// int fstatat(int fd, const char *path, struct stat *buf, int flag);
// int fstat(int fildes, struct stat *buf);
// int lstat(const char *path, struct stat *buf);
//

extern "C" {
    pub fn __nolibc_syscall5(_arg: __NR_statx, _arg: fd, _arg: path, _arg: flags, _arg: mask, _arg: buf) -> return;
}

extern "C" {
    pub fn __nolibc_enosys(_arg: __func__, _arg: fd, _arg: path, _arg: flags, _arg: mask, _arg: buf) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_statx(fd, _arg: path, _arg: flags, _arg: mask, _arg: buf)) -> return;
}
extern "C" {
    pub fn fstatat(_arg: AT_FDCWD, _arg: path, _arg: buf, _arg: 0) -> return;
}
extern "C" {
    pub fn fstatat(_arg: fildes, _arg: "", _arg: buf, _arg: AT_EMPTY_PATH) -> return;
}
extern "C" {
    pub fn fstatat(_arg: AT_FDCWD, _arg: path, _arg: buf, _arg: AT_SYMLINK_NOFOLLOW) -> return;
}
