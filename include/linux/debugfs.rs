//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/debugfs.h
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
// debugfs.h - a tiny little debug file system
//
// Copyright (C) 2004 Greg Kroah-Hartman <greg@kroah.com>
// Copyright (C) 2004 IBM Inc.
//
// debugfs is for people to use instead of /proc or /sys.
// See Documentation/filesystems/ for more details.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debugfs_blob_wrapper {
    pub data: *mut c_void,
    pub size: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debugfs_reg32 {
    pub name: *mut c_char,
    pub offset: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debugfs_regset32 {
    pub regs: *const debugfs_reg32,
    pub nregs: c_int,
    pub base: *mut void __iomem,
    pub /: *mut *mut *mut device dev; / Optional device for Runtime PM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debugfs_u32_array {
    pub array: *mut u32,
    pub n_elements: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debugfs_short_fops {
    pub ): *mut *mut *mut *mut ssize_t (read)(struct file , char __user , size_t, loff_t,
    pub ): *const *const *const *const ssize_t (write)(struct file , char __user , size_t, loff_t,
    pub int): *mut *mut *mut loff_t (llseek) (struct file , loff_t,,
}

//
// debugfs_create_file - create a file in the debugfs filesystem
// @name: a pointer to a string containing the name of the file to create.
// @mode: the permission that the file should have.
// @parent: a pointer to the parent dentry for this file.  This should be a
// directory dentry if set.  If this parameter is NULL, then the
// file will be created in the root of the debugfs filesystem.
// @data: a pointer to something that the caller will want to get to later
// on.  The inode.i_private pointer will point to this value on
// the open() call.
// @fops: a pointer to a struct file_operations or struct debugfs_short_fops that
// should be used for this file.
//
// This is the basic "create a file" function for debugfs.  It allows for a
// wide range of flexibility in creating a file, or a directory (if you want
// to create a directory, the debugfs_create_dir() function is
// recommended to be used instead.)
//
// This function will return a pointer to a dentry if it succeeds.  This
// pointer must be passed to the debugfs_remove() function when the file is
// to be removed (no automatic cleanup happens if your module is unloaded,
// you are responsible here.)  If an error occurs, ERR_PTR(-ERROR) will be
// returned.
//
// If debugfs is not enabled in the kernel, the value -%ENODEV will be
// returned.
//
// If fops points to a struct debugfs_short_fops, then simple_open() will be
// used for the open, and only read/write/llseek are supported and are proxied,
// so no module reference or release are needed.
//
// NOTE: it's expected that most callers should _ignore_ the errors returned
// by this function. Other debugfs functions handle the fact that the "dentry"
// passed to them could be an error and they don't crash in that case.
// Drivers should generally work fine even if debugfs fails to init anyway.
//

extern "C" {
    pub fn debugfs_remove(dentry: *mut dentry);
}

extern "C" {
    pub fn debugfs_lookup_and_remove(name: *const c_char, parent: *mut dentry);
}
extern "C" {
    pub fn debugfs_file_get(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn debugfs_file_put(dentry: *mut dentry);
}
extern "C" {
    pub fn debugfs_change_name(dentry: *mut dentry, fmt: *const c_char, __printf(2: ...), _arg: 3) -> c_int;
}
extern "C" {
    pub fn debugfs_initialized() -> bool;
}
//
// struct debugfs_cancellation - cancellation data
// @list: internal, for keeping track
// @cancel: callback to call
// @cancel_data: extra data for the callback to call
//

//
// We do not return NULL from these functions if CONFIG_DEBUG_FS is not enabled
// so users have a chance to detect if there was a real error or not.  We don't
// want to duplicate the design decision mistakes of procfs and devfs again.
//
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

//
// debugfs_create_xul - create a debugfs file that is used to read and write an
// unsigned long value, formatted in hexadecimal
// @name: a pointer to a string containing the name of the file to create.
// @mode: the permission that the file should have
// @parent: a pointer to the parent dentry for this file.  This should be a
// directory dentry if set.  If this parameter is %NULL, then the
// file will be created in the root of the debugfs filesystem.
// @value: a pointer to the variable that the file should read to and write
// from.
//
