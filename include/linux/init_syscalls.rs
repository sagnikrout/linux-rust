//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/init_syscalls.h
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
extern "C" {
    pub fn init_umount(name: *const c_char, flags: c_int) -> int __init;
}
extern "C" {
    pub fn init_chdir(filename: *const c_char) -> int __init;
}
extern "C" {
    pub fn init_chroot(filename: *const c_char) -> int __init;
}
extern "C" {
    pub fn init_chown(filename: *const c_char, user: uid_t, group: gid_t, flags: c_int) -> int __init;
}
extern "C" {
    pub fn init_chmod(filename: *const c_char, mode: umode_t) -> int __init;
}
extern "C" {
    pub fn init_eaccess(filename: *const c_char) -> int __init;
}
extern "C" {
    pub fn init_stat(filename: *const c_char, stat: *mut kstat, flags: c_int) -> int __init;
}
extern "C" {
    pub fn init_mknod(filename: *const c_char, mode: umode_t, dev: c_uint) -> int __init;
}
extern "C" {
    pub fn init_link(oldname: *const c_char, newname: *const c_char) -> int __init;
}
extern "C" {
    pub fn init_symlink(oldname: *const c_char, newname: *const c_char) -> int __init;
}
extern "C" {
    pub fn init_unlink(pathname: *const c_char) -> int __init;
}
extern "C" {
    pub fn init_mkdir(pathname: *const c_char, mode: umode_t) -> int __init;
}
extern "C" {
    pub fn init_rmdir(pathname: *const c_char) -> int __init;
}
extern "C" {
    pub fn init_utimes(filename: *mut c_char, ts: *mut timespec64) -> int __init;
}
extern "C" {
    pub fn init_dup(file: *mut file) -> int __init;
}
extern "C" {
    pub fn init_pivot_root(new_root: *const c_char, put_old: *const c_char) -> int __init;
}
