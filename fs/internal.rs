//! Automatically rewritten from C Header to Rust Module
//! Source: fs/internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// fs/ internal definitions
//
// Copyright (C) 2006 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// block/bdev.c
//

extern "C" {
    pub fn bdev_cache_init() -> void __init;
}

//
// buffer.c
//
// char_dev.c
//
extern "C" {
    pub fn chrdev_init() -> void __init;
}
//
// fs_context.c
//
extern "C" {
    pub fn parse_monolithic_mount_data(: *mut fs_context, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn vfs_clean_context(fc: *mut fs_context);
}
extern "C" {
    pub fn finish_clean_context(fc: *mut fs_context) -> c_int;
}
//
// namei.c
//
extern "C" {
    pub fn filename_rmdir(dfd: c_int, name: *mut filename) -> c_int;
}
extern "C" {
    pub fn filename_unlinkat(dfd: c_int, name: *mut filename) -> c_int;
}
extern "C" {
    pub fn may_linkat(idmap: *mut mnt_idmap, link: *const path) -> c_int;
}
extern "C" {
    pub fn filename_mkdirat(dfd: c_int, name: *mut filename, mode: umode_t) -> c_int;
}
extern "C" {
    pub fn filename_mknodat(dfd: c_int, name: *mut filename, mode: umode_t, dev: c_uint) -> c_int;
}
extern "C" {
    pub fn filename_symlinkat(from: *mut filename, newdfd: c_int, to: *mut filename) -> c_int;
}
extern "C" {
    pub fn lookup_noperm_common(qname: *mut qstr, base: *mut dentry) -> c_int;
}
extern "C" {
    pub fn filename_init() -> void __init;
}
//
// namespace.c
//
extern "C" {
    pub fn finish_automount(: *mut vfsmount, : *const path) -> c_int;
}
extern "C" {
    pub fn sb_prepare_remount_readonly(: *mut super_block) -> c_int;
}
extern "C" {
    pub fn mnt_init() -> void __init;
}
extern "C" {
    pub fn mnt_get_write_access_file(file: *mut file) -> c_int;
}
extern "C" {
    pub fn mnt_put_write_access_file(file: *mut file);
}
extern "C" {
    pub fn dissolve_on_fput(: *mut vfsmount);
}
extern "C" {
    pub fn may_mount() -> bool;
}
extern "C" {
    pub fn path_umount(path: *const path, flags: c_int) -> c_int;
}
extern "C" {
    pub fn path_pivot_root(new: *mut path, old: *mut path) -> c_int;
}
extern "C" {
    pub fn show_path(m: *mut seq_file, root: *mut dentry) -> c_int;
}
//
// fs_struct.c
//
extern "C" {
    pub fn chroot_fs_refs(: *const path, : *const path);
}
//
// file_table.c
//
extern "C" {
    pub fn backing_file_set_user_path(f: *mut file, path: *const path);
}
extern "C" {
    pub fn fput_close_sync(: *mut file);
}
extern "C" {
    pub fn fput_close(: *mut file);
}
//
// super.c
//
extern "C" {
    pub fn reconfigure_super(: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn super_trylock_shared(sb: *mut super_block) -> bool;
}
extern "C" {
    pub fn put_super(sb: *mut super_block);
}
extern "C" {
    pub fn super_dev_init() -> void __init;
}
extern "C" {
    pub fn mount_capable(: *mut fs_context) -> bool;
}
//
// Prepare superblock for changing its read-only state (i.e., either remount
// read-write superblock read-only or vice versa). After this function returns
// mnt_is_readonly() will return true for any mount of the superblock if its
// caller is able to observe any changes done by the remount. This holds until
// sb_end_ro_state_change() is called.
//
// For RO->RW transition, the barrier pairs with the barrier in
// mnt_is_readonly() making sure if mnt_is_readonly() sees SB_RDONLY
// cleared, it will see s_readonly_remount set.
// For RW->RO transition, the barrier pairs with the barrier in
// mnt_get_write_access() before the mnt_is_readonly() check.
// The barrier makes sure if mnt_get_write_access() sees MNT_WRITE_HOLD
// already cleared, it will see s_readonly_remount set.
//
// Ends section changing read-only state of the superblock. After this function
// returns if mnt_is_readonly() returns false, the caller will be able to
// observe all the changes remount did to the superblock.
//
// This barrier provides release semantics that pairs with
// the smp_rmb() acquire semantics in mnt_is_readonly().
// This barrier pair ensure that when mnt_is_readonly() sees
// 0 for sb->s_readonly_remount, it will also see all the
// preceding flag changes that were made during the RO state
// change.
//
// open.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct open_flags {
    pub open_flag: c_int,
    pub mode: umode_t,
    pub acc_mode: c_int,
    pub intent: c_int,
    pub lookup_flags: c_int,
}

extern "C" {
    pub fn build_open_how(flags: c_int, mode: umode_t) -> open_how;
}
extern "C" {
    pub fn build_open_flags(how: *const open_how, op: *mut open_flags) -> c_int;
}
extern "C" {
    pub fn do_ftruncate(file: *mut file, length: loff_t, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn chmod_common(path: *const path, mode: umode_t) -> c_int;
}
extern "C" {
    pub fn chown_common(path: *const path, user: uid_t, group: gid_t) -> c_int;
}
extern "C" {
    pub fn vfs_open(: *const path, : *mut file) -> c_int;
}
//
// inode.c
//
extern "C" {
    pub fn prune_icache_sb(sb: *mut super_block, sc: *mut shrink_control) -> c_long;
}
extern "C" {
    pub fn dentry_needs_remove_privs(: *mut mnt_idmap, dentry: *mut dentry) -> c_int;
}
//
// fs-writeback.c
//
extern "C" {
    pub fn get_nr_dirty_inodes() -> c_long;
}
extern "C" {
    pub fn sync_lazytime(inode: *mut inode) -> bool;
}
//
// dcache.c
//
extern "C" {
    pub fn d_set_mounted(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn prune_dcache_sb(sb: *mut super_block, sc: *mut shrink_control) -> c_long;
}
extern "C" {
    pub fn d_alloc_pseudo(: *mut super_block, : *const qstr) -> *mut dentry;
}
extern "C" {
    pub fn dput_to_list(: *mut dentry, : *mut list_head);
}
extern "C" {
    pub fn shrink_dentry_list(: *mut list_head);
}
extern "C" {
    pub fn shrink_dcache_for_umount(: *mut super_block);
}
//
// pipe.c
//
// fs_pin.c
//
extern "C" {
    pub fn group_pin_kill(p: *mut hlist_head);
}
extern "C" {
    pub fn mnt_pin_kill(m: *mut mount);
}
//
// fs/nsfs.c
//
extern "C" {
    pub fn open_namespace(ns: *mut ns_common) -> c_int;
}
//
// fs/stat.c:
//
// fs/splice.c:
//
// fs/xattr.c:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xattr_name {
    pub 1]: char name[XATTR_NAME_MAX +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_xattr_ctx {
// Value of attribute
    pub cvalue: *const void __user,
    pub value: *mut void __user,
}

// Attribute name
extern "C" {
    pub fn file_getxattr(file: *mut file, ctx: *mut kernel_xattr_ctx) -> isize;
}
extern "C" {
    pub fn file_setxattr(file: *mut file, ctx: *mut kernel_xattr_ctx) -> c_int;
}
extern "C" {
    pub fn setxattr_copy(name: *const char __user, ctx: *mut kernel_xattr_ctx) -> c_int;
}
extern "C" {
    pub fn import_xattr_name(kname: *mut xattr_name, name: *const char __user) -> c_int;
}
extern "C" {
    pub fn may_write_xattr(idmap: *mut mnt_idmap, inode: *mut inode) -> c_int;
}

extern "C" {
    pub fn __kernel_write_iter(file: *mut file, from: *mut iov_iter, pos: *mut loff_t) -> isize;
}
//
// fs/attr.c
//
extern "C" {
    pub fn mnt_idmap_put(idmap: *mut mnt_idmap);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stashed_operations {
    pub dentry): *mut dentry,
    pub data): *mut *mut void (put_data)(void,
    pub data): *mut *mut *mut int (init_inode)(struct inode inode, void,
}

extern "C" {
    pub fn stashed_dentry_prune(dentry: *mut dentry);
}
//
// path_mounted - check whether path is mounted
// @path: path to check
//
// Determine whether @path refers to the root of a mount.
//
// Return: true if @path is the root of a mount, false if not.
//
extern "C" {
    pub fn file_f_owner_release(file: *mut file);
}
extern "C" {
    pub fn file_seek_cur_needs_f_lock(file: *mut file) -> bool;
}
extern "C" {
    pub fn statmount_mnt_idmap(idmap: *mut mnt_idmap, seq: *mut seq_file, uid_map: bool) -> c_int;
}
extern "C" {
    pub fn pidfs_get_root(path: *mut path);
}
extern "C" {
    pub fn nsfs_get_root(path: *mut path);
}
extern "C" {
    pub fn failfs_get_root(path: *mut path);
}
extern "C" {
    pub fn failfs_init() -> void __init;
}
extern "C" {
    pub fn failfs_mnt(mnt: *const vfsmount) -> bool;
}
extern "C" {
    pub fn failfs_current_chdir() -> c_int;
}
