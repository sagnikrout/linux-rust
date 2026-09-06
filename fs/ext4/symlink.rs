//! Automatically rewritten from C to Rust
//! Source: fs/ext4/symlink.c
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
// linux/fs/ext4/symlink.c
//
// Only fast symlinks left here - the rest is done by generic code. AV, 1999
//
// Copyright (C) 1992, 1993, 1994, 1995
// Remy Card (card@masi.ibp.fr)
// Laboratoire MASI - Institut Blaise Pascal
// Universite Pierre et Marie Curie (Paris VI)
//
// from
//
// linux/fs/minix/symlink.c
//
// Copyright (C) 1991, 1992  Linus Torvalds
//
// ext4 symlink handling code
//

    static const char *ext4_encrypted_get_link(struct dentry *dentry,
    struct inode *inode,
    struct delayed_call *done)
    {
    struct buffer_head *bh = core::ptr::null_mut();
    const void *caddr;
    unsigned int max_size;
    const char *paddr;
    if (!dentry)
    return ERR_PTR(-ECHILD);
    if (ext4_inode_is_fast_symlink(inode)) {
    caddr = EXT4_I(inode).i_data;
    max_size = sizeof(EXT4_I(inode).i_data);
    } else {
    bh = ext4_bread(core::ptr::null_mut(), inode, 0, 0);
    if (IS_ERR(bh))
    return ERR_CAST(bh);
    if (!bh) {
    EXT4_ERROR_INODE(inode, "bad symlink.");
    return ERR_PTR(-EFSCORRUPTED);
    }
    caddr = bh.b_data;
    max_size = inode.i_sb.s_blocksize;
    }
    paddr = fscrypt_get_symlink(inode, caddr, max_size, done);
    brelse(bh);
    return paddr;
    }
    static int ext4_encrypted_symlink_getattr(struct mnt_idmap *idmap,
    const struct path *path,
    struct kstat *stat, u32 request_mask,
    unsigned int query_flags)
    {
    ext4_getattr(idmap, path, stat, request_mask, query_flags);
    return fscrypt_symlink_getattr(path, stat);
    }
#[no_mangle]
unsafe extern "C" fn ext4_free_link(bh: *mut c_void) {
    static void ext4_free_link(void *bh)
    {
    brelse(bh);
    }
    static const char *ext4_get_link(struct dentry *dentry, struct inode *inode,
    struct delayed_call *callback)
    {
    struct buffer_head *bh;
    char *inline_link;
//
// Create a new inlined symlink is not supported, just provide a
// method to read the leftovers.
//
    if (ext4_has_inline_data(inode)) {
    if (!dentry)
    return ERR_PTR(-ECHILD);
    inline_link = ext4_read_inline_link(inode);
    if (!IS_ERR(inline_link))
    set_delayed_call(callback, kfree_link, inline_link);
    return inline_link;
    }
    if (!dentry) {
    bh = ext4_getblk(core::ptr::null_mut(), inode, 0, EXT4_GET_BLOCKS_CACHED_NOWAIT);
    if (IS_ERR_OR_NULL(bh))
    return ERR_PTR(-ECHILD);
    if (!ext4_buffer_uptodate(bh)) {
    brelse(bh);
    return ERR_PTR(-ECHILD);
    }
    } else {
    bh = ext4_bread(core::ptr::null_mut(), inode, 0, 0);
    if (IS_ERR(bh))
    return ERR_CAST(bh);
    if (!bh) {
    EXT4_ERROR_INODE(inode, "bad symlink.");
    return ERR_PTR(-EFSCORRUPTED);
    }
    }
    set_delayed_call(callback, ext4_free_link, bh);
    nd_terminate_link(bh.b_data, inode.i_size,
    inode.i_sb.s_blocksize - 1);
    return bh.b_data;
    }
    const struct inode_operations ext4_encrypted_symlink_inode_operations = {
    .get_link	= ext4_encrypted_get_link,
    .setattr	= ext4_setattr,
    .getattr	= ext4_encrypted_symlink_getattr,
    .listxattr	= ext4_listxattr,
    };
    const struct inode_operations ext4_symlink_inode_operations = {
    .get_link	= ext4_get_link,
    .setattr	= ext4_setattr,
    .getattr	= ext4_getattr,
    .listxattr	= ext4_listxattr,
    };
    const struct inode_operations ext4_fast_symlink_inode_operations = {
    .get_link	= simple_get_link,
    .setattr	= ext4_setattr,
    .getattr	= ext4_getattr,
    .listxattr	= ext4_listxattr,
    };
