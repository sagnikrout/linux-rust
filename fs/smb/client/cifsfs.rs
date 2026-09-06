//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/cifsfs.h
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


// SPDX-License-Identifier: LGPL-2.1
//
// Copyright (c) International Business Machines  Corp., 2002, 2007
// Author(s): Steve French (sfrench@us.ibm.com)
//

pub const ROOT_I: c_int = 2;
//
// ino_t is 32-bits on 32-bit arch. We have to squash the 64-bit value down
// so that it will fit. We use hash_64 to convert the value to 31 bits, and
// then add 1, to ensure that we don't end up with a 0 as the value.
//
// Functions related to super block operations
extern "C" {
    pub fn cifs_sb_active(sb: *mut super_block);
}
extern "C" {
    pub fn cifs_sb_deactive(sb: *mut super_block);
}
// Functions related to inodes
extern "C" {
    pub fn cifs_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn cifs_rmdir(inode: *mut inode, direntry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn cifs_revalidate_file_attr(filp: *mut file) -> c_int;
}
extern "C" {
    pub fn cifs_revalidate_dentry_attr(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn cifs_revalidate_file(filp: *mut file) -> c_int;
}
extern "C" {
    pub fn cifs_revalidate_dentry(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn cifs_revalidate_mapping(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn cifs_zap_mapping(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn cifs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
// Functions related to files and directories
extern "C" {
    pub fn cifs_open(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn cifs_close(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn cifs_closedir(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn cifs_strict_readv(iocb: *mut kiocb, to: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn cifs_strict_writev(iocb: *mut kiocb, from: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn cifs_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn cifs_direct_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn cifs_loose_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn cifs_flock(file: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int;
}
extern "C" {
    pub fn cifs_lock(file: *mut file, cmd: c_int, flock: *mut file_lock) -> c_int;
}
extern "C" {
    pub fn cifs_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int;
}
extern "C" {
    pub fn cifs_flush(file: *mut file, id: fl_owner_t) -> c_int;
}
extern "C" {
    pub fn cifs_file_mmap_prepare(desc: *mut vm_area_desc) -> c_int;
}
extern "C" {
    pub fn cifs_file_strict_mmap_prepare(desc: *mut vm_area_desc) -> c_int;
}
extern "C" {
    pub fn cifs_readdir(file: *mut file, ctx: *mut dir_context) -> c_int;
}
// Functions related to dir entries
// Functions related to symlinks

extern "C" {
    pub fn cifs_listxattr(direntry: *mut dentry, data: *mut c_char, buf_size: usize) -> isize;
}

extern "C" {
    pub fn cifs_ioctl(filep: *mut file, command: c_uint, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn cifs_setsize(inode: *mut inode, offset: loff_t);
}
extern "C" {
    pub fn cifs_resize_file_locked(inode: *mut inode, offset: loff_t);
}

// when changing internal version - update following two lines at same time
pub const SMB3_PRODUCT_BUILD: c_int = 61;

