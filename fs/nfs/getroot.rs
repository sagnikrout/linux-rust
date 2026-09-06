//! Automatically rewritten from C to Rust
//! Source: fs/nfs/getroot.c
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
// getroot.c: get the root dentry for an NFS mount
//
// Copyright (C) 2006 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// get a root dentry from the root filehandle
//
#[no_mangle]
pub unsafe extern "C" fn nfs_get_root(s: *mut super_block, fc: *mut fs_context) -> c_int {
    int nfs_get_root(struct super_block *s, struct fs_context *fc)
    {
    struct nfs_fs_context *ctx = nfs_fc2context(fc);
    struct nfs_server *server = NFS_SB(s), *clone_server;
    struct nfs_fsinfo fsinfo;
    struct dentry *root;
    struct inode *inode;
    char *name;
    let mut error: c_int = -ENOMEM;
    let mut kflags: c_ulong = 0, kflags_out = 0;
    name = kstrdup(fc.source, GFP_KERNEL);
    if (!name)
    goto out;
// get the actual root for this mount
    fsinfo.fattr = nfs_alloc_fattr_with_label(server);
    if (fsinfo.fattr == core::ptr::null_mut())
    goto out_name;
    error = server.nfs_client.rpc_ops.getroot(server, ctx.mntfh, &fsinfo);
    if (error < 0) {
    dprintk("nfs_get_root: getattr error = %d\n", -error);
    nfs_errorf(fc, "NFS: Couldn't getattr on root");
    goto out_fattr;
    }
    inode = nfs_fhget(s, ctx.mntfh, fsinfo.fattr);
    if (IS_ERR(inode)) {
    dprintk("nfs_get_root: get root inode failed\n");
    error = PTR_ERR(inode);
    nfs_errorf(fc, "NFS: Couldn't get root inode");
    goto out_fattr;
    }
// root dentries normally start off anonymous and get spliced in later
// if the dentry tree reaches them; however if the dentry already
// exists, we'll pick it up at this point and use it as the root
//
    root = d_obtain_root(inode);
    if (IS_ERR(root)) {
    dprintk("nfs_get_root: get root dentry failed\n");
    error = PTR_ERR(root);
    nfs_errorf(fc, "NFS: Couldn't get root dentry");
    goto out_fattr;
    }
    security_d_instantiate(root, inode);
    spin_lock(&root.d_lock);
    if (IS_ROOT(root) && !root.d_fsdata &&
    !(root.d_flags & DCACHE_NFSFS_RENAMED)) {
    root.d_fsdata = name;
    name = core::ptr::null_mut();
    }
    spin_unlock(&root.d_lock);
    if (!s.s_root)
    s.s_root = dget(root);
    fc.root = root;
    if (server.caps & NFS_CAP_SECURITY_LABEL)
    kflags |= SECURITY_LSM_NATIVE_LABELS;
    if (ctx.clone_data.sb) {
    if (d_inode(fc.root).i_fop != &nfs_dir_operations) {
    error = -ESTALE;
    goto error_splat_root;
    }
// clone lsm security options from the parent to the new sb
    error = security_sb_clone_mnt_opts(ctx.clone_data.sb,
    s, kflags, &kflags_out);
    if (error)
    goto error_splat_root;
    clone_server = NFS_SB(ctx.clone_data.sb);
    server.has_sec_mnt_opts = clone_server.has_sec_mnt_opts;
    } else {
    error = security_sb_set_mnt_opts(s, fc.security,
    kflags, &kflags_out);
    }
    if (error)
    goto error_splat_root;
    if (server.caps & NFS_CAP_SECURITY_LABEL &&
    !(kflags_out & SECURITY_LSM_NATIVE_LABELS))
    server.caps &= ~NFS_CAP_SECURITY_LABEL;
    nfs_setsecurity(inode, fsinfo.fattr);
    error = 0;
    out_fattr:
    nfs_free_fattr(fsinfo.fattr);
    out_name:
    kfree(name);
    out:
    return error;
    error_splat_root:
    dput(fc.root);
    fc.root = core::ptr::null_mut();
    goto out_fattr;
    }
