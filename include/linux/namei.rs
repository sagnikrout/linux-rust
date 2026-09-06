//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/namei.h
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

pub const MAXSYMLINKS: c_int = 40;
// pathwalk mode

// 5 spare bits for pathwalk
// These tell filesystem methods that we are dealing with the final component...

// 4 spare bits for intent
// Scoping flags for lookup.

// LOOKUP_* flags which do scope-related checks based on the dirfd.

// 3 spare bits for scoping
extern "C" {
    pub fn path_pts(path: *mut path) -> c_int;
}
extern "C" {
    pub fn user_path_at(_arg: c_int, : *const char __user, _arg: unsigned, : *mut path) -> c_int;
}
extern "C" {
    pub fn kern_path(: *const c_char, _arg: unsigned, : *mut path) -> c_int;
}
extern "C" {
    pub fn end_creating_path(: *const path, : *mut dentry);
}
// end_creating - finish action started with start_creating
// @child: dentry returned by start_creating() or vfs_mkdir()
//
// Unlock and release the child. This can be called after
// start_creating() whether that function succeeded or not,
// but it is not needed on failure.
//
// If vfs_mkdir() was called then the value returned from that function
// should be given for @child rather than the original dentry, as vfs_mkdir()
// may have provided a new dentry.
//
// If vfs_mkdir() was not called, then @child will be a valid dentry and
// @parent will be ignored.
//
// end_creating_keep - finish action started with start_creating() and return result
// @child: dentry returned by start_creating() or vfs_mkdir()
//
// Unlock and return the child. This can be called after
// start_creating() whether that function succeeded or not,
// but it is not needed on failure.
//
// If vfs_mkdir() was called then the value returned from that function
// should be given for @child rather than the original dentry, as vfs_mkdir()
// may have provided a new dentry.
//
// Returns: @child, which may be a dentry or an error.
//
// end_removing - finish action started with start_removing
// @child:  dentry returned by start_removing()
// @parent: dentry given to start_removing()
//
// Unlock and release the child.
//
// This is identical to end_dirop().  It can be passed the result of
// start_removing() whether that was successful or not, but it not needed
// if start_removing() failed.
//
extern "C" {
    pub fn follow_down_one(: *mut path) -> c_int;
}
extern "C" {
    pub fn follow_down(path: *mut path, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn follow_up(: *mut path) -> c_int;
}
extern "C" {
    pub fn end_renaming(rd: *mut renamedata);
}
//
// mode_strip_umask - handle vfs umask stripping
// @dir:	parent directory of the new inode
// @mode:	mode of the new inode to be created in @dir
//
// In most filesystems, umask stripping depends on whether or not the
// filesystem supports POSIX ACLs. If the filesystem doesn't support it umask
// stripping is done directly in here. If the filesystem does support POSIX
// ACLs umask stripping is deferred until the filesystem calls
// posix_acl_create().
//
// Some filesystems (like NFSv4) also want to avoid umask stripping by the
// VFS, but don't support POSIX ACLs. Those filesystems can set SB_I_NOUMASK
// to get this effect without declaring that they support POSIX ACLs.
//
// Returns: mode
//
extern "C" {
    pub fn nd_jump_link(path: *const path) -> int __must_check;
}
//
// retry_estale - determine whether the caller should retry an operation
// @error: the error that would currently be returned
// @flags: flags being used for next lookup attempt
//
// Check to see if the error code was -ESTALE, and then determine whether
// to retry the call based on whether "flags" already has LOOKUP_REVAL set.
//
// Returns true if the caller should try the operation again.
//
extern "C" {
    pub fn unlikely(LOOKUP_REVAL): error == -ESTALE && !(flags &) -> return;
}
