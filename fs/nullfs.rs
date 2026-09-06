//! Automatically rewritten from C to Rust
//! Source: fs/nullfs.c
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

    static const struct super_operations nullfs_super_operations = {
    .statfs	= simple_statfs,
    };
#[no_mangle]
unsafe extern "C" fn nullfs_fs_fill_super(s: *mut super_block, fc: *mut fs_context) -> c_int {
    static int nullfs_fs_fill_super(struct super_block *s, struct fs_context *fc)
    {
    struct inode *inode;
    s.s_maxbytes		= MAX_LFS_FILESIZE;
    s.s_blocksize		= PAGE_SIZE;
    s.s_blocksize_bits	= PAGE_SHIFT;
    s.s_magic		= NULL_FS_MAGIC;
    s.s_op			= &nullfs_super_operations;
    s.s_export_op		= core::ptr::null_mut();
    s.s_xattr		= core::ptr::null_mut();
    s.s_time_gran		= 1;
    s.s_d_flags		= 0;
    inode = new_inode(s);
    if (!inode)
    return -ENOMEM;
// nullfs is permanently empty...
    make_empty_dir_inode(inode);
    simple_inode_init_ts(inode);
    inode.i_ino	= 1;
// ... and immutable.
    inode.i_flags |= S_IMMUTABLE;
    s.s_root = d_make_root(inode);
    if (!s.s_root)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nullfs_fs_get_tree(fc: *mut fs_context) -> c_int {
    static int nullfs_fs_get_tree(struct fs_context *fc)
    {
    return get_tree_nodev(fc, nullfs_fs_fill_super);
    }
    static const struct fs_context_operations nullfs_fs_context_ops = {
    .get_tree	= nullfs_fs_get_tree,
    };
#[no_mangle]
unsafe extern "C" fn nullfs_init_fs_context(fc: *mut fs_context) -> c_int {
    static int nullfs_init_fs_context(struct fs_context *fc)
    {
    fc.ops		= &nullfs_fs_context_ops;
    fc.sb_flags	|= SB_NOUSER;
    fc.s_iflags	|= SB_I_NOEXEC | SB_I_NODEV;
    return 0;
    }
    struct file_system_type nullfs_fs_type = {
    .name			= "nullfs",
    .init_fs_context	= nullfs_init_fs_context,
    .kill_sb		= kill_anon_super,
    };
