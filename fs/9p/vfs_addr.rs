//! Automatically rewritten from C to Rust
//! Source: fs/9p/vfs_addr.c
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
// This file contians vfs address (mmap) ops for 9P2000.
//
// Copyright (C) 2005 by Eric Van Hensbergen <ericvh@gmail.com>
// Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
//

//
// Writeback calls this when it finds a folio that needs uploading.  This isn't
// called if writeback only has copy-to-cache to deal with.
//
#[no_mangle]
unsafe extern "C" fn v9fs_begin_writeback(wreq: *mut netfs_io_request) {
    static void v9fs_begin_writeback(struct netfs_io_request *wreq)
    {
    struct p9_fid *fid;
    fid = v9fs_fid_find_inode(wreq.inode, true, INVALID_UID, true);
    if (!fid) {
    WARN_ONCE(1, "folio expected an open fid inode.i_ino=%llx\n",
    wreq.inode.i_ino);
    return;
    }
    wreq.wsize = fid.clnt.msize - P9_IOHDRSZ;
    if (fid.iounit)
    wreq.wsize = min(wreq.wsize, fid.iounit);
    wreq.netfs_priv = fid;
    wreq.io_streams[0].avail = true;
    }
//
// Issue a subrequest to write to the server.
//
#[no_mangle]
unsafe extern "C" fn v9fs_issue_write(subreq: *mut netfs_io_subrequest) {
    static void v9fs_issue_write(struct netfs_io_subrequest *subreq)
    {
    struct p9_fid *fid = subreq.rreq.netfs_priv;
    int err, len;
    len = p9_client_write(fid, subreq.start, &subreq.io_iter, &err);
    if (len > 0)
    __set_bit(NETFS_SREQ_MADE_PROGRESS, &subreq.flags);
    netfs_write_subrequest_terminated(subreq, len ?: err);
    }
//
// v9fs_issue_read - Issue a read from 9P
// @subreq: The read to make
//
#[no_mangle]
unsafe extern "C" fn v9fs_issue_read(subreq: *mut netfs_io_subrequest) {
    static void v9fs_issue_read(struct netfs_io_subrequest *subreq)
    {
    struct netfs_io_request *rreq = subreq.rreq;
    struct p9_fid *fid = rreq.netfs_priv;
    char *target;
    let mut pos: c_ulonglong = subreq.start + subreq.transferred;
    let mut total: c_int = 0, err, len, n;
    if (S_ISLNK(rreq.inode.i_mode)) {
// p9_client_readlink() must not be called for legacy protocols
// 9p2000 or 9p2000.u.
//
    BUG_ON(!p9_is_proto_dotl(fid.clnt));
    if (WARN_ON_ONCE(pos)) {
// reading a link at a non null offset should
// not happen
//
    err = -EIO;
    goto fill_subreq;
    }
    err = p9_client_readlink(fid, &target);
    if (err != 0)
    goto fill_subreq;
    len = strlen(target);
    n = copy_to_iter(target, len, &subreq.io_iter);
    kfree(target);
    total = n;
    } else {
    total = p9_client_read(fid, pos, &subreq.io_iter, &err);
    }
    fill_subreq:
// if we just extended the file size, any portion not in
// cache won't be on server and is zeroes
    if (subreq.rreq.origin != NETFS_UNBUFFERED_READ &&
    subreq.rreq.origin != NETFS_DIO_READ)
    __set_bit(NETFS_SREQ_CLEAR_TAIL, &subreq.flags);
    if (pos + total >= i_size_read(rreq.inode))
    __set_bit(NETFS_SREQ_HIT_EOF, &subreq.flags);
    if (!err && total) {
    subreq.transferred += total;
    __set_bit(NETFS_SREQ_MADE_PROGRESS, &subreq.flags);
    }
    subreq.error = err;
    netfs_read_subreq_terminated(subreq);
    }
//
// v9fs_init_request - Initialise a request
// @rreq: The read request
// @file: The file being read from
//
#[no_mangle]
unsafe extern "C" fn v9fs_init_request(rreq: *mut netfs_io_request, file: *mut file) -> c_int {
    static int v9fs_init_request(struct netfs_io_request *rreq, struct file *file)
    {
    struct p9_fid *fid;
    struct dentry *dentry;
    bool writing = (rreq.origin == NETFS_READ_FOR_WRITE ||
    rreq.origin == NETFS_WRITETHROUGH ||
    rreq.origin == NETFS_UNBUFFERED_WRITE ||
    rreq.origin == NETFS_DIO_WRITE);
    if (rreq.origin == NETFS_WRITEBACK)
    return 0; /* We don't get the write handle until we find we
// have actually dirty data and not just
// copy-to-cache data.
//
    if (file) {
    fid = file.private_data;
    if (!fid)
    goto no_fid;
    p9_fid_get(fid);
    } else if (S_ISLNK(rreq.inode.i_mode)) {
    dentry = d_find_any_alias(rreq.inode);
    if (!dentry)
    goto no_fid;
    fid = v9fs_fid_lookup(dentry);
    dput(dentry);
    if (IS_ERR(fid))
    goto no_fid;
    } else {
    fid = v9fs_fid_find_inode(rreq.inode, writing, INVALID_UID, true);
    if (!fid)
    goto no_fid;
    }
    rreq.wsize = fid.clnt.msize - P9_IOHDRSZ;
    if (fid.iounit)
    rreq.wsize = min(rreq.wsize, fid.iounit);
// we might need to read from a fid that was opened write-only
// for read-modify-write of page cache, use the writeback fid
// for that
    WARN_ON(rreq.origin == NETFS_READ_FOR_WRITE && !(fid.mode & P9_ORDWR));
    rreq.netfs_priv = fid;
    return 0;
    no_fid:
    WARN_ONCE(1, "folio expected an open fid inode.i_ino=%llx\n",
    rreq.inode.i_ino);
    return -EINVAL;
    }
//
// v9fs_free_request - Cleanup request initialized by v9fs_init_rreq
// @rreq: The I/O request to clean up
//
#[no_mangle]
unsafe extern "C" fn v9fs_free_request(rreq: *mut netfs_io_request) {
    static void v9fs_free_request(struct netfs_io_request *rreq)
    {
    struct p9_fid *fid = rreq.netfs_priv;
    p9_fid_put(fid);
    }
    const struct netfs_request_ops v9fs_req_ops = {
    .init_request		= v9fs_init_request,
    .free_request		= v9fs_free_request,
    .issue_read		= v9fs_issue_read,
    .begin_writeback	= v9fs_begin_writeback,
    .issue_write		= v9fs_issue_write,
    };
    const struct address_space_operations v9fs_addr_operations = {
    .read_folio		= netfs_read_folio,
    .readahead		= netfs_readahead,
    .dirty_folio		= netfs_dirty_folio,
    .release_folio		= netfs_release_folio,
    .invalidate_folio	= netfs_invalidate_folio,
    .direct_IO		= noop_direct_IO,
    .writepages		= netfs_writepages,
    .migrate_folio		= filemap_migrate_folio,
    };
