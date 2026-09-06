//! Automatically rewritten from C to Rust
//! Source: fs/failfs.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2026 Christian Brauner <brauner@kernel.org>

    let mut failfs_root_path: static struct path = {};
#[no_mangle]
pub unsafe extern "C" fn failfs_get_root(path: *mut path) {
    void failfs_get_root(struct path *path)
    {
// path = failfs_root_path;
    path_get(path);
    }
#[no_mangle]
pub unsafe extern "C" fn failfs_mnt(mnt: *const vfsmount) -> bool {
    bool failfs_mnt(const struct vfsmount *mnt)
    {
    return mnt.mnt_sb == failfs_root_path.mnt.mnt_sb;
    }
    static int failfs_permission(struct mnt_idmap *idmap, struct inode *inode,
    int mask)
    {
    return -EOPNOTSUPP;
    }
    static struct dentry *failfs_lookup(struct inode *dir, struct dentry *dentry,
    unsigned int flags)
    {
// Unreachable: ->permission() already failed the walk.
    return ERR_PTR(-EOPNOTSUPP);
    }
    static int failfs_getattr(struct mnt_idmap *idmap, const struct path *path,
    struct kstat *stat, u32 request_mask,
    unsigned int query_flags)
    {
    return -EOPNOTSUPP;
    }
    static const struct inode_operations failfs_dir_inode_operations = {
    .permission	= failfs_permission,
    .lookup		= failfs_lookup,
    .getattr	= failfs_getattr,
    };
    let mut failfs_dir_operations: static struct file_operations = {};
#[no_mangle]
unsafe extern "C" fn failfs_d_weak_revalidate(dentry: *mut dentry, flags: c_uint) -> c_int {
    static int failfs_d_weak_revalidate(struct dentry *dentry, unsigned int flags)
    {
//
// The root is only ever reached as a path-walk terminal by jumping
// to it: as "/" when it is the caller's root, or through a
// /proc/<pid>/{root,cwd} magic link. ->permission() already fails
// every walk of a component, but a jump lands on the root without
// one. Refuse here too so the root cannot be pinned by an O_PATH
// open or encoded into a file handle.
//
    return -EOPNOTSUPP;
    }
    static char *failfs_dname(struct dentry *dentry, char *buffer, int buflen)
    {
    return dynamic_dname(buffer, buflen, "failfs:/");
    }
    static const struct dentry_operations failfs_dentry_operations = {
    .d_dname		= failfs_dname,
    .d_weak_revalidate	= failfs_d_weak_revalidate,
    };
#[no_mangle]
unsafe extern "C" fn failfs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int {
    static int failfs_statfs(struct dentry *dentry, struct kstatfs *buf)
    {
    return -EOPNOTSUPP;
    }
    static const struct super_operations failfs_super_operations = {
    .statfs	= failfs_statfs,
    };
#[no_mangle]
unsafe extern "C" fn failfs_fill_super(s: *mut super_block, fc: *mut fs_context) -> c_int {
    static int failfs_fill_super(struct super_block *s, struct fs_context *fc)
    {
    struct inode *inode;
    s.s_maxbytes		= MAX_LFS_FILESIZE;
    s.s_blocksize		= PAGE_SIZE;
    s.s_blocksize_bits	= PAGE_SHIFT;
    s.s_magic		= FAIL_FS_MAGIC;
    s.s_op			= &failfs_super_operations;
    s.s_export_op		= core::ptr::null_mut();
    s.s_xattr		= core::ptr::null_mut();
    s.s_time_gran		= 1;
    s.s_d_flags		= 0;
    inode = new_inode(s);
    if (!inode)
    return -ENOMEM;
// failfs supports no operations...
    inode.i_mode	= S_IFDIR;
    set_nlink(inode, 2);
    inode.i_op	= &failfs_dir_inode_operations;
    inode.i_fop	= &failfs_dir_operations;
    simple_inode_init_ts(inode);
    inode.i_ino	= 1;
// ... and is immutable.
    inode.i_flags |= S_IMMUTABLE;
    set_default_d_op(s, &failfs_dentry_operations);
    s.s_root = d_make_root(inode);
    if (!s.s_root)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn failfs_get_tree(fc: *mut fs_context) -> c_int {
    static int failfs_get_tree(struct fs_context *fc)
    {
    return get_tree_single(fc, failfs_fill_super);
    }
    static const struct fs_context_operations failfs_context_ops = {
    .get_tree	= failfs_get_tree,
    };
#[no_mangle]
unsafe extern "C" fn failfs_init_fs_context(fc: *mut fs_context) -> c_int {
    static int failfs_init_fs_context(struct fs_context *fc)
    {
    fc.ops		= &failfs_context_ops;
    fc.global	= true;
    fc.sb_flags	|= SB_NOUSER;
    fc.s_iflags	|= SB_I_NOEXEC | SB_I_NODEV;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn failfs_current_chdir() -> c_int {
    int failfs_current_chdir(void)
    {
    struct path path;
    failfs_get_root(&path);
    set_fs_pwd(current.fs, &path);
    path_put(&path);
    return 0;
    }
    static struct file_system_type failfs_fs_type = {
    .name			= "failfs",
    .init_fs_context	= failfs_init_fs_context,
    .kill_sb		= kill_anon_super,
    };
#[no_mangle]
pub unsafe extern "C" fn failfs_init() -> void __init {
    void __init failfs_init(void)
    {
    struct vfsmount *mnt;
// A single instance that is member of no mount namespace.
    mnt = kern_mount(&failfs_fs_type);
    if (IS_ERR(mnt))
    panic("VFS: Failed to create failfs");
    failfs_root_path.mnt	= mnt;
    failfs_root_path.dentry	= mnt.mnt_root;
    }
