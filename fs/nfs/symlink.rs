//! Automatically rewritten from C to Rust
//! Source: fs/nfs/symlink.c
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
// linux/fs/nfs/symlink.c
//
// Copyright (C) 1992  Rick Sladkey
//
// Optimization changes Copyright (C) 1994 Florian La Roche
//
// Jun 7 1999, cache symlink lookups in the page cache.  -DaveM
//
// nfs symlink handling code
//

// Symlink caching in the page cache is even more simplistic
// and straight-forward than readdir caching.
//
#[no_mangle]
unsafe extern "C" fn nfs_symlink_filler(file: *mut file, folio: *mut folio) -> c_int {
    static int nfs_symlink_filler(struct file *file, struct folio *folio)
    {
    struct inode *inode = folio.mapping.host;
    int error;
    error = NFS_PROTO(inode).readlink(inode, &folio.page, 0, PAGE_SIZE);
    folio_end_read(folio, error == 0);
    return error;
    }
    static const char *nfs_get_link(struct dentry *dentry,
    struct inode *inode,
    struct delayed_call *done)
    {
    struct folio *folio;
    void *err;
    if (!dentry) {
    err = ERR_PTR(nfs_revalidate_mapping_rcu(inode));
    if (err)
    return err;
    folio = filemap_get_folio(inode.i_mapping, 0);
    if (IS_ERR(folio))
    return ERR_PTR(-ECHILD);
    if (!folio_test_uptodate(folio)) {
    folio_put(folio);
    return ERR_PTR(-ECHILD);
    }
    } else {
    err = ERR_PTR(nfs_revalidate_mapping(inode, inode.i_mapping));
    if (err)
    return err;
    folio = read_cache_folio(&inode.i_data, 0, nfs_symlink_filler,
    core::ptr::null_mut());
    if (IS_ERR(folio))
    return ERR_CAST(folio);
    }
    set_delayed_call(done, page_put_link, folio);
    return folio_address(folio);
    }
//
// symlinks can't do much...
//
    const struct inode_operations nfs_symlink_inode_operations = {
    .get_link	= nfs_get_link,
    .getattr	= nfs_getattr,
    .setattr	= nfs_setattr,
    .fileattr_get	= nfs_fileattr_get,
    };
