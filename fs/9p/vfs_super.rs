//! Automatically rewritten from C to Rust
//! Source: fs/9p/vfs_super.c
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
//
// Copyright (C) 2004 by Eric Van Hensbergen <ericvh@gmail.com>
// Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
//

    static const struct super_operations v9fs_super_ops, v9fs_super_ops_dotl;
#[no_mangle]
unsafe extern "C" fn v9fs_fill_super(sb: *mut super_block) -> c_int {
    static int v9fs_fill_super(struct super_block *sb)
    {
    int ret;
    struct v9fs_session_info *v9ses = v9ses = sb.s_fs_info;
    sb.s_maxbytes = MAX_LFS_FILESIZE;
    sb.s_blocksize_bits = fls(v9ses.maxdata - 1);
    sb.s_blocksize = 1 << sb.s_blocksize_bits;
    sb.s_magic = V9FS_MAGIC;
    if (v9fs_proto_dotl(v9ses)) {
    sb.s_op = &v9fs_super_ops_dotl;
    if (!(v9ses.flags & V9FS_NO_XATTR))
    sb.s_xattr = v9fs_xattr_handlers;
    } else {
    sb.s_op = &v9fs_super_ops;
    sb.s_time_max = U32_MAX;
    }
    sb.s_time_min = 0;
    ret = super_setup_bdi(sb);
    if (ret)
    return ret;
    if (!v9ses.cache) {
    sb.s_bdi.ra_pages = 0;
    sb.s_bdi.io_pages = 0;
    } else {
    sb.s_bdi.ra_pages = v9ses.maxdata >> PAGE_SHIFT;
    sb.s_bdi.io_pages = v9ses.maxdata >> PAGE_SHIFT;
    }
    sb.s_flags |= SB_ACTIVE;

    if ((v9ses.flags & V9FS_ACL_MASK) == V9FS_POSIX_ACL)
    sb.s_flags |= SB_POSIXACL;

    return 0;
    }
//
// v9fs_get_tree - create the mountable root and superblock
// @fc: the filesystem context
//
#[no_mangle]
unsafe extern "C" fn v9fs_get_tree(fc: *mut fs_context) -> c_int {
    static int v9fs_get_tree(struct fs_context *fc)
    {
    struct super_block *sb = core::ptr::null_mut();
    struct inode *inode = core::ptr::null_mut();
    struct dentry *root = core::ptr::null_mut();
    struct v9fs_session_info *v9ses = core::ptr::null_mut();
    struct p9_fid *fid;
    let mut retval: c_int = 0;
    p9_debug(P9_DEBUG_VFS, "\n");
    v9ses = kzalloc_obj(struct v9fs_session_info);
    if (!v9ses)
    return -ENOMEM;
    fid = v9fs_session_init(v9ses, fc);
    if (IS_ERR(fid)) {
    retval = PTR_ERR(fid);
    goto free_session;
    }
    fc.s_fs_info = v9ses;
    sb = sget_fc(fc, core::ptr::null_mut(), set_anon_super_fc);
    if (IS_ERR(sb)) {
    retval = PTR_ERR(sb);
    goto clunk_fid;
    }
    retval = v9fs_fill_super(sb);
    if (retval)
    goto release_sb;
    if (v9ses.cache & (CACHE_META|CACHE_LOOSE)) {
    set_default_d_op(sb, &v9fs_cached_dentry_operations);
    } else {
    set_default_d_op(sb, &v9fs_dentry_operations);
    sb.s_d_flags |= DCACHE_DONTCACHE;
    }
    inode = v9fs_get_new_inode_from_fid(v9ses, fid, sb);
    if (IS_ERR(inode)) {
    retval = PTR_ERR(inode);
    goto release_sb;
    }
    root = d_make_root(inode);
    if (!root) {
    retval = -ENOMEM;
    goto release_sb;
    }
    sb.s_root = root;
    retval = v9fs_get_acl(inode, fid);
    if (retval)
    goto release_sb;
    v9fs_fid_add(root, &fid);
    p9_debug(P9_DEBUG_VFS, " simple set mount, return 0\n");
    fc.root = dget(sb.s_root);
    return 0;
    clunk_fid:
    p9_fid_put(fid);
    v9fs_session_close(v9ses);
    free_session:
    kfree(v9ses);
    return retval;
    release_sb:
//
// we will do the session_close and root dentry release
// in the below call. But we need to clunk fid, because we haven't
// attached the fid to dentry so it won't get clunked
// automatically.
//
    p9_fid_put(fid);
    deactivate_locked_super(sb);
    return retval;
    }
//
// v9fs_kill_super - Kill Superblock
// @s: superblock
//
#[no_mangle]
unsafe extern "C" fn v9fs_kill_super(s: *mut super_block) {
    static void v9fs_kill_super(struct super_block *s)
    {
    struct v9fs_session_info *v9ses = s.s_fs_info;
    p9_debug(P9_DEBUG_VFS, " %p\n", s);
    kill_anon_super(s);
    v9fs_session_cancel(v9ses);
    v9fs_session_close(v9ses);
    kfree(v9ses);
    s.s_fs_info = core::ptr::null_mut();
    p9_debug(P9_DEBUG_VFS, "exiting kill_super\n");
    }
    static void
    v9fs_umount_begin(struct super_block *sb)
    {
    struct v9fs_session_info *v9ses;
    v9ses = sb.s_fs_info;
    v9fs_session_begin_cancel(v9ses);
    }
#[no_mangle]
unsafe extern "C" fn v9fs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int {
    static int v9fs_statfs(struct dentry *dentry, struct kstatfs *buf)
    {
    struct v9fs_session_info *v9ses;
    struct p9_fid *fid;
    struct p9_rstatfs rs;
    int res;
    fid = v9fs_fid_lookup(dentry);
    if (IS_ERR(fid)) {
    res = PTR_ERR(fid);
    goto done;
    }
    v9ses = v9fs_dentry2v9ses(dentry);
    if (v9fs_proto_dotl(v9ses)) {
    res = p9_client_statfs(fid, &rs);
    if (res == 0) {
    buf.f_type = rs.type;
    buf.f_bsize = rs.bsize;
    buf.f_blocks = rs.blocks;
    buf.f_bfree = rs.bfree;
    buf.f_bavail = rs.bavail;
    buf.f_files = rs.files;
    buf.f_ffree = rs.ffree;
    buf.f_fsid = u64_to_fsid(rs.fsid);
    buf.f_namelen = rs.namelen;
    }
    if (res != -ENOSYS)
    goto done;
    }
    res = simple_statfs(dentry, buf);
    done:
    p9_fid_put(fid);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn v9fs_drop_inode(inode: *mut inode) -> c_int {
    static int v9fs_drop_inode(struct inode *inode)
    {
    struct v9fs_session_info *v9ses;
    v9ses = v9fs_inode2v9ses(inode);
    if (v9ses.cache & (CACHE_META|CACHE_LOOSE))
    return inode_generic_drop(inode);
//
// in case of non cached mode always drop the
// inode because we want the inode attribute
// to always match that on the server.
//
    return 1;
    }
    static int v9fs_write_inode(struct inode *inode,
    struct writeback_control *wbc)
    {
//
// send an fsync request to server irrespective of
// wbc->sync_mode.
//
    p9_debug(P9_DEBUG_VFS, "%s: inode %p\n", __func__, inode);
    return netfs_unpin_writeback(inode, wbc);
    }
    static int v9fs_write_inode_dotl(struct inode *inode,
    struct writeback_control *wbc)
    {
    p9_debug(P9_DEBUG_VFS, "%s: inode %p\n", __func__, inode);
    return netfs_unpin_writeback(inode, wbc);
    }
    static const struct super_operations v9fs_super_ops = {
    .alloc_inode = v9fs_alloc_inode,
    .free_inode = v9fs_free_inode,
    .statfs = simple_statfs,
    .drop_inode = v9fs_drop_inode,
    .evict_inode = v9fs_evict_inode,
    .show_options = v9fs_show_options,
    .umount_begin = v9fs_umount_begin,
    .write_inode = v9fs_write_inode,
    };
    static const struct super_operations v9fs_super_ops_dotl = {
    .alloc_inode = v9fs_alloc_inode,
    .free_inode = v9fs_free_inode,
    .statfs = v9fs_statfs,
    .drop_inode = v9fs_drop_inode,
    .evict_inode = v9fs_evict_inode,
    .show_options = v9fs_show_options,
    .umount_begin = v9fs_umount_begin,
    .write_inode = v9fs_write_inode_dotl,
    };
#[no_mangle]
unsafe extern "C" fn v9fs_free_fc(fc: *mut fs_context) {
    static void v9fs_free_fc(struct fs_context *fc)
    {
    struct v9fs_context *ctx = fc.fs_private;
    if (!ctx)
    return;
// These should be NULL by now but guard against leaks
    kfree(ctx.session_opts.uname);
    kfree(ctx.session_opts.aname);

    kfree(ctx.session_opts.cachetag);

    if (ctx.client_opts.trans_mod)
    v9fs_put_trans(ctx.client_opts.trans_mod);
    kfree(ctx);
    }
    static const struct fs_context_operations v9fs_context_ops = {
    .parse_param	= v9fs_parse_param,
    .get_tree	= v9fs_get_tree,
    .free		= v9fs_free_fc,
    };
#[no_mangle]
unsafe extern "C" fn v9fs_init_fs_context(fc: *mut fs_context) -> c_int {
    static int v9fs_init_fs_context(struct fs_context *fc)
    {
    struct v9fs_context	*ctx;
    ctx = kzalloc_obj(*ctx);
    if (!ctx)
    return -ENOMEM;
    fc.ops = &v9fs_context_ops;
    fc.fs_private = ctx;
// initialize core options
    ctx.session_opts.afid = ~0;
    ctx.session_opts.cache = CACHE_NONE;
    ctx.session_opts.session_lock_timeout = P9_LOCK_TIMEOUT;
    ctx.session_opts.uname = kstrdup(V9FS_DEFUSER, GFP_KERNEL);
    if (!ctx.session_opts.uname)
    goto error;
    ctx.session_opts.aname = kstrdup(V9FS_DEFANAME, GFP_KERNEL);
    if (!ctx.session_opts.aname)
    goto error;
    ctx.session_opts.uid = INVALID_UID;
    ctx.session_opts.dfltuid = V9FS_DEFUID;
    ctx.session_opts.dfltgid = V9FS_DEFGID;
    ctx.session_opts.ndentry_timeout_ms = 0;
// initialize client options
    ctx.client_opts.proto_version = p9_proto_2000L;
    ctx.client_opts.msize = DEFAULT_MSIZE;
// initialize fd transport options
    ctx.fd_opts.port = P9_FD_PORT;
    ctx.fd_opts.rfd = ~0;
    ctx.fd_opts.wfd = ~0;
    ctx.fd_opts.privport = false;
// initialize rdma transport options
    ctx.rdma_opts.port = P9_RDMA_PORT;
    ctx.rdma_opts.sq_depth = P9_RDMA_SQ_DEPTH;
    ctx.rdma_opts.rq_depth = P9_RDMA_RQ_DEPTH;
    ctx.rdma_opts.timeout = P9_RDMA_TIMEOUT;
    ctx.rdma_opts.privport = false;
    return 0;
    error:
    fc.need_free = 1;
    return -ENOMEM;
    }
    struct file_system_type v9fs_fs_type = {
    .name = "9p",
    .kill_sb = v9fs_kill_super,
    .owner = THIS_MODULE,
    .fs_flags = FS_RENAME_DOES_D_MOVE,
    .init_fs_context = v9fs_init_fs_context,
    .parameters = v9fs_param_spec,
    };
    MODULE_ALIAS_FS("9p");
