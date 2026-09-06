//! Automatically rewritten from C Header to Rust Module
//! Source: fs/hostfs/hostfs.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hostfs_timespec {
    pub tv_sec: c_longlong,
    pub tv_nsec: c_longlong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hostfs_iattr {
    pub ia_valid: c_uint,
    pub ia_mode: c_ushort,
    pub ia_uid: uid_t,
    pub ia_gid: gid_t,
    pub ia_size: loff_t,
    pub ia_atime: hostfs_timespec,
    pub ia_mtime: hostfs_timespec,
    pub ia_ctime: hostfs_timespec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hostfs_stat {
    pub ino: c_ulonglong,
    pub mode: c_uint,
    pub nlink: c_uint,
    pub uid: c_uint,
    pub gid: c_uint,
    pub size: c_ulonglong,
    pub btime: hostfs_timespec atime, mtime, ctime,,
    pub blksize: c_uint,
    pub blocks: c_ulonglong,
    pub maj: c_uint,
    pub min: c_uint,
    pub dev: } rdev,,
}

extern "C" {
    pub fn stat_file(path: *const c_char, p: *mut hostfs_stat, fd: c_int) -> c_int;
}
extern "C" {
    pub fn access_file(path: *mut c_char, r: c_int, w: c_int, x: c_int) -> c_int;
}
extern "C" {
    pub fn open_file(path: *mut c_char, r: c_int, w: c_int, append: c_int) -> c_int;
}
extern "C" {
    pub fn seek_dir(stream: *mut c_void, pos: c_ulonglong);
}
extern "C" {
    pub fn close_file(stream: *mut c_void);
}
extern "C" {
    pub fn replace_file(oldfd: c_int, fd: c_int) -> c_int;
}
extern "C" {
    pub fn close_dir(stream: *mut c_void);
}
extern "C" {
    pub fn read_file(fd: c_int, offset: *mut c_ulonglong, buf: *mut c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn lseek_file(fd: c_int, offset: c_longlong, whence: c_int) -> c_int;
}
extern "C" {
    pub fn fsync_file(fd: c_int, datasync: c_int) -> c_int;
}
extern "C" {
    pub fn file_create(name: *mut c_char, mode: c_int) -> c_int;
}
extern "C" {
    pub fn set_attr(file: *const c_char, attrs: *mut hostfs_iattr, fd: c_int) -> c_int;
}
extern "C" {
    pub fn make_symlink(from: *const c_char, to: *const c_char) -> c_int;
}
extern "C" {
    pub fn unlink_file(file: *const c_char) -> c_int;
}
extern "C" {
    pub fn do_mkdir(file: *const c_char, mode: c_int) -> c_int;
}
extern "C" {
    pub fn hostfs_do_rmdir(file: *const c_char) -> c_int;
}
extern "C" {
    pub fn link_file(to: *const c_char, from: *const c_char) -> c_int;
}
extern "C" {
    pub fn hostfs_do_readlink(file: *mut c_char, buf: *mut c_char, size: c_int) -> c_int;
}
extern "C" {
    pub fn rename_file(from: *mut c_char, to: *mut c_char) -> c_int;
}
extern "C" {
    pub fn rename2_file(from: *mut c_char, to: *mut c_char, flags: c_uint) -> c_int;
}
