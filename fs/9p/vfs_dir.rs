//! Automatically rewritten from C to Rust
//! Source: fs/9p/vfs_dir.c
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
// This file contains vfs directory ops for the 9P2000 protocol.
//
// Copyright (C) 2004 by Eric Van Hensbergen <ericvh@gmail.com>
// Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
//

//
// struct p9_rdir - readdir accounting
// @head: start offset of current dirread buffer
// @tail: end offset of current dirread buffer
// @offset: file position the data at @head corresponds to
// @buf: dirread buffer
//
// private structure for keeping track of readdir
// allocated on demand
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p9_rdir {
    pub head: c_int,
    pub tail: c_int,
    pub offset: loff_t,
    pub buf: [u8; ],
}

//
// dt_type - return file type
// @mistat: mistat structure
//
#[no_mangle]
pub unsafe extern "C" fn dt_type(mistat: *mut p9_wstat) -> c_int {
    static inline int dt_type(struct p9_wstat *mistat)
    {
    let mut perm: c_ulong = mistat.mode;
    let mut rettype: c_int = DT_REG;
    if (perm & P9_DMDIR)
    rettype = DT_DIR;
    if (perm & P9_DMSYMLINK)
    rettype = DT_LNK;
    return rettype;
    }
//
// v9fs_alloc_rdir_buf - Allocate buffer used for read and readdir
// @filp: opened file structure
// @buflen: Length in bytes of buffer to allocate
//
    static struct p9_rdir *v9fs_alloc_rdir_buf(struct file *filp, int buflen)
    {
    struct p9_fid *fid = filp.private_data;
    if (!fid.rdir)
    fid.rdir = kvzalloc(sizeof(struct p9_rdir) + buflen, GFP_KERNEL);
    return fid.rdir;
    }
//
// v9fs_dir_readdir - iterate through a directory
// @file: opened file structure
// @ctx: actor we feed the entries to
//
#[no_mangle]
unsafe extern "C" fn v9fs_dir_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    static int v9fs_dir_readdir(struct file *file, struct dir_context *ctx)
    {
    bool over;
    struct p9_wstat st;
    let mut err: c_int = 0;
    struct p9_fid *fid;
    int buflen;
    struct p9_rdir *rdir;
    struct kvec kvec;
    p9_debug(P9_DEBUG_VFS, "name %pD\n", file);
    fid = file.private_data;
    buflen = fid.clnt.msize - P9_IOHDRSZ;
    rdir = v9fs_alloc_rdir_buf(file, buflen);
    if (!rdir)
    return -ENOMEM;
    kvec.iov_base = rdir.buf;
    kvec.iov_len = buflen;
    if (rdir.head < rdir.tail && rdir.offset != ctx.pos)
    rdir.head = rdir.tail = 0;
    while (1) {
    if (rdir.tail == rdir.head) {
    struct iov_iter to;
    int n;
    iov_iter_kvec(&to, ITER_DEST, &kvec, 1, buflen);
    n = p9_client_read(file.private_data, ctx.pos, &to,
    &err);
    if (err)
    return err;
    if (n == 0)
    return 0;
    rdir.head = 0;
    rdir.tail = n;
    rdir.offset = ctx.pos;
    }
    while (rdir.head < rdir.tail) {
    err = p9stat_read(fid.clnt, rdir.buf + rdir.head,
    rdir.tail - rdir.head, &st);
    if (err <= 0) {
    p9_debug(P9_DEBUG_VFS, "returned %d\n", err);
    return -EIO;
    }
    over = !dir_emit(ctx, st.name, strlen(st.name),
    QID2INO(&st.qid), dt_type(&st));
    p9stat_free(&st);
    if (over)
    return 0;
    rdir.head += err;
    ctx.pos += err;
    rdir.offset = ctx.pos;
    }
    }
    }
//
// v9fs_dir_readdir_dotl - iterate through a directory
// @file: opened file structure
// @ctx: actor we feed the entries to
//
#[no_mangle]
unsafe extern "C" fn v9fs_dir_readdir_dotl(file: *mut file, ctx: *mut dir_context) -> c_int {
    static int v9fs_dir_readdir_dotl(struct file *file, struct dir_context *ctx)
    {
    let mut err: c_int = 0;
    struct p9_fid *fid;
    int buflen;
    struct p9_rdir *rdir;
    struct p9_dirent curdirent;
    p9_debug(P9_DEBUG_VFS, "name %pD\n", file);
    fid = file.private_data;
    buflen = fid.clnt.msize - P9_READDIRHDRSZ;
    rdir = v9fs_alloc_rdir_buf(file, buflen);
    if (!rdir)
    return -ENOMEM;
    if (rdir.head < rdir.tail && rdir.offset != ctx.pos)
    rdir.head = rdir.tail = 0;
    while (1) {
    if (rdir.tail == rdir.head) {
    err = p9_client_readdir(fid, rdir.buf, buflen,
    ctx.pos);
    if (err <= 0)
    return err;
    rdir.head = 0;
    rdir.tail = err;
    rdir.offset = ctx.pos;
    }
    while (rdir.head < rdir.tail) {
    err = p9dirent_read(fid.clnt, rdir.buf + rdir.head,
    rdir.tail - rdir.head,
    &curdirent);
    if (err < 0) {
    p9_debug(P9_DEBUG_VFS, "returned %d\n", err);
    return -EIO;
    }
    if (!dir_emit(ctx, curdirent.d_name,
    strlen(curdirent.d_name),
    QID2INO(&curdirent.qid),
    curdirent.d_type))
    return 0;
    ctx.pos = curdirent.d_off;
    rdir.head += err;
    rdir.offset = ctx.pos;
    }
    }
    }
//
// v9fs_dir_release - close a directory or a file
// @inode: inode of the directory or file
// @filp: file pointer to a directory or file
//
#[no_mangle]
pub unsafe extern "C" fn v9fs_dir_release(inode: *mut inode, filp: *mut file) -> c_int {
    int v9fs_dir_release(struct inode *inode, struct file *filp)
    {
    struct v9fs_inode *v9inode = V9FS_I(inode);
    struct p9_fid *fid;
    __le32 version;
    loff_t i_size;
    let mut retval: c_int = 0, put_err;
    fid = filp.private_data;
    p9_debug(P9_DEBUG_VFS, "inode: %p filp: %p fid: %d\n",
    inode, filp, fid ? fid.fid : -1);
    if (fid) {
    if ((S_ISREG(inode.i_mode)) && (filp.f_mode & FMODE_WRITE))
    retval = filemap_fdatawrite(inode.i_mapping);
    spin_lock(&inode.i_lock);
    hlist_del(&fid.ilist);
    spin_unlock(&inode.i_lock);
    put_err = p9_fid_put(fid);
    retval = retval < 0 ? retval : put_err;
    }
    if ((filp.f_mode & FMODE_WRITE)) {
    version = cpu_to_le32(v9inode.qid.version);
    i_size = i_size_read(inode);
    fscache_unuse_cookie(v9fs_inode_cookie(v9inode),
    &version, &i_size);
    } else {
    fscache_unuse_cookie(v9fs_inode_cookie(v9inode), core::ptr::null_mut(), core::ptr::null_mut());
    }
    return retval;
    }
    const struct file_operations v9fs_dir_operations = {
    .read = generic_read_dir,
    .llseek = generic_file_llseek,
    .iterate_shared = v9fs_dir_readdir,
    .open = v9fs_file_open,
    .release = v9fs_dir_release,
    };
    const struct file_operations v9fs_dir_operations_dotl = {
    .read = generic_read_dir,
    .llseek = generic_file_llseek,
    .iterate_shared = v9fs_dir_readdir_dotl,
    .open = v9fs_file_open,
    .release = v9fs_dir_release,
    .fsync = v9fs_file_fsync_dotl,
    };
