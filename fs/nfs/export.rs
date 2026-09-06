//! Automatically rewritten from C to Rust
//! Source: fs/nfs/export.c
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
// Copyright (c) 2015, Primary Data, Inc. All rights reserved.
//
// Tao Peng <bergwolf@primarydata.com>
//

    enum {
    FILEID_HIGH_OFF = 0,	/* inode fileid high */
    FILEID_LOW_OFF,		/* inode fileid low */
    FILE_I_TYPE_OFF,	/* inode type */
    EMBED_FH_OFF		/* embeded server fh */
    };
    static struct nfs_fh *nfs_exp_embedfh(__u32 *p)
    {
    return (struct nfs_fh *)(p + EMBED_FH_OFF);
    }
//
// Let's break subtree checking for now... otherwise we'll have to embed parent fh
// but there might not be enough space.
//
    static int
    nfs_encode_fh(struct inode *inode, __u32 *p, int *max_len, struct inode *parent)
    {
    struct nfs_fh *server_fh = NFS_FH(inode);
    struct nfs_fh *clnt_fh = nfs_exp_embedfh(p);
    let mut fh_size: usize = offsetof(struct nfs_fh, data) + server_fh.size;
    let mut len: c_int = EMBED_FH_OFF + XDR_QUADLEN(fh_size);
    dprintk("%s: max fh len %d inode %p parent %p",
    __func__, *max_len, inode, parent);
    if (*max_len < len) {
    dprintk("%s: fh len %d too small, required %d\n",
    __func__, *max_len, len);
// max_len = len;
    return FILEID_INVALID;
    }
    p[FILEID_HIGH_OFF] = inode.i_ino >> 32;
    p[FILEID_LOW_OFF] = inode.i_ino;
    p[FILE_I_TYPE_OFF] = inode.i_mode & S_IFMT;
    p[len - 1] = 0; /* Padding */
    nfs_copy_fh(clnt_fh, server_fh);
// max_len = len;
    dprintk("%s: result fh fileid %llu mode %u size %d\n",
    __func__, inode.i_ino, inode.i_mode, *max_len);
    return *max_len;
    }
    static struct dentry *
    nfs_fh_to_dentry(struct super_block *sb, struct fid *fid,
    int fh_len, int fh_type)
    {
    struct nfs_fattr *fattr = core::ptr::null_mut();
    struct nfs_fh *server_fh = nfs_exp_embedfh(fid.raw);
    let mut fh_size: usize = offsetof(struct nfs_fh, data);
    const struct nfs_rpc_ops *rpc_ops;
    struct dentry *dentry;
    struct inode *inode;
    let mut len: c_int = EMBED_FH_OFF;
    u32 *p = fid.raw;
    int ret;
// Initial check of bounds
    if (fh_len < len + XDR_QUADLEN(fh_size) ||
    fh_len > XDR_QUADLEN(NFS_MAXFHSIZE))
    return core::ptr::null_mut();
// Calculate embedded filehandle size
    fh_size += server_fh.size;
    len += XDR_QUADLEN(fh_size);
// NULL translates to ESTALE
    if (fh_len < len || fh_type != len)
    return core::ptr::null_mut();
    fattr = nfs_alloc_fattr_with_label(NFS_SB(sb));
    if (fattr == core::ptr::null_mut()) {
    dentry = ERR_PTR(-ENOMEM);
    goto out;
    }
    fattr.fileid = ((u64)p[FILEID_HIGH_OFF] << 32) + p[FILEID_LOW_OFF];
    fattr.mode = p[FILE_I_TYPE_OFF];
    fattr.valid |= NFS_ATTR_FATTR_FILEID | NFS_ATTR_FATTR_TYPE;
    dprintk("%s: fileid %llu mode %d\n", __func__, fattr.fileid, fattr.mode);
    inode = nfs_ilookup(sb, fattr, server_fh);
    if (inode)
    goto out_found;
    rpc_ops = NFS_SB(sb).nfs_client.rpc_ops;
    ret = rpc_ops.getattr(NFS_SB(sb), server_fh, fattr, core::ptr::null_mut());
    if (ret) {
    dprintk("%s: getattr failed %d\n", __func__, ret);
    trace_nfs_fh_to_dentry(sb, server_fh, fattr.fileid, ret);
    dentry = ERR_PTR(ret);
    goto out_free_fattr;
    }
    inode = nfs_fhget(sb, server_fh, fattr);
    out_found:
    dentry = d_obtain_alias(inode);
    out_free_fattr:
    nfs_free_fattr(fattr);
    out:
    return dentry;
    }
    static struct dentry *
    nfs_get_parent(struct dentry *dentry)
    {
    int ret;
    struct inode *inode = d_inode(dentry), *pinode;
    struct super_block *sb = inode.i_sb;
    struct nfs_server *server = NFS_SB(sb);
    struct nfs_fattr *fattr = core::ptr::null_mut();
    struct dentry *parent;
    struct nfs_rpc_ops const *ops = server.nfs_client.rpc_ops;
    struct nfs_fh fh;
    if (!ops.lookupp)
    return ERR_PTR(-EACCES);
    fattr = nfs_alloc_fattr_with_label(server);
    if (fattr == core::ptr::null_mut())
    return ERR_PTR(-ENOMEM);
    ret = ops.lookupp(inode, &fh, fattr);
    if (ret) {
    parent = ERR_PTR(ret);
    goto out;
    }
    pinode = nfs_fhget(sb, &fh, fattr);
    parent = d_obtain_alias(pinode);
    out:
    nfs_free_fattr(fattr);
    return parent;
    }
    const struct export_operations nfs_export_ops = {
    .encode_fh = nfs_encode_fh,
    .fh_to_dentry = nfs_fh_to_dentry,
    .get_parent = nfs_get_parent,
    .flags = EXPORT_OP_NOWCC		|
    EXPORT_OP_NOSUBTREECHK		|
    EXPORT_OP_CLOSE_BEFORE_UNLINK	|
    EXPORT_OP_REMOTE_FS		|
    EXPORT_OP_NOATOMIC_ATTR	|
    EXPORT_OP_FLUSH_ON_CLOSE	|
    EXPORT_OP_NOLOCKS,
    };
