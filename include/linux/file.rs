//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/file.h
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
//
// Wrapper functions for accessing the file_struct fd array.
//

extern "C" {
    pub fn fput(: *mut file);
}
// either a reference to struct file + flags
// (cloned vs. borrowed, pos locked), with
// flags stored in lower bits of value,
// or empty (represented by 0).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd {
    pub word: c_ulong,
}

pub const FDPUT_FPUT: c_int = 1;
pub const FDPUT_POS_UNLOCK: c_int = 2;

extern "C" {
    pub fn unlikely(_arg: !f.word) -> return;
}

extern "C" {
    pub fn __f_unlock_pos(: *mut file);
}
extern "C" {
    pub fn fdget(fd: c_uint) -> fd;
}
extern "C" {
    pub fn fdget_raw(fd: c_uint) -> fd;
}
extern "C" {
    pub fn fdget_pos(fd: c_uint) -> fd;
}
extern "C" {
    pub fn f_dupfd(from: c_uint, file: *mut file, flags: unsigned) -> c_int;
}
extern "C" {
    pub fn replace_fd(fd: unsigned, file: *mut file, flags: unsigned) -> c_int;
}
extern "C" {
    pub fn set_close_on_exec(fd: c_uint, flag: c_int);
}
extern "C" {
    pub fn get_close_on_exec(fd: c_uint) -> bool;
}
extern "C" {
    pub fn __get_unused_fd_flags(flags: unsigned, nofile: c_ulong) -> c_int;
}
extern "C" {
    pub fn get_unused_fd_flags(flags: unsigned) -> c_int;
}
extern "C" {
    pub fn put_unused_fd(fd: c_uint);
}
//
// take_fd() will take care to set @fd to -EBADF ensuring that
// CLASS(get_unused_fd) won't call put_unused_fd(). This makes it
// easier to rely on CLASS(get_unused_fd):
//
// struct file *f;
//
// CLASS(get_unused_fd, fd)(O_CLOEXEC);
// if (fd < 0)
// return fd;
//
// f = dentry_open(&path, O_RDONLY, current_cred());
// if (IS_ERR(f))
// return PTR_ERR(f);
//
// fd_install(fd, f);
// return take_fd(fd);
//

extern "C" {
    pub fn fd_install(fd: c_uint, file: *mut file);
}
extern "C" {
    pub fn receive_fd(file: *mut file, ufd: *mut int __user, o_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn receive_fd_replace(new_fd: c_int, file: *mut file, o_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn flush_delayed_fput();
}
extern "C" {
    pub fn __fput_sync(: *mut file);
}
//
// fd_prepare: Combined fd + file allocation cleanup class.
// @err: Error code to indicate if allocation succeeded.
// @__fd: Allocated fd (may not be accessed directly)
// @__file: Allocated struct file pointer (may not be accessed directly)
//
// Allocates an fd and a file together. On error paths, automatically cleans
// up whichever resource was successfully allocated. Allows flexible file
// allocation with different functions per usage.
//
// Do not use directly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd_prepare {
    pub err: i32,
    pub /: *mut *mut s32 __fd; / do not access directly,
    pub /: *mut *mut *mut file __file; / do not access directly,
}

// Typedef for fd_prepare cleanup guards.
pub type class_fd_prepare_t = fd_prepare;
//
// Accessors for fd_prepare class members.
// _Generic() is used for zero-cost type safety.
//

// Do not use directly.
extern "C" {
    pub fn PTR_ERR(_arg: fdf->__file) -> return;
}
//
// __FD_PREPARE_INIT - Helper to initialize fd_prepare class.
// @_fd_flags: flags for get_unused_fd_flags()
// @_file_owned: expression that returns struct file
//
// Returns a struct fd_prepare with fd, file, and err set.
// If fd allocation fails, fd will be negative and err will be set. If
// fd succeeds but file_init_expr fails, file will be ERR_PTR and err
// will be set. The err field is the single source of truth for error
// checking.
//

//
// FD_PREPARE - Macro to declare and initialize an fd_prepare variable.
//
// Declares and initializes an fd_prepare variable with automatic
// cleanup. No separate scope required - cleanup happens when variable
// goes out of scope.
//
// @_fdf: name of struct fd_prepare variable to define
// @_fd_flags: flags for get_unused_fd_flags()
// @_file_owned: struct file to take ownership of (can be expression)
//

//
// fd_publish - Publish prepared fd and file to the fd table.
// @_fdf: struct fd_prepare variable
//

// Do not use directly.

//
// FD_ADD - Allocate and install an fd and file in one step.
// @_fd_flags: flags for get_unused_fd_flags()
// @_file_owned: struct file to take ownership of
//
// Returns the allocated fd number, or negative error code on failure.
//

